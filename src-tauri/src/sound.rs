//! Sound when a break ends: a built-in sound or a user-chosen audio file.
//! Built-in sounds are synthesized to WAV at runtime and written to the temp directory, then played the same way as custom files:
//! WinRT MediaPlayer on Windows, afplay on macOS, paplay on Linux (falling back to pw-play, aplay).

use crate::{config::Config, i18n};
use std::{
    env, f32::consts::TAU, fs, path::Path, sync::Mutex, thread, time::Duration,
};

/// Maximum playback time for one alert, so a player that never reports finishing doesn't hold on forever; long custom audio is cut off here too
#[cfg_attr(not(windows), allow(dead_code))]
const MAX_LENGTH: Duration = Duration::from_secs(60);

/// Built-in sound used when the custom file is missing
const FALLBACK: &str = "beep";

/// Audio extensions this platform's player supports, used to filter the file picker
pub const EXTENSIONS: &[&str] = imp::EXTENSIONS;

/// Break over: plays the sound according to the config
pub fn announce(cfg: &Config) {
    if cfg.sound_prompt {
        play(cfg);
    }
}

/// Held while playing; new requests are ignored until the previous one finishes, so repeated preview clicks don't overlap
static PLAYING: Mutex<()> = Mutex::new(());

/// Plays the sound selected by `cfg` on a background thread; falls back to a built-in sound if the custom file can't play, so the alert always sounds
pub fn play(cfg: &Config) {
    let cfg = cfg.clone();
    thread::spawn(move || {
        let Ok(_guard) = PLAYING.try_lock() else {
            return;
        };
        if let Err(e) = play_blocking(&cfg, true) {
            eprintln!("Failed to play sound: {e}");
        }
    });
}

/// Preview (regardless of the toggle): returns after playback; if the custom file can't play, returns a user-facing message instead of falling back to a built-in sound
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
                eprintln!("Failed to play {}: {e}", cfg.sound_file);
                if fallback {
                    play_builtin(FALLBACK)?;
                }
                Err(i18n::t(&i18n::text::SOUND_FILE_FAILED).to_owned())
            })
        }
        "file" => {
            eprintln!("Sound file {} not found, using the built-in sound", cfg.sound_file);
            play_builtin(FALLBACK)
        }
        name => play_builtin(name),
    }
}

fn play_builtin(name: &str) -> Result<(), String> {
    let notes = builtin(name).unwrap_or_else(|| builtin(FALLBACK).unwrap());
    // Rewrite every time: the temp directory may be cleaned by the system, and the files are small
    let path = env::temp_dir().join(format!("restguard-{name}.wav"));
    fs::write(&path, wav(&render(notes))).map_err(|e| e.to_string())?;
    imp::play_file(&path)
}

/// A tone: starts at `start` seconds and lasts `len` seconds; `decay` is the decay rate, 0 for constant volume (an electronic beep)
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

/// Built-in sounds; names match config::SOUNDS
fn builtin(name: &str) -> Option<&'static [Note]> {
    const BEEP: &[Note] = &[note(0.0, 1800.0, 0.12, 0.0), note(0.2, 1800.0, 0.12, 0.0)];
    // Doorbell-like "ding-dong": E6, C6
    const CHIME: &[Note] = &[note(0.0, 1318.5, 0.6, 4.0), note(0.45, 1046.5, 1.0, 3.0)];
    // C5, E5, G5 rising in turn
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

