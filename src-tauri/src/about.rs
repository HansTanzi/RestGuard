//! About window: opened from the tray menu, loads the frontend /about route (src/routes/about/+page.svelte).

use crate::i18n::{t, text};
use serde::Serialize;
use std::process::Command;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const LABEL: &str = "about";

/// Repository URL taken from the repository field in Cargo.toml
const REPO_URL: &str = env!("CARGO_PKG_REPOSITORY");
/// Project website (GitHub Pages, built from site/) taken from the homepage field in Cargo.toml
const HOMEPAGE_URL: &str = env!("CARGO_PKG_HOMEPAGE");

#[derive(Serialize)]
pub struct AboutInfo {
    version: String,
    /// The license field in Cargo.toml, e.g. "MIT OR Apache-2.0"
    license: &'static str,
    website_url: &'static str,
    repo_url: &'static str,
    issues_url: String,
    privacy_url: String,
}

pub fn info(app: &AppHandle) -> AboutInfo {
    AboutInfo {
        version: app.package_info().version.to_string(),
        license: env!("CARGO_PKG_LICENSE"),
        website_url: HOMEPAGE_URL,
        repo_url: REPO_URL,
        issues_url: issues_url(),
        privacy_url: privacy_url(),
    }
}

fn issues_url() -> String {
    format!("{REPO_URL}/issues")
}

fn privacy_url() -> String {
    format!("{}/privacy.html", HOMEPAGE_URL.trim_end_matches('/'))
}

/// Opens a link in the system default browser. Only a fixed set of links is accepted, so the page can't open arbitrary URLs
pub fn open_link(link: &str) -> Result<(), String> {
    let url = match link {
        "website" => HOMEPAGE_URL.to_string(),
        "repo" => REPO_URL.to_string(),
        "issues" => issues_url(),
        "privacy" => privacy_url(),
        _ => return Err("Unknown link".into()),
    };
    #[cfg(windows)]
    let mut cmd = Command::new("explorer");
    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open");
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut cmd = Command::new("xdg-open");
    // explorer returns a non-zero exit code after opening a URL, so only check that it starts
    cmd.arg(url).spawn().map(drop).map_err(|e| e.to_string())
}

fn title() -> &'static str {
    t(&text::ABOUT_TITLE)
}

/// Opens the About window; brings it to the front if already open.
/// Like settings::open, callers must run this on a separate thread
pub fn open(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    let built = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("about".into()))
        .title(title())
        .inner_size(360.0, 420.0)
        .resizable(false)
        .center()
        .maximizable(false)
        .minimizable(false)
        .build();
    if let Err(e) = built {
        eprintln!("Failed to create the About window: {e}");
    }
}

/// Updates the window title after a language change (the page text is updated by the frontend listening for LANG_EVENT)
pub fn refresh_language(app: &AppHandle) {
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.set_title(title());
    }
}
