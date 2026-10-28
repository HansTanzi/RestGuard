mod about;
mod bypass;
mod config;
mod i18n;
mod idle;
#[cfg(windows)]
mod msix;
mod overlay;
mod phone;
mod settings;
mod sound;
mod timer;
mod tray;

use config::Config;
use std::{
    path::PathBuf,
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, Manager, RunEvent, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use timer::{Phase, Snapshot, Timer, TimerEvent};

const STATE_EVENT: &str = "timer:state";
/// Payload is the language actually displayed after the switch (see ui_lang); matches LANG_EVENT in src/lib/i18n.ts
const LANG_EVENT: &str = "lang:changed";
/// Payload is whether the overlay is shrunk; matches SHRINK_EVENT in src/routes/overlay/+page.svelte
const SHRINK_EVENT: &str = "overlay:shrunk";
/// Payload is whether autostart is enabled; matches AUTOSTART_EVENT in src/lib/config.ts
const AUTOSTART_EVENT: &str = "autostart:changed";
/// Payload is the new config; matches CONFIG_EVENT in src/lib/config.ts
const CONFIG_EVENT: &str = "config:changed";

struct AppState {
    timer: Mutex<Timer>,
    config: Mutex<Config>,
    config_path: PathBuf,
}

impl AppState {
    /// Fraction of the screen the overlay covers; `urgent` is the fraction in urgent mode (overlay shrunk).
    /// Taken from the timer's profile, since this break was started with that same profile
    fn overlay_coverage(&self, urgent: bool) -> f64 {
        self.timer.lock().unwrap().profile().coverage(urgent)
    }
}

#[tauri::command]
fn get_state(app: AppHandle) -> Snapshot {
    let state = app.state::<AppState>();
    let snap = state.timer.lock().unwrap().snapshot(Instant::now());
    snap
}

/// The UI language actually displayed (never auto); pages pick their text by it
#[tauri::command]
fn ui_lang() -> &'static str {
    i18n::resolved().id()
}

/// The user's language choice (possibly auto), shared by the language selectors in the settings panel and overlay.
/// See i18n::Lang::id for values
#[tauri::command]
fn get_lang() -> &'static str {
    i18n::current().id()
}

#[tauri::command]
fn set_lang(app: AppHandle, lang: String) -> Result<(), String> {
    let lang = i18n::Lang::from_id(&lang).ok_or("Unknown language")?;
    set_language(&app, lang);
    Ok(())
}

#[tauri::command]
fn get_config(app: AppHandle) -> Config {
    let cfg = app.state::<AppState>().config.lock().unwrap().clone();
    cfg
}

/// Settings panel save: writes the config file and applies it immediately, returning the sanitized config in effect.
/// Not allowed during or while postponing a break, so the break can't be dodged by shortening it
#[tauri::command]
fn save_config(app: AppHandle, config: Config) -> Result<Config, String> {
    apply_config(&app, config)
}

/// Switches the enforcement profile from the tray: only level changes, the rest of the config stays
pub(crate) fn set_level(app: &AppHandle, level: &str) {
    let mut config = app.state::<AppState>().config.lock().unwrap().clone();
    config.level = level.to_string();
    if let Err(e) = apply_config(app, config) {
        eprintln!("Failed to switch enforcement profile: {e}");
        // The click already toggled the clicked item's check, so reset to the actual selection
        tray::refresh_levels(app);
    }
}

/// "Open config file" in the settings panel: opens it with the system default program for manual editing.
/// Not allowed during or while postponing a break, like save_config
#[tauri::command]
fn open_config_file(app: AppHandle) -> Result<(), String> {
    settings_unlocked(&get_state(app.clone()))?;
    let state = app.state::<AppState>();
    // If the file was deleted, write the current config back first so the editor doesn't open an empty file
    if !state.config_path.exists() {
        let config = state.config.lock().unwrap().clone();
        config
            .save(&state.config_path)
            .map_err(|e| i18n::t(&i18n::text::SAVE_FAILED).replace("{e}", &e.to_string()))?;
    }
    settings::open_file(&state.config_path)
}

