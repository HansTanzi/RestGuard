//! 空闲检测：距离上一次键盘鼠标输入过了多久。人离开电脑的时间不算工作

use std::time::Duration;

// 各平台的 idle_time：用户已经多久没有操作键盘鼠标。无法检测时返回 None，此时按一直在操作处理。
// 不参考“屏幕常亮”请求来识别看视频：很多应用没在播放时也一直持有它，会让空闲检测永远不生效

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
        // 两者都是开机后的毫秒数，约 49.7 天回绕一次，用回绕减法
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

/// Linux：通过 D-Bus 询问桌面环境。每次都要启动 gdbus 进程，
/// 所以结果缓存几秒，期间按经过的时间推算
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

    /// (服务名, 对象路径, 方法)。两者返回的都是毫秒
    const SOURCES: &[(&str, &str, &str)] = &[
        // GNOME（X11 和 Wayland 都支持）
        (
            "org.gnome.Mutter.IdleMonitor",
            "/org/gnome/Mutter/IdleMonitor/Core",
            "org.gnome.Mutter.IdleMonitor.GetIdletime",
        ),
        // KDE Plasma 等实现了 freedesktop 屏保接口的桌面
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

    /// gdbus 的输出形如 `(uint64 12345,)`
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
        // 只要不崩溃即可；打印出来方便手动核对
        println!("idle: {:?}", super::idle_time());
    }
}
