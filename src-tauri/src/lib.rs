mod config;
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
fn start_work(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::start_work)
}

#[tauri::command]
fn postpone(app: AppHandle) -> Result<(), String> {
    end_rest(&app, Timer::postpone)
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
            let path = app.path().app_config_dir()?.join("config.toml");
            let cfg = Config::load_or_create(&path);
            app.manage(AppState {
                timer: Mutex::new(Timer::new(&cfg, Instant::now())),
                overlay_coverage: cfg.overlay_coverage,
            });
            tray::init(app.handle())?;
            spawn_ticker(app.handle().clone());
            Ok(())
        })
        .on_window_event(|window, event| {
            // 拦截 Alt+F4 等系统关闭操作，遮罩只能由程序销毁
            if let WindowEvent::CloseRequested { api, .. } = event {
                if overlay::is_overlay(window.label()) {
                    api.prevent_close();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![get_state, start_work, postpone])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            // 没有窗口时保持后台运行；只有托盘“退出”（带退出码）才真正退出
            if let RunEvent::ExitRequested { code: None, api, .. } = event {
                api.prevent_exit();
            }
        });
}
