//! Exempt apps: when a break is due and an app that shouldn't be interrupted (such as a meeting) is in the foreground or using the microphone, the forced break is held off
//! until it's no longer in use. Only a built-in list for now; it could be made user-configurable later

struct App {
    /// Name shown in the tray
    name: &'static str,
    /// Windows process names, case-insensitive
    exe: &'static [&'static str],
    /// Package family names of Microsoft Store (MSIX) versions, used to detect microphone use
    #[cfg_attr(not(windows), allow(dead_code))]
    package: &'static [&'static str],
    /// macOS bundle identifier
    bundle_id: &'static [&'static str],
}

const BUILTIN: &[App] = &[
    App {
        name: "Zoom",
        exe: &["Zoom.exe"],
        package: &[],
        bundle_id: &["us.zoom.xos"],
    },
    App {
        name: "Microsoft Teams",
        // ms-teams.exe is the new Teams (installed as an MSIX package), Teams.exe is Teams classic
        exe: &["ms-teams.exe", "Teams.exe"],
        package: &["MSTeams_8wekyb3d8bbwe"],
        bundle_id: &["com.microsoft.teams2", "com.microsoft.teams"],
    },
    App {
        name: "Webex",
        exe: &["CiscoCollabHost.exe", "webex.exe", "atmgr.exe"],
        package: &[],
        bundle_id: &["Cisco-Systems.Spark"],
    },
    App {
        name: "腾讯会议",
        exe: &["wemeetapp.exe"],
        package: &[],
        bundle_id: &["com.tencent.meeting"],
    },
];

/// Returns the name of an app on the exemption list that is in the foreground or using the microphone
pub fn active_app() -> Option<String> {
    foreground_app()
        .or_else(mic_app)
        .map(|app| app.name.to_string())
}

fn foreground_app() -> Option<&'static App> {
    let id = foreground_id()?;
    let ids = |app: &App| {
        if cfg!(target_os = "macos") {
            app.bundle_id
        } else {
            app.exe
        }
    };
    BUILTIN.iter().find(|app| contains(ids(app), &id))
}

fn contains(list: &[&str], id: &str) -> bool {
    list.iter().any(|x| x.eq_ignore_ascii_case(id))
}

/// Windows: executable name of the process owning the foreground window
#[cfg(windows)]
fn foreground_id() -> Option<String> {
    use windows::{
        core::PWSTR,
        Win32::{
            Foundation::CloseHandle,
            System::Threading::{
                OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
                PROCESS_QUERY_LIMITED_INFORMATION,
            },
            UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
        },
    };
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            return None;
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return None;
        }
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = [0u16; 1024];
        let mut len = buf.len() as u32;
        let result = QueryFullProcessImageNameW(
            process,
            PROCESS_NAME_WIN32,
            PWSTR(buf.as_mut_ptr()),
            &mut len,
        );
        let _ = CloseHandle(process);
        result.ok()?;
        let path = String::from_utf16_lossy(&buf[..len as usize]);
        path.rsplit('\\').next().map(str::to_string)
    }
}

/// macOS: bundle identifier of the frontmost app
#[cfg(target_os = "macos")]
fn foreground_id() -> Option<String> {
    use objc2_app_kit::NSWorkspace;
    #[allow(unused_unsafe)]
    unsafe {
        let app = NSWorkspace::sharedWorkspace().frontmostApplication()?;
        Some(app.bundleIdentifier()?.to_string())
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
fn foreground_id() -> Option<String> {
    None
}

/// Windows: an exempt app currently using the microphone.
/// If an app crashes, the system record may stay marked as in use, so its process must also still be running
#[cfg(windows)]
fn mic_app() -> Option<&'static App> {
    let users = win::mic_users();
    if users.is_empty() {
        return None;
    }
    let running = win::running_exes();
    BUILTIN.iter().find(|app| {
        users
            .iter()
            .any(|id| contains(app.exe, id) || contains(app.package, id))
            && running.iter().any(|exe| contains(app.exe, exe))
    })
}

/// macOS: an exempt app currently using the microphone.
/// macOS 14.2+ reports which process is recording; older versions only say the microphone is in use,
/// so then any running exempt app is assumed to be the one using it
#[cfg(target_os = "macos")]
fn mic_app() -> Option<&'static App> {
    match mac::mic_users() {
        Some(users) => BUILTIN.iter().find(|app| {
            users
                .iter()
                .any(|id| app.bundle_id.iter().any(|b| mac::same_app(b, id)))
        }),
        None if mac::mic_in_use() => running(),
        None => None,
    }
}

