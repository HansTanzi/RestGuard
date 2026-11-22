<!-- Launch dialog: one small window on the display under the mouse, opened by src-tauri/src/already_running.rs when
     RestGuard is launched by hand ("started") or launched again ("already running"); both say where the tray icon is.
     "Open settings" is disabled during a break, like the tray's "Settings…". -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { STATE_EVENT, type Snapshot } from "#lib/timer.ts";
  import {
    LANG_EVENT,
    alreadyRunningStrings,
    applyDocumentLang,
    type AlreadyRunningStrings,
    type UiLang,
  } from "#lib/i18n.ts";

  // Matches Kind and KIND_EVENT in src-tauri/src/already_running.rs
  type LaunchKind = "started" | "alreadyRunning";
  const KIND_EVENT = "already-running:kind";

  let s = $state<AlreadyRunningStrings | null>(null);
  let kind = $state<LaunchKind | null>(null);
  let phase = $state<Snapshot["phase"]>("working");
  let main: HTMLElement;

  const locked = $derived(phase !== "working");
  // WebView2 reports "Windows NT", WKWebView "Macintosh"
  const ua = navigator.userAgent;
  const os = ua.includes("Windows") ? "windows" : ua.includes("Macintosh") ? "mac" : "other";
  const trayHint = $derived(
    s && (os === "windows" ? s.trayWindows : os === "mac" ? s.trayMac : s.trayOther),
  );

  function applyLang(v: UiLang) {
    s = alreadyRunningStrings(v);
    applyDocumentLang(v);
  }

  function close() {
    invoke("close_already_running").catch(console.error);
  }

  function openSettings() {
    if (locked) return;
    invoke("already_running_settings").catch(console.error);
  }

  onMount(() => {
    invoke<UiLang>("ui_lang").then(applyLang);
    invoke<Snapshot>("get_state").then((v) => (phase = v.phase));
    invoke<LaunchKind>("already_running_kind").then((v) => (kind = v));
    const unlistens = [
      listen<UiLang>(LANG_EVENT, (e) => applyLang(e.payload)),
      listen<LaunchKind>(KIND_EVENT, (e) => (kind = e.payload)),
      listen<Snapshot>(STATE_EVENT, (e) => (phase = e.payload.phase)),
    ];
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") close();
    };
    window.addEventListener("keydown", onKey);
    // As on the away-lock prompt: when the texts wrap more than the window holds, ask the backend to grow it
    const fit = new ResizeObserver(() => {
      const need = Math.ceil(main.offsetHeight);
      if (need > window.innerHeight + 1) {
        invoke("fit_already_running", { height: need }).catch(console.error);
      }
    });
    fit.observe(main);
    return () => {
      unlistens.forEach((p) => p.then((off) => off()));
      window.removeEventListener("keydown", onKey);
      fit.disconnect();
    };
  });
</script>

<main bind:this={main}>
  {#if s && kind}
    {#if kind === "started"}
      <h1>{s.startedTitle}</h1>
      <p>{s.startedMessage}</p>
    {:else}
      <h1>{s.title}</h1>
    {/if}
    <p aria-live="polite">{locked ? s.settingsLocked : trayHint}</p>
    <div class="actions">
      <!-- svelte-ignore a11y_autofocus -->
      <button class="primary" autofocus onclick={close}>{s.ok}</button>
      <button class="ghost" disabled={locked} onclick={openSettings}>{s.openSettings}</button>
    </div>
  {/if}
</main>

<style>
  :global(html, body) {
    margin: 0;
    height: 100%;
    overflow: hidden;
    user-select: none;
    cursor: default;
    background: var(--bg-2);
  }

  main {
    box-sizing: border-box;
    /* Not height: content that doesn't fit makes main taller, which the ResizeObserver above notices */
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.9rem;
    padding: 1.5rem 1.5rem 1.25rem;
    text-align: center;
    color: var(--fg);
    border: 1px solid var(--line);
    font-family:
      system-ui,
      -apple-system,
      "Segoe UI",
      "PingFang SC",
      "Microsoft YaHei",
      sans-serif;
  }

  h1 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--accent);
  }

  p {
    margin: 0;
    color: var(--muted);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 0.75rem;
    margin-top: 0.4rem;
  }

  button {
    font: inherit;
    padding: 0.5rem 1.4rem;
    border-radius: 999px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .primary {
    border: none;
    color: var(--bg-1);
    background: var(--accent);
  }

  .primary:hover {
    background: var(--accent-hover);
  }

  .ghost {
    border: 1px solid var(--line);
    color: var(--muted);
    background: transparent;
  }

  .ghost:hover:not(:disabled) {
    background: var(--hover);
  }

  .ghost:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
