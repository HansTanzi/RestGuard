use crate::config::BUILTIN_LEVELS;
use crate::i18n::{self, t, text, Lang};
use crate::timer::{Phase, Snapshot};
use std::sync::Mutex;
use tauri::{
    image::Image,
    include_image,
    menu::{CheckMenuItem, IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};
use tauri_plugin_autostart::ManagerExt;

const TRAY_ID: &str = "main";
const LANG_ID_PREFIX: &str = "lang:";
/// Followed by a built-in level or custom profile name (they never clash, see Config::sanitized)
const LEVEL_ID_PREFIX: &str = "level:";

/// Menu items that update with state or language
struct TrayItems {
    status: MenuItem<Wry>,
    rest_now: MenuItem<Wry>,
    settings: MenuItem<Wry>,
    autostart: CheckMenuItem<Wry>,
    /// Enforcement profile submenu; the list changes with the config and is rebuilt wholesale by refresh_levels
    level_menu: Submenu<Wry>,
    /// Same order as Lang::ALL
    langs: Vec<CheckMenuItem<Wry>>,
    about: MenuItem<Wry>,
    quit: MenuItem<Wry>,
    /// Current tray icon, reset only when it changes
    icon: Mutex<IconKey>,
}

/// What the tray icon is chosen by: (on a break, light taskbar)
type IconKey = (bool, bool);

fn icon_key(phase: Phase) -> IconKey {
    // macOS uses a template image tinted by the system to match the menu bar, so a single icon is always used
    if cfg!(target_os = "macos") {
        return (false, false);
    }
    (phase != Phase::Working, light_taskbar())
}

fn icon_image(key: IconKey) -> Image<'static> {
    if cfg!(target_os = "macos") {
        return include_image!("icons/tray/tray-template@2x.png");
    }
    match key {
        (false, false) => include_image!("icons/tray/tray-white.png"),
        (false, true) => include_image!("icons/tray/tray-template.png"),
        (true, false) => include_image!("icons/tray/tray-rest.png"),
        (true, true) => include_image!("icons/tray/tray-rest-dark.png"),
    }
}

/// Windows reads the taskbar's light/dark setting; Linux panels are mostly dark, so treat them as dark
fn light_taskbar() -> bool {
    #[cfg(windows)]
    {
        use windows::{
            core::w,
            Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD},
        };
        let mut value = 0u32;
        let mut size = std::mem::size_of::<u32>() as u32;
        let result = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                w!(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"),
                w!("SystemUsesLightTheme"),
                RRF_RT_REG_DWORD,
                None,
                Some(&mut value as *mut u32 as *mut _),
                Some(&mut size),
            )
        };
        result.is_ok() && value != 0
    }
    #[cfg(not(windows))]
    false
}

fn rest_now_text() -> &'static str {
    t(&text::REST_NOW)
}

fn settings_text() -> &'static str {
    t(&text::SETTINGS_MENU)
}

fn autostart_text() -> &'static str {
    t(&text::AUTOSTART)
}

fn about_text() -> &'static str {
    t(&text::ABOUT_MENU)
}

fn quit_text() -> &'static str {
    t(&text::QUIT)
}

fn lang_text(lang: Lang) -> &'static str {
    match lang {
        Lang::Auto => t(&text::FOLLOW_SYSTEM),
        lang => lang.native_name(),
    }
}

fn level_menu_text() -> &'static str {
    t(&text::LEVEL_MENU)
}

/// Built-in levels show their translated name; custom profiles show their own name
fn level_text(level: &str) -> &str {
    match level {
        "high" => t(&text::LEVEL_HIGH),
        "medium" => t(&text::LEVEL_MEDIUM),
        "low" => t(&text::LEVEL_LOW),
        name => name,
    }
}