/// Detecting microphone use isn't supported on other platforms yet
#[cfg(not(any(windows, target_os = "macos")))]
fn mic_app() -> Option<&'static App> {
    None
}

/// An exempt app that is running (not necessarily in a meeting)
#[cfg(windows)]
pub fn running_app() -> Option<String> {
    let running = win::running_exes();
    BUILTIN
        .iter()
        .find(|app| running.iter().any(|exe| contains(app.exe, exe)))
        .map(|app| app.name.to_string())
}

/// An exempt app that is running (not necessarily in a meeting)
#[cfg(target_os = "macos")]
pub fn running_app() -> Option<String> {
    running().map(|app| app.name.to_string())
}

#[cfg(target_os = "macos")]
fn running() -> Option<&'static App> {
    BUILTIN
        .iter()
        .find(|app| app.bundle_id.iter().any(|id| mac::is_running(id)))
}

/// Detecting running processes isn't supported on other platforms yet
#[cfg(not(any(windows, target_os = "macos")))]
pub fn running_app() -> Option<String> {
    None
}

#[cfg(target_os = "macos")]
mod mac {
    use objc2_app_kit::NSRunningApplication;
    use objc2_core_foundation::{CFRetained, CFString};
    use objc2_foundation::NSString;
    use std::{ffi::c_void, ptr::NonNull};

    type AudioObjectId = u32;

    #[repr(C)]
    struct PropertyAddress {
        selector: u32,
        scope: u32,
        element: u32,
    }

    #[link(name = "CoreAudio", kind = "framework")]
    extern "C" {
        fn AudioObjectGetPropertyDataSize(
            object: AudioObjectId,
            address: *const PropertyAddress,
            qualifier_size: u32,
            qualifier: *const c_void,
            size: *mut u32,
        ) -> i32;
        fn AudioObjectGetPropertyData(
            object: AudioObjectId,
            address: *const PropertyAddress,
            qualifier_size: u32,
            qualifier: *const c_void,
            size: *mut u32,
            data: *mut c_void,
        ) -> i32;
    }

    const fn fourcc(code: &[u8; 4]) -> u32 {
        u32::from_be_bytes(*code)
    }

    const SYSTEM_OBJECT: AudioObjectId = 1;
    const SCOPE_GLOBAL: u32 = fourcc(b"glob");
    /// kAudioHardwarePropertyProcessObjectList, macOS 14.2+
    const PROCESS_OBJECT_LIST: u32 = fourcc(b"prs#");
    /// kAudioProcessPropertyBundleID
    const PROCESS_BUNDLE_ID: u32 = fourcc(b"pbid");
    /// kAudioProcessPropertyIsRunningInput
    const PROCESS_IS_RUNNING_INPUT: u32 = fourcc(b"piri");
    /// kAudioHardwarePropertyDefaultInputDevice
    const DEFAULT_INPUT_DEVICE: u32 = fourcc(b"dIn ");
    /// kAudioDevicePropertyDeviceIsRunningSomewhere
    const DEVICE_IS_RUNNING_SOMEWHERE: u32 = fourcc(b"gone");

    fn address(selector: u32) -> PropertyAddress {
        PropertyAddress {
            selector,
            scope: SCOPE_GLOBAL,
            element: 0,
        }
    }

    /// Reads a fixed-size property; None if the object doesn't have it
    fn get<T: Default>(object: AudioObjectId, selector: u32) -> Option<T> {
        let mut data = T::default();
        let mut size = std::mem::size_of::<T>() as u32;
        let status = unsafe {
            AudioObjectGetPropertyData(
                object,
                &address(selector),
                0,
                std::ptr::null(),
                &mut size,
                &mut data as *mut T as *mut c_void,
            )
        };
        (status == 0).then_some(data)
    }

