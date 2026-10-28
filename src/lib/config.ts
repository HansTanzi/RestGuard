import type { UiLang } from "./i18n";

// Settings profile for an enforcement level; keep in sync with Profile in src-tauri/src/config.rs
export interface Profile {
  // Name of a custom profile, used by Config.level to refer to it; empty for built-in profiles
  name: string;
  max_postpones: number;
  // Offer "Urgent? Shrink it" on the overlay
  urgent_mode: boolean;
  // Hold off breaks while meeting software is in use, and offer "I'm in a meeting" during a break
  meeting_detection: boolean;
  // When false the overlay can be closed
  force_rest: boolean;
  // Still offer "Skip this break" on the overlay when the break is forced. Unforced breaks can always be skipped
  allow_skip: boolean;
  // Fraction of the screen the overlay covers (normal mode)
  overlay_coverage: number;
  // Scale factor relative to normal mode in urgent mode (overlay shrunk); below 1 so it's always smaller than normal mode
  urgent_scale: number;
  // Whether the overlay can be dragged within its screen when it doesn't fill it (shrunk, or coverage below 1)
  movable: boolean;
}

export const BUILTIN_LEVELS = ["high", "medium", "low"] as const;
export type BuiltinLevel = (typeof BUILTIN_LEVELS)[number];

// Matches Profile::builtin in src-tauri/src/config.rs
export const BUILTIN_PROFILES: Record<BuiltinLevel, Profile> = {
  high: {
    name: "",
    max_postpones: 1,
    urgent_mode: false,
    meeting_detection: false,
    force_rest: true,
    allow_skip: false,
    overlay_coverage: 1,
    urgent_scale: 0.75,
    movable: false,
  },
  medium: {
    name: "",
    max_postpones: 3,
    urgent_mode: true,
    meeting_detection: true,
    force_rest: true,
    allow_skip: false,
    overlay_coverage: 1,
    urgent_scale: 0.75,
    movable: true,
  },
  low: {
    name: "",
    max_postpones: 3,
    urgent_mode: true,
    meeting_detection: true,
    force_rest: false,
    allow_skip: true,
    overlay_coverage: 0.8,
    urgent_scale: 0.75,
    movable: true,
  },
};

export function isBuiltinLevel(level: string): level is BuiltinLevel {
  return (BUILTIN_LEVELS as readonly string[]).includes(level);
}

// Sound played when a break ends: a built-in sound or a user-chosen audio file. Matches SOUNDS in src-tauri/src/config.rs
export const SOUNDS = ["beep", "chime", "rise", "file"] as const;
export type SoundId = (typeof SOUNDS)[number];

// Keep in sync with Config in src-tauri/src/config.rs (field names match the config file, in snake_case)
export interface Config {
  work_minutes: number;
  rest_minutes: number;
  postpone_minutes: number;
  idle_minutes: number;
  // Hold off breaks that fall due shortly before the end of the working day until it ends
  work_end_enabled: boolean;
  // End of the working day, as HH:MM in local time
  work_end: string;
  // A break due this long before work_end waits until work_end
  work_end_skip_minutes: number;
  // A built-in level or the name of a custom profile
  level: string;
  // Play a sound when a break ends
  sound_prompt: boolean;
  sound: SoundId;
  // When sound is "file", the audio file to play
  sound_file: string;
  // Push to the phone via ntfy when a break ends
  phone_notify: boolean;
  ntfy_server: string;
  // Subscribe to the same topic on the phone; letters, digits, - and _ only
  ntfy_topic: string;
  profiles: Profile[];
}

// Longest quiet window before the end of the working day; matches MAX_WORK_END_SKIP_MINUTES in src-tauri/src/config.rs
export const MAX_WORK_END_SKIP_MINUTES = 12 * 60;

// Characters allowed in an ntfy topic
export const NTFY_TOPIC_PATTERN = /^[A-Za-z0-9_-]{1,64}$/;

// Field order matches config.rs; the settings panel compares JSON directly to detect changes
export const DEFAULT_CONFIG: Config = {
  work_minutes: 60,
  rest_minutes: 10,
  postpone_minutes: 5,
  idle_minutes: 5,
  work_end_enabled: false,
  work_end: "18:00",
  work_end_skip_minutes: 30,
  level: "medium",
  sound_prompt: true,
  sound: "beep",
  sound_file: "",
  phone_notify: false,
  ntfy_server: "https://ntfy.sh",
  ntfy_topic: "",
  profiles: [],
};

// Matches Lang::id in src-tauri/src/i18n.rs
export type LangId = "auto" | UiLang;

// Broadcast when the tray or settings panel toggles autostart; payload is whether it's enabled
export const AUTOSTART_EVENT = "autostart:changed";

// Broadcast after the config is saved (including switching profiles from the tray); payload is the new config
export const CONFIG_EVENT = "config:changed";
