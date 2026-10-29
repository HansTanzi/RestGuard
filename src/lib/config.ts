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

// A fixed time of day when the user stops anyway; a break falling due shortly before it waits until then.
// Keep in sync with FixedBreak in src-tauri/src/config.rs
export interface FixedBreak {
  // A built-in fixed break (see BUILTIN_FIXED_BREAKS), shown translated, or the user's own name
  name: string;
  enabled: boolean;
  // Time of day, as HH:MM in local time
  time: string;
  // A break due this long before time waits until time
  quiet_minutes: number;
  // How long the break that starts at time lasts. Not used by work_end: the day is over then
  rest_minutes: number;
}

// Matches BUILTIN_FIXED_BREAKS in src-tauri/src/config.rs
export const BUILTIN_FIXED_BREAKS = ["work_end", "lunch"] as const;
export type BuiltinFixedBreak = (typeof BUILTIN_FIXED_BREAKS)[number];

// Matches FixedBreak::has_rest in src-tauri/src/config.rs
export function fixedBreakHasRest(b: FixedBreak): boolean {
  return b.name !== "work_end";
}

export function isBuiltinFixedBreak(name: string): name is BuiltinFixedBreak {
  return (BUILTIN_FIXED_BREAKS as readonly string[]).includes(name);
}

// A new custom fixed break; matches FixedBreak::default in src-tauri/src/config.rs
export function newFixedBreak(name: string): FixedBreak {
  return { name, enabled: true, time: "15:00", quiet_minutes: 30, rest_minutes: 30 };
}

// Longest quiet window before a fixed break, and longest fixed break; matches MAX_FIXED_MINUTES in src-tauri/src/config.rs
export const MAX_FIXED_MINUTES = 12 * 60;

// Sound played when a break ends: a built-in sound or a user-chosen audio file. Matches SOUNDS in src-tauri/src/config.rs
export const SOUNDS = ["beep", "chime", "rise", "file"] as const;
export type SoundId = (typeof SOUNDS)[number];

// Keep in sync with Config in src-tauri/src/config.rs (field names match the config file, in snake_case)
export interface Config {
  work_minutes: number;
  rest_minutes: number;
  postpone_minutes: number;
  idle_minutes: number;
  // Fixed times such as the end of the working day or lunch; the built-in ones come first, followed by the user's own
  fixed_breaks: FixedBreak[];
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

// Characters allowed in an ntfy topic
export const NTFY_TOPIC_PATTERN = /^[A-Za-z0-9_-]{1,64}$/;

// Field order matches config.rs; the settings panel compares JSON directly to detect changes
export const DEFAULT_CONFIG: Config = {
  work_minutes: 60,
  rest_minutes: 10,
  postpone_minutes: 5,
  idle_minutes: 5,
  fixed_breaks: [
    { name: "work_end", enabled: false, time: "18:00", quiet_minutes: 30, rest_minutes: 30 },
    { name: "lunch", enabled: false, time: "12:00", quiet_minutes: 30, rest_minutes: 30 },
  ],
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
