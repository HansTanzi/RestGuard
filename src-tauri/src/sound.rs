//! 休息结束时的提示音：内置音效，或用户自选的音频文件。
//! 内置音效在运行时合成为 WAV，写到临时目录后与自选文件走同一条播放路径：
//! Windows 用 WinRT 的 MediaPlayer，macOS 用 afplay，Linux 用 paplay（没有时退回 pw-play、aplay）。

use crate::{config::Config, i18n};
use std::{
    env, f32::consts::TAU, fs, path::Path, sync::Mutex, thread, time::Duration,
};

/// 一次提示最长播放多久，防止播放器没有报告结束时一直占着；自选的长音频也到此为止
#[cfg_attr(not(windows), allow(dead_code))]
const MAX_LENGTH: Duration = Duration::from_secs(60);

/// 自选文件丢失时改用的内置音效
const FALLBACK: &str = "beep";

/// 本平台播放器能播的音频后缀，供选择文件时筛选
pub const EXTENSIONS: &[&str] = imp::EXTENSIONS;

/// 休息结束：按配置播放提示音
pub fn announce(cfg: &Config) {
    if cfg.sound_prompt {
        play(cfg);
    }
}

/// 正在播放时持有，上一次还没播完时忽略新的，避免连点试听时叠在一起
static PLAYING: Mutex<()> = Mutex::new(());

/// 在后台线程播放 `cfg` 选择的提示音；自选文件播不了时改用内置音效，保证能提醒到
pub fn play(cfg: &Config) {
    let cfg = cfg.clone();
    thread::spawn(move || {
        let Ok(_guard) = PLAYING.try_lock() else {
            return;
        };
        if let Err(e) = play_blocking(&cfg, true) {
            eprintln!("播放提示音失败：{e}");
        }
    });
}

/// 试听（不管开关是否打开）：播完才返回，自选文件播不了时返回给用户看的提示，不改用内置音效
pub fn preview(cfg: &Config) -> Result<(), String> {
    let Ok(_guard) = PLAYING.try_lock() else {
        return Ok(());
    };
    play_blocking(cfg, false)
}

fn play_blocking(cfg: &Config, fallback: bool) -> Result<(), String> {
    match cfg.sound.as_str() {
        "file" if Path::new(&cfg.sound_file).is_file() => {
            imp::play_file(Path::new(&cfg.sound_file)).or_else(|e| {
                eprintln!("播放 {} 失败：{e}", cfg.sound_file);
                if fallback {
                    play_builtin(FALLBACK)?;
                }
                Err(i18n::t(&i18n::text::SOUND_FILE_FAILED).to_owned())
            })
        }
        "file" => {
            eprintln!("找不到提示音文件 {}，改用内置提示音", cfg.sound_file);
            play_builtin(FALLBACK)
        }
        name => play_builtin(name),
    }
}

fn play_builtin(name: &str) -> Result<(), String> {
    let notes = builtin(name).unwrap_or_else(|| builtin(FALLBACK).unwrap());
    // 每次都重写：临时目录可能被系统清理，文件也很小
    let path = env::temp_dir().join(format!("restguard-{name}.wav"));
    fs::write(&path, wav(&render(notes))).map_err(|e| e.to_string())?;
    imp::play_file(&path)
}

/// 一个音：从 `start` 秒开始，持续 `len` 秒；`decay` 为衰减速度，0 表示音量不变（电子“滴”声）
struct Note {
    start: f32,
    freq: f32,
    len: f32,
    decay: f32,
}

const fn note(start: f32, freq: f32, len: f32, decay: f32) -> Note {
    Note {
        start,
        freq,
        len,
        decay,
    }
}

/// 内置音效，名字与 config::SOUNDS 一致
fn builtin(name: &str) -> Option<&'static [Note]> {
    const BEEP: &[Note] = &[note(0.0, 1800.0, 0.12, 0.0), note(0.2, 1800.0, 0.12, 0.0)];
    // 门铃一样的“叮——咚”：E6、C6
    const CHIME: &[Note] = &[note(0.0, 1318.5, 0.6, 4.0), note(0.45, 1046.5, 1.0, 3.0)];
    // C5、E5、G5 依次上扬
    const RISE: &[Note] = &[
        note(0.0, 523.3, 0.5, 6.0),
        note(0.15, 659.3, 0.5, 6.0),
        note(0.3, 784.0, 0.8, 4.0),
    ];
    match name {
        "beep" => Some(BEEP),
        "chime" => Some(CHIME),
        "rise" => Some(RISE),
        _ => None,
    }
}

const RATE: u32 = 22050;

/// 把若干个音叠加成 16 位单声道采样，峰值归一到 60% 音量
fn render(notes: &[Note]) -> Vec<i16> {
    let end = notes.iter().map(|n| n.start + n.len).fold(0.0, f32::max);
    let mut buf = vec![0f32; ((end + 0.05) * RATE as f32) as usize];
    for n in notes {
        let start = (n.start * RATE as f32) as usize;
        let len = (n.len * RATE as f32) as usize;
        for (i, out) in buf[start..].iter_mut().take(len).enumerate() {
            let t = i as f32 / RATE as f32;
            // 首尾各淡入淡出几毫秒，避免爆音
            let fade = (t / 0.005).min(1.0) * ((n.len - t) / 0.01).clamp(0.0, 1.0);
            // 加一点二次泛音，听起来不那么单薄
            let wave = (TAU * n.freq * t).sin() + 0.3 * (TAU * 2.0 * n.freq * t).sin();
            *out += fade * (-n.decay * t).exp() * wave;
        }
    }
    let peak = buf.iter().fold(0f32, |m, s| m.max(s.abs())).max(f32::EPSILON);
    let gain = 0.6 * i16::MAX as f32 / peak;
    buf.iter().map(|s| (s * gain) as i16).collect()
}

