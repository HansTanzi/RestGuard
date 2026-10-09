mod config;
mod i18n;
mod overlay;
mod timer;
mod tray;

use config::Config;
use std::{
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, Manager, RunEvent, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;
use timer::{Snapshot, Timer, TimerEvent};

const STATE_EVENT: &str = "timer:state";
/// 载荷为切换后是否显示中文，与 src/lib/i18n.ts 的 LANG_EVENT 一致
const LANG_EVENT: &str = "lang:changed";
/// 载荷为遮罩是否已缩小，与 src/routes/overlay/+page.svelte 的 SHRINK_EVENT 一致
const SHRINK_EVENT: &str = "overlay:shrunk";

struct AppState {
    timer: Mutex<Timer>,
    overlay_coverage: f64,
}

#[tauri::command]
fn get_state(app: AppHandle) -> Snapshot {
    let state = app.state::<AppState>();
    let snap = state.timer.lock().unwrap().snapshot(Instant::now());
    snap
}

#[tauri::command]
fn is_zh() -> bool {
    i18n::is_zh()
}

/// 遮罩上的语言切换按钮：在中英文之间切换，与托盘菜单的手动选择等效
#[tauri::command]
fn set_zh(app: AppHandle, zh: bool) {
    set_language(&app, if zh { i18n::Lang::Zh } else { i18n::Lang::En });
}

#[tauri::command]
fn start_work(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::start_work)
}

#[tauri::command]
fn postpone(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::postpone)
}

/// 遮罩上的“缩小窗口”按钮：紧急时缩小遮罩处理一下手头的事，但不结束休息。
/// 用 async 让它跑在主线程之外，避免在 Windows 上调整窗口时卡住
#[tauri::command]
async fn set_overlay_shrunk(app: AppHandle, shrunk: bool) {
    let coverage = app.state::<AppState>().overlay_coverage;
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
    let _ = app.emit(LANG_EVENT, i18n::is_zh());
    let snap = {
        let state = app.state::<AppState>();
        let snap = state.timer.lock().unwrap().snapshot(Instant::now());
        snap
    };
    tray::update(app, &snap);
}

fn open_overlays(app: &AppHandle) {
    let app = app.clone();
    // 见 overlay::open_all 的说明：不能在事件回调线程里直接创建窗口
    thread::spawn(move || {
        let coverage = app.state::<AppState>().overlay_coverage;
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
            (timer.tick(now), timer.snapshot(now))
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
        .setup(|app| {
            let dir = app.path().app_config_dir()?;
            // 开发构建用单独的配置和语言文件，调试时不影响已安装的正式版
            let (lang_name, cfg_name) = if cfg!(debug_assertions) {
                ("language.dev", "config.dev.toml")
            } else {
                ("language", "config.toml")
            };
            // 先确定语言：首次运行生成的配置文件注释语言依赖它
            i18n::init(&dir.join(lang_name));
            let cfg = Config::load_or_create(&dir.join(cfg_name));
            app.manage(AppState {
                timer: Mutex::new(Timer::new(&cfg, Instant::now())),
                overlay_coverage: cfg.overlay_coverage,
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
            is_zh,
            set_zh,
            start_work,
            postpone,
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
