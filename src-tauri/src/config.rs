use crate::i18n::{self, text::config as comment, Lang};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path, time::Duration};

/// 与 src/lib/config.ts 的 Config 保持一致。前端也用配置文件里的 snake_case 字段名
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub work_minutes: f64,
    pub rest_minutes: f64,
    pub postpone_minutes: f64,
    pub max_postpones: u32,
    pub overlay_coverage: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_minutes: 60.0,
            rest_minutes: 10.0,
            postpone_minutes: 5.0,
            max_postpones: 3,
            overlay_coverage: 1.0,
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
        // {:?} 保证浮点数总带小数点（60.0 而不是 60），写出的仍是合法的 TOML 浮点数
        format!(
            "# {}\n# {}\n\n\
             # {}\nwork_minutes = {:?}\n\n\
             # {}\nrest_minutes = {:?}\n\n\
             # {}\npostpone_minutes = {:?}\n\n\
             # {}\nmax_postpones = {}\n\n\
             # {}\noverlay_coverage = {:?}\n",
            c(&comment::HEADER),
            c(&comment::UNITS),
            c(&comment::WORK),
            self.work_minutes,
            c(&comment::REST),
            self.rest_minutes,
            c(&comment::POSTPONE),
            self.postpone_minutes,
            c(&comment::MAX_POSTPONES),
            self.max_postpones,
            c(&comment::COVERAGE),
            self.overlay_coverage,
        )
    }

    /// 把明显无效的值替换成默认值，避免出现 0 分钟工作之类的死循环
    pub fn sanitized(self) -> Self {
        let d = Self::default();
        let positive = |v: f64, fallback: f64| if v.is_finite() && v > 0.0 { v } else { fallback };
        Self {
            work_minutes: positive(self.work_minutes, d.work_minutes),
            rest_minutes: positive(self.rest_minutes, d.rest_minutes),
            postpone_minutes: positive(self.postpone_minutes, d.postpone_minutes),
            max_postpones: self.max_postpones,
            overlay_coverage: if self.overlay_coverage.is_finite() {
                self.overlay_coverage.clamp(0.1, 1.0)
            } else {
                d.overlay_coverage
            },
        }
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
            max_postpones: 0,
            overlay_coverage: 0.8,
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
        let cfg: Config = toml::from_str("work_minutes = 0.0\noverlay_coverage = 3.0").unwrap();
        let cfg = cfg.sanitized();
        assert_eq!(cfg.work_minutes, 60.0);
        assert_eq!(cfg.overlay_coverage, 1.0);
    }
}
