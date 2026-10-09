// 与 src-tauri/src/timer.rs 中的 Snapshot 保持一致
export type Phase = "working" | "resting" | "restOver";

export interface Snapshot {
  phase: Phase;
  remainingSecs: number;
  totalSecs: number;
  canPostpone: boolean;
  postponesLeft: number;
}

export const STATE_EVENT = "timer:state";

export function formatClock(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}