/// Re-reads a manually edited config file and applies any changes immediately. Called when the settings panel gains focus.
/// Returns the new config in effect, or None if nothing changed
#[tauri::command]
fn reload_config(app: AppHandle) -> Result<Option<Config>, String> {
    let state = app.state::<AppState>();
    let config = Config::read(&state.config_path)?;
    if *state.config.lock().unwrap() == config {
        return Ok(None);
    }
    // Don't write back to the file: the user may still have it open in an editor
    commit_config(&app, config, false).map(Some)
}

/// Saves the config, applies it immediately and notifies the tray and settings panel. Shared by the settings panel and tray
fn apply_config(app: &AppHandle, config: Config) -> Result<Config, String> {
    commit_config(app, config.sanitized(), true)
}

/// Applies an already sanitized config; `save` controls whether it's written to the config file. Refuses changes during or postponing a break
fn commit_config(app: &AppHandle, config: Config, save: bool) -> Result<Config, String> {
    let state = app.state::<AppState>();
    let snap = {
        let mut timer = state.timer.lock().unwrap();
        let now = Instant::now();
        settings_unlocked(&timer.snapshot(now))?;
        if save {
            config
                .save(&state.config_path)
                .map_err(|e| i18n::t(&i18n::text::SAVE_FAILED).replace("{e}", &e.to_string()))?;
        }
        timer.apply_config(&config, now);
        timer.snapshot(now)
    };
    *state.config.lock().unwrap() = config.clone();
    tray::refresh_levels(app);
    let _ = app.emit(CONFIG_EVENT, &config);
    broadcast(app, &snap);
    Ok(config)
}

/// Settings (including the enforcement profile) can't change during a break, nor while one is postponed
fn settings_unlocked(snap: &Snapshot) -> Result<(), String> {
    if snap.phase != Phase::Working {
        return Err(i18n::t(&i18n::text::SETTINGS_LOCKED).into());
    }
    if snap.postponed {
        return Err(i18n::t(&i18n::text::SETTINGS_LOCKED_POSTPONED).into());
    }
    Ok(())
}

/// "Preview" in the settings panel: plays the sound using the form's unsaved settings, returning a message to the frontend if it can't play.
/// async: it waits for playback to finish and mustn't block the main thread
#[tauri::command]
async fn preview_sound(config: Config) -> Result<(), String> {
    sound::preview(&config.sanitized())
}

/// "Send test" in the settings panel: pushes a notification to the phone using the form's unsaved settings, returning a message to the frontend on failure.
/// async: it waits on a network request and mustn't block the main thread
#[tauri::command]
async fn test_phone_notify(config: Config) -> Result<(), String> {
    phone::test(&config.sanitized())
}

/// Picks a sound file for the settings panel and returns its full path; returns None if cancelled.
/// async: the dialog blocks until the user chooses, so it mustn't run on the main thread
#[tauri::command]
async fn pick_sound_file(window: tauri::WebviewWindow) -> Option<String> {
    use tauri_plugin_dialog::DialogExt;
    window
        .dialog()
        .file()
        .set_parent(&window)
        .add_filter(i18n::t(&i18n::text::AUDIO_FILES), sound::EXTENSIONS)
        .blocking_pick_file()?
        .into_path()
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
fn get_autostart(app: AppHandle) -> bool {
    tray::autostart_enabled(&app)
}

/// Returns the actual system state after toggling
#[tauri::command]
fn set_autostart(app: AppHandle, on: bool) -> bool {
    apply_autostart(&app, on)
}

#[tauri::command]
fn get_about(app: AppHandle) -> about::AboutInfo {
    about::info(&app)
}

/// Links on the About page: link is "website", "repo", "issues" or "privacy".
/// Opening a browser isn't allowed during a break, so it can't be used to dodge the break
#[tauri::command]
fn open_link(app: AppHandle, link: String) -> Result<(), String> {
    let snap = get_state(app);
    if snap.phase != Phase::Working {
        return Err(i18n::t(&i18n::text::LINKS_LOCKED).into());
    }
    about::open_link(&link)
}

#[tauri::command]
fn start_work(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::start_work)
}

#[tauri::command]
fn postpone(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::postpone)
}

/// "I'm in a meeting" on the overlay: the user confirms when meeting software is running but it's unclear whether a meeting is on
#[tauri::command]
fn in_meeting(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::in_meeting)
}

