<!-- Away-lock prompt: one small window per display, opened by src-tauri/src/away_lock.rs once the user counts as away.
     The backend owns the countdown and locks the screen when it runs out; clicking "Cancel" on any screen closes them all. -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import {
    LANG_EVENT,
    applyDocumentLang,
    lockPromptStrings,
    type LockPromptStrings,
    type UiLang,
  } from "#lib/i18n.ts";

  // Matches LEFT_EVENT in src-tauri/src/away_lock.rs; payload is the whole seconds left before locking
  const LEFT_EVENT = "lock-prompt:left";

  let s = $state<LockPromptStrings | null>(null);
  let left = $state<number | null>(null);
  let main: HTMLElement;

  function applyLang(v: UiLang) {
    s = lockPromptStrings(v);
    applyDocumentLang(v);
  }

  function cancel() {
    invoke("cancel_lock_prompt").catch(console.error);
  }

  function lockNow() {
    invoke("lock_prompt_now").catch(console.error);
  }

  onMount(() => {
    invoke<UiLang>("ui_lang").then(applyLang);
    invoke<number | null>("lock_prompt_left").then((v) => (left ??= v));
    const unlistens = [
      listen<UiLang>(LANG_EVENT, (e) => applyLang(e.payload)),
      listen<number>(LEFT_EVENT, (e) => (left = e.payload)),
    ];
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") cancel();
    };
    window.addEventListener("keydown", onKey);
    // main is at least as tall as the window and grows past it when the texts wrap more (long languages,
    // big system font): then ask the backend to grow the window to fit
    const fit = new ResizeObserver(() => {
      const need = Math.ceil(main.offsetHeight);
      if (need > window.innerHeight + 1) {
        invoke("fit_lock_prompt", { height: need }).catch(console.error);
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
  <!-- The prompt has no title bar, so name the app that opened it -->
  <div class="brand">RestGuard</div>
  {#if s}
    <h1>{s.title}</h1>
    <p>{s.message(left ?? 30)}</p>
    <div class="actions">
      <!-- svelte-ignore a11y_autofocus -->
      <button class="primary" autofocus onclick={cancel}>{s.cancel}</button>
      <button class="ghost" onclick={lockNow}>{s.lockNow}</button>
    </div>
    <p class="hint">{s.hint}</p>
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
    position: relative;
    box-sizing: border-box;
    /* Not height: content that doesn't fit makes main taller, which the ResizeObserver above notices */
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.9rem;
    /* Top padding keeps the content clear of .brand */
    padding: 2.2rem 1.5rem 1.25rem;
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

  .brand {
    position: absolute;
    top: 0.7rem;
    left: 50%;
    transform: translateX(-50%);
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    color: var(--muted);
  }

  h1 {
    margin: 0;
    font-size: 1.35rem;
    font-weight: 600;
    color: var(--warn);
  }

  p {
    margin: 0;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
  }

  .hint {
    font-size: 0.75rem;
    opacity: 0.8;
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

  .ghost:hover {
    background: var(--hover);
  }
</style>
