// Tauri doesn't have a Node.js server to do proper SSR, so we use adapter-static.
// Every route is prerendered (see src/routes/+layout.ts), so no SPA fallback page is needed;
// a fallback named index.html would just overwrite the prerendered root page.
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
  },
};

export default config;
