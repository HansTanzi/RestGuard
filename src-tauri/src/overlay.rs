//! 休息遮罩：每块显示器一个无边框、置顶、关不掉的窗口。

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Monitor, PhysicalPosition, PhysicalSize,
    Position, Size, WebviewUrl, WebviewWindowBuilder, Window,
};

const PREFIX: &str = "overlay-";
/// 主屏幕上的遮罩，打开时获得焦点
pub const PRIMARY_LABEL: &str = "overlay-primary";

/// 显示器区域，单位与系统窗口坐标一致（见 unit_scale）
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

/// 物理像素换算到窗口坐标单位的比例。Windows/Linux 用物理像素，避免多屏 DPI 不同时错位；
/// macOS 的全局坐标以逻辑点为单位，Retina 屏和普通外接屏混用时物理像素没有统一坐标系，所以用逻辑点
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

/// 每个遮罩所属的显示器。缩小后窗口可拖动，但只能留在这块屏幕上
static HOMES: LazyLock<Mutex<HashMap<String, Rect>>> = LazyLock::new(Default::default);

pub fn is_overlay(label: &str) -> bool {
    label.starts_with(PREFIX)
}

/// 在所有显示器上打开遮罩。
/// 在 Windows 上，事件回调或同步 command 里创建窗口会死锁，所以调用方要放在独立线程。
pub fn open_all(app: &AppHandle, coverage: f64) {
    let monitors = match app.available_monitors() {
        Ok(m) if !m.is_empty() => m,
        Ok(_) => {
            eprintln!("没有检测到显示器，无法打开遮罩");
            return;
        }
        Err(e) => {
            eprintln!("获取显示器失败：{e}");
            return;
        }
    };
    let primary_pos = app
        .primary_monitor()
        .ok()
        .flatten()
        .map(|m| *m.position())
        .unwrap_or(*monitors[0].position());

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
        // 先登记再建窗口，创建过程中触发的 Moved 事件也能被约束
        HOMES.lock().unwrap().insert(label.clone(), home);

        let built = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("overlay".into()))
            .title("RestGuard")
            .decorations(false)
            .always_on_top(true)
            .visible_on_all_workspaces(true)
            .skip_taskbar(true)
            .resizable(false)
            .minimizable(false)
            .maximizable(false)
            .closable(false)
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
                HOMES.lock().unwrap().remove(&label);
                eprintln!("创建遮罩窗口 {label} 失败：{e}");
            }
        }
    }
}

/// macOS 上 always_on_top 只是浮动窗口级别，盖不住菜单栏和 Dock，也进不了其他应用的全屏空间。
/// 提升到屏保级别，并让遮罩出现在所有空间（包括全屏应用）里
#[cfg(target_os = "macos")]
fn cover_everything(window: &tauri::WebviewWindow) {
    use objc2_app_kit::{NSWindow, NSWindowCollectionBehavior};
    /// kCGScreenSaverWindowLevel
    const SCREEN_SAVER_LEVEL: isize = 1000;

    let w = window.clone();
    // AppKit 只能在主线程调用
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
            // destroy 不触发 CloseRequested，绕过关闭拦截
            let _ = window.destroy();
        }
    }
    HOMES.lock().unwrap().clear();
}

/// 紧急模式：把所有遮罩的宽高缩小一半（仍居中置顶），露出四周以便保存文件等；
/// `shrunk = false` 时恢复原大小并回到屏幕中央
pub fn set_shrunk(app: &AppHandle, coverage: f64, shrunk: bool) {
    let coverage = if shrunk { coverage * 0.5 } else { coverage };
    let homes = HOMES.lock().unwrap().clone();
    for (label, window) in app.webview_windows() {
        if let Some(&home) = homes.get(&label) {
            let area = cover_rect(home, coverage);
            let _ = window.set_size(size(area.w, area.h));
            let _ = window.set_position(position(area.x, area.y));
        }
    }
}

/// 窗口移动后调用：如果被拖出（或用 Win+Shift+方向键移出）所属屏幕，就拉回屏幕内
pub fn keep_on_home(window: &Window, pos: PhysicalPosition<i32>) {
    let Some(home) = HOMES.lock().unwrap().get(window.label()).copied() else {
        return;
    };
    let (Ok(win_size), Ok(scale)) = (window.outer_size(), window.scale_factor()) else {
        return;
    };
    let s = unit_scale(scale);
    let (x, y) = (pos.x as f64 / s, pos.y as f64 / s);
    let (w, h) = (win_size.width as f64 / s, win_size.height as f64 / s);
    // 窗口比屏幕大时贴住左/上边
    let clamp = |v: f64, origin: f64, screen: f64, len: f64| {
        v.clamp(origin, origin + (screen - len).max(0.0))
    };
    let (fx, fy) = (clamp(x, home.x, home.w, w), clamp(y, home.y, home.h, h));
    // 忽略取整误差，避免反复触发 Moved
    if (fx - x).abs() >= 0.5 || (fy - y).abs() >= 0.5 {
        let _ = window.set_position(position(fx, fy));
    }
}

/// 按比例计算居中覆盖区域
fn cover_rect(home: Rect, coverage: f64) -> Rect {
    let (w, h) = (home.w * coverage, home.h * coverage);
    Rect {
        x: home.x + (home.w - w) / 2.0,
        y: home.y + (home.h - h) / 2.0,
        w,
        h,
    }
}
