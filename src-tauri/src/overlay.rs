//! Break overlay: one borderless, always-on-top window per display. It can't be closed during a forced break; otherwise closing it skips the break.

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Monitor, PhysicalPosition, PhysicalSize,
    Position, Size, WebviewUrl, WebviewWindowBuilder, Window,
};

const PREFIX: &str = "overlay-";
/// The overlay on the primary screen, focused when opened
pub const PRIMARY_LABEL: &str = "overlay-primary";

/// Display area, in the same units as system window coordinates (see unit_scale)
#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    fn of_monitor(monitor: &Monitor) -> Self {
        let s = unit_scale(monitor.scale_factor());
        let (pos, size) = (monitor.position(), monitor.size());
        Rect {
            x: pos.x as f64 / s,
            y: pos.y as f64 / s,
            w: size.width as f64 / s,
            h: size.height as f64 / s,
        }
    }
}

/// Ratio from physical pixels to window coordinate units. Windows/Linux use physical pixels to avoid misplacement across monitors with different DPI;
/// macOS global coordinates are in logical points, and physical pixels have no common coordinate space when mixing Retina and regular external displays, so logical points are used
fn unit_scale(scale_factor: f64) -> f64 {
    if cfg!(target_os = "macos") {
        scale_factor
    } else {
        1.0
    }
}

fn position(x: f64, y: f64) -> Position {
    if cfg!(target_os = "macos") {
        LogicalPosition::new(x, y).into()
    } else {
        PhysicalPosition::new(x.round() as i32, y.round() as i32).into()
    }
}

fn size(w: f64, h: f64) -> Size {
    if cfg!(target_os = "macos") {
        LogicalSize::new(w, h).into()
    } else {
        PhysicalSize::new(w.round() as u32, h.round() as u32).into()
    }
}

/// Overlay layout, used to constrain positions after a window moves
#[derive(Default)]
struct Layout {
    /// The display each overlay belongs to. Even when draggable it must stay on that screen
    homes: HashMap<String, Rect>,
    /// Current coverage fraction; when not draggable, windows are pinned to the centered position it implies
    coverage: f64,
    movable: bool,
}

static LAYOUT: LazyLock<Mutex<Layout>> = LazyLock::new(Default::default);

pub fn is_overlay(label: &str) -> bool {
    label.starts_with(PREFIX)
}

/// Opens overlays on all displays. When `forced` is false, windows can be closed with Alt+F4 etc. (closing skips the break);
/// when `movable` is false, windows stay centered and are pulled back if dragged or moved with shortcuts.
/// On Windows, creating windows in an event callback or synchronous command deadlocks, so callers must use a separate thread.
pub fn open_all(app: &AppHandle, coverage: f64, forced: bool, movable: bool) {
    let monitors = match app.available_monitors() {
        Ok(m) if !m.is_empty() => m,
        Ok(_) => {
            eprintln!("No display detected, cannot open the overlay");
            return;
        }
        Err(e) => {
            eprintln!("Failed to get displays: {e}");
            return;
        }
    };
    let primary_pos = app
        .primary_monitor()
        .ok()
        .flatten()
        .map(|m| *m.position())
        .unwrap_or(*monitors[0].position());
    {
        let mut layout = LAYOUT.lock().unwrap();
        layout.coverage = coverage;
        layout.movable = movable;
    }

    for (i, monitor) in monitors.iter().enumerate() {
        let is_primary = *monitor.position() == primary_pos;
        let label = if is_primary {
            PRIMARY_LABEL.to_string()
        } else {
            format!("{PREFIX}{i}")
        };
        if app.get_webview_window(&label).is_some() {
            continue;
        }

        let home = Rect::of_monitor(monitor);
        // Register before creating windows so Moved events fired during creation are constrained too
        LAYOUT.lock().unwrap().homes.insert(label.clone(), home);

        let built = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("overlay".into()))
            .title("RestGuard")
            .decorations(false)
            .always_on_top(true)
            .visible_on_all_workspaces(true)
            .skip_taskbar(true)
            .resizable(false)
            .minimizable(false)
            .maximizable(false)
            .closable(!forced)
            .visible(false)
            .build();

        match built {
            Ok(window) => {
                #[cfg(target_os = "macos")]
                cover_everything(&window);
                let area = cover_rect(home, coverage);
                let _ = window.set_position(position(area.x, area.y));
                let _ = window.set_size(size(area.w, area.h));
                let _ = window.show();
                if is_primary {
                    let _ = window.set_focus();
                }
            }
            Err(e) => {
                LAYOUT.lock().unwrap().homes.remove(&label);
                eprintln!("Failed to create overlay window {label}: {e}");
            }
        }
    }
}

