<!-- About page: opened from "About RestGuard…" in the tray menu (src-tauri/src/about.rs) -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { STATE_EVENT, type Snapshot } from "$lib/timer";
  import {
    LANG_EVENT,
    aboutStrings,
    applyDocumentLang,
    type AboutStrings,
    type UiLang,
  } from "$lib/i18n";
  // Use the app icon directly; static/favicon.png is only 32px and blurs when scaled up
  import icon from "../../../src-tauri/icons/128x128.png";

  // Keep in sync with AboutInfo in src-tauri/src/about.rs
  interface AboutInfo {
    version: string;
    license: string;
    website_url: string;
    repo_url: string;
    issues_url: string;
    privacy_url: string;
  }

  // Keys accepted by open_link in src-tauri/src/about.rs
  type Link = "website" | "repo" | "issues" | "privacy";

  let s = $state<AboutStrings | null>(null);
  let info = $state<AboutInfo | null>(null);
  let phase = $state<Snapshot["phase"]>("working");
  let error = $state("");

  const locked = $derived(phase !== "working");

  function applyLang(v: UiLang) {
    s = aboutStrings(v);
    applyDocumentLang(v);
  }

  // Links are opened by the backend in the system browser, not navigated in the WebView
  function open(e: MouseEvent, link: Link) {
    e.preventDefault();
    if (locked) return;
    error = "";
    invoke("open_link", { link }).catch((err) => (error = String(err)));
  }

  // Strip the scheme and trailing slash for a more compact display
  const short = (url: string) => url.replace(/^https?:\/\//, "").replace(/\/$/, "");

  const rows = $derived<{ link: Link; label: string; url: string }[]>(
    s && info
      ? [
          { link: "website", label: s.website, url: info.website_url },
          { link: "repo", label: s.github, url: info.repo_url },
          { link: "issues", label: s.feedback, url: info.issues_url },
        ]
      : [],
  );

  onMount(() => {
    invoke<UiLang>("ui_lang").then(applyLang);
    invoke<AboutInfo>("get_about").then((v) => (info = v));
    invoke<Snapshot>("get_state").then((v) => (phase = v.phase));
    const unlistens = [
      listen<UiLang>(LANG_EVENT, (e) => applyLang(e.payload)),
      listen<Snapshot>(STATE_EVENT, (e) => (phase = e.payload.phase)),
    ];
    return () => unlistens.forEach((p) => p.then((off) => off()));
  });
</script>

{#if s && info}
  <main>
    <img src={icon} alt="" width="64" height="64" />
    <h1>RestGuard</h1>
    <p class="tagline">{s.tagline}</p>
    <p class="version">{s.version} {info.version}</p>
    <!-- "MIT OR Apache-2.0" (SPDX) is shown as "MIT / Apache-2.0" -->
    <p class="license">{s.openSource} · {info.license.replaceAll(" OR ", " / ")}</p>

    <ul class:locked>
      {#each rows as row (row.link)}
        <li>
          <span>{row.label}</span>
          <a href={row.url} title={row.url} onclick={(e) => open(e, row.link)}>{short(row.url)}</a>
        </li>
      {/each}
    </ul>

    <p class="message" aria-live="polite">{locked ? s.lockedDuringBreak : error}</p>

    <footer class:locked>
      <a href={info.privacy_url} onclick={(e) => open(e, "privacy")}>{s.privacy}</a>
    </footer>
  </main>
{/if}

<style>
  :global(html, body) {
    margin: 0;
    min-height: 100%;
    background: #0f172a;
  }

  /* Colors match the settings panel (src/routes/+page.svelte) */
  main {
    --bg-1: #0f172a;
    --bg-2: #1e293b;
    --fg: #e2e8f0;
    --muted: #94a3b8;
    --accent: #38bdf8;
    --warn: #f87171;
    --line: rgb(255 255 255 / 0.12);

    display: flex;
    flex-direction: column;
    align-items: center;
    box-sizing: border-box;
    padding: 1.5rem 1.5rem 1rem;
    color: var(--fg);
    font-family:
      system-ui,
      -apple-system,
      "Segoe UI",
      "PingFang SC",
      "Microsoft YaHei",
      sans-serif;
    font-size: 0.95rem;
    text-align: center;
    color-scheme: dark;
    user-select: none;
  }

  img {
    border-radius: 14px;
  }

  h1 {
    margin: 0.6rem 0 0.25rem;
    font-size: 1.4rem;
    font-weight: 600;
  }

  .tagline {
    margin: 0;
    font-size: 0.85rem;
    color: var(--muted);
  }

  .version {
    margin: 0.5rem 0 0.2rem;
    font-size: 0.85rem;
    font-variant-numeric: tabular-nums;
    user-select: text;
  }

  .license {
    margin: 0 0 1rem;
    font-size: 0.8rem;
    color: var(--muted);
  }

  ul {
    width: 100%;
    margin: 0;
    padding: 0.5rem 1rem;
    box-sizing: border-box;
    list-style: none;
    border: 1px solid var(--line);
    border-radius: 10px;
    background: var(--bg-2);
  }

  .locked {
    opacity: 0.5;
  }

  .locked a {
    pointer-events: none;
  }

  li {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.4rem 0;
  }

  li + li {
    border-top: 1px solid var(--line);
  }

  li span {
    color: var(--muted);
    white-space: nowrap;
  }

  a {
    overflow: hidden;
    color: var(--accent);
    text-decoration: none;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  a:hover {
    text-decoration: underline;
  }

  .message {
    min-height: 1.2em;
    margin: 0.75rem 0 0;
    font-size: 0.8rem;
    color: var(--warn);
  }

  footer {
    margin-top: 0.5rem;
    font-size: 0.8rem;
  }

  footer a {
    color: var(--muted);
  }

  footer a:hover {
    color: var(--accent);
  }
</style>