/// "Skip this break" on the overlay; refused when the enforcement level doesn't allow skipping
#[tauri::command]
fn dismiss_break(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::skip)
}

/// The overlay's "shrink" button: shrinks the overlay to deal with something urgent without ending the break.
/// Refused when the current enforcement level doesn't allow urgent mode.
/// async so it runs off the main thread, avoiding a hang while resizing windows on Windows
#[tauri::command]
async fn set_overlay_shrunk(app: AppHandle, shrunk: bool) -> Result<(), String> {
    if shrunk && !get_state(app.clone()).can_shrink {
        return Err("The current enforcement level does not allow urgent mode".into());
    }
    let coverage = app.state::<AppState>().overlay_coverage(shrunk);
    overlay::set_coverage(&app, coverage);
    // Notify all overlays (including secondary screens) to update their draggable state
    let _ = app.emit(SHRINK_EVENT, shrunk);
    Ok(())
}

/// Dev builds only: pressing Esc on the overlay ends the break immediately, for debugging
#[tauri::command]
fn dev_skip_rest(app: AppHandle) -> Result<(), String> {
    if !cfg!(debug_assertions) {
        return Err("Only available in dev builds".into());
    }
    end_rest(&app, Timer::skip_rest)
}

fn end_rest(app: &AppHandle, action: fn(&mut Timer, Instant) -> bool) -> Result<(), String> {
    let snap = {
        let state = app.state::<AppState>();
        let mut timer = state.timer.lock().unwrap();
        let now = Instant::now();
        if !action(&mut timer, now) {
            return Err("Not allowed in the current state".into());
        }
        timer.snapshot(now)
    };
    overlay::close_all(app);
    broadcast(app, &snap);
    Ok(())
}

pub(crate) fn rest_now(app: &AppHandle) {
    let snap = {
        let state = app.state::<AppState>();
        let mut timer = state.timer.lock().unwrap();
        let now = Instant::now();
        if !timer.rest_now(now) {
            return;
        }
        timer.snapshot(now)
    };
    open_overlays(app);
    broadcast(app, &snap);
}

pub(crate) fn set_language(app: &AppHandle, lang: i18n::Lang) {
    if let Err(e) = i18n::set(lang) {
        eprintln!("Failed to save language setting: {e}");
    }
    tray::refresh_language(app);
    settings::refresh_language(app);
    about::refresh_language(app);
    let _ = app.emit(LANG_EVENT, i18n::resolved().id());
    let snap = {
        let state = app.state::<AppState>();
        let snap = state.timer.lock().unwrap().snapshot(Instant::now());
        snap
    };
    tray::update(app, &snap);
}

/// Shared by the tray and settings panel; notifies the settings panel to sync the toggle afterward
pub(crate) fn apply_autostart(app: &AppHandle, on: bool) -> bool {
    let actual = tray::set_autostart(app, on);
    let _ = app.emit(AUTOSTART_EVENT, actual);
    actual
}

pub(crate) fn open_settings(app: &AppHandle) {
    let app = app.clone();
    // See overlay::open_all: windows can't be created directly on the event callback thread
    thread::spawn(move || settings::open(&app));
}

pub(crate) fn open_about(app: &AppHandle) {
    let app = app.clone();
    // See overlay::open_all: windows can't be created directly on the event callback thread
    thread::spawn(move || about::open(&app));
}

fn open_overlays(app: &AppHandle) {
    let app = app.clone();
    // See overlay::open_all: windows can't be created directly on the event callback thread
    thread::spawn(move || {
        let state = app.state::<AppState>();
        // Use the timer's profile, since this break was started with it
        let profile = state.timer.lock().unwrap().profile().clone();
        overlay::open_all(&app, profile.coverage(false), profile.force_rest, profile.movable);
    });
}

fn broadcast(app: &AppHandle, snap: &Snapshot) {
    let _ = app.emit(STATE_EVENT, snap);
    tray::update(app, snap);
}

