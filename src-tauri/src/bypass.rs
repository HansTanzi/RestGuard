//! 豁免应用：休息时间到时，如果会议等不便打断的应用在前台或正在使用麦克风，暂缓强制休息，
//! 等它不再占用后再开始。目前只有内置清单，后续可以开放给用户自定义

struct App {
    /// 显示在托盘里的名字
    name: &'static str,
    /// Windows 进程名，不区分大小写
    exe: &'static [&'static str],
    /// Windows 商店（MSIX）版的包系列名，用于识别麦克风占用
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
        // ms-teams.exe 是新版 Teams（以 MSIX 包安装），Teams.exe 是经典版
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

/// 豁免清单里有应用在前台或正在使用麦克风时，返回它的名字
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

/// Windows：前台窗口所属进程的可执行文件名
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

/// macOS：前台应用的 bundle identifier
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

/// Windows：正在使用麦克风的豁免应用。
/// 应用异常退出时系统记录可能停留在“使用中”，所以还要求它的进程仍在运行
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

/// 其他平台暂不支持检测麦克风占用
#[cfg(not(windows))]
fn mic_app() -> Option<&'static App> {
    None
}

/// 正在运行（不一定在开会）的豁免应用
#[cfg(windows)]
pub fn running_app() -> Option<String> {
    let running = win::running_exes();
    BUILTIN
        .iter()
        .find(|app| running.iter().any(|exe| contains(app.exe, exe)))
        .map(|app| app.name.to_string())
}

/// 其他平台暂不支持检测运行中的进程
#[cfg(not(windows))]
pub fn running_app() -> Option<String> {
    None
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

    /// 系统“隐私 > 麦克风”页面的数据来源。每个用过麦克风的应用一个子键：
    /// 商店应用以包系列名命名，其余应用在 NonPackaged 下以完整路径（`\` 换成 `#`）命名。
    /// LastUsedTimeStop 为 0 表示正在使用
    const CONSENT_STORE: PCWSTR = w!(
        r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone"
    );

    /// 正在使用麦克风的应用：商店应用返回包系列名，其余返回可执行文件名
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

    /// 所有正在运行的进程的可执行文件名
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

    /// 打开的注册表键，离开作用域时关闭
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
            // 注册表键名最长 255 个字符
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

        /// 子键 name 记录的应用是否正在使用麦克风
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
            // 只要能读出来、不崩溃即可；打印出来方便开会时手动核对
            println!("mic users: {:?}", mic_users());
        }
    }
}
