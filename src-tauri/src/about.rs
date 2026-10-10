//! 关于窗口：从托盘菜单打开，加载前端 /about 路由（src/routes/about/+page.svelte）。

use crate::i18n::{t, text};
use serde::Serialize;
use std::process::Command;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const LABEL: &str = "about";

/// 仓库地址取自 Cargo.toml 的 repository 字段
const REPO_URL: &str = env!("CARGO_PKG_REPOSITORY");

#[derive(Serialize)]
pub struct AboutInfo {
    version: String,
    repo_url: &'static str,
    issues_url: String,
}

pub fn info(app: &AppHandle) -> AboutInfo {
    AboutInfo {
        version: app.package_info().version.to_string(),
        repo_url: REPO_URL,
        issues_url: issues_url(),
    }
}

fn issues_url() -> String {
    format!("{REPO_URL}/issues")
}

/// 用系统默认浏览器打开链接。只接受固定的几个链接，不让页面打开任意地址
pub fn open_link(link: &str) -> Result<(), String> {
    let url = match link {
        "repo" => REPO_URL.to_string(),
        "issues" => issues_url(),
        _ => return Err("未知的链接".into()),
    };
    #[cfg(windows)]
    let mut cmd = Command::new("explorer");
    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open");
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut cmd = Command::new("xdg-open");
    // explorer 打开网址后返回非零退出码，所以只检查能否启动
    cmd.arg(url).spawn().map(drop).map_err(|e| e.to_string())
}

fn title() -> &'static str {
    t(&text::ABOUT_TITLE)
}

/// 打开关于窗口；已经打开时把它带到前台。
/// 与 settings::open 一样，调用方要放在独立线程
pub fn open(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    let built = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("about".into()))
        .title(title())
        .inner_size(360.0, 340.0)
        .resizable(false)
        .center()
        .maximizable(false)
        .minimizable(false)
        .build();
    if let Err(e) = built {
        eprintln!("创建关于窗口失败：{e}");
    }
}

/// 语言切换后更新窗口标题（页面内文字由前端监听 LANG_EVENT 自行更新）
pub fn refresh_language(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.set_title(title());
    }
}
