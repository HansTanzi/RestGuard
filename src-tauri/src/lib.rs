mod about;
mod away_lock;
mod bypass;
mod config;
mod i18n;
mod idle;
mod lock;
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
/// Total time the overlay may stay shrunk per break; then it restores itself and can't be shrunk again,
/// so urgent mode can't be used to sit out the whole break
const URGENT_LIMIT: Duration = Duration::from_secs(2 * 60);
/// How often to try opening the overlay again when a break has none, e.g. it started while the displays were asleep
const OVERLAY_RETRY: Duration = Duration::from_secs(5);

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
    with_urgent(snap)
}

/// Adds the urgent mode state, which lives with the overlays rather than the timer
fn with_urgent(mut snap: Snapshot) -> Snapshot {
    let left = overlay::urgent_left(URGENT_LIMIT);
    snap.can_shrink &= !left.is_zero();
    snap.urgent_left_secs = overlay::is_shrunk().then(|| timer::ceil_secs(left));
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

/// Links on the About page: link is "website", "repo", "issues", "discussions", "email" or "privacy".
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

/// "Keep using the computer" on the overlay: leaves a long fixed break once a regular break's length has passed
#[tauri::command]
fn keep_using(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::keep_using)
}

/// "Skip this break" on the overlay; refused when the enforcement level doesn't allow skipping
#[tauri::command]
fn dismiss_break(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::skip)
}

/// The overlay's "shrink" button: shrinks the overlay to deal with something urgent without ending the break; the break starts over and pauses meanwhile.
/// Refused when the current enforcement level doesn't allow urgent mode.
/// async so it runs off the main thread, avoiding a hang while resizing windows on Windows
#[tauri::command]
async fn set_overlay_shrunk(app: AppHandle, shrunk: bool) -> Result<(), String> {
    if shrunk && !get_state(app.clone()).can_shrink {
        return Err("Urgent mode is not allowed now, or its time in this break is used up".into());
    }
    shrink_overlays(&app, shrunk);
    Ok(())
}

fn shrink_overlays(app: &AppHandle, shrunk: bool) {
    let state = app.state::<AppState>();
    // Dealing with something urgent interrupts the break: it starts over and waits until the overlay is restored
    state.timer.lock().unwrap().pause_rest(Instant::now(), shrunk);
    let coverage = state.overlay_coverage(shrunk);
    overlay::set_coverage(app, coverage, shrunk);
    // Notify all overlays (including secondary screens) to update their draggable state
    let _ = app.emit(SHRINK_EVENT, shrunk);
}

/// Seconds left before the away-lock prompt locks the screen; None when it isn't showing
#[tauri::command]
fn lock_prompt_left() -> Option<u64> {
    away_lock::left_secs()
}

/// "Cancel" on the away-lock prompt
#[tauri::command]
fn cancel_lock_prompt(app: AppHandle) {
    away_lock::cancel(&app);
}

/// "Lock now" on the away-lock prompt
#[tauri::command]
fn lock_prompt_now(app: AppHandle) {
    away_lock::lock_now(&app);
}

/// The away-lock prompt's content needs `height` logical pixels; grows that prompt to fit
#[tauri::command]
fn fit_lock_prompt(window: tauri::WebviewWindow, height: f64) {
    away_lock::fit(&window, height);
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

/// Tray status line clicked once the break is over: same as "Start working" on the overlay
pub(crate) fn start_work_now(app: &AppHandle) {
    let _ = end_rest(app, Timer::start_work);
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

/// Tray "Pause": no breaks until the given local time; refused when quitting would be
pub(crate) fn pause(app: &AppHandle, until: chrono::NaiveDateTime) {
    update_timer(app, |timer, _| timer.pause(until));
}

/// Tray "Resume": ends a pause early
pub(crate) fn resume(app: &AppHandle) {
    update_timer(app, Timer::resume);
}

fn update_timer(app: &AppHandle, action: impl FnOnce(&mut Timer, Instant) -> bool) {
    let snap = {
        let state = app.state::<AppState>();
        let mut timer = state.timer.lock().unwrap();
        let now = Instant::now();
        if !action(&mut timer, now) {
            return;
        }
        timer.snapshot(now)
    };
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
        let working = || state.timer.lock().unwrap().snapshot(Instant::now()).phase == Phase::Working;
        // Serialized so a second call (tray "Rest now" racing the ticker's retry) only skips windows that already exist
        overlay::while_opening(|| {
            // The break may have ended before this thread ran
            if working() {
                return;
            }
            // Use the timer's profile, since this break was started with it
            let profile = state.timer.lock().unwrap().profile().clone();
            overlay::open_all(&app, profile.coverage(false), profile.force_rest, profile.movable);
            // The break may have ended while windows were being created; close_all then missed the ones made after it
            if working() {
                overlay::close_all(&app);
            }
        });
    });
}

