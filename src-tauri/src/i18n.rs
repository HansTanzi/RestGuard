//! 界面语言：默认跟随系统（中文系统显示中文，其他显示英文），也可以在托盘菜单里手动切换。
//! 手动选择保存在配置目录下的 `language` 文件里。

use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU8, Ordering},
        OnceLock,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Auto,
    Zh,
    En,
}

impl Lang {
    pub const ALL: [Lang; 3] = [Lang::Auto, Lang::Zh, Lang::En];

    pub fn id(self) -> &'static str {
        match self {
            Lang::Auto => "auto",
            Lang::Zh => "zh",
            Lang::En => "en",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|l| l.id() == id.trim())
    }

    fn from_u8(v: u8) -> Self {
        match v {
            1 => Lang::Zh,
            2 => Lang::En,
            _ => Lang::Auto,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            Lang::Auto => 0,
            Lang::Zh => 1,
            Lang::En => 2,
        }
    }
}

static CURRENT: AtomicU8 = AtomicU8::new(0);
static FILE: OnceLock<PathBuf> = OnceLock::new();

fn system_is_zh() -> bool {
    static ZH: OnceLock<bool> = OnceLock::new();
    *ZH.get_or_init(|| {
        sys_locale::get_locale().is_some_and(|l| l.to_ascii_lowercase().starts_with("zh"))
    })
}

/// 读取保存的语言选择；文件不存在或内容无效时跟随系统
pub fn init(path: &Path) {
    let _ = FILE.set(path.to_path_buf());
    let lang = fs::read_to_string(path)
        .ok()
        .and_then(|s| Lang::from_id(&s))
        .unwrap_or(Lang::Auto);
    CURRENT.store(lang.to_u8(), Ordering::Relaxed);
}

pub fn current() -> Lang {
    Lang::from_u8(CURRENT.load(Ordering::Relaxed))
}

/// 切换语言并保存到文件
pub fn set(lang: Lang) -> io::Result<()> {
    CURRENT.store(lang.to_u8(), Ordering::Relaxed);
    let Some(path) = FILE.get() else {
        return Ok(());
    };
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, lang.id())
}

pub fn is_zh() -> bool {
    match current() {
        Lang::Auto => system_is_zh(),
        Lang::Zh => true,
        Lang::En => false,
    }
}

/// 按当前语言二选一
pub fn t(zh: &'static str, en: &'static str) -> &'static str {
    if is_zh() {
        zh
    } else {
        en
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_round_trip() {
        for lang in Lang::ALL {
            assert_eq!(Lang::from_id(lang.id()), Some(lang));
            assert_eq!(Lang::from_u8(lang.to_u8()), lang);
        }
        assert_eq!(Lang::from_id("en\n"), Some(Lang::En));
        assert_eq!(Lang::from_id("fr"), None);
    }
}
