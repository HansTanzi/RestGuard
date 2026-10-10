<!-- 设置面板：从托盘菜单“设置…”打开（src-tauri/src/settings.rs）。休息遮罩在 /overlay。 -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { STATE_EVENT, type Snapshot } from "$lib/timer";
  import { LANG_EVENT, settingsStrings, type SettingsStrings } from "$lib/i18n";
  import { AUTOSTART_EVENT, DEFAULT_CONFIG, type Config, type LangId } from "$lib/config";

  let s = $state<SettingsStrings | null>(null);
  let lang = $state<LangId>("auto");
  let autostart = $state(false);
  let phase = $state<Snapshot["phase"]>("working");

  // saved 是后端当前生效的配置，draft 是表单里正在编辑的值
  let saved = $state<Config | null>(null);
  let draft = $state<Config>({ ...DEFAULT_CONFIG });
  let saving = $state(false);
  let message = $state<{ text: string; error: boolean } | null>(null);
  let messageTimer: ReturnType<typeof setTimeout> | undefined;

  const locked = $derived(phase !== "working");
  // bind:value 在输入框清空时给出 null
  const positive = (v: number | null) => typeof v === "number" && Number.isFinite(v) && v > 0;
  const errors = $derived({
    work_minutes: !positive(draft.work_minutes),
    rest_minutes: !positive(draft.rest_minutes),
    postpone_minutes: !positive(draft.postpone_minutes),
    max_postpones: !(Number.isInteger(draft.max_postpones) && draft.max_postpones >= 0),
  });
  const valid = $derived(!Object.values(errors).some(Boolean));
  const dirty = $derived(
    saved !== null &&
      (Object.keys(saved) as (keyof Config)[]).some((k) => saved![k] !== draft[k]),
  );

  function applyLang(isZh: boolean) {
    s = settingsStrings(isZh);
    document.documentElement.lang = isZh ? "zh-CN" : "en";
    invoke<LangId>("get_lang").then((v) => (lang = v));
  }

  function flash(text: string, error = false) {
    clearTimeout(messageTimer);
    message = { text, error };
    if (!error) messageTimer = setTimeout(() => (message = null), 2000);
  }

  async function save() {
    if (!s) return;
    if (!valid) return flash(s.invalid, true);
    saving = true;
    try {
      const result = await invoke<Config>("save_config", { config: $state.snapshot(draft) });
      // 后端可能修正了越界的值，以返回值为准
      saved = result;
      draft = { ...result };
      flash(s.saved);
    } catch (e) {
      flash(String(e), true);
    } finally {
      saving = false;
    }
  }

  function changeLang() {
    invoke("set_lang", { lang }).catch(console.error);
  }

  function changeAutostart() {
    invoke<boolean>("set_autostart", { on: autostart })
      .then((v) => (autostart = v))
      .catch(console.error);
  }

  onMount(() => {
    invoke<boolean>("is_zh").then(applyLang);
    invoke<Config>("get_config").then((v) => {
      saved = v;
      draft = { ...v };
    });
    invoke<boolean>("get_autostart").then((v) => (autostart = v));
    invoke<Snapshot>("get_state").then((v) => (phase = v.phase));
    const unlistens = [
      listen<boolean>(LANG_EVENT, (e) => applyLang(e.payload)),
      listen<boolean>(AUTOSTART_EVENT, (e) => (autostart = e.payload)),
      listen<Snapshot>(STATE_EVENT, (e) => (phase = e.payload.phase)),
    ];
    return () => {
      unlistens.forEach((p) => p.then((off) => off()));
      clearTimeout(messageTimer);
    };
  });
</script>