fn broadcast(app: &AppHandle, snap: &Snapshot) {
    let snap = &with_urgent(snap.clone());
    let _ = app.emit(STATE_EVENT, snap);
    tray::update(app, snap);
}

fn spawn_ticker(app: AppHandle) {
    // Whether the screen was already locked in this break; the user may unlock and keep resting at the overlay
    let mut locked = false;
    // Last attempt to open the overlay during this break
    let mut opened_at: Option<Instant> = None;
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        // Query before taking the lock: on Linux this spawns an external process
        let idle = idle::idle_time();
        let (event, snap) = {
            let state = app.state::<AppState>();
            let mut timer = state.timer.lock().unwrap();
            let now = Instant::now();
            let local = chrono::Local::now();
            timer.wake(now, local.naive_local());
            if let Some(idle) = idle {
                timer.idle(now, idle);
            }
            timer.defer(now, local.time());
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
            Some(TimerEvent::RestStarted) => {
                opened_at = Some(Instant::now());
                open_overlays(&app);
            }
            Some(TimerEvent::RestFinished) => {
                let cfg = app.state::<AppState>().config.lock().unwrap().clone();
                sound::announce(&cfg);
                phone::announce(&cfg);
            }
            None => {}
        }
        // A break without an overlay can't be ended, as "Start working" is only on the overlay. Opening fails while
        // displays are asleep (macOS drops sleeping external displays), so keep retrying until one shows
        if snap.phase == Phase::Working {
            opened_at = None;
        } else if !overlay::any_open(&app) && opened_at.is_none_or(|at| at.elapsed() >= OVERLAY_RETRY) {
            opened_at = Some(Instant::now());
            open_overlays(&app);
        }
        // Urgent time used up: restore the overlay; auto-lock below then applies again
        if overlay::is_shrunk() && overlay::urgent_left(URGENT_LIMIT).is_zero() {
            shrink_overlays(&app, false);
        }
        {
            let cfg = app.state::<AppState>().config.lock().unwrap().clone();
            let away = cfg.idle().zip(idle).is_some_and(|(threshold, idle)| idle >= threshold);
            away_lock::tick(&app, cfg.idle_lock() && snap.phase == Phase::Working, away);
        }
        if snap.phase != Phase::Resting {
            locked = false;
        } else if !locked && !overlay::is_shrunk() {
            // Never while shrunk for urgent work; once restored, locks right away if the delay has passed
            let delay = app.state::<AppState>().config.lock().unwrap().lock_delay();
            let elapsed = Duration::from_secs(snap.total_secs.saturating_sub(snap.remaining_secs));
            if delay.is_some_and(|d| elapsed >= d) {
                locked = true;
                thread::spawn(lock::lock_screen);
            }
        }
        broadcast(&app, &snap);
    });
}

