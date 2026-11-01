import { getCurrentWindow } from "@tauri-apps/api/window";

// Color theme shared by all windows. Stored in the WebView's localStorage, which every window shares,
// so a change in the settings panel reaches the other open windows through the storage event
export const THEMES = ["system", "dark", "light"] as const;
export type Theme = (typeof THEMES)[number];

// Dark was the only theme before the switch existed, so keep it as the default
export const DEFAULT_THEME: Theme = "dark";

const KEY = "theme";
const systemDark = () => window.matchMedia("(prefers-color-scheme: dark)").matches;

export function getTheme(): Theme {
  try {
    const value = localStorage.getItem(KEY);
    return (THEMES as readonly string[]).includes(value ?? "") ? (value as Theme) : DEFAULT_THEME;
  } catch {
    return DEFAULT_THEME;
  }
}

export function setTheme(theme: Theme) {
  try {
    localStorage.setItem(KEY, theme);
  } catch (e) {
    console.error(e);
  }
  applyTheme();
}

// Resolves "system" to dark or light for the stylesheet (src/lib/theme.css), and matches the native title bar.
// The window theme must go back to the system's first: on macOS it also drives prefers-color-scheme in the WebView.
// Break overlays (labels overlay-*, see overlay.rs) always stay dark: a full-screen light page is glaring right when the eyes should rest
function applyTheme() {
  const win = getCurrentWindow();
  const theme: Theme = win.label.startsWith("overlay-") ? "dark" : getTheme();
  win
    .setTheme(theme === "system" ? null : theme)
    .catch(console.error);
  document.documentElement.dataset.theme = theme === "system" ? (systemDark() ? "dark" : "light") : theme;
}

// Called once per window, before the page renders, so it never flashes the wrong theme
export function initTheme() {
  applyTheme();
  window.addEventListener("storage", (e) => e.key === KEY && applyTheme());
  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
    if (getTheme() === "system") applyTheme();
  });
}
