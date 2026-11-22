//! Auto-lock when away: once the user counts as away while working (no input for the idle threshold), a prompt opens
//! on every display, and the screen locks unless one of them is clicked within PROMPT_TIME. Breaks have their own
//! auto-lock (see the ticker in lib.rs); this one only runs while working.
//! The prompt loads the frontend /lock-prompt route (src/routes/lock-prompt/+page.svelte).

use crate::{lock, overlay, timer::ceil_secs};
use std::{
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const PREFIX: &str = "lockprompt-";
/// How long the prompt waits for a click before locking; the "30 seconds" in the texts of src/lib/i18n.ts and the config comment match it
const PROMPT_TIME: Duration = Duration::from_secs(30);
/// Payload is the whole seconds left before locking; matches LEFT_EVENT in src/routes/lock-prompt/+page.svelte
const LEFT_EVENT: &str = "lock-prompt:left";
/// Logical size of each prompt window; the frontend asks for more height when its content needs it (see fit)
const WIDTH: f64 = 460.0;
const HEIGHT: f64 = 280.0;

struct State {
    /// When the screen locks; Some while the prompt is showing
    deadline: Option<Instant>,
    /// Whether the next stretch away may prompt. Cleared on prompting and set again once the user is back,
    /// so one stretch away prompts only once, however it ended
    armed: bool,
}

static STATE: Mutex<State> = Mutex::new(State {
    deadline: None,
    armed: true,
});

/// Held while prompts are being opened, so two openings can't both create the same window (see overlay::OPENING)
static OPENING: Mutex<()> = Mutex::new(());

pub fn is_prompt(label: &str) -> bool {
    label.starts_with(PREFIX)
}

/// Called by the ticker every round. `watch`: auto-lock when away is on and the user is working;
/// `away`: no input for the idle threshold
pub fn tick(app: &AppHandle, watch: bool, away: bool) {
    let mut state = STATE.lock().unwrap();
    if !away {
        state.armed = true;
    }
    match state.deadline {
        // Turned off, or a break started meanwhile: drop the prompt without locking
        Some(_) if !watch => {
            state.deadline = None;
            drop(state);
            close_all(app);
        }
        // Only a click counts: input without one (say, a bumped mouse) still locks
        Some(deadline) => {
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                state.deadline = None;
                drop(state);
                close_all(app);
                thread::spawn(lock::lock_screen);
            } else {
                drop(state);
                let _ = app.emit(LEFT_EVENT, ceil_secs(left));
            }
        }
        None if watch && away && state.armed => {
            state.armed = false;
            state.deadline = Some(Instant::now() + PROMPT_TIME);
            drop(state);
            let app = app.clone();
            // See overlay::open_all: windows can't be created on an event callback thread, and opening shouldn't hold up the ticker
            thread::spawn(move || open_all(&app));
        }
        None => {}
    }
}

/// Seconds left before locking, None when no prompt is showing
pub fn left_secs() -> Option<u64> {
    let state = STATE.lock().unwrap();
    state.deadline.map(|d| ceil_secs(d.saturating_duration_since(Instant::now())))
}

/// "Cancel" on the prompt, or closing one: no lock this time
pub fn cancel(app: &AppHandle) {
    STATE.lock().unwrap().deadline = None;
    close_all(app);
}

/// "Lock now" on the prompt
pub fn lock_now(app: &AppHandle) {
    cancel(app);
    thread::spawn(lock::lock_screen);
}

/// The prompt's content needs `height` logical pixels (long texts wrap more in some languages): grows the prompt to
/// fit and centers it again on its display. Never shrinks below HEIGHT
pub fn fit(window: &WebviewWindow, height: f64) {
    if !is_prompt(window.label()) || !height.is_finite() {
        return;
    }
    let Ok(Some(monitor)) = window.current_monitor() else {
        return;
    };
    // centered_on also caps it at the display's height
    let (pos, size) = overlay::centered_on(&monitor, WIDTH, height.max(HEIGHT));
    let _ = window.set_position(pos);
    let _ = window.set_size(size);
}

fn showing() -> bool {
    STATE.lock().unwrap().deadline.is_some()
}

fn open_all(app: &AppHandle) {
    let _guard = OPENING.lock().unwrap_or_else(|e| e.into_inner());
    // Cancelled before this thread ran
    if !showing() {
        return;
    }
    let monitors = match app.available_monitors() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to get displays: {e}");
            return;
        }
    };
    let primary_pos = app.primary_monitor().ok().flatten().map(|m| *m.position());
    for (i, monitor) in monitors.iter().enumerate() {
        let label = format!("{PREFIX}{i}");
        if app.get_webview_window(&label).is_some() {
            continue;
        }
        let built = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("lock-prompt".into()))
            .title("RestGuard")
            .decorations(false)
            .always_on_top(true)
            .visible_on_all_workspaces(true)
            .skip_taskbar(true)
            .resizable(false)
            .minimizable(false)
            .maximizable(false)
            .focused(false)
            .visible(false)
            .build();
        match built {
            Ok(window) => {
                let (pos, size) = overlay::centered_on(monitor, WIDTH, HEIGHT);
                // Move first, then size (as overlay::open_all does): the window is created on the primary display, and
                // moving it to a display of another scale makes Windows rescale it by the ratio of the two scales,
                // which would shrink or grow a size set before the move
                let _ = window.set_position(pos);
                let _ = window.set_size(size);
                let _ = window.show();
                // Focus the one on the primary screen so Esc and Enter work there
                if primary_pos.is_none_or(|p| p == *monitor.position()) {
                    let _ = window.set_focus();
                }
            }
            Err(e) => eprintln!("Failed to create lock prompt {label}: {e}"),
        }
    }
    // Cancelled or timed out while windows were being created; close_all then missed the ones made after it
    if !showing() {
        close_all(app);
    }
}

fn close_all(app: &AppHandle) {
    for (label, window) in app.webview_windows() {
        if is_prompt(&label) {
            // destroy doesn't fire CloseRequested, which would count as cancelling
            let _ = window.destroy();
        }
    }
}