fn spawn_ticker(app: AppHandle) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        // Query before taking the lock: on Linux this spawns an external process
        let idle = idle::idle_time();
        let (event, snap) = {
            let state = app.state::<AppState>();
            let mut timer = state.timer.lock().unwrap();
            let now = Instant::now();
            if let Some(idle) = idle {
                timer.idle(now, idle);
            }
            timer.defer(now, chrono::Local::now().time());
            // Only check the foreground app once a break is due; the enforcement level can disable meeting detection
            let detect = timer.profile().meeting_detection;
            let held = if detect && timer.rest_due(now) {
                bypass::active_app()
            } else {
                None
            };
            timer.hold(held);
            let event = timer.tick(now);
            if detect && event == Some(TimerEvent::RestStarted) {
                timer.set_meeting_app(bypass::running_app());
            }
            (event, timer.snapshot(now))
        };
        match event {
            Some(TimerEvent::RestStarted) => open_overlays(&app),
            Some(TimerEvent::RestFinished) => {
                let cfg = app.state::<AppState>().config.lock().unwrap().clone();
                sound::announce(&cfg);
                phone::announce(&cfg);
            }
            None => {}
        }
        broadcast(&app, &snap);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Must be registered first: a second launch exits the new process immediately
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .plugin(tauri_plugin_dialog::init())
        // The default macOS menu has Cmd+Q, which would quit the app during a break; setup installs an Edit-only menu instead
        .enable_macos_default_menu(false)
        .setup(|app| {
            // On macOS, show only a menu bar icon, not a Dock icon
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            // Text fields only get Cmd+C/V/X/A/Z through the key equivalents of an Edit menu; it stays hidden under the Accessory policy
            #[cfg(target_os = "macos")]
            {
                use tauri::menu::{Menu, PredefinedMenuItem, Submenu};
                let edit = Submenu::with_items(
                    app,
                    "Edit",
                    true,
                    &[
                        &PredefinedMenuItem::undo(app, None)?,
                        &PredefinedMenuItem::redo(app, None)?,
                        &PredefinedMenuItem::separator(app)?,
                        &PredefinedMenuItem::cut(app, None)?,
                        &PredefinedMenuItem::copy(app, None)?,
                        &PredefinedMenuItem::paste(app, None)?,
                        &PredefinedMenuItem::select_all(app, None)?,
                    ],
                )?;
                app.set_menu(Menu::with_items(app, &[&edit])?)?;
            }
            let dir = app.path().app_config_dir()?;
            // Dev builds use separate config and language files so debugging doesn't affect the installed release
            let (lang_name, cfg_name) = if cfg!(debug_assertions) {
                ("language.dev", "config.dev.toml")
            } else {
                ("language", "config.toml")
            };
            // Resolve the language first: the comments in the config file generated on first run depend on it
            i18n::init(&dir.join(lang_name));
            let config_path = dir.join(cfg_name);
            let cfg = Config::load_or_create(&config_path);
            app.manage(AppState {
                timer: Mutex::new(Timer::new(&cfg, Instant::now())),
                config: Mutex::new(cfg),
                config_path,
            });
            tray::init(app.handle())?;
            spawn_ticker(app.handle().clone());
            Ok(())
        })
        .on_window_event(|window, event| {
            if !overlay::is_overlay(window.label()) {
                return;
            }
            match event {
                // Intercept system close actions such as Alt+F4; overlays can only be destroyed by the app.
                // When the break isn't forced, closing any overlay skips the break (Timer::dismiss refuses when it is forced)
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let app = window.app_handle().clone();
                    thread::spawn(move || {
                        let _ = end_rest(&app, Timer::dismiss);
                    });
                }
                // Draggable when the enforcement level allows, but can't leave its own screen
                WindowEvent::Moved(pos) => overlay::keep_on_home(window, *pos),
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_state,
            ui_lang,
            get_lang,
            set_lang,
            get_config,
            save_config,
            open_config_file,
            reload_config,
            preview_sound,
            test_phone_notify,
            pick_sound_file,
            get_autostart,
            set_autostart,
            get_about,
            open_link,
            start_work,
            postpone,
            in_meeting,
            dismiss_break,
            set_overlay_shrunk,
            dev_skip_rest
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            // Keep running in the background with no windows; only the tray's "Quit" (with an exit code) really exits
            if let RunEvent::ExitRequested { code: None, api, .. } = event {
                api.prevent_exit();
            }
        });
}
