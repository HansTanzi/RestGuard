use crate::i18n::{self, text::config as comment, Lang};
use chrono::{NaiveTime, Timelike};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path, time::Duration};

const DEFAULT_LEVEL: &str = "medium";
/// Built-in enforcement levels; matches BUILTIN_LEVELS in src/lib/config.ts
pub const BUILTIN_LEVELS: [&str; 3] = ["high", "medium", "low"];
/// Sound played when a break ends: a built-in sound (see sound::builtin) or a user-chosen audio file.
/// Matches SOUNDS in src/lib/config.ts
pub const SOUNDS: [&str; 4] = ["beep", "chime", "rise", "file"];
const DEFAULT_SOUND: &str = "beep";
/// ntfy server for phone notifications; matches DEFAULT_CONFIG in src/lib/config.ts
const DEFAULT_NTFY_SERVER: &str = "https://ntfy.sh";
/// Built-in fixed breaks: the end of the working day and lunch. Matches BUILTIN_FIXED_BREAKS in src/lib/config.ts
pub const BUILTIN_FIXED_BREAKS: [&str; 2] = ["work_end", "lunch"];
/// Time of day for a new custom fixed break, as HH:MM; matches newFixedBreak in src/lib/config.ts
const DEFAULT_FIXED_TIME: &str = "15:00";
/// Default quiet window before a fixed break
const DEFAULT_QUIET_MINUTES: f64 = 30.0;
/// Longest quiet window before a fixed break; matches MAX_QUIET_MINUTES in src/lib/config.ts
const MAX_QUIET_MINUTES: f64 = 12.0 * 60.0;

/// The settings profile for an enforcement level. Three are built in (high, medium, low), and users can define more.
/// Keep in sync with Profile in src/lib/config.ts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Profile {
    /// Name of a custom profile, used by Config::level to refer to it; empty for built-in profiles
    pub name: String,
    /// Maximum postponements before a full break
    pub max_postpones: u32,
    /// Offer "Urgent? Shrink it" on the overlay. When off, the overlay is always full screen, so there's no way to switch to Task Manager and kill the app
    pub urgent_mode: bool,
    /// Hold off breaks while meeting software is in use, and offer "I'm in a meeting" during a break
    pub meeting_detection: bool,
    /// When false the break isn't forced and the overlay can be closed (closing it skips the break)
    pub force_rest: bool,
    /// Still offer "Skip this break" on the overlay when the break is forced. Unforced breaks can always be skipped
    pub allow_skip: bool,
    /// Fraction of the screen the overlay covers (normal mode)
    pub overlay_coverage: f64,
    /// Scale factor relative to overlay_coverage in urgent mode (overlay shrunk); below 1 so it's always smaller than normal mode
    pub urgent_scale: f64,
    /// Whether the overlay can be dragged when it doesn't fill the screen (shrunk, or coverage below 1). It can never be dragged to another screen
    pub movable: bool,
}

impl Profile {
    /// Profiles for the built-in levels high, medium and low; matches BUILTIN_PROFILES in src/lib/config.ts
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

    /// Offer "Skip this break" on the overlay
    pub fn can_skip(&self) -> bool {
        self.allow_skip || !self.force_rest
    }

    /// Actual fraction of the screen the overlay covers; `urgent` is the fraction in urgent mode (overlay shrunk)
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

/// A fixed time of day when the user stops anyway, such as the end of the working day, lunch or a tea break.
/// A break falling due shortly before it waits until then. Keep in sync with FixedBreak in src/lib/config.ts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct FixedBreak {
    /// A built-in fixed break (see BUILTIN_FIXED_BREAKS), shown translated, or the user's own name
    pub name: String,
    pub enabled: bool,
    /// Time of day, as HH:MM in local time
    pub time: String,
    /// A break due this long before `time` waits until `time`
    pub quiet_minutes: f64,
}

impl FixedBreak {
    /// Built-in fixed breaks, off by default; matches DEFAULT_CONFIG in src/lib/config.ts
    pub fn builtin(name: &str) -> Option<Self> {
        let time = match name {
            "work_end" => "18:00",
            "lunch" => "12:00",
            _ => return None,
        };
        Some(Self {
            name: name.to_string(),
            enabled: false,
            time: time.to_string(),
            quiet_minutes: DEFAULT_QUIET_MINUTES,
        })
    }
}

impl Default for FixedBreak {
    fn default() -> Self {
        Self {
            name: String::new(),
            enabled: true,
            time: DEFAULT_FIXED_TIME.to_string(),
            quiet_minutes: DEFAULT_QUIET_MINUTES,
        }
    }
}

