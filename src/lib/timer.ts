// 与 src-tauri/src/timer.rs 中的 Snapshot 保持一致
export type Phase = "working" | "resting" | "restOver";

export interface Snapshot {
  phase: Phase;
  remainingSecs: number;
  totalSecs: number;
  canPostpone: boolean;
  postponesLeft: number;
  /** 休息时间已到，但因该豁免应用在前台而暂缓 */
  heldBy: string | null;
  /** 休息开始时正在运行的会议软件，此时遮罩提供“我在开会” */
  meetingApp: string | null;
}

export const STATE_EVENT = "timer:state";

export function formatClock(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = secs % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}
