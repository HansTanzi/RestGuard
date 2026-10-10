use crate::i18n::{self, t, Lang};
use crate::timer::{Phase, Snapshot};
use std::sync::Mutex;
use tauri::{
    image::Image,
    include_image,
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};
use tauri_plugin_autostart::ManagerExt;

const TRAY_ID: &str = "main";
const LANG_ID_PREFIX: &str = "lang:";

/// 需要随状态或语言更新的菜单项
struct TrayItems {
    status: MenuItem<Wry>,
    rest_now: MenuItem<Wry>,
    settings: MenuItem<Wry>,
    autostart: CheckMenuItem<Wry>,
    /// 与 Lang::ALL 顺序一致
    langs: Vec<CheckMenuItem<Wry>>,
    quit: MenuItem<Wry>,
    /// 当前托盘图标，变化时才重设
    icon: Mutex<IconKey>,
}

/// 托盘图标的选择依据：(是否在休息, 任务栏是否为浅色)
type IconKey = (bool, bool);

fn icon_key(phase: Phase) -> IconKey {
    // macOS 用模板图，由系统按菜单栏配色着色，始终只用一个图标
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

/// Windows 读取任务栏的深浅色设置；Linux 面板大多是深色，按深色处理
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
    t("提前休息", "Rest now")
}

fn settings_text() -> &'static str {
    t("设置…", "Settings…")
}

fn autostart_text() -> &'static str {
    t("开机自启", "Start at login")
}

fn quit_text() -> &'static str {
    t("退出", "Quit")
}

fn lang_text(lang: Lang) -> &'static str {
    match lang {
        Lang::Auto => t("跟随系统", "Follow system"),
        // 语言名称始终用其本身的文字显示，方便看不懂当前语言的用户找到
        Lang::Zh => "简体中文",
        Lang::En => "English",
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
    let lang_menu = Submenu::with_items(
        app,
        "语言 / Language",
        true,
        &[&langs[0], &PredefinedMenuItem::separator(app)?, &langs[1], &langs[2]],
    )?;
    let quit = MenuItem::with_id(app, "quit", quit_text(), true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &status,
            &PredefinedMenuItem::separator(app)?,
            &rest_now,
            &settings,
            &autostart,
            &lang_menu,
            &PredefinedMenuItem::separator(app)?,
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
            "autostart" => {
                crate::apply_autostart(app, !autostart_enabled(app));
            }
            "quit" => app.exit(0),
            id => {
                if let Some(lang) = id.strip_prefix(LANG_ID_PREFIX).and_then(Lang::from_id) {
                    crate::set_language(app, lang);
                }
            }
        })
        .build(app)?;

    app.manage(TrayItems {
        status,
        rest_now,
        settings,
        autostart,
        langs,
        quit,
        icon: Mutex::new(icon),
    });
    Ok(())
}

/// 语言切换后刷新菜单文字和勾选状态（状态行由随后的 update 刷新）
pub fn refresh_language(app: &AppHandle) {
    let Some(items) = app.try_state::<TrayItems>() else {
        return;
    };
    let _ = items.rest_now.set_text(rest_now_text());
    let _ = items.settings.set_text(settings_text());
    let _ = items.autostart.set_text(autostart_text());
    let _ = items.quit.set_text(quit_text());
    // 点击 CheckMenuItem 会自动切换它的勾选，这里统一按实际选择重设
    let current = i18n::current();
    for (item, lang) in items.langs.iter().zip(Lang::ALL) {
        let _ = item.set_text(lang_text(lang));
        let _ = item.set_checked(lang == current);
    }
}

pub fn update(app: &AppHandle, snap: &Snapshot) {
    let text = match snap.phase {
        Phase::Working => format!(
            "{} {}",
            t("距离休息", "Break in"),
            mm_ss(snap.remaining_secs)
        ),
        Phase::Resting => format!("{} {}", t("休息中", "Resting"), mm_ss(snap.remaining_secs)),
        Phase::RestOver => t("休息结束", "Break over").to_string(),
    };
    let working = snap.phase == Phase::Working;
    let tray = app.tray_by_id(TRAY_ID);

    if let Some(items) = app.try_state::<TrayItems>() {
        let _ = items.status.set_text(&text);
        let _ = items.rest_now.set_enabled(working);
        // 休息期间不允许从托盘退出，也不允许打开设置缩短休息
        let _ = items.settings.set_enabled(working);
        let _ = items.quit.set_enabled(working);

        // 每次刷新都重新判断，任务栏切换深浅色后图标随之更新
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

/// 开启或关闭开机自启，返回系统里的实际状态
pub fn set_autostart(app: &AppHandle, on: bool) -> bool {
    if let Err(e) = toggle_autostart(app, on) {
        eprintln!("切换开机自启失败：{e}");
    }
    // 以系统里的实际状态为准，防止菜单勾选与真实状态不一致
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
