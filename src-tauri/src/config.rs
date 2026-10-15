use crate::i18n::{self, text::config as comment, Lang};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path, time::Duration};

const DEFAULT_LEVEL: &str = "medium";
/// 内置强制等级，与 src/lib/config.ts 的 BUILTIN_LEVELS 一致
pub const BUILTIN_LEVELS: [&str; 3] = ["high", "medium", "low"];

/// 强制等级对应的配置方案。内置高、中、低三种，用户还可以自定义多个。
/// 与 src/lib/config.ts 的 Profile 保持一致
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Profile {
    /// 自定义方案的名字，Config::level 用它引用方案；内置方案为空
    pub name: String,
    /// 完整休息一次之前，最多推迟几次
    pub max_postpones: u32,
    /// 遮罩上提供“有急事，缩小一下”。关闭后遮罩始终全屏，也就没法切到任务管理器结束程序
    pub urgent_mode: bool,
    /// 会议软件在使用时暂缓休息，休息时提供“我在开会”
    pub meeting_detection: bool,
    /// false 时不强制休息，遮罩可以关掉（关掉即跳过这次休息）
    pub force_rest: bool,
    /// 强制休息时，遮罩上仍提供“跳过这次休息”。不强制休息时总能跳过
    pub allow_skip: bool,
    /// 遮罩覆盖屏幕的比例（正常模式）
    pub overlay_coverage: f64,
    /// 紧急模式下（遮罩缩小后）相对 overlay_coverage 的缩放因子，小于 1 保证比正常模式小
    pub urgent_scale: f64,
    /// 遮罩没有占满屏幕时（缩小后或覆盖比例小于 1）能否拖动。无论如何都不能拖到别的屏幕
    pub movable: bool,
}

impl Profile {
    /// 内置等级 high、medium、low 的方案，与 src/lib/config.ts 的 BUILTIN_PROFILES 一致
    pub fn builtin(level: &str) -> Option<Self> {
        let (max_postpones, urgent_mode, meeting_detection, force_rest, allow_skip, overlay_coverage, movable) =
            match level {
                "high" => (1, false, false, true, false, 1.0, false),
                "medium" => (3, true, true, true, false, 1.0, true),
                "low" => (3, true, true, false, true, 0.8, true),
                _ => return None,
            };
        Some(Self {
            name: String::new(),
            max_postpones,
            urgent_mode,
            meeting_detection,
            force_rest,
            allow_skip,
            overlay_coverage,
            urgent_scale: 0.75,
            movable,
        })
    }

    /// 遮罩上提供“跳过这次休息”
    pub fn can_skip(&self) -> bool {
        self.allow_skip || !self.force_rest
    }

    /// 遮罩覆盖屏幕的实际比例，`urgent` 为紧急模式（遮罩缩小）时的比例
    pub fn coverage(&self, urgent: bool) -> f64 {
        if urgent {
            self.overlay_coverage * self.urgent_scale
        } else {
            self.overlay_coverage
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::builtin(DEFAULT_LEVEL).unwrap()
    }
}

/// 与 src/lib/config.ts 的 Config 保持一致。前端也用配置文件里的 snake_case 字段名
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub work_minutes: f64,
    pub rest_minutes: f64,
    pub postpone_minutes: f64,
    /// 无操作超过这么久算离开电脑，0 表示关闭空闲检测
    pub idle_minutes: f64,
    /// 当前强制等级：内置等级（见 Profile::builtin），或某个自定义方案的名字
    pub level: String,
    /// 自定义方案
    pub profiles: Vec<Profile>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_minutes: 60.0,
            rest_minutes: 10.0,
            postpone_minutes: 5.0,
            idle_minutes: 5.0,
            level: DEFAULT_LEVEL.to_string(),
            profiles: Vec::new(),
        }
    }
}