/// Keep in sync with Config in src/lib/config.ts. The frontend also uses the config file's snake_case field names
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub work_minutes: f64,
    pub rest_minutes: f64,
    pub postpone_minutes: f64,
    /// No input for this long counts as being away from the computer; 0 disables idle detection
    pub idle_minutes: f64,
    /// Fixed times such as the end of the working day or lunch; breaks due shortly before one wait for it.
    /// The built-in ones always come first, followed by the user's own
    pub fixed_breaks: Vec<FixedBreak>,
    /// Current enforcement level: a built-in level (see Profile::builtin) or the name of a custom profile
    pub level: String,
    /// Play a sound when a break ends
    pub sound_prompt: bool,
    /// The sound; see SOUNDS for values
    pub sound: String,
    /// When sound is "file", the audio file to play
    pub sound_file: String,
    /// Push to the phone via ntfy when a break ends
    pub phone_notify: bool,
    /// ntfy server URL; the public server by default, or a self-hosted one
    pub ntfy_server: String,
    /// ntfy topic; subscribe to the same topic on the phone. Anyone who knows the topic can subscribe on the public server, so pick a hard-to-guess name
    pub ntfy_topic: String,
    /// Custom profiles
    pub profiles: Vec<Profile>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_minutes: 60.0,
            rest_minutes: 10.0,
            postpone_minutes: 5.0,
            idle_minutes: 5.0,
            fixed_breaks: BUILTIN_FIXED_BREAKS.iter().filter_map(|b| FixedBreak::builtin(b)).collect(),
            level: DEFAULT_LEVEL.to_string(),
            sound_prompt: true,
            sound: DEFAULT_SOUND.to_string(),
            sound_file: String::new(),
            phone_notify: false,
            ntfy_server: DEFAULT_NTFY_SERVER.to_string(),
            ntfy_topic: String::new(),
            profiles: Vec::new(),
        }
    }
}