/// The single-instance plugin makes a second launch exit silently; say why on the console before that happens.
/// Dev builds share the identifier with the release, so this also covers `tauri dev` while an installed RestGuard runs.
fn warn_if_already_running(identifier: &str) {
    if !already_running(identifier) {
        return;
    }
    // Release builds on Windows have no console of their own: attach to the launching terminal's, if any.
    // A double-click has none; the dialog the running instance shows is the cue there.
    #[cfg(all(windows, not(debug_assertions)))]
    unsafe {
        use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
    eprintln!(
        "WARNING: another RestGuard ({identifier}) is already running, so this instance will exit. \
         Quit the other one from its tray menu first."
    );
}

/// Run by the running instance when RestGuard is launched again. Non-blocking, and at most one dialog at a time,
/// so repeated launches don't stack dialogs up.
fn show_already_running(app: &AppHandle) {
    use std::sync::atomic::{AtomicBool, Ordering};
    static SHOWING: AtomicBool = AtomicBool::new(false);
    if SHOWING.swap(true, Ordering::SeqCst) {
        return;
    }
    #[cfg(windows)]
    {
        // The work area of the display under the cursor: that's where the user just launched RestGuard from.
        // An ownerless message box would otherwise open on the primary display, easy to miss with several
        let area = app
            .cursor_position()
            .ok()
            .and_then(|p| app.monitor_from_point(p.x, p.y).ok().flatten())
            .map(|m| *m.work_area());
        thread::spawn(move || {
            already_running_box(area);
            SHOWING.store(false, Ordering::SeqCst);
        });
    }
    // AppKit and GTK only work on the main thread
    #[cfg(target_os = "macos")]
    if app
        .run_on_main_thread(|| {
            already_running_alert();
            SHOWING.store(false, Ordering::SeqCst);
        })
        .is_err()
    {
        SHOWING.store(false, Ordering::SeqCst);
    }
    #[cfg(target_os = "linux")]
    if app.run_on_main_thread(|| already_running_dialog(|| SHOWING.store(false, Ordering::SeqCst))).is_err() {
        SHOWING.store(false, Ordering::SeqCst);
    }
    #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
    {
        use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
        app.dialog()
            .message(i18n::t(&i18n::text::ALREADY_RUNNING))
            .title("RestGuard")
            .kind(MessageDialogKind::Info)
            .show(|_| SHOWING.store(false, Ordering::SeqCst));
    }
}

/// Shows the "already running" message box, blocking until it's dismissed, centered on `area` (physical pixels) and
/// above other windows. A CBT hook on this thread moves the box when it's activated, before it's first painted.
#[cfg(windows)]
fn already_running_box(area: Option<tauri::PhysicalRect<i32, u32>>) {
    use std::cell::Cell;
    use windows::{
        core::HSTRING,
        Win32::{
            Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
            System::Threading::GetCurrentThreadId,
            UI::WindowsAndMessaging::{
                CallNextHookEx, GetWindowRect, MessageBoxW, SetWindowPos, SetWindowsHookExW, UnhookWindowsHookEx,
                HCBT_ACTIVATE, HHOOK, MB_ICONINFORMATION, MB_OK, MB_SETFOREGROUND, MB_TOPMOST, SWP_NOACTIVATE,
                SWP_NOSIZE, SWP_NOZORDER, WH_CBT,
            },
        },
    };
    thread_local! {
        static HOOK: Cell<Option<(HHOOK, tauri::PhysicalRect<i32, u32>)>> = const { Cell::new(None) };
    }
    unsafe extern "system" fn on_cbt(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code == HCBT_ACTIVATE as i32 {
            if let Some((hook, area)) = HOOK.take() {
                let hwnd = HWND(wparam.0 as _);
                // Twice: the box is laid out for the primary display, and moving it to one of another scale makes
                // Windows rescale it, so the second pass centers it at its new size
                for _ in 0..2 {
                    let mut rect = RECT::default();
                    if unsafe { GetWindowRect(hwnd, &mut rect) }.is_err() {
                        break;
                    }
                    let (w, h) = (rect.right - rect.left, rect.bottom - rect.top);
                    let (aw, ah) = (area.size.width as i32, area.size.height as i32);
                    let x = area.position.x + ((aw - w) / 2).max(0);
                    let y = area.position.y + ((ah - h) / 2).max(0);
                    if (x, y) == (rect.left, rect.top) {
                        break;
                    }
                    let flags = SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE;
                    let _ = unsafe { SetWindowPos(hwnd, None, x, y, 0, 0, flags) };
                }
                let _ = unsafe { UnhookWindowsHookEx(hook) };
            }
        }
        unsafe { CallNextHookEx(None, code, wparam, lparam) }
    }
    if let Some(area) = area {
        if let Ok(hook) = unsafe { SetWindowsHookExW(WH_CBT, Some(on_cbt), None, GetCurrentThreadId()) } {
            HOOK.set(Some((hook, area)));
        }
    }
    let text = HSTRING::from(i18n::t(&i18n::text::ALREADY_RUNNING));
    let flags = MB_OK | MB_ICONINFORMATION | MB_TOPMOST | MB_SETFOREGROUND;
    unsafe { MessageBoxW(None, &text, &HSTRING::from("RestGuard"), flags) };
    // The box never activated (it failed to open): don't leave the hook behind
    if let Some((hook, _)) = HOOK.take() {
        let _ = unsafe { UnhookWindowsHookEx(hook) };
    }
}

/// Shows the "already running" alert on the screen under the mouse, blocking until it's dismissed. Main thread only.
/// An alert with no parent window otherwise opens on the main screen, easy to miss with several
#[cfg(target_os = "macos")]
fn already_running_alert() {
    use objc2_app_kit::{NSAlert, NSApplication, NSEvent, NSScreen};
    use objc2_foundation::{MainThreadMarker, NSPoint, NSString};
    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let alert = NSAlert::new(mtm);
    alert.setMessageText(&NSString::from_str("RestGuard"));
    alert.setInformativeText(&NSString::from_str(i18n::t(&i18n::text::ALREADY_RUNNING)));
    // Lay out now rather than in runModal, so the window has its final size to center
    alert.layout();
    let window = alert.window();
    // Both in global points with the origin at the bottom left of the main screen
    let mouse = NSEvent::mouseLocation();
    let screen = NSScreen::screens(mtm).iter().find(|s| {
        let f = s.frame();
        (f.origin.x..f.origin.x + f.size.width).contains(&mouse.x)
            && (f.origin.y..f.origin.y + f.size.height).contains(&mouse.y)
    });
    if let Some(screen) = screen {
        let area = screen.visibleFrame();
        let size = window.frame().size;
        window.setFrameOrigin(NSPoint::new(
            area.origin.x + (area.size.width - size.width).max(0.0) / 2.0,
            area.origin.y + (area.size.height - size.height).max(0.0) / 2.0,
        ));
    }
    // RestGuard runs as an accessory app that's normally inactive: bring the alert in front of other apps so that
    // Return dismisses it. The replacement, activate, needs macOS 14
    #[allow(deprecated)]
    NSApplication::sharedApplication(mtm).activateIgnoringOtherApps(true);
    alert.runModal();
}

/// Shows the "already running" dialog without blocking and calls `closed` once it's dismissed. Main thread only.
/// GTK centers it on the monitor under the pointer; on Wayland the compositor places it instead
#[cfg(target_os = "linux")]
fn already_running_dialog(closed: impl Fn() + 'static) {
    use gtk::prelude::*;
    let dialog = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::empty(),
        gtk::MessageType::Info,
        gtk::ButtonsType::Ok,
        i18n::t(&i18n::text::ALREADY_RUNNING),
    );
    dialog.set_title("RestGuard");
    dialog.set_position(gtk::WindowPosition::Center);
    dialog.set_keep_above(true);
    dialog.connect_response(move |dialog, _| {
        dialog.close();
        closed();
    });
    dialog.present();
}

