<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { STATE_EVENT, formatClock, type Snapshot } from "$lib/timer";
  import {
    LANG_EVENT,
    LANG_NAMES,
    UI_LANGS,
    applyDocumentLang,
    strings,
    type Strings,
    type UiLang,
  } from "$lib/i18n";

  const win = getCurrentWebviewWindow();
  // Matches SHRINK_EVENT in src-tauri/src/lib.rs; payload is whether the overlay is shrunk
  const SHRINK_EVENT = "overlay:shrunk";

  const RADIUS = 120;
  const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

  let snap = $state<Snapshot | null>(null);
  let s = $state<Strings | null>(null);
  let busy = $state(false);
  let lang = $state<UiLang>("en");
  // Overlay windows are destroyed when the break ends, so they reopen at the original size
  let shrunk = $state(false);

  // Draggable only when the enforcement level allows it and the overlay doesn't fill the screen
  const movable = $derived(!!snap?.movable && (shrunk || !snap.fullScreen));
  const resting = $derived(snap?.phase === "resting");
  const restOver = $derived(snap?.phase === "restOver");
  const progress = $derived(
    snap && snap.totalSecs > 0 ? 1 - snap.remainingSecs / snap.totalSecs : 0,
  );

  function applyLang(v: UiLang) {
    lang = v;
    s = strings(v);
    applyDocumentLang(v);
  }

  // The backend saves the choice and broadcasts LANG_EVENT, updating all overlays and the tray.
  // Same as picking a language manually in the tray menu
  function changeLang(e: Event) {
    const value = (e.currentTarget as HTMLSelectElement).value;
    invoke("set_lang", { lang: value }).catch(console.error);
  }

  // Urgent mode: all overlays shrink by half but stay on top, enough to handle something urgent but not to keep working comfortably.
  // The backend broadcasts SHRINK_EVENT after scaling, and each overlay updates shrunk accordingly
  function toggleShrink() {
    invoke("set_overlay_shrunk", { shrunk: !shrunk }).catch(console.error);
  }

  // When draggable, dragging empty space moves the window; the backend keeps it on its own screen
  function onMouseDown(e: MouseEvent) {
    if (!movable || e.button !== 0) return;
    if ((e.target as Element).closest("button, select")) return;
    win.startDragging().catch(console.error);
  }

  onMount(() => {
    invoke<UiLang>("ui_lang").then(applyLang);
    invoke<Snapshot>("get_state").then((v) => (snap = v));
    const unlistens = [
      listen<Snapshot>(STATE_EVENT, (e) => (snap = e.payload)),
      listen<UiLang>(LANG_EVENT, (e) => applyLang(e.payload)),
      listen<boolean>(SHRINK_EVENT, (e) => (shrunk = e.payload)),
    ];
    // pnpm tauri dev only: Esc ends the break immediately so the overlay doesn't lock you out while debugging
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") invoke("dev_skip_rest").catch(console.error);
    };
    if (import.meta.env.DEV) window.addEventListener("keydown", onKey);
    return () => {
      unlistens.forEach((p) => p.then((off) => off()));
      window.removeEventListener("keydown", onKey);
    };
  });

  async function run(command: "start_work" | "postpone" | "in_meeting" | "dismiss_break") {
    busy = true;
    try {
      await invoke(command);
    } catch (e) {
      console.error(e);
    } finally {
      busy = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<main class:movable onmousedown={onMouseDown} in:fade={{ duration: 600 }}>
  {#if snap && s}
    <!-- The enforcement level can disable urgent mode; unforced breaks can just be skipped, no need to shrink -->
    {#if snap.canShrink || shrunk}
      <button class="ghost shrink" onclick={toggleShrink}>
        {shrunk ? s.unshrink : s.shrink}
      </button>
    {/if}
    <!-- Language names are always shown in their own language -->
    <select class="ghost lang" aria-label="Language" value={lang} onchange={changeLang}>
      {#each UI_LANGS as id (id)}
        <option value={id}>{LANG_NAMES[id]}</option>
      {/each}
    </select>
    <h1 in:fly={{ y: -20, duration: 600 }}>{s.title}</h1>
    <p class="tip">{s.tip}</p>

    <div class="ring">
      <svg viewBox="0 0 280 280" aria-hidden="true">
        <circle class="track" cx="140" cy="140" r={RADIUS} />
        <circle
          class="bar"
          cx="140"
          cy="140"
          r={RADIUS}
          stroke-dasharray={CIRCUMFERENCE}
          stroke-dashoffset={CIRCUMFERENCE * (1 - progress)}
        />
      </svg>
      <div class="clock" class:done={restOver}>
        {restOver ? s.restOver : formatClock(snap.remainingSecs)}
      </div>
    </div>

    <div class="actions">
      {#if restOver}
        <button class="primary" disabled={busy} onclick={() => run("start_work")} in:fade>
          {s.startWork}
        </button>
      {/if}
      {#if resting && snap.canPostpone}
        <button class="ghost" disabled={busy} onclick={() => run("postpone")}>
          {s.postpone(snap.postponesLeft)}
        </button>
      {/if}
      {#if resting && snap.meetingApp}
        <button class="ghost" disabled={busy} onclick={() => run("in_meeting")}>
          {s.inMeeting(snap.meetingApp)}
        </button>
      {/if}
      {#if resting && snap.canSkip}
        <button class="ghost" disabled={busy} onclick={() => run("dismiss_break")}>
          {s.skipBreak}
        </button>
      {/if}
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
  }

  main {
    --bg-1: #0f172a;
    --bg-2: #1e3a5f;
    --fg: #e2e8f0;
    --muted: #94a3b8;
    --accent: #38bdf8;
    --warn: #f87171;

    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1.5rem;
    color: var(--fg);
    background: radial-gradient(ellipse at center, var(--bg-2), var(--bg-1) 70%);
    font-family:
      system-ui,
      -apple-system,
      "Segoe UI",
      "PingFang SC",
      "Microsoft YaHei",
      sans-serif;
  }

  main.movable {
    cursor: move;
  }

  h1 {
    margin: 0;
    font-size: clamp(1.8rem, 4vw, 3rem);
    font-weight: 600;
    color: var(--warn);
    letter-spacing: 0.05em;
  }

  .tip {
    margin: 0;
    color: var(--muted);
    font-size: 1.1rem;
  }

  .ring {
    position: relative;
    width: min(280px, 50vmin);
    aspect-ratio: 1;
  }

  svg {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }

  circle {
    fill: none;
    stroke-width: 10;
  }

  .track {
    stroke: rgb(255 255 255 / 0.08);
  }

  .bar {
    stroke: var(--accent);
    stroke-linecap: round;
    transition: stroke-dashoffset 0.5s linear;
  }

  .clock {
    position: absolute;
    inset: 0;
    display: grid;
    place-items: center;
    font-size: clamp(2rem, 9vmin, 4rem);
    font-variant-numeric: tabular-nums;
    font-weight: 300;
  }

  .clock.done {
    font-size: clamp(1.4rem, 5vmin, 2.2rem);
    color: var(--accent);
  }

  .actions {
    min-height: 6rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
  }

  button,
  select {
    font: inherit;
    border-radius: 999px;
    cursor: pointer;
    transition:
      background 0.2s,
      opacity 0.2s;
  }

  button:disabled {
    opacity: 0.5;
    cursor: wait;
  }

  .primary {
    padding: 0.8rem 2.5rem;
    font-size: 1.2rem;
    border: none;
    color: var(--bg-1);
    background: var(--accent);
  }

  .primary:hover:not(:disabled) {
    background: #7dd3fc;
  }

  .ghost {
    padding: 0.4rem 1.2rem;
    font-size: 0.9rem;
    border: 1px solid rgb(255 255 255 / 0.2);
    color: var(--muted);
    background: transparent;
  }

  .ghost:hover:not(:disabled) {
    background: rgb(255 255 255 / 0.06);
  }

  select.ghost {
    color-scheme: dark;
    outline: none;
  }

  /* The dropdown list doesn't inherit the transparent background, so give it a dark one */
  option {
    color: var(--fg);
    background: var(--bg-1);
  }

  .lang,
  .shrink {
    position: absolute;
    top: 1.25rem;
  }

  .lang {
    right: 1.25rem;
  }

  .shrink {
    left: 1.25rem;
  }

  /* The shrunk window is shorter, so tighten the layout to avoid overflow */
  @media (max-height: 640px) {
    main {
      gap: 0.75rem;
    }

    .tip {
      display: none;
    }

    .actions {
      min-height: 0;
    }
  }
</style>
