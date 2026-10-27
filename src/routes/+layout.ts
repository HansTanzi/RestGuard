// Tauri doesn't have a Node.js server to do proper SSR, so render everything on the client
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;
// Generate a separate html per route (e.g. overlay.html) so overlay windows load it directly by path
export const prerender = true;

// Release builds block the WebView's built-in browser behavior (context menu reload/save as/print/inspect,
// and the reload, print and devtools shortcuts); only pnpm tauri dev keeps them for debugging.
// Prerendering runs this module in Node, so check whether window exists
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