    /// Bundle identifiers of the processes currently recording audio.
    /// None if the system can't tell (before macOS 14.2)
    pub fn mic_users() -> Option<Vec<String>> {
        let list = address(PROCESS_OBJECT_LIST);
        let mut size = 0u32;
        let status = unsafe {
            AudioObjectGetPropertyDataSize(SYSTEM_OBJECT, &list, 0, std::ptr::null(), &mut size)
        };
        if status != 0 {
            return None;
        }
        let mut processes = vec![0 as AudioObjectId; size as usize / 4];
        let status = unsafe {
            AudioObjectGetPropertyData(
                SYSTEM_OBJECT,
                &list,
                0,
                std::ptr::null(),
                &mut size,
                processes.as_mut_ptr() as *mut c_void,
            )
        };
        if status != 0 {
            return None;
        }
        processes.truncate(size as usize / 4);
        let users = processes
            .into_iter()
            .filter(|&p| get::<u32>(p, PROCESS_IS_RUNNING_INPUT).unwrap_or(0) != 0)
            .filter_map(bundle_id)
            .collect();
        Some(users)
    }

    fn bundle_id(process: AudioObjectId) -> Option<String> {
        // The returned CFString is owned by the caller
        let raw = get::<usize>(process, PROCESS_BUNDLE_ID)?;
        let string = NonNull::new(raw as *mut CFString)?;
        let string = unsafe { CFRetained::from_raw(string) };
        Some(string.to_string())
    }

    /// Whether some process is recording from the default microphone
    pub fn mic_in_use() -> bool {
        get::<AudioObjectId>(SYSTEM_OBJECT, DEFAULT_INPUT_DEVICE)
            .filter(|&device| device != 0)
            .and_then(|device| get::<u32>(device, DEVICE_IS_RUNNING_SOMEWHERE))
            .is_some_and(|running| running != 0)
    }

    /// Recording is often done by a helper process whose bundle identifier extends the app's
    pub fn same_app(app: &str, id: &str) -> bool {
        id.eq_ignore_ascii_case(app)
            || id.len() > app.len()
                && id.as_bytes()[app.len()] == b'.'
                && id[..app.len()].eq_ignore_ascii_case(app)
    }

    pub fn is_running(bundle_id: &str) -> bool {
        let id = NSString::from_str(bundle_id);
        !NSRunningApplication::runningApplicationsWithBundleIdentifier(&id).is_empty()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn matches_helper_bundle_ids() {
            assert!(same_app("us.zoom.xos", "us.zoom.xos"));
            assert!(same_app("com.microsoft.teams2", "com.microsoft.teams2.helper"));
            assert!(!same_app("com.microsoft.teams", "com.microsoft.teams2"));
            assert!(!same_app("us.zoom.xos", "us.zoom"));
        }

        #[test]
        fn reads_audio_processes() {
            // Only needs to read without crashing; printed for manual checking during a meeting
            println!("mic users: {:?}", mic_users());
            println!("mic in use: {}", mic_in_use());
        }
    }
}

#[cfg(windows)]
mod win {
    use windows::{
        core::{w, PCWSTR, PWSTR},
        Win32::{
            Foundation::{CloseHandle, ERROR_SUCCESS},
            System::{
                Diagnostics::ToolHelp::{
                    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
                    TH32CS_SNAPPROCESS,
                },
                Registry::{
                    RegCloseKey, RegEnumKeyExW, RegGetValueW, RegOpenKeyExW, HKEY,
                    HKEY_CURRENT_USER, KEY_READ, RRF_RT_REG_QWORD,
                },
            },
        },
    };

    /// Data source of the system's "Privacy > Microphone" page. One subkey per app that has used the microphone:
    /// Store apps are named by package family name, other apps under NonPackaged by full path (with `\` replaced by `#`).
    /// LastUsedTimeStop of 0 means currently in use
    const CONSENT_STORE: PCWSTR = w!(
        r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone"
    );

