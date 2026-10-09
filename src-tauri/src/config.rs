use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path, time::Duration};

/// 配置文件模板，带注释方便用户手改。`{name}` 占位符由 render 填入实际值
const TEMPLATE_ZH: &str = r#"# RestGuard 配置文件。可以在托盘菜单的“设置”里修改；手动编辑后需重启生效
# 时间单位均为分钟，可以写小数（例如 0.5 表示 30 秒）

# 连续工作多久后强制休息
work_minutes = {work_minutes}

# 每次休息多久
rest_minutes = {rest_minutes}

# 每次推迟多久
postpone_minutes = {postpone_minutes}

# 完整休息一次之前，最多推迟几次
max_postpones = {max_postpones}

# 遮罩覆盖每块屏幕的比例，0.1 ~ 1.0
overlay_coverage = {overlay_coverage}
"#;

const TEMPLATE_EN: &str = r#"# RestGuard config. Edit it from "Settings" in the tray menu, or by hand and then restart the app.
# All durations are in minutes and may be fractional (e.g. 0.5 = 30 seconds).

# How long to work before a forced break
work_minutes = {work_minutes}

# How long each break lasts
rest_minutes = {rest_minutes}

# How long each postpone lasts
postpone_minutes = {postpone_minutes}

# Max postpones before a full break is required
max_postpones = {max_postpones}

# Fraction of each screen covered by the overlay, 0.1 ~ 1.0
overlay_coverage = {overlay_coverage}
"#;

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
        fs::write(path, self.render(crate::i18n::is_zh()))
    }

    fn render(&self, zh: bool) -> String {
        let template = if zh { TEMPLATE_ZH } else { TEMPLATE_EN };
        // {:?} 保证浮点数总带小数点（60.0 而不是 60），写出的仍是合法的 TOML 浮点数
        template
            .replace("{work_minutes}", &format!("{:?}", self.work_minutes))
            .replace("{rest_minutes}", &format!("{:?}", self.rest_minutes))
            .replace(
                "{postpone_minutes}",
                &format!("{:?}", self.postpone_minutes),
            )
            .replace("{max_postpones}", &self.max_postpones.to_string())
            .replace(
                "{overlay_coverage}",
                &format!("{:?}", self.overlay_coverage),
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
            for zh in [true, false] {
                let parsed: Config = toml::from_str(&cfg.render(zh)).unwrap();
                assert_eq!(parsed, cfg);
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
