use serde::Deserialize;
use std::{fs, io, path::Path, time::Duration};

/// 首次运行时写入的配置文件，带注释方便用户手改
const DEFAULT_TOML: &str = r#"# RestGuard 配置文件，修改后重启生效
# 时间单位均为分钟，可以写小数（例如 0.5 表示 30 秒）

# 连续工作多久后强制休息
work_minutes = 60.0

# 每次休息多久
rest_minutes = 10.0

# 每次推迟多久
postpone_minutes = 5.0

# 完整休息一次之前，最多推迟几次
max_postpones = 3

# 遮罩覆盖每块屏幕的比例，0.1 ~ 1.0
overlay_coverage = 1.0
"#;

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
                if let Err(e) = write_default(path) {
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

    /// 把明显无效的值替换成默认值，避免出现 0 分钟工作之类的死循环
    fn sanitized(self) -> Self {
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

fn write_default(path: &Path) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(path, DEFAULT_TOML)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_file_matches_default_struct() {
        let parsed: Config = toml::from_str(DEFAULT_TOML).unwrap();
        assert_eq!(parsed, Config::default());
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