/// 16 位单声道 PCM 的 WAV 文件
fn wav(samples: &[i16]) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    // PCM、单声道
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&RATE.to_le_bytes());
    // 每秒字节数、每帧字节数、位深
    out.extend_from_slice(&(RATE * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

/// 用 MediaPlayer 播放，播完（或出错、超时）才返回
#[cfg(windows)]
fn play_media(source: &windows::Media::Core::MediaSource) -> windows::core::Result<()> {
    use std::sync::mpsc;
    use windows::{
        core::IInspectable,
        Foundation::TypedEventHandler,
        Media::Playback::{MediaPlayer, MediaPlayerFailedEventArgs},
    };
    let player = MediaPlayer::new()?;
    let (tx, rx) = mpsc::channel();
    let failed = tx.clone();
    player.MediaEnded(&TypedEventHandler::<MediaPlayer, IInspectable>::new(move |_, _| {
        let _ = tx.send(());
        Ok(())
    }))?;
    player.MediaFailed(&TypedEventHandler::<MediaPlayer, MediaPlayerFailedEventArgs>::new(
        move |_, _| {
            let _ = failed.send(());
            Ok(())
        },
    ))?;
    player.SetSource(source)?;
    player.Play()?;
    // 播放器在这里释放，要等它播完
    let _ = rx.recv_timeout(MAX_LENGTH);
    player.Close()
}

#[cfg(windows)]
mod imp {
    use std::path::Path;
    use windows::{core::HSTRING, Media::Core::MediaSource, Storage::StorageFile};

    /// MediaPlayer 支持 mp3、wav、m4a、wma、flac 等系统能解码的格式
    pub const EXTENSIONS: &[&str] = &["mp3", "wav", "m4a", "aac", "wma", "flac"];

    pub fn play_file(path: &Path) -> Result<(), String> {
        let play = || {
            let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(path.as_os_str()))?.join()?;
            super::play_media(&MediaSource::CreateFromStorageFile(&file)?)
        };
        play().map_err(|e: windows::core::Error| e.to_string())
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use std::{path::Path, process::Command};

    /// afplay 支持 mp3、wav、m4a、aiff 等常见格式
    pub const EXTENSIONS: &[&str] = &["mp3", "wav", "m4a", "aac", "aif", "aiff", "caf", "flac"];

    pub fn play_file(path: &Path) -> Result<(), String> {
        let status = Command::new("afplay").arg(path).status().map_err(|e| e.to_string())?;
        status.success().then_some(()).ok_or_else(|| format!("afplay: {status}"))
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod imp {
    use std::{io, path::Path, process::Command};

    /// PulseAudio 和 PipeWire 桌面大多带 paplay；都没有时用 ALSA 的 aplay（只支持 wav）
    const PLAYERS: [&str; 3] = ["paplay", "pw-play", "aplay"];

    /// paplay、pw-play 靠 libsndfile 解码，不一定支持 mp3
    pub const EXTENSIONS: &[&str] = &["wav", "ogg", "oga", "flac"];

    pub fn play_file(path: &Path) -> Result<(), String> {
        for player in PLAYERS {
            // aplay 认不出的格式会当成原始数据播放，放出来是噪音
            let wav = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("wav"));
            if player == "aplay" && !wav {
                return Err("aplay 只能播放 wav".into());
            }
            match Command::new(player).arg(path).status() {
                Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                Err(e) => return Err(e.to_string()),
                Ok(status) if status.success() => return Ok(()),
                Ok(status) => return Err(format!("{player}: {status}")),
            }
        }
        Err(format!("找不到播放器（{}）", PLAYERS.join("、")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SOUNDS;

    #[test]
    fn builtin_sounds_exist() {
        for name in SOUNDS {
            assert_eq!(builtin(name).is_some(), name != "file", "{name}");
        }
    }

    #[test]
    fn rendered_sound_is_valid_wav() {
        let samples = render(builtin("chime").unwrap());
        // 叮咚约 1.5 秒
        assert!((1.4..1.6).contains(&(samples.len() as f32 / RATE as f32)));
        let peak = samples.iter().map(|s| s.unsigned_abs()).max().unwrap();
        assert!((19000..=19700).contains(&peak), "峰值约为 60% 音量：{peak}");

        let bytes = wav(&samples);
        assert_eq!(&bytes[..4], b"RIFF");
        assert_eq!(&bytes[8..16], b"WAVEfmt ");
        assert_eq!(bytes.len(), 44 + samples.len() * 2);
        assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize, bytes.len() - 8);
    }

    /// 真的会出声，默认不跑：cargo test plays_every_sound -- --ignored --nocapture
    #[test]
    #[ignore]
    fn plays_every_sound() {
        for name in ["beep", "chime", "rise"] {
            let cfg = Config {
                sound: name.into(),
                ..Config::default()
            };
            let start = std::time::Instant::now();
            play_blocking(&cfg, false).unwrap();
            // 远小于 MAX_LENGTH 说明播放器正常报告了播放结束
            let elapsed = start.elapsed();
            println!("{name}: {elapsed:?}");
            assert!(elapsed < Duration::from_secs(10), "{name}");
        }
    }
}
