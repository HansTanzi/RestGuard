// Keep in sync with Snapshot in src-tauri/src/timer.rs
export type Phase = "working" | "resting" | "restOver";

export interface Snapshot {
  phase: Phase;
  remainingSecs: number;
  totalSecs: number;
  canPostpone: boolean;
  postponesLeft: number;
  /** Counting down a postponed break; like a break, quitting and changing settings or profiles aren't allowed */
  postponed: boolean;
  /** A break is due but held off because this exempt app is in the foreground */
  heldBy: string | null;
  /** A break is due but held off until this fixed break (HH:MM) */
  deferredUntil: string | null;
  /** Name of that fixed break: a built-in one or the user's own */
  deferredFor: string | null;
  /** Reminders are paused from the tray until this local time (HH:MM) */
  pausedUntil: string | null;
  /** Name of the fixed break the next break will be, which remainingSecs then counts down to */
  nextFixed: string | null;
  /** Name of the fixed break this break is for, named in the overlay's title: a built-in one or the user's own */
  fixedBreak: string | null;
  /** Meeting software running when the break started; the overlay then offers "I'm in a meeting" */
  meetingApp: string | null;
  /** Forced break; when false the overlay can be closed */
  forced: boolean;
  /** Offer "Skip this break" on the overlay */
  canSkip: boolean;
  /** Offer "Urgent? Shrink it" on the overlay */
  canShrink: boolean;
  /** On a fixed break longer than a regular one: how long until "Keep using the computer" can be clicked, 0 once it can */
  keepUsingInSecs: number | null;
  /** While shrunk, how long until the overlay restores itself */
  urgentLeftSecs: number | null;
  /** The overlay can be dragged within its screen when it doesn't fill it */
  movable: boolean;
  /** The overlay fills the screen in normal mode (so it can't be dragged even if movable) */
  fullScreen: boolean;
}

export const STATE_EVENT = "timer:state";

export function formatClock(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}
