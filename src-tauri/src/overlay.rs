//! 休息遮罩：每块显示器一个无边框、置顶、关不掉的窗口。

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder, Window,
};

const PREFIX: &str = "overlay-";
/// 主屏幕上的遮罩，打开时获得焦点
pub const PRIMARY_LABEL: &str = "overlay-primary";

/// 显示器区域（物理像素）
#[derive(Clone, Copy)]
struct Rect {
    origin: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,
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

        let home = Rect {
            origin: *monitor.position(),
            size: *monitor.size(),
        };
        // 先登记再建窗口，创建过程中触发的 Moved 事件也能被约束
        HOMES.lock().unwrap().insert(label.clone(), home);

        let built = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("overlay".into()))
            .title("RestGuard")
            .decorations(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .resizable(false)
            .minimizable(false)
            .maximizable(false)
            .closable(false)
            .visible(false)
            .build();

        match built {
            Ok(window) => {
                // 用物理像素定位，避免多屏 DPI 不同时错位
                let (pos, size) = cover_rect(home, coverage);
                let _ = window.set_position(pos);
                let _ = window.set_size(size);
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
            let (pos, size) = cover_rect(home, coverage);
            let _ = window.set_size(size);
            let _ = window.set_position(pos);
        }
    }
}

/// 窗口移动后调用：如果被拖出（或用 Win+Shift+方向键移出）所属屏幕，就拉回屏幕内
pub fn keep_on_home(window: &Window, pos: PhysicalPosition<i32>) {
    let Some(home) = HOMES.lock().unwrap().get(window.label()).copied() else {
        return;
    };
    let Ok(size) = window.outer_size() else {
        return;
    };
    let clamp = |v: i32, origin: i32, screen: u32, len: u32| {
        // 窗口比屏幕大时贴住左/上边
        let max = origin + screen.saturating_sub(len) as i32;
        v.clamp(origin, max.max(origin))
    };
    let fixed = PhysicalPosition::new(
        clamp(pos.x, home.origin.x, home.size.width, size.width),
        clamp(pos.y, home.origin.y, home.size.height, size.height),
    );
    if fixed != pos {
        let _ = window.set_position(fixed);
    }
}

/// 按比例计算居中覆盖区域
fn cover_rect(home: Rect, coverage: f64) -> (PhysicalPosition<i32>, PhysicalSize<u32>) {
    let PhysicalSize { width, height } = home.size;
    let w = (width as f64 * coverage) as u32;
    let h = (height as f64 * coverage) as u32;
    let pos = PhysicalPosition::new(
        home.origin.x + ((width - w) / 2) as i32,
        home.origin.y + ((height - h) / 2) as i32,
    );
    (pos, PhysicalSize::new(w, h))
}
