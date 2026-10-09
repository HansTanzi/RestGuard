//! 界面语言：系统语言为中文时显示中文，其他情况显示英文。

use std::sync::OnceLock;

pub fn is_zh() -> bool {
    static ZH: OnceLock<bool> = OnceLock::new();
    *ZH.get_or_init(|| {
        sys_locale::get_locale().is_some_and(|l| l.to_ascii_lowercase().starts_with("zh"))
    })
}

/// 按当前语言二选一
pub fn t(zh: &'static str, en: &'static str) -> &'static str {
    if is_zh() {
        zh
    } else {
        en
    }
}
