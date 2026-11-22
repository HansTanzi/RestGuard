//! Launch dialog: one small window centered on the display under the mouse (that's where the user just launched
//! RestGuard from), saying where the tray icon is. Shown in two cases (see Kind):
//! - "Started": RestGuard was launched by hand (not by autostart), as otherwise only a tray icon appears and it
//!   looks like nothing happened
//! - "Already running": the running instance shows it when RestGuard is launched again
//!
//! It loads the frontend /already-running route (src/routes/already-running/+page.svelte).

use crate::overlay;
use serde::Serialize;
use std::{sync::Mutex, thread};
use tauri::{AppHandle, Emitter, Manager, Monitor, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const LABEL: &str = "already-running";
/// Payload is the new Kind; matches KIND_EVENT in src/routes/already-running/+page.svelte
const KIND_EVENT: &str = "already-running:kind";
/// Logical size of the dialog; the frontend asks for more height when its content needs it (see fit)
const WIDTH: f64 = 420.0;
const HEIGHT: f64 = 220.0;

/// Which message the dialog shows; matches LaunchKind in src/routes/already-running/+page.svelte
#[derive(Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Started,
    AlreadyRunning,
}

/// Held while the dialog is being opened, so two quick launches can't both create it (see overlay::OPENING)
static OPENING: Mutex<()> = Mutex::new(());
/// The kind the dialog shows, which the page asks for on load (see kind). Separate from OPENING: the page's command
/// runs on the main thread, which mustn't wait while open holds OPENING and waits on the main thread for window calls
static KIND: Mutex<Kind> = Mutex::new(Kind::AlreadyRunning);

/// Shows the dialog, or brings the open one to the front on the display under the mouse
pub fn show(app: &AppHandle, kind: Kind) {
    let app = app.clone();
    // See overlay::open_all: windows can't be created on an event callback thread
    thread::spawn(move || open(&app, kind));
}

/// The kind the dialog shows, asked for by the page when it loads
pub fn kind() -> Kind {
    *KIND.lock().unwrap_or_else(|e| e.into_inner())
}

/// "OK" or "Open settings" on the dialog
pub fn close(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.destroy();
    }
}

/// The dialog's content needs `height` logical pixels (long texts wrap more in some languages): grows it to fit and
/// centers it again on its display. Never shrinks below HEIGHT
pub fn fit(window: &WebviewWindow, height: f64) {
    if window.label() != LABEL || !height.is_finite() {
        return;
    }
    let Ok(Some(monitor)) = window.current_monitor() else {
        return;
    };
    let (pos, size) = overlay::centered_on(&monitor, WIDTH, height.max(HEIGHT));
    let _ = window.set_position(pos);
    let _ = window.set_size(size);
}

fn open(app: &AppHandle, kind: Kind) {
    let _guard = OPENING.lock().unwrap_or_else(|e| e.into_inner());
    let Some(monitor) = monitor_under_cursor(app) else {
        eprintln!("No display detected, cannot show the already-running dialog");
        return;
    };
    *KIND.lock().unwrap_or_else(|e| e.into_inner()) = kind;
    if let Some(window) = app.get_webview_window(LABEL) {
        // Launched again while the "started" dialog is still open: switch it to "already running"
        let _ = window.emit_to(LABEL, KIND_EVENT, kind);
        // Keep the height the frontend asked for (see fit)
        let height = window
            .inner_size()
            .ok()
            .zip(window.scale_factor().ok())
            .map_or(HEIGHT, |(size, scale)| size.height as f64 / scale);
        place(&window, &monitor, height);
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    let built = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("already-running".into()))
        .title("RestGuard")
        .decorations(false)
        .always_on_top(true)
        .visible_on_all_workspaces(true)
        .skip_taskbar(true)
        .resizable(false)
        .minimizable(false)
        .maximizable(false)
        .visible(false)
        .build();
    match built {
        Ok(window) => {
            place(&window, &monitor, HEIGHT);
            let _ = window.show();
            // Focus it so Enter and Esc work right away
            let _ = window.set_focus();
        }
        Err(e) => eprintln!("Failed to create the already-running dialog: {e}"),
    }
}

/// Centers the dialog on `monitor` at `height` logical pixels (at least HEIGHT).
/// Moves first, then sizes, as away_lock::open_all does: moving to a display of another scale makes Windows rescale
/// the window, which would shrink or grow a size set before the move
fn place(window: &WebviewWindow, monitor: &Monitor, height: f64) {
    let (pos, size) = overlay::centered_on(monitor, WIDTH, height.max(HEIGHT));
    let _ = window.set_position(pos);
    let _ = window.set_size(size);
}

/// The display under the mouse, else the primary one
fn monitor_under_cursor(app: &AppHandle) -> Option<Monitor> {
    app.cursor_position()
        .ok()
        .and_then(|p| app.monitor_from_point(p.x, p.y).ok().flatten())
        .or_else(|| app.primary_monitor().ok().flatten())
}