/// Mixes tones into 16-bit mono samples, normalizing the peak to 60% volume
fn render(notes: &[Note]) -> Vec<i16> {
    let end = notes.iter().map(|n| n.start + n.len).fold(0.0, f32::max);
    let mut buf = vec![0f32; ((end + 0.05) * RATE as f32) as usize];
    for n in notes {
        let start = (n.start * RATE as f32) as usize;
        let len = (n.len * RATE as f32) as usize;
        for (i, out) in buf[start..].iter_mut().take(len).enumerate() {
            let t = i as f32 / RATE as f32;
            // Fade in and out for a few milliseconds at each end to avoid clicks
            let fade = (t / 0.005).min(1.0) * ((n.len - t) / 0.01).clamp(0.0, 1.0);
            // Add a bit of the second harmonic so it sounds less thin
            let wave = (TAU * n.freq * t).sin() + 0.3 * (TAU * 2.0 * n.freq * t).sin();
            *out += fade * (-n.decay * t).exp() * wave;
        }
    }
    let peak = buf.iter().fold(0f32, |m, s| m.max(s.abs())).max(f32::EPSILON);
    let gain = 0.6 * i16::MAX as f32 / peak;
    buf.iter().map(|s| (s * gain) as i16).collect()
}

/// A 16-bit mono PCM WAV file
fn wav(samples: &[i16]) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    // PCM, mono
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&RATE.to_le_bytes());
    // Bytes per second, bytes per frame, bit depth
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

/// Plays with MediaPlayer, returning only when finished (or on error or timeout)
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
    // The player is dropped here, so wait for it to finish
    let _ = rx.recv_timeout(MAX_LENGTH);
    player.Close()
}

#[cfg(windows)]
mod imp {
    use std::path::Path;
    use windows::{core::HSTRING, Media::Core::MediaSource, Storage::StorageFile};

    /// MediaPlayer supports mp3, wav, m4a, wma, flac and other formats the system can decode
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

    /// afplay supports mp3, wav, m4a, aiff and other common formats
    pub const EXTENSIONS: &[&str] = &["mp3", "wav", "m4a", "aac", "aif", "aiff", "caf", "flac"];

    pub fn play_file(path: &Path) -> Result<(), String> {
        let status = Command::new("afplay").arg(path).status().map_err(|e| e.to_string())?;
        status.success().then_some(()).ok_or_else(|| format!("afplay: {status}"))
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod imp {
    use std::{io, path::Path, process::Command};

    /// PulseAudio and PipeWire desktops mostly ship paplay; otherwise use ALSA's aplay (wav only)
    const PLAYERS: [&str; 3] = ["paplay", "pw-play", "aplay"];

    /// paplay and pw-play decode via libsndfile and may not support mp3
    pub const EXTENSIONS: &[&str] = &["wav", "ogg", "oga", "flac"];

    pub fn play_file(path: &Path) -> Result<(), String> {
        for player in PLAYERS {
            // aplay plays formats it doesn't recognize as raw data, which comes out as noise
            let wav = path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("wav"));
            if player == "aplay" && !wav {
                return Err("aplay can only play wav".into());
            }
            match Command::new(player).arg(path).status() {
                Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                Err(e) => return Err(e.to_string()),
                Ok(status) if status.success() => return Ok(()),
                Ok(status) => return Err(format!("{player}: {status}")),
            }
        }
        Err(format!("No player found ({})", PLAYERS.join(", ")))
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
        // The chime is about 1.5 seconds
        assert!((1.4..1.6).contains(&(samples.len() as f32 / RATE as f32)));
        let peak = samples.iter().map(|s| s.unsigned_abs()).max().unwrap();
        assert!((19000..=19700).contains(&peak), "Peak should be about 60% volume: {peak}");

        let bytes = wav(&samples);
        assert_eq!(&bytes[..4], b"RIFF");
        assert_eq!(&bytes[8..16], b"WAVEfmt ");
        assert_eq!(bytes.len(), 44 + samples.len() * 2);
        assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize, bytes.len() - 8);
    }

    /// Actually makes sound, so it doesn't run by default: cargo test plays_every_sound -- --ignored --nocapture
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
            // Well under MAX_LENGTH means the player reported the end of playback properly
            let elapsed = start.elapsed();
            println!("{name}: {elapsed:?}");
            assert!(elapsed < Duration::from_secs(10), "{name}");
        }
    }
}
