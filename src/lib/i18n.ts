// 界面语言由后端决定（src-tauri/src/i18n.rs），与托盘保持一致

// 托盘切换语言时广播，载荷为是否显示中文
export const LANG_EVENT = "lang:changed";

export interface Strings {
  title: string;
  tip: string;
  restOver: string;
  startWork: string;
  postpone: (left: number) => string;
  shrink: string;
  unshrink: string;
  // 语言切换按钮上显示的“另一种”语言
  switchLang: string;
}

const zh: Strings = {
  title: "保护眼睛，小心猝死！",
  tip: "起来走走，看看远处，喝口水。",
  restOver: "休息结束",
  startWork: "开始工作",
  postpone: (left) => `过会儿再休息（还剩 ${left} 次）`,
  shrink: "有急事，缩小一下",
  unshrink: "恢复大小",
  switchLang: "English",
};

const en: Strings = {
  title: "Time for a break!",
  tip: "Stand up, look into the distance, drink some water.",
  restOver: "Break over",
  startWork: "Back to work",
  postpone: (left) => `Not now (${left} left)`,
  shrink: "Urgent? Shrink",
  unshrink: "Restore size",
  switchLang: "中文",
};

export function strings(isZh: boolean): Strings {
  return isZh ? zh : en;
}
