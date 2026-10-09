<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import { STATE_EVENT, formatClock, type Snapshot } from "$lib/timer";

  // 与 src-tauri/src/overlay.rs 的 PRIMARY_LABEL 一致
  const primary = getCurrentWebviewWindow().label === "overlay-primary";

  const RADIUS = 120;
  const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

  let snap = $state<Snapshot | null>(null);
  let busy = $state(false);

  const resting = $derived(snap?.phase === "resting");
  const restOver = $derived(snap?.phase === "restOver");
  const progress = $derived(
    snap && snap.totalSecs > 0 ? 1 - snap.remainingSecs / snap.totalSecs : 0,
  );

  onMount(() => {
    invoke<Snapshot>("get_state").then((s) => (snap = s));
    const unlisten = listen<Snapshot>(STATE_EVENT, (e) => (snap = e.payload));
    return () => {
      unlisten.then((off) => off());
    };
  });

  async function run(command: "start_work" | "postpone") {
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

<main class:secondary={!primary} in:fade={{ duration: 600 }}>
  {#if snap}
    {#if primary}
      <h1 in:fly={{ y: -20, duration: 600 }}>保护眼睛，小心猝死！</h1>
      <p class="tip">起来走走，看看远处，喝口水。</p>
    {/if}

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
        {restOver ? "休息结束" : formatClock(snap.remainingSecs)}
      </div>
    </div>

    {#if primary}
      <div class="actions">
        {#if restOver}
          <button class="primary" disabled={busy} onclick={() => run("start_work")} in:fade>
            开始工作
          </button>
        {/if}
        {#if resting && snap.canPostpone}
          <button class="ghost" disabled={busy} onclick={() => run("postpone")}>
            过会儿再休息（还剩 {snap.postponesLeft} 次）
          </button>
        {/if}
      </div>
    {/if}
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

  main.secondary {
    background: var(--bg-1);
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

  button {
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
</style>
