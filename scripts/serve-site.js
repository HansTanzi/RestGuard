// Serves the website locally with the same layout as the GitHub Pages deploy (.github/workflows/pages.yml):
// site/ at the root, docs/images at images/ and the app icon at icon.png. Files are read on every request,
// so edits show up on reload.
// Usage: pnpm site [port]
import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { extname, join, normalize, sep } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = fileURLToPath(new URL("..", import.meta.url));
const PORT = Number(process.argv[2]) || 8000;

const TYPES = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".png": "image/png",
  ".webp": "image/webp",
  ".svg": "image/svg+xml",
};

// Maps a URL path to a file in the repo, or null if it would leave its directory
function resolve(path) {
  if (path === "/icon.png") return join(ROOT, "src-tauri/icons/128x128@2x.png");
  if (path.endsWith("/")) path += "index.html";
  const [base, rel] = path.startsWith("/images/")
    ? [join(ROOT, "docs/images"), path.slice("/images/".length)]
    : [join(ROOT, "site"), path.slice(1)];
  const file = normalize(join(base, rel));
  return file.startsWith(base + sep) ? file : null;
}

createServer(async (req, res) => {
  const path = decodeURIComponent(new URL(req.url, "http://localhost").pathname);
  const file = resolve(path);
  try {
    if (!file) throw new Error("outside the site");
    const body = await readFile(file);
    res.writeHead(200, {
      "Content-Type": TYPES[extname(file)] ?? "application/octet-stream",
      "Cache-Control": "no-store",
    });
    res.end(body);
  } catch {
    res.writeHead(404, { "Content-Type": "text/plain; charset=utf-8" });
    res.end("Not found");
  }
}).listen(PORT, () => console.log(`Website at http://localhost:${PORT}/ (Ctrl+C to stop)`));
