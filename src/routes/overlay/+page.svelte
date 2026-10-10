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
  // 与 src-tauri/src/lib.rs 的 SHRINK_EVENT 一致，载荷为是否已缩小
  const SHRINK_EVENT = "overlay:shrunk";

  const RADIUS = 120;
  const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

  let snap = $state<Snapshot | null>(null);
  let s = $state<Strings | null>(null);
  let busy = $state(false);
  let lang = $state<UiLang>("en");
  // 遮罩窗口在休息结束时销毁，下次打开自然恢复原大小
  let shrunk = $state(false);

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

  // 后端保存选择并广播 LANG_EVENT，所有遮罩和托盘随之更新。
  // 与托盘菜单里手动选择某种语言等效
  function changeLang(e: Event) {
    const value = (e.currentTarget as HTMLSelectElement).value;
    invoke("set_lang", { lang: value }).catch(console.error);
  }

  // 紧急模式：所有遮罩缩小一半但仍置顶，能处理急事又没法舒服地继续工作。
  // 后端缩放后广播 SHRINK_EVENT，各遮罩据此更新 shrunk
  function toggleShrink() {
    invoke("set_overlay_shrunk", { shrunk: !shrunk }).catch(console.error);
  }

  // 缩小后按住空白处可拖动窗口，后端会把它限制在本屏幕内
  function onMouseDown(e: MouseEvent) {
    if (!shrunk || e.button !== 0) return;
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
    // 仅 pnpm tauri dev：按 Esc 直接结束休息，避免调试时被遮罩锁住
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") invoke("dev_skip_rest").catch(console.error);
    };
    if (import.meta.env.DEV) window.addEventListener("keydown", onKey);
    return () => {
      unlistens.forEach((p) => p.then((off) => off()));
      window.removeEventListener("keydown", onKey);
    };
  });

  async function run(command: "start_work" | "postpone" | "in_meeting") {
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
<main class:movable={shrunk} onmousedown={onMouseDown} in:fade={{ duration: 600 }}>
  {#if snap && s}
    <button class="ghost shrink" onclick={toggleShrink}>
      {shrunk ? s.unshrink : s.shrink}
    </button>
    <!-- 语言名称始终用其本身的文字显示 -->
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

  /* 下拉列表本身不继承透明背景，单独设深色 */
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

  /* 缩小后窗口较矮，收紧布局避免内容溢出 */
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
