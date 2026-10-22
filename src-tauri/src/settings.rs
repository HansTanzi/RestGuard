//! Settings panel: a regular window opened from the tray menu that loads the frontend root route (src/routes/+page.svelte).

use crate::i18n::{t, text};
use std::{path::Path, process::Command};
use tauri::{window::Color, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const LABEL: &str = "settings";

fn title() -> &'static str {
    t(&text::SETTINGS_TITLE)
}

/// Opens the settings window; brings it to the front if already open.
/// Like overlay::open_all, windows can't be created directly on the event callback thread on Windows, so callers must use a separate thread
pub fn open(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    // WebviewUrl::default() is index.html, i.e. the root route
    let built = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::default())
        .title(title())
        // Size when restored
        .inner_size(440.0, 600.0)
        .min_inner_size(360.0, 480.0)
        .center()
        // Matches the page background (--bg-1 in +page.svelte) so there's no white flash before the page loads
        .background_color(Color(0x0f, 0x17, 0x2a, 0xff))
        // Create hidden and show after maximizing, so a narrow window doesn't appear first and then widen
        .visible(false)
        .build();
    match built {
        Ok(window) => {
            // Maximized by default so the groups can spread across multiple columns
            let _ = window.maximize();
            let _ = window.show();
            let _ = window.set_focus();
        }
        Err(e) => eprintln!("Failed to create the settings window: {e}"),
    }
}

/// Opens the config file with the system default program for manual editing.
/// Calls ShellExecute in-process: AppData writes in the Store (MSIX) version are virtualized,
/// so explorer wouldn't find the file; only an editor launched by this process sees the same file
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
    // A return value greater than 32 means success
    if code.0 as isize > 32 {
        return Ok(());
    }
    // Use Notepad when no program is associated with .toml
    Command::new("notepad").arg(path).spawn().map(drop).map_err(|e| e.to_string())
}

#[cfg(target_os = "macos")]
pub fn open_file(path: &Path) -> Result<(), String> {
    // -t: open with the default text editor, which works even when no program is associated with .toml
    Command::new("open").arg("-t").arg(path).spawn().map(drop).map_err(|e| e.to_string())
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn open_file(path: &Path) -> Result<(), String> {
    Command::new("xdg-open").arg(path).spawn().map(drop).map_err(|e| e.to_string())
}

/// Updates the window title after a language change (the page text is updated by the frontend listening for LANG_EVENT)
pub fn refresh_language(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.set_title(title());
    }
}
