//! 界面语言：默认跟随系统（系统语言不在支持列表里时显示英文），也可以在托盘菜单里手动切换。
//! 手动选择保存在配置目录下的 `language` 文件里。

pub mod text;

use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU8, Ordering},
        OnceLock,
    },
};

/// 判别值与 ALL 中的下标一致，用于存进 AtomicU8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Auto,
    Zh,
    En,
    Hi,
    Es,
    Ar,
    Fr,
    Bn,
    Pt,
    De,
    Ja,
    It,
    Ko,
    Id,
}

impl Lang {
    pub const ALL: [Lang; 14] = [
        Lang::Auto,
        Lang::Zh,
        Lang::En,
        Lang::Hi,
        Lang::Es,
        Lang::Ar,
        Lang::Fr,
        Lang::Bn,
        Lang::Pt,
        Lang::De,
        Lang::Ja,
        Lang::It,
        Lang::Ko,
        Lang::Id,
    ];

    /// 与 src/lib/i18n.ts 的 UI_LANGS 一致
    pub fn id(self) -> &'static str {
        match self {
            Lang::Auto => "auto",
            Lang::Zh => "zh",
            Lang::En => "en",
            Lang::Hi => "hi",
            Lang::Es => "es",
            Lang::Ar => "ar",
            Lang::Fr => "fr",
            Lang::Bn => "bn",
            Lang::Pt => "pt",
            Lang::De => "de",
            Lang::Ja => "ja",
            Lang::It => "it",
            Lang::Ko => "ko",
            Lang::Id => "id",
        }
    }

    /// 语言名称始终用其本身的文字显示，方便看不懂当前语言的用户找到。
    /// 与 src/lib/i18n.ts 的 LANG_NAMES 一致
    pub fn native_name(self) -> &'static str {
        match self {
            Lang::Auto => "Auto",
            Lang::Zh => "简体中文",
            Lang::En => "English",
            Lang::Hi => "हिन्दी",
            Lang::Es => "Español",
            Lang::Ar => "العربية",
            Lang::Fr => "Français",
            Lang::Bn => "বাংলা",
            Lang::Pt => "Português",
            Lang::De => "Deutsch",
            Lang::Ja => "日本語",
            Lang::It => "Italiano",
            Lang::Ko => "한국어",
            Lang::Id => "Bahasa Indonesia",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|l| l.id() == id.trim())
    }

    /// 由系统区域设置（如 "pt-BR"、"zh_CN"）得到界面语言，不支持的语言显示英文
    fn from_locale(locale: &str) -> Self {
        let primary = locale
            .split(['-', '_'])
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        match primary.as_str() {
            // 旧式的印尼语代码
            "in" => Lang::Id,
            p => Self::from_id(p)
                .filter(|l| *l != Lang::Auto)
                .unwrap_or(Lang::En),
        }
    }

    fn from_u8(v: u8) -> Self {
        Self::ALL.get(v as usize).copied().unwrap_or(Lang::Auto)
    }

    fn to_u8(self) -> u8 {
        self as u8
    }
}

/// 各语言的同一句文字，按 resolved() 取用
pub struct Tr {
    pub zh: &'static str,
    pub en: &'static str,
    pub hi: &'static str,
    pub es: &'static str,
    pub ar: &'static str,
    pub fr: &'static str,
    pub bn: &'static str,
    pub pt: &'static str,
    pub de: &'static str,
    pub ja: &'static str,
    pub it: &'static str,
    pub ko: &'static str,
    pub id: &'static str,
}

impl Tr {
    pub fn get(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::Auto => self.get(resolved()),
            Lang::Zh => self.zh,
            Lang::En => self.en,
            Lang::Hi => self.hi,
            Lang::Es => self.es,
            Lang::Ar => self.ar,
            Lang::Fr => self.fr,
            Lang::Bn => self.bn,
            Lang::Pt => self.pt,
            Lang::De => self.de,
            Lang::Ja => self.ja,
            Lang::It => self.it,
            Lang::Ko => self.ko,
            Lang::Id => self.id,
        }
    }
}

static CURRENT: AtomicU8 = AtomicU8::new(0);
static FILE: OnceLock<PathBuf> = OnceLock::new();

fn system_lang() -> Lang {
    static SYSTEM: OnceLock<Lang> = OnceLock::new();
    *SYSTEM.get_or_init(|| sys_locale::get_locale().map_or(Lang::En, |l| Lang::from_locale(&l)))
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

/// 用户的选择，可能是 Auto
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

/// 实际显示的语言，不会是 Auto
pub fn resolved() -> Lang {
    match current() {
        Lang::Auto => system_lang(),
        lang => lang,
    }
}

/// 按当前语言取文字
pub fn t(tr: &Tr) -> &'static str {
    tr.get(resolved())
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
        assert_eq!(Lang::from_id("xx"), None);
    }

    #[test]
    fn locales_map_to_languages() {
        assert_eq!(Lang::from_locale("zh-CN"), Lang::Zh);
        assert_eq!(Lang::from_locale("zh_TW"), Lang::Zh);
        assert_eq!(Lang::from_locale("pt-BR"), Lang::Pt);
        assert_eq!(Lang::from_locale("AR-eg"), Lang::Ar);
        assert_eq!(Lang::from_locale("in-ID"), Lang::Id);
        assert_eq!(Lang::from_locale("ru-RU"), Lang::En);
        assert_eq!(Lang::from_locale("auto"), Lang::En);
        assert_eq!(Lang::from_locale(""), Lang::En);
    }
}
