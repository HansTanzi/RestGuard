//! UI language: follows the system by default (English if the system language isn't supported) and can be switched manually from the tray menu.
//! A manual choice is saved in the `language` file in the config directory.

pub mod text;

use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU8, Ordering},
        OnceLock,
    },
};

/// Discriminants match the indices in ALL, so the value can be stored in an AtomicU8
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

    /// Matches UI_LANGS in src/lib/i18n.ts
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

    /// Language names are always shown in their own language, so users who can't read the current language can still find theirs.
    /// Matches LANG_NAMES in src/lib/i18n.ts
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

    /// UI language for a system locale (e.g. "pt-BR", "zh_CN"); unsupported languages fall back to English
    fn from_locale(locale: &str) -> Self {
        let primary = locale
            .split(['-', '_'])
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        match primary.as_str() {
            // Legacy Indonesian code
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

/// The same text in every language, picked by resolved()
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

/// Reads the saved language choice; follows the system if the file is missing or invalid
pub fn init(path: &Path) {
    let _ = FILE.set(path.to_path_buf());
    let lang = fs::read_to_string(path)
        .ok()
        .and_then(|s| Lang::from_id(&s))
        .unwrap_or(Lang::Auto);
    CURRENT.store(lang.to_u8(), Ordering::Relaxed);
}

/// The user's choice, possibly Auto
pub fn current() -> Lang {
    Lang::from_u8(CURRENT.load(Ordering::Relaxed))
}

/// Switches the language and saves it to the file
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

/// The language actually displayed, never Auto
pub fn resolved() -> Lang {
    match current() {
        Lang::Auto => system_lang(),
        lang => lang,
    }
}

/// Picks the text for the current language
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
