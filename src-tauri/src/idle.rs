//! Idle detection: how long since the last keyboard or mouse input. Time away from the computer doesn't count as work

use std::time::Duration;

// Per-platform idle_time: how long the user hasn't touched the keyboard or mouse. Returns None when it can't be detected, which is treated as always active.
// Keep-display-on requests aren't used to detect video playback: many apps hold them even when not playing, which would stop idle detection from ever kicking in

#[cfg(windows)]
pub fn idle_time() -> Option<Duration> {
    use windows::Win32::{
        System::SystemInformation::GetTickCount,
        UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
    };
    let mut info = LASTINPUTINFO {
        cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };
    unsafe {
        if !GetLastInputInfo(&mut info).as_bool() {
            return None;
        }
        // Both are milliseconds since boot and wrap around about every 49.7 days, so use wrapping subtraction
        let ms = GetTickCount().wrapping_sub(info.dwTime);
        Some(Duration::from_millis(ms.into()))
    }
}

#[cfg(target_os = "macos")]
pub fn idle_time() -> Option<Duration> {
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventSourceSecondsSinceLastEventType(state: i32, event_type: u32) -> f64;
    }
    /// kCGEventSourceStateCombinedSessionState
    const COMBINED_SESSION_STATE: i32 = 0;
    /// kCGAnyInputEventType
    const ANY_INPUT_EVENT: u32 = !0;
    let secs = unsafe {
        CGEventSourceSecondsSinceLastEventType(COMBINED_SESSION_STATE, ANY_INPUT_EVENT)
    };
    Duration::try_from_secs_f64(secs).ok()
}

/// Linux: asks the desktop environment over D-Bus. Each query spawns a gdbus process,
/// so the result is cached for a few seconds and extrapolated from the elapsed time in between
#[cfg(not(any(windows, target_os = "macos")))]
pub fn idle_time() -> Option<Duration> {
    use std::{sync::Mutex, time::Instant};

    const CACHE_FOR: Duration = Duration::from_secs(5);
    static CACHE: Mutex<Option<(Instant, Option<Duration>)>> = Mutex::new(None);

    let mut cache = CACHE.lock().unwrap();
    if let Some((at, idle)) = *cache {
        if at.elapsed() < CACHE_FOR {
            return idle.map(|d| d + at.elapsed());
        }
    }
    let idle = linux::query();
    *cache = Some((Instant::now(), idle));
    idle
}

#[cfg(not(any(windows, target_os = "macos")))]
mod linux {
    use std::{process::Command, time::Duration};

    /// (service name, object path, method). Both return milliseconds
    const SOURCES: &[(&str, &str, &str)] = &[
        // GNOME (both X11 and Wayland)
        (
            "org.gnome.Mutter.IdleMonitor",
            "/org/gnome/Mutter/IdleMonitor/Core",
            "org.gnome.Mutter.IdleMonitor.GetIdletime",
        ),
        // KDE Plasma and other desktops implementing the freedesktop screensaver interface
        (
            "org.freedesktop.ScreenSaver",
            "/org/freedesktop/ScreenSaver",
            "org.freedesktop.ScreenSaver.GetSessionIdleTime",
        ),
    ];

    pub fn query() -> Option<Duration> {
        SOURCES.iter().find_map(|&(dest, path, method)| {
            let out = Command::new("gdbus")
                .args(["call", "--session", "--dest", dest])
                .args(["--object-path", path, "--method", method])
                .output()
                .ok()?;
            if !out.status.success() {
                return None;
            }
            parse(&String::from_utf8_lossy(&out.stdout)).map(Duration::from_millis)
        })
    }

    /// gdbus output looks like `(uint64 12345,)`
    fn parse(text: &str) -> Option<u64> {
        let digits = text.trim().trim_start_matches('(').split_whitespace().nth(1)?;
        digits.trim_end_matches([',', ')']).parse().ok()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn parses_gdbus_output() {
            assert_eq!(super::parse("(uint64 12345,)\n"), Some(12345));
            assert_eq!(super::parse("(uint32 7,)"), Some(7));
            assert_eq!(super::parse("()"), None);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn reads_idle_time() {
        // Only needs not to crash; printed for manual checking
        println!("idle: {:?}", super::idle_time());
    }
}