/// Probes the named mutex tauri-plugin-single-instance holds on Windows, so it also detects
/// releases that predate this check.
#[cfg(windows)]
fn already_running(identifier: &str) -> bool {
    use windows::{
        core::HSTRING,
        Win32::{
            Foundation::CloseHandle,
            System::Threading::{OpenMutexW, SYNCHRONIZATION_SYNCHRONIZE},
        },
    };
    let name = HSTRING::from(format!("{identifier}-sim"));
    match unsafe { OpenMutexW(SYNCHRONIZATION_SYNCHRONIZE, false, &name) } {
        Ok(handle) => {
            let _ = unsafe { CloseHandle(handle) };
            true
        }
        Err(_) => false,
    }
}

/// The plugin's state isn't reachable from outside on macOS and Linux, so each instance holds an exclusive
/// lock on a per-user file for its whole lifetime; the OS releases it when the process exits, even on a crash.
#[cfg(not(windows))]
fn already_running(identifier: &str) -> bool {
    use std::{
        fs::{File, TryLockError},
        sync::OnceLock,
    };
    static LOCK: OnceLock<File> = OnceLock::new();
    // Per-user on both: XDG_RUNTIME_DIR on Linux (/tmp is shared between users), $TMPDIR on macOS
    let dir = std::env::var_os("XDG_RUNTIME_DIR").map_or_else(std::env::temp_dir, PathBuf::from);
    let Ok(file) = File::create(dir.join(format!("{identifier}.lock"))) else {
        return false;
    };
    match file.try_lock() {
        Ok(()) => {
            let _ = LOCK.set(file);
            false
        }
        Err(TryLockError::WouldBlock) => true,
        Err(TryLockError::Error(_)) => false,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    warn_if_already_running(&context.config().identifier);
    tauri::Builder::default()
        // Must be registered first: a second launch exits the new process immediately, and the running
        // instance shows a message so the user sees that RestGuard is already running
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_already_running(app);
        }))
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
            // Closing an away-lock prompt (e.g. Alt+F4) counts as cancelling it
            if away_lock::is_prompt(window.label()) {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let app = window.app_handle().clone();
                    thread::spawn(move || away_lock::cancel(&app));
                }
                return;
            }
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
            keep_using,
            dismiss_break,
            set_overlay_shrunk,
            lock_prompt_left,
            cancel_lock_prompt,
            lock_prompt_now,
            fit_lock_prompt,
            dev_skip_rest
        ])
        .build(context)
        .expect("error while building tauri application")
        .run(|_app, event| {
            // Keep running in the background with no windows; only the tray's "Quit" (with an exit code) really exits
            if let RunEvent::ExitRequested { code: None, api, .. } = event {
                api.prevent_exit();
            }
        });
}
