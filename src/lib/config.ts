// 与 src-tauri/src/config.rs 中的 Config 保持一致（字段名与配置文件相同，用 snake_case）
export interface Config {
  work_minutes: number;
  rest_minutes: number;
  postpone_minutes: number;
  max_postpones: number;
  overlay_coverage: number;
}

export const DEFAULT_CONFIG: Config = {
  work_minutes: 60,
  rest_minutes: 10,
  postpone_minutes: 5,
  max_postpones: 3,
  overlay_coverage: 1,
};

// 与 src-tauri/src/i18n.rs 的 Lang::id 一致
export type LangId = "auto" | "zh" | "en";

// 托盘或设置面板切换开机自启时广播，载荷为是否开启
export const AUTOSTART_EVENT = "autostart:changed";
