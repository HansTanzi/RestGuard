import type { UiLang } from "./i18n";

// 强制等级的配置方案，与 src-tauri/src/config.rs 中的 Profile 保持一致
export interface Profile {
  // 自定义方案的名字，Config.level 用它引用方案；内置方案为空
  name: string;
  max_postpones: number;
  // 遮罩上提供“有急事，缩小一下”
  urgent_mode: boolean;
  // 会议软件在使用时暂缓休息，休息时提供“我在开会”
  meeting_detection: boolean;
  // false 时遮罩可以关掉
  force_rest: boolean;
  // 强制休息时，遮罩上仍提供“跳过这次休息”。不强制休息时总能跳过
  allow_skip: boolean;
  // 遮罩覆盖屏幕的比例（正常模式）
  overlay_coverage: number;
  // 紧急模式（遮罩缩小后）相对正常模式的缩放因子，小于 1 保证比正常模式小
  urgent_scale: number;
  // 遮罩没有占满屏幕时（缩小后或覆盖比例小于 1）能否在本屏幕内拖动
  movable: boolean;
}

export const BUILTIN_LEVELS = ["high", "medium", "low"] as const;
export type BuiltinLevel = (typeof BUILTIN_LEVELS)[number];

// 与 src-tauri/src/config.rs 的 Profile::builtin 一致
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

// 与 src-tauri/src/config.rs 中的 Config 保持一致（字段名与配置文件相同，用 snake_case）
export interface Config {
  work_minutes: number;
  rest_minutes: number;
  postpone_minutes: number;
  idle_minutes: number;
  // 内置等级，或某个自定义方案的名字
  level: string;
  profiles: Profile[];
}

export const DEFAULT_CONFIG: Config = {
  work_minutes: 60,
  rest_minutes: 10,
  postpone_minutes: 5,
  idle_minutes: 5,
  level: "medium",
  profiles: [],
};

// 与 src-tauri/src/i18n.rs 的 Lang::id 一致
export type LangId = "auto" | UiLang;

// 托盘或设置面板切换开机自启时广播，载荷为是否开启
export const AUTOSTART_EVENT = "autostart:changed";

// 配置保存后广播（包括从托盘切换强制方案），载荷为新的配置
export const CONFIG_EVENT = "config:changed";