impl Config {
    /// Loads the config; writes the default config if the file doesn't exist and falls back to defaults if parsing fails
    pub fn load_or_create(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(text) => match toml::from_str::<Config>(&text) {
                Ok(cfg) => cfg.sanitized(),
                Err(e) => {
                    eprintln!("Failed to parse config file {}, using defaults: {e}", path.display());
                    Self::default()
                }
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                if let Err(e) = Self::default().save(path) {
                    eprintln!("Failed to write default config {}: {e}", path.display());
                }
                Self::default()
            }
            Err(e) => {
                eprintln!("Failed to read config {}: {e}", path.display());
                Self::default()
            }
        }
    }

    /// Re-reads a config file the user edited manually. On error, returns a message for the settings panel to show
    pub fn read(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path)
            .map_err(|e| i18n::t(&i18n::text::CONFIG_READ_FAILED).replace("{e}", &e.to_string()))?;
        toml::from_str::<Config>(&text).map(Self::sanitized).map_err(|e| {
            // The full error includes a multi-line code snippet, but the message must fit on one line, so keep only the line number and description
            let line = e.span().map_or(1, |s| text[..s.start].matches('\n').count() + 1);
            i18n::t(&i18n::text::CONFIG_INVALID)
                .replace("{line}", &line.to_string())
                .replace("{e}", e.message())
        })
    }

    /// Saves to the config file (keeping the commented template)
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, self.render(i18n::resolved()))
    }

    /// Generates a commented config file for easy manual editing. Comments follow the UI language
    fn render(&self, lang: Lang) -> String {
        let c = |tr: &i18n::Tr| tr.get(lang);
        // Let toml escape strings so quotes and the like in names can't corrupt the file
        let quote = |s: &str| toml::Value::String(s.to_string()).to_string();
        // {:?} always includes a decimal point (60.0, not 60), which is still a valid TOML float
        let mut text = format!(
            "# {}\n# {}\n\n\
             # {}\nwork_minutes = {:?}\n\n\
             # {}\nrest_minutes = {:?}\n\n\
             # {}\npostpone_minutes = {:?}\n\n\
             # {}\nidle_minutes = {:?}\n\n\
             # {}\nlevel = {}\n\n\
             # {}\nsound_prompt = {}\n\n\
             # {}\nsound = {}\n\n\
             # {}\nsound_file = {}\n\n\
             # {}\nphone_notify = {}\n\n\
             # {}\nntfy_server = {}\n\n\
             # {}\nntfy_topic = {}\n\n\
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
            c(&comment::SOUND_PROMPT),
            self.sound_prompt,
            c(&comment::SOUND),
            quote(&self.sound),
            c(&comment::SOUND_FILE),
            quote(&self.sound_file),
            c(&comment::PHONE_NOTIFY),
            self.phone_notify,
            c(&comment::NTFY_SERVER),
            quote(&self.ntfy_server),
            c(&comment::NTFY_TOPIC),
            quote(&self.ntfy_topic),
            c(&comment::FIXED_BREAKS),
        );
        // Tables must come after all top-level keys
        for b in &self.fixed_breaks {
            text += &format!(
                "\n[[fixed_breaks]]\nname = {}\nenabled = {}\ntime = {}\nquiet_minutes = {:?}\n",
                quote(&b.name),
                b.enabled,
                quote(&b.time),
                b.quiet_minutes,
            );
        }
        text += &format!("\n# {}\n", c(&comment::PROFILES));
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

    /// Replaces clearly invalid values with defaults, avoiding endless loops like 0-minute work periods
    pub fn sanitized(self) -> Self {
        let d = Self::default();
        let positive = |v: f64, fallback: f64| if v.is_finite() && v > 0.0 { v } else { fallback };
        let ratio = |v: f64, max: f64, fallback: f64| if v.is_finite() { v.clamp(0.1, max) } else { fallback };
        let dp = Profile::default();
        // Profile names are used as references: drop empty names, names clashing with built-in levels, and duplicates
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
        // Names tell built-in fixed breaks apart and label the user's own: drop empty names and duplicates.
        // Built-in ones come first, in a fixed order, and are restored if missing
        let mut fixed_breaks: Vec<FixedBreak> = BUILTIN_FIXED_BREAKS.iter().filter_map(|b| FixedBreak::builtin(b)).collect();
        let mut seen: Vec<String> = Vec::new();
        for mut b in self.fixed_breaks {
            b.name = b.name.trim().to_string();
            if b.name.is_empty() || seen.contains(&b.name) {
                continue;
            }
            seen.push(b.name.clone());
            let fallback = FixedBreak::builtin(&b.name).unwrap_or_default();
            // Normalize to zero-padded HH:MM, as the settings panel's time input expects
            b.time = parse_clock(&b.time).map_or(fallback.time, |t| t.format("%H:%M").to_string());
            b.quiet_minutes = if b.quiet_minutes.is_finite() && b.quiet_minutes > 0.0 {
                b.quiet_minutes.min(MAX_QUIET_MINUTES)
            } else {
                fallback.quiet_minutes
            };
            match fixed_breaks.iter_mut().find(|f| f.name == b.name) {
                Some(builtin) => *builtin = b,
                None => fixed_breaks.push(b),
            }
        }
        let level = self.level.trim();
        let level = if Profile::builtin(level).is_some() || profiles.iter().any(|p| p.name == level) {
            level.to_string()
        } else {
            d.level
        };
        // Fall back to the default sound if a custom file is selected but no path is given
        let sound_file = self.sound_file.trim().to_string();
        let sound = match self.sound.trim() {
            "file" if sound_file.is_empty() => d.sound,
            s if SOUNDS.contains(&s) => s.to_string(),
            _ => d.sound,
        };
        // Assume https when no scheme is given, strip trailing slashes, and use the public server when empty
        let ntfy_server = match self.ntfy_server.trim().trim_end_matches('/') {
            "" => d.ntfy_server,
            s if s.contains("://") => s.to_string(),
            s => format!("https://{s}"),
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
            fixed_breaks,
            level,
            sound_prompt: self.sound_prompt,
            sound,
            sound_file,
            phone_notify: self.phone_notify,
            ntfy_server,
            ntfy_topic: self.ntfy_topic.trim().to_string(),
            profiles,
        }
    }

    /// The profile for the current enforcement level
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

    /// Idle detection threshold; None means disabled
    pub fn idle(&self) -> Option<Duration> {
        (self.idle_minutes > 0.0).then(|| minutes(self.idle_minutes))
    }

    /// The enabled fixed breaks and the quiet windows before them
    pub fn fixed_times(&self) -> Vec<FixedTime> {
        self.fixed_breaks
            .iter()
            .filter(|b| b.enabled)
            .filter_map(|b| {
                Some(FixedTime {
                    name: b.name.clone(),
                    at: parse_clock(&b.time)?,
                    window: minutes(b.quiet_minutes),
                })
            })
            .collect()
    }
}

/// An enabled fixed break: a break falling due within `window` before `at` waits until `at`
#[derive(Debug, Clone, PartialEq)]
pub struct FixedTime {
    /// FixedBreak::name: a built-in fixed break or the user's own name
    pub name: String,
    pub at: NaiveTime,
    pub window: Duration,
}

impl FixedTime {
    /// Time left until `at` if `now` falls in the quiet window before it, otherwise None.
    /// The window may cross midnight
    pub fn wait(&self, now: NaiveTime) -> Option<Duration> {
        const DAY: i64 = 24 * 60 * 60;
        let ms = |t: NaiveTime| i64::from(t.num_seconds_from_midnight()) * 1000 + i64::from(t.nanosecond() / 1_000_000);
        let left = Duration::from_millis((ms(self.at) - ms(now)).rem_euclid(DAY * 1000) as u64);
        (!left.is_zero() && left <= self.window).then_some(left)
    }
}

