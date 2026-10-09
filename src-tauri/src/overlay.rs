//! 休息遮罩：每块显示器一个无边框、置顶、关不掉的窗口。

use tauri::{
    AppHandle, Manager, Monitor, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder,
};

const PREFIX: &str = "overlay-";
/// 主屏幕上的遮罩显示完整界面，其他屏幕只显示倒计时
pub const PRIMARY_LABEL: &str = "overlay-primary";

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
                let (pos, size) = cover_rect(monitor, coverage);
                let _ = window.set_position(pos);
                let _ = window.set_size(size);
                let _ = window.show();
                if is_primary {
                    let _ = window.set_focus();
                }
            }
            Err(e) => eprintln!("创建遮罩窗口 {label} 失败：{e}"),
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
}

/// 按比例计算居中覆盖区域
fn cover_rect(monitor: &Monitor, coverage: f64) -> (PhysicalPosition<i32>, PhysicalSize<u32>) {
    let PhysicalSize { width, height } = *monitor.size();
    let w = (width as f64 * coverage) as u32;
    let h = (height as f64 * coverage) as u32;
    let origin = monitor.position();
    let pos = PhysicalPosition::new(
        origin.x + ((width - w) / 2) as i32,
        origin.y + ((height - h) / 2) as i32,
    );
    (pos, PhysicalSize::new(w, h))
}
