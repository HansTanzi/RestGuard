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

// 设置面板（src/routes/+page.svelte）
export interface SettingsStrings {
  title: string;
  timing: string;
  workMinutes: string;
  restMinutes: string;
  postponeMinutes: string;
  maxPostpones: string;
  minutesHint: string;
  workRestartHint: string;
  overlay: string;
  coverage: string;
  general: string;
  language: string;
  followSystem: string;
  autostart: string;
  save: string;
  saved: string;
  resetDefaults: string;
  invalid: string;
  lockedDuringBreak: string;
}

const settingsZh: SettingsStrings = {
  title: "设置",
  timing: "时间",
  workMinutes: "工作时长（分钟）",
  restMinutes: "休息时长（分钟）",
  postponeMinutes: "每次推迟（分钟）",
  maxPostpones: "最多推迟次数",
  minutesHint: "可以写小数，例如 0.5 表示 30 秒",
  workRestartHint: "修改工作时长后，当前的工作倒计时会按新时长重新开始。",
  overlay: "遮罩",
  coverage: "覆盖屏幕比例",
  general: "通用",
  language: "语言",
  followSystem: "跟随系统",
  autostart: "开机自启",
  save: "保存",
  saved: "已保存",
  resetDefaults: "恢复默认值",
  invalid: "请检查标红的输入项",
  lockedDuringBreak: "休息期间不能修改设置",
};

const settingsEn: SettingsStrings = {
  title: "Settings",
  timing: "Timing",
  workMinutes: "Work duration (minutes)",
  restMinutes: "Break duration (minutes)",
  postponeMinutes: "Postpone length (minutes)",
  maxPostpones: "Max postpones",
  minutesHint: "Fractions are allowed, e.g. 0.5 = 30 seconds",
  workRestartHint: "Changing the work duration restarts the current work countdown.",
  overlay: "Overlay",
  coverage: "Screen coverage",
  general: "General",
  language: "Language",
  followSystem: "Follow system",
  autostart: "Start at login",
  save: "Save",
  saved: "Saved",
  resetDefaults: "Restore defaults",
  invalid: "Please fix the highlighted fields",
  lockedDuringBreak: "Settings can't be changed during a break",
};

export function settingsStrings(isZh: boolean): SettingsStrings {
  return isZh ? settingsZh : settingsEn;
}

// 关于页面（src/routes/about/+page.svelte）
export interface AboutStrings {
  tagline: string;
  version: string;
  github: string;
  feedback: string;
  lockedDuringBreak: string;
}

const aboutZh: AboutStrings = {
  tagline: "一个真的会让你休息的休息提醒工具",
  version: "版本",
  github: "GitHub 主页",
  feedback: "问题反馈",
  lockedDuringBreak: "休息期间不能打开链接",
};

const aboutEn: AboutStrings = {
  tagline: "A break reminder that actually makes you rest",
  version: "Version",
  github: "GitHub",
  feedback: "Report an issue",
  lockedDuringBreak: "Links can't be opened during a break",
};

export function aboutStrings(isZh: boolean): AboutStrings {
  return isZh ? aboutZh : aboutEn;
}
