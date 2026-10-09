// 界面语言由后端按系统语言决定（src-tauri/src/i18n.rs），与托盘保持一致
export interface Strings {
  title: string;
  tip: string;
  restOver: string;
  startWork: string;
  postpone: (left: number) => string;
}

const zh: Strings = {
  title: "保护眼睛，小心猝死！",
  tip: "起来走走，看看远处，喝口水。",
  restOver: "休息结束",
  startWork: "开始工作",
  postpone: (left) => `过会儿再休息（还剩 ${left} 次）`,
};

const en: Strings = {
  title: "Time for a break!",
  tip: "Stand up, look into the distance, drink some water.",
  restOver: "Break over",
  startWork: "Back to work",
  postpone: (left) => `Not now (${left} left)`,
};

export function strings(isZh: boolean): Strings {
  return isZh ? zh : en;
}