/// Parses a time of day such as "18:00" or "9:30"
fn parse_clock(s: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(s.trim(), "%H:%M").ok()
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
            fixed_breaks: vec![
                FixedBreak {
                    enabled: true,
                    time: "17:30".into(),
                    quiet_minutes: 45.0,
                    ..FixedBreak::builtin("work_end").unwrap()
                },
                FixedBreak::builtin("lunch").unwrap(),
                FixedBreak {
                    name: "下午茶 \"tea\"".into(),
                    ..FixedBreak::default()
                },
            ],
            level: "我的 \"方案\"".into(),
            sound_prompt: false,
            sound: "file".into(),
            sound_file: "C:\\Users\\me\\Music\\叮 \"ding\".mp3".into(),
            phone_notify: true,
            ntfy_server: "https://ntfy.example.com".into(),
            ntfy_topic: "restguard-x7k2".into(),
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
    fn invalid_sounds_fall_back_to_default() {
        let sound = |sound: &str, file: &str| {
            Config {
                sound: sound.into(),
                sound_file: file.into(),
                ..Config::default()
            }
            .sanitized()
            .sound
        };
        assert_eq!(sound(" chime ", ""), "chime");
        assert_eq!(sound("file", " a.mp3 "), "file");
        assert_eq!(sound("file", "  "), "beep", "Default sound when there is no file");
        assert_eq!(sound("trumpet", ""), "beep");
    }

    #[test]
    fn ntfy_server_is_normalized() {
        let server = |s: &str| {
            Config {
                ntfy_server: s.into(),
                ..Config::default()
            }
            .sanitized()
            .ntfy_server
        };
        assert_eq!(server("  "), "https://ntfy.sh");
        assert_eq!(server("http://192.168.1.2:8080/"), "http://192.168.1.2:8080");
        assert_eq!(server("ntfy.example.com"), "https://ntfy.example.com", "Assume https when no scheme is given");
    }

    #[test]
    fn fixed_breaks_are_normalized() {
        let named = |name: &str, time: &str, quiet: f64| FixedBreak {
            name: name.into(),
            time: time.into(),
            quiet_minutes: quiet,
            ..FixedBreak::default()
        };
        let cfg = Config {
            fixed_breaks: vec![
                named(" Tea ", " 9:05 ", 30.0),
                named("", "10:00", 30.0),
                named("Tea", "11:00", 30.0),
                named("Late", "25:00", 0.0),
                named("lunch", "soon", 5000.0),
            ],
            ..Config::default()
        }
        .sanitized();
        let summary: Vec<_> = cfg
            .fixed_breaks
            .iter()
            .map(|b| (b.name.as_str(), b.enabled, b.time.as_str(), b.quiet_minutes))
            .collect();
        assert_eq!(
            summary,
            [
                // The missing built-in one is restored, and built-in ones come first
                ("work_end", false, "18:00", 30.0),
                ("lunch", true, "12:00", 720.0),
                ("Tea", true, "09:05", 30.0),
                ("Late", true, "15:00", 30.0),
            ]
        );
        assert_eq!(Config::default().fixed_times(), [], "Off by default");
    }

    #[test]
    fn fixed_breaks_can_be_omitted() {
        let cfg: Config = toml::from_str("work_minutes = 50.0").unwrap();
        assert_eq!(cfg.fixed_breaks, Config::default().fixed_breaks);
    }

    #[test]
    fn quiet_window() {
        let at = |h, m| NaiveTime::from_hms_opt(h, m, 0).unwrap();
        let end = FixedTime {
            name: "work_end".into(),
            at: at(18, 0),
            window: Duration::from_secs(30 * 60),
        };
        assert_eq!(end.wait(at(17, 29)), None);
        assert_eq!(end.wait(at(17, 30)), Some(Duration::from_secs(30 * 60)));
        assert_eq!(end.wait(at(17, 45)), Some(Duration::from_secs(15 * 60)));
        assert_eq!(end.wait(at(18, 0)), None, "The break goes ahead once the time comes");
        assert_eq!(end.wait(at(18, 10)), None);

        // The window may cross midnight
        let end = FixedTime { at: at(0, 15), ..end };
        assert_eq!(end.wait(at(23, 50)), Some(Duration::from_secs(25 * 60)));
        assert_eq!(end.wait(at(0, 10)), Some(Duration::from_secs(5 * 60)));
        assert_eq!(end.wait(at(0, 20)), None);
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
        assert_eq!(cfg.level, "medium", "Fall back to the default level when referencing a missing profile");
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
