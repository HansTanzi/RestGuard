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

/// 需要随状态或语言更新的菜单项
struct TrayItems {
    status: MenuItem<Wry>,
    rest_now: MenuItem<Wry>,
    settings: MenuItem<Wry>,
    autostart: CheckMenuItem<Wry>,
    /// 与 Lang::ALL 顺序一致
    langs: Vec<CheckMenuItem<Wry>>,
    about: MenuItem<Wry>,
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
    // “跟随系统”与具体语言之间加分隔线
    let separator = PredefinedMenuItem::separator(app)?;
    let mut lang_entries: Vec<&dyn IsMenuItem<Wry>> = vec![&langs[0], &separator];
    lang_entries.extend(langs[1..].iter().map(|item| item as &dyn IsMenuItem<Wry>));
    let lang_menu = Submenu::with_items(app, "语言 / Language", true, &lang_entries)?;
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
        about,
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
    let _ = items.about.set_text(about_text());
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
        // 休息期间不允许从托盘退出，也不允许打开设置缩短休息；
        // 关于页面能打开浏览器，同样禁用
        let _ = items.settings.set_enabled(working);
        let _ = items.about.set_enabled(working);
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