impl Config {
    /// 读取配置；文件不存在时写入默认配置，解析失败时回退到默认值
    pub fn load_or_create(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(text) => match toml::from_str::<Config>(&text) {
                Ok(cfg) => cfg.sanitized(),
                Err(e) => {
                    eprintln!("配置文件 {} 解析失败，使用默认值：{e}", path.display());
                    Self::default()
                }
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                if let Err(e) = Self::default().save(path) {
                    eprintln!("无法写入默认配置 {}：{e}", path.display());
                }
                Self::default()
            }
            Err(e) => {
                eprintln!("无法读取配置 {}：{e}", path.display());
                Self::default()
            }
        }
    }

    /// 重新读取用户手动编辑过的配置文件。出错时返回给设置面板显示的提示
    pub fn read(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path)
            .map_err(|e| i18n::t(&i18n::text::CONFIG_READ_FAILED).replace("{e}", &e.to_string()))?;
        toml::from_str::<Config>(&text).map(Self::sanitized).map_err(|e| {
            // 完整的错误信息带多行代码片段，提示只能一行，只取行号和说明
            let line = e.span().map_or(1, |s| text[..s.start].matches('\n').count() + 1);
            i18n::t(&i18n::text::CONFIG_INVALID)
                .replace("{line}", &line.to_string())
                .replace("{e}", e.message())
        })
    }

    /// 保存到配置文件（保留注释模板）
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, self.render(i18n::resolved()))
    }

    /// 生成带注释的配置文件，方便用户手改。注释语言跟随界面语言
    fn render(&self, lang: Lang) -> String {
        let c = |tr: &i18n::Tr| tr.get(lang);
        // 字符串交给 toml 转义，名字里有引号等字符也不会写坏文件
        let quote = |s: &str| toml::Value::String(s.to_string()).to_string();
        // {:?} 保证浮点数总带小数点（60.0 而不是 60），写出的仍是合法的 TOML 浮点数
        let mut text = format!(
            "# {}\n# {}\n\n\
             # {}\nwork_minutes = {:?}\n\n\
             # {}\nrest_minutes = {:?}\n\n\
             # {}\npostpone_minutes = {:?}\n\n\
             # {}\nidle_minutes = {:?}\n\n\
             # {}\nlevel = {}\n\n\
             # {}\n",
            c(&comment::HEADER),
            c(&comment::UNITS),
            c(&comment::WORK),
            self.work_minutes,
            c(&comment::REST),
            self.rest_minutes,
            c(&comment::POSTPONE),
            self.postpone_minutes,
            c(&comment::IDLE),
            self.idle_minutes,
            c(&comment::LEVEL),
            quote(&self.level),
            c(&comment::PROFILES),
        );
        // 表必须写在所有顶层键之后
        for p in &self.profiles {
            text += &format!(
                "\n[[profiles]]\nname = {}\nmax_postpones = {}\nurgent_mode = {}\n\
                 meeting_detection = {}\nforce_rest = {}\nallow_skip = {}\n\
                 overlay_coverage = {:?}\nurgent_scale = {:?}\nmovable = {}\n",
                quote(&p.name),
                p.max_postpones,
                p.urgent_mode,
                p.meeting_detection,
                p.force_rest,
                p.allow_skip,
                p.overlay_coverage,
                p.urgent_scale,
                p.movable,
            );
        }
        text
    }

    /// 把明显无效的值替换成默认值，避免出现 0 分钟工作之类的死循环
    pub fn sanitized(self) -> Self {
        let d = Self::default();
        let positive = |v: f64, fallback: f64| if v.is_finite() && v > 0.0 { v } else { fallback };
        let ratio = |v: f64, max: f64, fallback: f64| if v.is_finite() { v.clamp(0.1, max) } else { fallback };
        let dp = Profile::default();
        // 方案名字用来引用方案：去掉空名字、与内置等级重名的和重复的
        let mut profiles: Vec<Profile> = Vec::new();
        for mut p in self.profiles {
            p.name = p.name.trim().to_string();
            p.overlay_coverage = ratio(p.overlay_coverage, 1.0, dp.overlay_coverage);
            p.urgent_scale = ratio(p.urgent_scale, 0.9, dp.urgent_scale);
            let taken = p.name.is_empty()
                || Profile::builtin(&p.name).is_some()
                || profiles.iter().any(|q| q.name == p.name);
            if !taken {
                profiles.push(p);
            }
        }
        let level = self.level.trim();
        let level = if Profile::builtin(level).is_some() || profiles.iter().any(|p| p.name == level) {
            level.to_string()
        } else {
            d.level
        };
        Self {
            work_minutes: positive(self.work_minutes, d.work_minutes),
            rest_minutes: positive(self.rest_minutes, d.rest_minutes),
            postpone_minutes: positive(self.postpone_minutes, d.postpone_minutes),
            idle_minutes: if self.idle_minutes.is_finite() && self.idle_minutes >= 0.0 {
                self.idle_minutes
            } else {
                d.idle_minutes
            },
            level,
            profiles,
        }
    }

    /// 当前强制等级对应的方案
    pub fn profile(&self) -> Profile {
        Profile::builtin(&self.level)
            .or_else(|| self.profiles.iter().find(|p| p.name == self.level).cloned())
            .unwrap_or_default()
    }

    pub fn work(&self) -> Duration {
        minutes(self.work_minutes)
    }

    pub fn rest(&self) -> Duration {
        minutes(self.rest_minutes)
    }

    pub fn postpone(&self) -> Duration {
        minutes(self.postpone_minutes)
    }

    /// 空闲检测的阈值，None 表示关闭
    pub fn idle(&self) -> Option<Duration> {
        (self.idle_minutes > 0.0).then(|| minutes(self.idle_minutes))
    }
}

