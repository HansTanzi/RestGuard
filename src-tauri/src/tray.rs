use crate::i18n::{self, t, Lang};
use crate::timer::{Phase, Snapshot};
use tauri::{
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
    autostart: CheckMenuItem<Wry>,
    /// 与 Lang::ALL 顺序一致
    langs: Vec<CheckMenuItem<Wry>>,
    quit: MenuItem<Wry>,
}

fn rest_now_text() -> &'static str {
    t("提前休息", "Rest now")
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
    let autostart_on = app.autolaunch().is_enabled().unwrap_or(false);
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
            &autostart,
            &lang_menu,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;

    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("RestGuard")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "rest_now" => crate::rest_now(app),
            "autostart" => toggle_autostart(app),
            "quit" => app.exit(0),
            id => {
                if let Some(lang) = id.strip_prefix(LANG_ID_PREFIX).and_then(Lang::from_id) {
                    crate::set_language(app, lang);
                }
            }
        });
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;

    app.manage(TrayItems {
        status,
        rest_now,
        autostart,
        langs,
        quit,
    });
    Ok(())
}

/// 语言切换后刷新菜单文字和勾选状态（状态行由随后的 update 刷新）
pub fn refresh_language(app: &AppHandle) {
    let Some(items) = app.try_state::<TrayItems>() else {
        return;
    };
    let _ = items.rest_now.set_text(rest_now_text());
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

    if let Some(items) = app.try_state::<TrayItems>() {
        let _ = items.status.set_text(&text);
        let _ = items.rest_now.set_enabled(working);
        // 休息期间不允许从托盘退出
        let _ = items.quit.set_enabled(working);
    }
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let _ = tray.set_tooltip(Some(format!("RestGuard · {text}")));
    }
}

fn toggle_autostart(app: &AppHandle) {
    let launcher = app.autolaunch();
    let result = if launcher.is_enabled().unwrap_or(false) {
        launcher.disable()
    } else {
        launcher.enable()
    };
    if let Err(e) = result {
        eprintln!("切换开机自启失败：{e}");
    }
    // 以系统里的实际状态为准，防止菜单勾选与真实状态不一致
    if let Some(items) = app.try_state::<TrayItems>() {
        let _ = items
            .autostart
            .set_checked(launcher.is_enabled().unwrap_or(false));
    }
}

fn mm_ss(secs: u64) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