/// On macOS, always_on_top is only the floating window level, which doesn't cover the menu bar or Dock or enter other apps' full-screen spaces.
/// Raise it to the screen saver level and make the overlay appear in all spaces (including full-screen apps)
#[cfg(target_os = "macos")]
fn cover_everything(window: &tauri::WebviewWindow) {
    use objc2_app_kit::{NSWindow, NSWindowCollectionBehavior};
    /// kCGScreenSaverWindowLevel
    const SCREEN_SAVER_LEVEL: isize = 1000;

    let w = window.clone();
    // AppKit can only be called on the main thread
    let _ = window.run_on_main_thread(move || {
        let Ok(ptr) = w.ns_window() else {
            return;
        };
        #[allow(unused_unsafe)]
        unsafe {
            let ns_window = &*ptr.cast::<NSWindow>();
            ns_window.setLevel(SCREEN_SAVER_LEVEL);
            ns_window.setCollectionBehavior(
                NSWindowCollectionBehavior::CanJoinAllSpaces
                    | NSWindowCollectionBehavior::Stationary
                    | NSWindowCollectionBehavior::FullScreenAuxiliary
                    | NSWindowCollectionBehavior::IgnoresCycle,
            );
        }
    });
}

pub fn close_all(app: &AppHandle) {
    for (label, window) in app.webview_windows() {
        if is_overlay(&label) {
            // destroy doesn't fire CloseRequested, bypassing the close interception
            let _ = window.destroy();
        }
    }
    LAYOUT.lock().unwrap().homes.clear();
}

/// Scales all overlays to a fraction (still centered and on top). Urgent mode passes a smaller fraction, exposing the edges for saving files etc.;
/// leaving it passes the normal fraction back, restoring the original size and centering
pub fn set_coverage(app: &AppHandle, coverage: f64) {
    // Update the fraction before moving windows so the resulting Moved events are constrained to the new position
    let homes = {
        let mut layout = LAYOUT.lock().unwrap();
        layout.coverage = coverage;
        layout.homes.clone()
    };
    for (label, window) in app.webview_windows() {
        if let Some(&home) = homes.get(&label) {
            let area = cover_rect(home, coverage);
            let _ = window.set_size(size(area.w, area.h));
            let _ = window.set_position(position(area.x, area.y));
        }
    }
}

/// Called after a window moves: if it was dragged (or moved with Win+Shift+arrow) off its screen, pulls it back;
/// when not draggable, pulls it back to the center
pub fn keep_on_home(window: &Window, pos: PhysicalPosition<i32>) {
    let (home, coverage, movable) = {
        let layout = LAYOUT.lock().unwrap();
        let Some(&home) = layout.homes.get(window.label()) else {
            return;
        };
        (home, layout.coverage, layout.movable)
    };
    let (Ok(win_size), Ok(scale)) = (window.outer_size(), window.scale_factor()) else {
        return;
    };
    let s = unit_scale(scale);
    let (x, y) = (pos.x as f64 / s, pos.y as f64 / s);
    let (fx, fy) = if movable {
        let (w, h) = (win_size.width as f64 / s, win_size.height as f64 / s);
        // Stick to the left/top edge when the window is larger than the screen
        let clamp = |v: f64, origin: f64, screen: f64, len: f64| {
            v.clamp(origin, origin + (screen - len).max(0.0))
        };
        (clamp(x, home.x, home.w, w), clamp(y, home.y, home.h, h))
    } else {
        let area = cover_rect(home, coverage);
        (area.x, area.y)
    };
    // Ignore rounding errors to avoid repeatedly triggering Moved
    if (fx - x).abs() >= 0.5 || (fy - y).abs() >= 0.5 {
        let _ = window.set_position(position(fx, fy));
    }
}

/// Computes the centered area covering the given fraction
fn cover_rect(home: Rect, coverage: f64) -> Rect {
    let (w, h) = (home.w * coverage, home.h * coverage);
    Rect {
        x: home.x + (home.w - w) / 2.0,
        y: home.y + (home.h - h) / 2.0,
        w,
        h,
    }
}
