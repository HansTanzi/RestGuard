// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;
// 为每个路由生成独立的 html（如 overlay.html），遮罩窗口按路径直接加载
export const prerender = true;

// 正式版屏蔽 WebView 自带的浏览器行为（右键菜单的刷新/另存为/打印/检查，
// 以及刷新、打印、开发者工具快捷键），仅 pnpm tauri dev 保留以便调试。
// 预渲染在 Node 中执行本模块，需判断 window 是否存在
if (!import.meta.env.DEV && typeof window !== "undefined") {
  window.addEventListener("contextmenu", (e) => e.preventDefault());
  window.addEventListener("keydown", (e) => {
    const key = e.key.toLowerCase();
    const mod = e.ctrlKey || e.metaKey;
    if (
      key === "f5" ||
      key === "f12" ||
      (mod && (key === "r" || key === "p" || key === "s")) ||
      (mod && e.shiftKey && (key === "i" || key === "j" || key === "c"))
    ) {
      e.preventDefault();
    }
  });
}
