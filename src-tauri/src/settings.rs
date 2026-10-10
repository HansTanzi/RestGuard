//! 设置面板：从托盘菜单打开的普通窗口，加载前端根路由（src/routes/+page.svelte）。

use crate::i18n::{t, text};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const LABEL: &str = "settings";

fn title() -> &'static str {
    t(&text::SETTINGS_TITLE)
}

/// 打开设置窗口；已经打开时把它带到前台。
/// 与 overlay::open_all 一样，在 Windows 上不能在事件回调线程里直接创建窗口，调用方要放在独立线程
pub fn open(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    // WebviewUrl::default() 即 index.html，对应根路由
    let built = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::default())
        .title(title())
        .inner_size(440.0, 600.0)
        .min_inner_size(360.0, 480.0)
        .center()
        .maximizable(false)
        .build();
    if let Err(e) = built {
        eprintln!("创建设置窗口失败：{e}");
    }
}

/// 语言切换后更新窗口标题（页面内文字由前端监听 LANG_EVENT 自行更新）
pub fn refresh_language(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.set_title(title());
    }
}