fn minutes(m: f64) -> Duration {
    Duration::from_secs_f64(m * 60.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rendered_files_round_trip() {
        let custom = Config {
            work_minutes: 25.0,
            rest_minutes: 0.5,
            postpone_minutes: 2.5,
            idle_minutes: 0.0,
            level: "我的 \"方案\"".into(),
            profiles: vec![
                Profile {
                    name: "我的 \"方案\"".into(),
                    max_postpones: 1,
                    urgent_mode: false,
                    meeting_detection: true,
                    force_rest: true,
                    allow_skip: true,
                    overlay_coverage: 0.8,
                    urgent_scale: 0.3,
                    movable: false,
                },
                Profile {
                    name: "Focus".into(),
                    ..Profile::default()
                },
            ],
        };
        for cfg in [Config::default(), custom] {
            for lang in Lang::ALL {
                let parsed: Config = toml::from_str(&cfg.render(lang)).unwrap();
                assert_eq!(parsed, cfg);
            }
        }
    }

    #[test]
    fn comments_are_single_line() {
        for tr in comment::ALL {
            for lang in Lang::ALL {
                assert!(!tr.get(lang).contains(['\n', '\r']), "{lang:?}");
            }
        }
    }

    #[test]
    fn missing_fields_use_defaults() {
        let cfg: Config = toml::from_str("rest_minutes = 5.0").unwrap();
        assert_eq!(cfg.rest_minutes, 5.0);
        assert_eq!(cfg.work_minutes, 60.0);
    }

    #[test]
    fn invalid_values_are_sanitized() {
        let cfg: Config = toml::from_str(
            "work_minutes = 0.0\n[[profiles]]\nname = \"A\"\noverlay_coverage = 3.0\nurgent_scale = 1.0",
        )
        .unwrap();
        let cfg = cfg.sanitized();
        assert_eq!(cfg.work_minutes, 60.0);
        assert_eq!(cfg.profiles[0].overlay_coverage, 1.0);
        assert_eq!(cfg.profiles[0].urgent_scale, 0.9);
    }

    #[test]
    fn coverage_follows_level() {
        let coverage = |level: &str| Profile::builtin(level).unwrap().coverage(false);
        assert_eq!(coverage("high"), 1.0);
        assert_eq!(coverage("medium"), 1.0);
        assert_eq!(coverage("low"), 0.8);
        assert_eq!(Profile::builtin("medium").unwrap().coverage(true), 0.75);
    }

    #[test]
    fn invalid_profiles_are_dropped() {
        let named = |name: &str| Profile {
            name: name.into(),
            ..Profile::default()
        };
        let cfg = Config {
            level: "gone".into(),
            profiles: vec![named(" A "), named(""), named("high"), named("A")],
            ..Config::default()
        }
        .sanitized();
        assert_eq!(cfg.profiles, vec![named("A")]);
        assert_eq!(cfg.level, "medium", "引用不存在的方案时回退到默认等级");
    }

    #[test]
    fn profile_follows_level() {
        let mine = Profile {
            name: "Mine".into(),
            max_postpones: 7,
            ..Profile::default()
        };
        let mut cfg = Config {
            profiles: vec![mine.clone()],
            ..Config::default()
        };
        assert_eq!(cfg.profile(), Profile::builtin("medium").unwrap());
        cfg.level = "high".into();
        assert!(!cfg.profile().urgent_mode);
        cfg.level = "Mine".into();
        assert_eq!(cfg.profile(), mine);
    }
}
