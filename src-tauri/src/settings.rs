//! 设置面板：从托盘菜单打开的普通窗口，加载前端根路由（src/routes/+page.svelte）。

use crate::i18n::{t, text};
use std::{path::Path, process::Command};
use tauri::{window::Color, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

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
        // 还原后是这个大小
        .inner_size(440.0, 600.0)
        .min_inner_size(360.0, 480.0)
        .center()
        // 与页面背景（+page.svelte 的 --bg-1）一致，页面加载出来之前不会先闪一下白色
        .background_color(Color(0x0f, 0x17, 0x2a, 0xff))
        // 先隐藏创建，最大化后再显示，避免先出现一个窄窗口再变宽
        .visible(false)
        .build();
    match built {
        Ok(window) => {
            // 默认最大化，分组可以多列排开
            let _ = window.maximize();
            let _ = window.show();
            let _ = window.set_focus();
        }
        Err(e) => eprintln!("创建设置窗口失败：{e}"),
    }
}

/// 用系统默认的程序打开配置文件，供用户手动编辑。
/// 在进程内调用 ShellExecute：商店（MSIX）版的 AppData 写入是虚拟化的，
/// 交给 explorer 打开会找不到文件，由本进程启动的编辑器才能看到同一份文件
#[cfg(windows)]
pub fn open_file(path: &Path) -> Result<(), String> {
    use windows::{
        core::{w, HSTRING, PCWSTR},
        Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_SHOWNORMAL},
    };
    let file = HSTRING::from(path.as_os_str());
    let code = unsafe {
        ShellExecuteW(None, w!("open"), &file, PCWSTR::null(), PCWSTR::null(), SW_SHOWNORMAL)
    };
    // 返回值大于 32 表示成功
    if code.0 as isize > 32 {
        return Ok(());
    }
    // .toml 没有关联的程序时用记事本打开
    Command::new("notepad").arg(path).spawn().map(drop).map_err(|e| e.to_string())
}

#[cfg(target_os = "macos")]
pub fn open_file(path: &Path) -> Result<(), String> {
    // -t：用默认的文本编辑器打开，.toml 没有关联的程序时也能打开
    Command::new("open").arg("-t").arg(path).spawn().map(drop).map_err(|e| e.to_string())
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn open_file(path: &Path) -> Result<(), String> {
    Command::new("xdg-open").arg(path).spawn().map(drop).map_err(|e| e.to_string())
}

/// 语言切换后更新窗口标题（页面内文字由前端监听 LANG_EVENT 自行更新）
pub fn refresh_language(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.set_title(title());
    }
}
