//! Auto-lock: locks the screen when a break starts, so the user has to get up and unlock it. The overlay stays open behind the lock screen
//! and shows the rest of the break after unlocking. Failures are only logged; the overlay still enforces the break.

/// Locks the screen. Blocks briefly on Linux (runs an external command), so call it off the main thread
pub fn lock_screen() {
    if let Err(e) = imp::lock() {
        eprintln!("Failed to lock the screen: {e}");
    }
}

#[cfg(windows)]
mod imp {
    pub fn lock() -> Result<(), String> {
        unsafe { windows::Win32::System::Shutdown::LockWorkStation() }.map_err(|e| e.to_string())
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use std::{
        ffi::{c_char, c_int, c_void},
        process::Command,
    };

    extern "C" {
        fn dlopen(path: *const c_char, mode: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }
    const RTLD_LAZY: c_int = 1;

    /// SACLockScreenImmediate from the private login framework is what Ctrl+Cmd+Q calls; it has no public equivalent.
    /// Loaded at runtime so a missing symbol on a future macOS falls back instead of failing to launch
    pub fn lock() -> Result<(), String> {
        unsafe {
            let handle = dlopen(c"/System/Library/PrivateFrameworks/login.framework/Versions/Current/login".as_ptr(), RTLD_LAZY);
            if !handle.is_null() {
                let sym = dlsym(handle, c"SACLockScreenImmediate".as_ptr());
                if !sym.is_null() {
                    let lock: extern "C" fn() -> c_int = std::mem::transmute(sym);
                    lock();
                    return Ok(());
                }
            }
        }
        // Sleeping the display only locks when "Require password immediately" is set, but beats doing nothing
        let status = Command::new("pmset").arg("displaysleepnow").status().map_err(|e| e.to_string())?;
        status.success().then_some(()).ok_or_else(|| format!("pmset exited with {status}"))
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
mod imp {
    use std::process::Command;

    /// systemd-logind asks the desktop's locker to lock; xdg-screensaver covers desktops without logind
    const COMMANDS: &[&[&str]] = &[&["loginctl", "lock-session"], &["xdg-screensaver", "lock"]];

    pub fn lock() -> Result<(), String> {
        for args in COMMANDS {
            if let Ok(status) = Command::new(args[0]).args(&args[1..]).status() {
                if status.success() {
                    return Ok(());
                }
            }
        }
        Err("neither loginctl nor xdg-screensaver could lock the session".into())
    }
}
