<!-- 关于页面：从托盘菜单“关于 RestGuard…”打开（src-tauri/src/about.rs） -->
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
  // 直接用应用图标，static/favicon.png 只有 32px，放大会糊
  import icon from "../../../src-tauri/icons/128x128.png";

  // 与 src-tauri/src/about.rs 中的 AboutInfo 保持一致
  interface AboutInfo {
    version: string;
    repo_url: string;
    issues_url: string;
  }

  let s = $state<AboutStrings | null>(null);
  let info = $state<AboutInfo | null>(null);
  let phase = $state<Snapshot["phase"]>("working");
  let error = $state("");

  const locked = $derived(phase !== "working");

  function applyLang(v: UiLang) {
    s = aboutStrings(v);
    applyDocumentLang(v);
  }

  // 链接由后端用系统浏览器打开，WebView 里不跳转
  function open(e: MouseEvent, link: "repo" | "issues") {
    e.preventDefault();
    if (locked) return;
    error = "";
    invoke("open_link", { link }).catch((err) => (error = String(err)));
  }

  // 去掉协议头，显示得更紧凑
  const short = (url: string) => url.replace(/^https?:\/\//, "");

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

    <ul class:locked>
      <li>
        <span>{s.github}</span>
        <a href={info.repo_url} onclick={(e) => open(e, "repo")}>{short(info.repo_url)}</a>
      </li>
      <li>
        <span>{s.feedback}</span>
        <a href={info.issues_url} onclick={(e) => open(e, "issues")}>{short(info.issues_url)}</a>
      </li>
    </ul>

    <p class="message" aria-live="polite">{locked ? s.lockedDuringBreak : error}</p>
  </main>
{/if}

<style>
  :global(html, body) {
    margin: 0;
    min-height: 100%;
    background: #0f172a;
  }

  /* 配色与设置面板（src/routes/+page.svelte）一致 */
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
    margin: 0.5rem 0 1rem;
    font-size: 0.85rem;
    font-variant-numeric: tabular-nums;
    user-select: text;
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

  ul.locked {
    opacity: 0.5;
  }

  ul.locked a {
    pointer-events: none;
  }

  li {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.35rem 0;
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
</style>