{#if s}
  <main>
    <h1>{s.title}</h1>

    <form
      onsubmit={(e) => {
        e.preventDefault();
        save();
      }}
    >
      <fieldset disabled={locked}>
        <section>
          <h2>{s.timing}</h2>
          <label>
            <span>{s.workMinutes}</span>
            <input
              type="number"
              min="0"
              step="any"
              class:invalid={errors.work_minutes}
              bind:value={draft.work_minutes}
            />
          </label>
          <label>
            <span>{s.restMinutes}</span>
            <input
              type="number"
              min="0"
              step="any"
              class:invalid={errors.rest_minutes}
              bind:value={draft.rest_minutes}
            />
          </label>
          <label>
            <span>{s.postponeMinutes}</span>
            <input
              type="number"
              min="0"
              step="any"
              class:invalid={errors.postpone_minutes}
              bind:value={draft.postpone_minutes}
            />
          </label>
          <label>
            <span>{s.maxPostpones}</span>
            <input
              type="number"
              min="0"
              step="1"
              class:invalid={errors.max_postpones}
              bind:value={draft.max_postpones}
            />
          </label>
          <p class="hint">{s.minutesHint}</p>
          {#if saved && draft.work_minutes !== saved.work_minutes}
            <p class="hint accent">{s.workRestartHint}</p>
          {/if}
        </section>

        <section>
          <h2>{s.overlay}</h2>
          <label>
            <span>{s.coverage}</span>
            <span class="range">
              <input
                type="range"
                min="0.1"
                max="1"
                step="0.05"
                bind:value={draft.overlay_coverage}
              />
              <output>{Math.round(draft.overlay_coverage * 100)}%</output>
            </span>
          </label>
        </section>

        <div class="actions">
          <button
            type="button"
            class="ghost"
            onclick={() => (draft = { ...DEFAULT_CONFIG })}
          >
            {s.resetDefaults}
          </button>
          <button type="submit" class="primary" disabled={!dirty || saving}>
            {s.save}
          </button>
        </div>
        <p class="message" class:error={message?.error || locked} aria-live="polite">
          {locked ? s.lockedDuringBreak : (message?.text ?? "")}
        </p>
      </fieldset>
    </form>

    <!-- 语言和开机自启立即生效，不需要保存 -->
    <section>
      <h2>{s.general}</h2>
      <label>
        <span>{s.language}</span>
        <select bind:value={lang} onchange={changeLang}>
          <option value="auto">{s.followSystem}</option>
          <!-- 语言名称始终用其本身的文字显示，与托盘菜单一致 -->
          <option value="zh">简体中文</option>
          <option value="en">English</option>
        </select>
      </label>
      <label class="check">
        <input type="checkbox" bind:checked={autostart} onchange={changeAutostart} />
        <span>{s.autostart}</span>
      </label>
    </section>
  </main>
{/if}

<style>
  :global(html, body) {
    margin: 0;
    min-height: 100%;
    background: #0f172a;
  }

  main {
    --bg-1: #0f172a;
    --bg-2: #1e293b;
    --fg: #e2e8f0;
    --muted: #94a3b8;
    --accent: #38bdf8;
    --warn: #f87171;
    --line: rgb(255 255 255 / 0.12);

    box-sizing: border-box;
    max-width: 520px;
    margin: 0 auto;
    padding: 1.25rem 1.5rem 1.5rem;
    color: var(--fg);
    font-family:
      system-ui,
      -apple-system,
      "Segoe UI",
      "PingFang SC",
      "Microsoft YaHei",
      sans-serif;
    font-size: 0.95rem;
    color-scheme: dark;
  }

  h1 {
    margin: 0 0 1rem;
    font-size: 1.4rem;
    font-weight: 600;
  }

  h2 {
    margin: 0 0 0.75rem;
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--muted);
  }

  fieldset {
    margin: 0;
    padding: 0;
    border: none;
    min-width: 0;
  }

  fieldset:disabled section {
    opacity: 0.5;
  }

  section {
    padding: 1rem;
    margin-bottom: 1rem;
    border: 1px solid var(--line);
    border-radius: 10px;
    background: var(--bg-2);
  }

  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.35rem 0;
  }

  label.check {
    justify-content: flex-start;
    gap: 0.6rem;
  }

  input[type="number"],
  select {
    width: 7.5rem;
    box-sizing: border-box;
    padding: 0.35rem 0.5rem;
    font: inherit;
    color: var(--fg);
    background: var(--bg-1);
    border: 1px solid var(--line);
    border-radius: 6px;
  }

  input[type="number"]:focus,
  select:focus {
    outline: 2px solid var(--accent);
    outline-offset: -1px;
  }

  input.invalid {
    border-color: var(--warn);
  }

  input[type="checkbox"],
  input[type="range"] {
    accent-color: var(--accent);
  }

  .range {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .range input {
    width: 8rem;
  }

  output {
    width: 3rem;
    text-align: right;
    font-variant-numeric: tabular-nums;
  }

  .hint {
    margin: 0.5rem 0 0;
    font-size: 0.8rem;
    color: var(--muted);
  }

  .hint.accent {
    color: var(--accent);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }

  button {
    font: inherit;
    padding: 0.45rem 1.4rem;
    border-radius: 999px;
    cursor: pointer;
    transition:
      background 0.2s,
      opacity 0.2s;
  }

  button:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .primary {
    border: none;
    color: var(--bg-1);
    background: var(--accent);
  }

  .primary:hover:not(:disabled) {
    background: #7dd3fc;
  }

  .ghost {
    border: 1px solid var(--line);
    color: var(--muted);
    background: transparent;
  }

  .ghost:hover:not(:disabled) {
    background: rgb(255 255 255 / 0.06);
  }

  .message {
    min-height: 1.2em;
    margin: 0.5rem 0 1rem;
    text-align: right;
    font-size: 0.85rem;
    color: var(--accent);
  }

  .message.error {
    color: var(--warn);
  }
</style>
