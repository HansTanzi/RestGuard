//! 休息结束时推送到手机：通过 ntfy（https://ntfy.sh 或自建服务器）发一条最高优先级的通知。
//! 手机上装 ntfy 应用并订阅同一个主题即可收到；Android 上打开“最高优先级持续提醒”后会像闹钟一样一直响到点掉。

use crate::{config::Config, i18n};
use std::{thread, time::Duration};

/// 网络不好时最多等这么久，免得试听按钮一直转
const TIMEOUT: Duration = Duration::from_secs(10);

/// 休息结束：按配置在后台推送，失败只记日志
pub fn announce(cfg: &Config) {
    if !cfg.phone_notify || cfg.ntfy_topic.is_empty() {
        return;
    }
    let cfg = cfg.clone();
    thread::spawn(move || {
        if let Err(e) = send(&cfg) {
            eprintln!("推送到手机失败：{e}");
        }
    });
}

/// 设置面板的“发送测试”（不管开关是否打开）：发完才返回，失败时返回给用户看的提示
pub fn test(cfg: &Config) -> Result<(), String> {
    send(cfg).map_err(|e| i18n::t(&i18n::text::PHONE_FAILED).replace("{e}", &e))
}

fn send(cfg: &Config) -> Result<(), String> {
    if cfg.ntfy_topic.is_empty() {
        return Err("ntfy_topic 为空".into());
    }
    // 用 JSON 发布：标题和正文可以直接带中文等非 ASCII 字符，不用编码请求头
    let body = serde_json::json!({
        "topic": cfg.ntfy_topic,
        "title": i18n::t(&i18n::text::BREAK_OVER),
        "message": i18n::t(&i18n::text::PHONE_MESSAGE),
        // 5 = urgent：Android 上长震动并弹出，可设为持续响铃；iOS 上为时效性通知
        "priority": 5,
        "tags": ["alarm_clock"],
    });
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(TIMEOUT))
        .build()
        .new_agent();
    agent
        .post(&cfg.ntfy_server)
        .header("Content-Type", "application/json")
        .send(body.to_string())
        .map(|_| ())
        .map_err(|e| e.to_string())
}
