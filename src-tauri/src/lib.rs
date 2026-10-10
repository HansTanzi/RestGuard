mod about;
mod bypass;
mod config;
mod i18n;
#[cfg(windows)]
mod msix;
mod overlay;
mod settings;
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
/// 载荷为切换后实际显示的语言（见 ui_lang），与 src/lib/i18n.ts 的 LANG_EVENT 一致
const LANG_EVENT: &str = "lang:changed";
/// 载荷为遮罩是否已缩小，与 src/routes/overlay/+page.svelte 的 SHRINK_EVENT 一致
const SHRINK_EVENT: &str = "overlay:shrunk";
/// 载荷为开机自启是否开启，与 src/lib/config.ts 的 AUTOSTART_EVENT 一致
const AUTOSTART_EVENT: &str = "autostart:changed";

struct AppState {
    timer: Mutex<Timer>,
    config: Mutex<Config>,
    config_path: PathBuf,
}

impl AppState {
    fn overlay_coverage(&self) -> f64 {
        self.config.lock().unwrap().overlay_coverage
    }
}

#[tauri::command]
fn get_state(app: AppHandle) -> Snapshot {
    let state = app.state::<AppState>();
    let snap = state.timer.lock().unwrap().snapshot(Instant::now());
    snap
}

/// 实际显示的界面语言（不会是 auto），各页面据此选择文字
#[tauri::command]
fn ui_lang() -> &'static str {
    i18n::resolved().id()
}

/// 用户的语言选择（可能是 auto），设置面板和遮罩的语言选项共用。
/// 取值见 i18n::Lang::id
#[tauri::command]
fn get_lang() -> &'static str {
    i18n::current().id()
}

#[tauri::command]
fn set_lang(app: AppHandle, lang: String) -> Result<(), String> {
    let lang = i18n::Lang::from_id(&lang).ok_or("未知的语言")?;
    set_language(&app, lang);
    Ok(())
}

#[tauri::command]
fn get_config(app: AppHandle) -> Config {
    let cfg = app.state::<AppState>().config.lock().unwrap().clone();
    cfg
}

/// 设置面板保存：写入配置文件并立即生效，返回修正后的实际配置。
/// 休息期间不允许修改，防止通过缩短休息时长绕过休息
#[tauri::command]
fn save_config(app: AppHandle, config: Config) -> Result<Config, String> {
    let config = config.sanitized();
    let state = app.state::<AppState>();
    let snap = {
        let mut timer = state.timer.lock().unwrap();
        let now = Instant::now();
        if timer.snapshot(now).phase != Phase::Working {
            return Err(i18n::t(&i18n::text::SETTINGS_LOCKED).into());
        }
        config
            .save(&state.config_path)
            .map_err(|e| i18n::t(&i18n::text::SAVE_FAILED).replace("{e}", &e.to_string()))?;
        timer.apply_config(&config, now);
        timer.snapshot(now)
    };
    *state.config.lock().unwrap() = config.clone();
    broadcast(&app, &snap);
    Ok(config)
}

#[tauri::command]
fn get_autostart(app: AppHandle) -> bool {
    tray::autostart_enabled(&app)
}

/// 返回切换后系统里的实际状态
#[tauri::command]
fn set_autostart(app: AppHandle, on: bool) -> bool {
    apply_autostart(&app, on)
}

#[tauri::command]
fn get_about(app: AppHandle) -> about::AboutInfo {
    about::info(&app)
}

/// 关于页面的链接：link 为 "repo" 或 "issues"。
/// 休息期间不允许打开浏览器，防止借此绕过休息
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

/// 遮罩上的“我在开会”：会议软件在运行但无法确定是否在开会时，由用户确认
#[tauri::command]
fn in_meeting(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::in_meeting)
}

/// 遮罩上的“缩小窗口”按钮：紧急时缩小遮罩处理一下手头的事，但不结束休息。
/// 用 async 让它跑在主线程之外，避免在 Windows 上调整窗口时卡住
#[tauri::command]
async fn set_overlay_shrunk(app: AppHandle, shrunk: bool) {
    let coverage = app.state::<AppState>().overlay_coverage();
    overlay::set_shrunk(&app, coverage, shrunk);
    // 通知所有遮罩（包括副屏）切换可拖动状态
    let _ = app.emit(SHRINK_EVENT, shrunk);
}