    /// Apps currently using the microphone: package family name for Store apps, executable name otherwise
    pub fn mic_users() -> Vec<String> {
        let mut users = Vec::new();
        let Some(root) = Key::open(HKEY_CURRENT_USER, CONSENT_STORE) else {
            return users;
        };
        for name in root.subkeys() {
            if name.eq_ignore_ascii_case("NonPackaged") {
                let Some(key) = root.child(&name) else {
                    continue;
                };
                for path in key.subkeys() {
                    if key.in_use(&path) {
                        let exe = path.rsplit('#').next().unwrap_or(&path);
                        users.push(exe.to_string());
                    }
                }
            } else if root.in_use(&name) {
                users.push(name);
            }
        }
        users
    }

    /// Executable names of all running processes
    pub fn running_exes() -> Vec<String> {
        let mut exes = Vec::new();
        unsafe {
            let Ok(snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) else {
                return exes;
            };
            let mut entry = PROCESSENTRY32W {
                dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                ..Default::default()
            };
            let mut ok = Process32FirstW(snapshot, &mut entry).is_ok();
            while ok {
                exes.push(utf16(&entry.szExeFile));
                ok = Process32NextW(snapshot, &mut entry).is_ok();
            }
            let _ = CloseHandle(snapshot);
        }
        exes
    }

    fn utf16(buf: &[u16]) -> String {
        let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        String::from_utf16_lossy(&buf[..len])
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(Some(0)).collect()
    }

    /// An open registry key, closed when it goes out of scope
    struct Key(HKEY);

    impl Key {
        fn open(parent: HKEY, path: PCWSTR) -> Option<Self> {
            let mut key = HKEY::default();
            let result = unsafe { RegOpenKeyExW(parent, path, None, KEY_READ, &mut key) };
            (result == ERROR_SUCCESS).then_some(Self(key))
        }

        fn child(&self, name: &str) -> Option<Self> {
            let name = wide(name);
            Self::open(self.0, PCWSTR(name.as_ptr()))
        }

        fn subkeys(&self) -> Vec<String> {
            let mut names = Vec::new();
            // Registry key names are at most 255 characters
            let mut buf = [0u16; 256];
            for index in 0.. {
                let mut len = buf.len() as u32;
                let result = unsafe {
                    RegEnumKeyExW(
                        self.0,
                        index,
                        Some(PWSTR(buf.as_mut_ptr())),
                        &mut len,
                        None,
                        None,
                        None,
                        None,
                    )
                };
                if result != ERROR_SUCCESS {
                    break;
                }
                names.push(String::from_utf16_lossy(&buf[..len as usize]));
            }
            names
        }

        /// Whether the app recorded under subkey name is using the microphone
        fn in_use(&self, name: &str) -> bool {
            let start = self.qword(name, w!("LastUsedTimeStart"));
            let stop = self.qword(name, w!("LastUsedTimeStop"));
            matches!((start, stop), (Some(start), Some(0)) if start != 0)
        }

        fn qword(&self, subkey: &str, value: PCWSTR) -> Option<u64> {
            let subkey = wide(subkey);
            let mut data = 0u64;
            let mut size = std::mem::size_of::<u64>() as u32;
            let result = unsafe {
                RegGetValueW(
                    self.0,
                    PCWSTR(subkey.as_ptr()),
                    value,
                    RRF_RT_REG_QWORD,
                    None,
                    Some(&mut data as *mut u64 as *mut _),
                    Some(&mut size),
                )
            };
            (result == ERROR_SUCCESS).then_some(data)
        }
    }

    impl Drop for Key {
        fn drop(&mut self) {
            unsafe {
                let _ = RegCloseKey(self.0);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn running_exes_includes_self() {
            let me = std::env::current_exe().unwrap();
            let me = me.file_name().unwrap().to_str().unwrap();
            assert!(running_exes().iter().any(|exe| exe.eq_ignore_ascii_case(me)));
        }

        #[test]
        fn reads_consent_store() {
            // Only needs to read without crashing; printed for manual checking during a meeting
            println!("mic users: {:?}", mic_users());
        }
    }
}
