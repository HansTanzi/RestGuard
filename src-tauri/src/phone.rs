//! Phone notification when a break ends: sends a highest-priority notification via ntfy (https://ntfy.sh or a self-hosted server).
//! Install the ntfy app on the phone and subscribe to the same topic to receive it; on Android, with "keep alerting for highest priority" enabled, it rings like an alarm until dismissed.

use crate::{config::Config, i18n};
use std::{thread, time::Duration};

/// Maximum wait on a poor network, so the test button doesn't spin forever
const TIMEOUT: Duration = Duration::from_secs(10);

/// Break over: pushes in the background according to the config; failures are only logged
pub fn announce(cfg: &Config) {
    if !cfg.phone_notify || cfg.ntfy_topic.is_empty() {
        return;
    }
    let cfg = cfg.clone();
    thread::spawn(move || {
        if let Err(e) = send(&cfg) {
            eprintln!("Failed to push to phone: {e}");
        }
    });
}

/// "Send test" in the settings panel (regardless of the toggle): returns once sent, or a user-facing message on failure
pub fn test(cfg: &Config) -> Result<(), String> {
    send(cfg).map_err(|e| i18n::t(&i18n::text::PHONE_FAILED).replace("{e}", &e))
}

fn send(cfg: &Config) -> Result<(), String> {
    if cfg.ntfy_topic.is_empty() {
        return Err("ntfy_topic is empty".into());
    }
    // Publish as JSON: the title and body can contain non-ASCII text (e.g. Chinese) without encoding headers
    let body = serde_json::json!({
        "topic": cfg.ntfy_topic,
        "title": i18n::t(&i18n::text::BREAK_OVER),
        "message": i18n::t(&i18n::text::PHONE_MESSAGE),
        // 5 = urgent: long vibration and pop-up on Android, can be set to ring continuously; a time-sensitive notification on iOS
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