/// 仅开发构建可用：遮罩上按 Esc 直接结束休息，方便调试
#[tauri::command]
fn dev_skip_rest(app: AppHandle) -> Result<(), String> {
    if !cfg!(debug_assertions) {
        return Err("仅开发构建可用".into());
    }
    end_rest(&app, Timer::skip_rest)
}

fn end_rest(app: &AppHandle, action: fn(&mut Timer, Instant) -> bool) -> Result<(), String> {
    let snap = {
        let state = app.state::<AppState>();
        let mut timer = state.timer.lock().unwrap();
        let now = Instant::now();
        if !action(&mut timer, now) {
            return Err("当前状态不允许此操作".into());
        }
        timer.snapshot(now)
    };
    overlay::close_all(app);
    broadcast(app, &snap);
    Ok(())
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

pub(crate) fn set_language(app: &AppHandle, lang: i18n::Lang) {
    if let Err(e) = i18n::set(lang) {
        eprintln!("无法保存语言设置：{e}");
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

/// 托盘和设置面板共用，切换后通知设置面板同步开关状态
pub(crate) fn apply_autostart(app: &AppHandle, on: bool) -> bool {
    let actual = tray::set_autostart(app, on);
    let _ = app.emit(AUTOSTART_EVENT, actual);
    actual
}

pub(crate) fn open_settings(app: &AppHandle) {
    let app = app.clone();
    // 见 overlay::open_all 的说明：不能在事件回调线程里直接创建窗口
    thread::spawn(move || settings::open(&app));
}

pub(crate) fn open_about(app: &AppHandle) {
    let app = app.clone();
    // 见 overlay::open_all 的说明：不能在事件回调线程里直接创建窗口
    thread::spawn(move || about::open(&app));
}

fn open_overlays(app: &AppHandle) {
    let app = app.clone();
    // 见 overlay::open_all 的说明：不能在事件回调线程里直接创建窗口
    thread::spawn(move || {
        let coverage = app.state::<AppState>().overlay_coverage();
        overlay::open_all(&app, coverage);
    });
}

fn broadcast(app: &AppHandle, snap: &Snapshot) {
    let _ = app.emit(STATE_EVENT, snap);
    tray::update(app, snap);
}

fn spawn_ticker(app: AppHandle) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        let (event, snap) = {
            let state = app.state::<AppState>();
            let mut timer = state.timer.lock().unwrap();
            let now = Instant::now();
            // 只在休息到点后才检测前台应用
            let held = if timer.rest_due(now) {
                bypass::active_app()
            } else {
                None
            };
            timer.hold(held);
            let event = timer.tick(now);
            if event == Some(TimerEvent::RestStarted) {
                timer.set_meeting_app(bypass::running_app());
            }
            (event, timer.snapshot(now))
        };
        if event == Some(TimerEvent::RestStarted) {
            open_overlays(&app);
        }
        broadcast(&app, &snap);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 必须最先注册：重复启动时直接退出新进程
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        // macOS 默认菜单带 Cmd+Q，休息时按下会直接退出
        .enable_macos_default_menu(false)
        .setup(|app| {
            // macOS 上只在菜单栏显示图标，不占 Dock
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let dir = app.path().app_config_dir()?;
            // 开发构建用单独的配置和语言文件，调试时不影响已安装的正式版
            let (lang_name, cfg_name) = if cfg!(debug_assertions) {
                ("language.dev", "config.dev.toml")
            } else {
                ("language", "config.toml")
            };
            // 先确定语言：首次运行生成的配置文件注释语言依赖它
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
            if !overlay::is_overlay(window.label()) {
                return;
            }
            match event {
                // 拦截 Alt+F4 等系统关闭操作，遮罩只能由程序销毁
                WindowEvent::CloseRequested { api, .. } => api.prevent_close(),
                // 缩小后可拖动，但不能离开所属屏幕
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
            get_autostart,
            set_autostart,
            get_about,
            open_link,
            start_work,
            postpone,
            in_meeting,
            set_overlay_shrunk,
            dev_skip_rest
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            // 没有窗口时保持后台运行；只有托盘“退出”（带退出码）才真正退出
            if let RunEvent::ExitRequested { code: None, api, .. } = event {
                api.prevent_exit();
            }
        });
}