/// Rebuilds the enforcement profile submenu from the current config, with a separator between built-in levels and custom profiles
fn fill_levels(app: &AppHandle, menu: &Submenu<Wry>) -> tauri::Result<()> {
    let (current, customs) = {
        let state = app.state::<crate::AppState>();
        let config = state.config.lock().unwrap();
        let customs: Vec<String> = config.profiles.iter().map(|p| p.name.clone()).collect();
        (config.level.clone(), customs)
    };
    while menu.remove_at(0)?.is_some() {}
    let check = |level: &str| {
        CheckMenuItem::with_id(
            app,
            format!("{LEVEL_ID_PREFIX}{level}"),
            level_text(level),
            true,
            level == current,
            None::<&str>,
        )
    };
    for level in BUILTIN_LEVELS {
        menu.append(&check(level)?)?;
    }
    if !customs.is_empty() {
        menu.append(&PredefinedMenuItem::separator(app)?)?;
    }
    for name in &customs {
        menu.append(&check(name)?)?;
    }
    Ok(())
}

/// Refreshes the submenu after a config change or profile switch (custom profiles may be added, removed or renamed;
/// clicking a CheckMenuItem toggles its check automatically, so it also has to be reset to the actual selection)
pub fn refresh_levels(app: &AppHandle) {
    let Some(items) = app.try_state::<TrayItems>() else {
        return;
    };
    if let Err(e) = fill_levels(app, &items.level_menu) {
        eprintln!("Failed to refresh the enforcement profile menu: {e}");
    }
}

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    let status = MenuItem::with_id(app, "status", "RestGuard", false, None::<&str>)?;
    let rest_now = MenuItem::with_id(app, "rest_now", rest_now_text(), true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", settings_text(), true, None::<&str>)?;
    let autostart_on = autostart_enabled(app);
    let autostart = CheckMenuItem::with_id(
        app,
        "autostart",
        autostart_text(),
        true,
        autostart_on,
        None::<&str>,
    )?;
    let current = i18n::current();
    let langs = Lang::ALL
        .into_iter()
        .map(|lang| {
            CheckMenuItem::with_id(
                app,
                format!("{LANG_ID_PREFIX}{}", lang.id()),
                lang_text(lang),
                true,
                lang == current,
                None::<&str>,
            )
        })
        .collect::<tauri::Result<Vec<_>>>()?;
    // Separator between "Follow system" and the specific languages
    let separator = PredefinedMenuItem::separator(app)?;
    let mut lang_entries: Vec<&dyn IsMenuItem<Wry>> = vec![&langs[0], &separator];
    lang_entries.extend(langs[1..].iter().map(|item| item as &dyn IsMenuItem<Wry>));
    let lang_menu = Submenu::with_items(app, "语言 / Language", true, &lang_entries)?;
    let level_menu = Submenu::new(app, level_menu_text(), true)?;
    fill_levels(app, &level_menu)?;
    let about = MenuItem::with_id(app, "about", about_text(), true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", quit_text(), true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &status,
            &PredefinedMenuItem::separator(app)?,
            &rest_now,
            &settings,
            &autostart,
            &level_menu,
            &lang_menu,
            &PredefinedMenuItem::separator(app)?,
            &about,
            &quit,
        ],
    )?;

    let icon = icon_key(Phase::Working);
    TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon_image(icon))
        .icon_as_template(cfg!(target_os = "macos"))
        .tooltip("RestGuard")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "rest_now" => crate::rest_now(app),
            "settings" => crate::open_settings(app),
            "about" => crate::open_about(app),
            "autostart" => {
                crate::apply_autostart(app, !autostart_enabled(app));
            }
            "quit" => app.exit(0),
            id => {
                if let Some(lang) = id.strip_prefix(LANG_ID_PREFIX).and_then(Lang::from_id) {
                    crate::set_language(app, lang);
                } else if let Some(level) = id.strip_prefix(LEVEL_ID_PREFIX) {
                    crate::set_level(app, level);
                }
            }
        })
        .build(app)?;

    app.manage(TrayItems {
        status,
        rest_now,
        settings,
        autostart,
        level_menu,
        langs,
        about,
        quit,
        icon: Mutex::new(icon),
    });
    Ok(())
}

/// Refreshes menu text and checks after a language change (the status line is refreshed by the following update)
pub fn refresh_language(app: &AppHandle) {
    let Some(items) = app.try_state::<TrayItems>() else {
        return;
    };
    let _ = items.rest_now.set_text(rest_now_text());
    let _ = items.settings.set_text(settings_text());
    let _ = items.autostart.set_text(autostart_text());
    let _ = items.level_menu.set_text(level_menu_text());
    let _ = items.about.set_text(about_text());
    let _ = items.quit.set_text(quit_text());
    // Clicking a CheckMenuItem toggles its check automatically, so reset all of them to the actual selection here
    let current = i18n::current();
    for (item, lang) in items.langs.iter().zip(Lang::ALL) {
        let _ = item.set_text(lang_text(lang));
        let _ = item.set_checked(lang == current);
    }
    refresh_levels(app);
}

pub fn update(app: &AppHandle, snap: &Snapshot) {
    let text = match snap.phase {
        Phase::Working if snap.deferred_until.is_some() => {
            let time = snap.deferred_until.as_deref().unwrap_or_default();
            let name = snap.deferred_for.as_deref().unwrap_or_default();
            match name {
                "work_end" => t(&text::BREAK_DEFERRED_WORK_END).replace("{time}", time),
                "lunch" => t(&text::BREAK_DEFERRED_LUNCH).replace("{time}", time),
                _ => t(&text::BREAK_DEFERRED).replace("{name}", name).replace("{time}", time),
            }
        }
        Phase::Working if snap.held_by.is_some() => {
            let app = snap.held_by.as_deref().unwrap_or_default();
            t(&text::BREAK_HELD).replace("{app}", app)
        }
        Phase::Working => format!("{} {}", t(&text::BREAK_IN), mm_ss(snap.remaining_secs)),
        Phase::Resting => format!("{} {}", t(&text::RESTING), mm_ss(snap.remaining_secs)),
        Phase::RestOver => t(&text::BREAK_OVER).to_string(),
    };
    let working = snap.phase == Phase::Working;
    let tray = app.tray_by_id(TRAY_ID);

    if let Some(items) = app.try_state::<TrayItems>() {
        let _ = items.status.set_text(&text);
        let _ = items.rest_now.set_enabled(working);
        // During a break, quitting from the tray isn't allowed, nor is opening settings or switching profiles to shorten the break;
        // the About page can open a browser, so it's disabled too
        let _ = items.settings.set_enabled(working);
        // A postponed break is still owed, so switching profiles or quitting stays off until it's taken
        let unlocked = working && !snap.postponed;
        let _ = items.level_menu.set_enabled(unlocked);
        let _ = items.about.set_enabled(working);
        let _ = items.quit.set_enabled(unlocked);

        // Re-evaluated on every refresh so the icon follows taskbar light/dark changes
        let key = icon_key(snap.phase);
        let mut icon = items.icon.lock().unwrap();
        if *icon != key {
            if let Some(tray) = &tray {
                let _ = tray.set_icon(Some(icon_image(key)));
            }
            *icon = key;
        }
    }
    if let Some(tray) = tray {
        let _ = tray.set_tooltip(Some(format!("RestGuard · {text}")));
    }
}

pub fn autostart_enabled(app: &AppHandle) -> bool {
    #[cfg(windows)]
    if crate::msix::is_packaged() {
        return crate::msix::startup_enabled();
    }
    app.autolaunch().is_enabled().unwrap_or(false)
}

/// Enables or disables autostart and returns the actual system state
pub fn set_autostart(app: &AppHandle, on: bool) -> bool {
    if let Err(e) = toggle_autostart(app, on) {
        eprintln!("Failed to toggle autostart: {e}");
    }
    // Go by the actual system state so the menu check never disagrees with reality
    let actual = autostart_enabled(app);
    if let Some(items) = app.try_state::<TrayItems>() {
        let _ = items.autostart.set_checked(actual);
    }
    actual
}

fn toggle_autostart(app: &AppHandle, on: bool) -> Result<(), String> {
    #[cfg(windows)]
    if crate::msix::is_packaged() {
        return crate::msix::set_startup(on).map_err(|e| e.to_string());
    }
    let launcher = app.autolaunch();
    let result = if on {
        launcher.enable()
    } else {
        launcher.disable()
    };
    result.map_err(|e| e.to_string())
}

fn mm_ss(secs: u64) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
