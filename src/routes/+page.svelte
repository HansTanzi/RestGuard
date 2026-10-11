<!-- 设置面板：从托盘菜单“设置…”打开（src-tauri/src/settings.rs）。休息遮罩在 /overlay。 -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { STATE_EVENT, type Snapshot } from "$lib/timer";
  import {
    LANG_EVENT,
    LANG_NAMES,
    UI_LANGS,
    applyDocumentLang,
    settingsStrings,
    type SettingsStrings,
    type UiLang,
  } from "$lib/i18n";
  import {
    AUTOSTART_EVENT,
    BUILTIN_LEVELS,
    BUILTIN_PROFILES,
    DEFAULT_CONFIG,
    isBuiltinLevel,
    type Config,
    type LangId,
    type Profile,
  } from "$lib/config";

  let s = $state<SettingsStrings | null>(null);
  let lang = $state<LangId>("auto");
  let autostart = $state(false);
  let phase = $state<Snapshot["phase"]>("working");

  // saved 是后端当前生效的配置，draft 是表单里正在编辑的值
  let saved = $state<Config | null>(null);
  let draft = $state<Config>(structuredClone(DEFAULT_CONFIG));
  // 选中的强制等级：内置等级 id，或 "custom:<下标>"。
  // 自定义方案按下标而不是名字选中，改名的过程中不会跳到别的方案
  let selected = $state<string>(DEFAULT_CONFIG.level);
  let saving = $state(false);
  let message = $state<{ text: string; error: boolean } | null>(null);
  let messageTimer: ReturnType<typeof setTimeout> | undefined;

  const locked = $derived(phase !== "working");
  const customIndex = $derived(selected.startsWith("custom:") ? Number(selected.slice(7)) : -1);
  const profile = $derived<Profile>(
    customIndex >= 0
      ? draft.profiles[customIndex]
      : BUILTIN_PROFILES[isBuiltinLevel(selected) ? selected : "medium"],
  );
  // 要保存的配置：level 换成选中方案的名字
  const output = $derived<Config>({
    ...draft,
    level: customIndex >= 0 ? draft.profiles[customIndex].name.trim() : selected,
  });

  // bind:value 在输入框清空时给出 null
  const positive = (v: number | null) => typeof v === "number" && Number.isFinite(v) && v > 0;
  const count = (v: number | null) => Number.isInteger(v) && v! >= 0;
  // 名字用来引用方案，不能为空、与内置等级重名或彼此重复（与 config.rs 的 sanitized 一致）
  const badName = (i: number) => {
    const name = draft.profiles[i].name.trim();
    return (
      !name ||
      isBuiltinLevel(name) ||
      draft.profiles.some((p, j) => j !== i && p.name.trim() === name)
    );
  };
  const errors = $derived({
    work_minutes: !positive(draft.work_minutes),
    rest_minutes: !positive(draft.rest_minutes),
    postpone_minutes: !positive(draft.postpone_minutes),
    idle_minutes: !(positive(draft.idle_minutes) || draft.idle_minutes === 0),
    profile_name: customIndex >= 0 && badName(customIndex),
    max_postpones: !count(profile.max_postpones),
  });
  // 切到别的方案后，之前编辑过的方案也要有效
  const valid = $derived(
    !Object.values(errors).some(Boolean) &&
      draft.profiles.every((p, i) => !badName(i) && count(p.max_postpones)),
  );
  // 字段顺序都来自后端返回值（新方案也按同样的顺序构造），可以直接比较 JSON
  const dirty = $derived(saved !== null && JSON.stringify(saved) !== JSON.stringify(output));

  function load(config: Config) {
    saved = config;
    draft = structuredClone(config);
    const i = config.profiles.findIndex((p) => p.name === config.level);
    selected = i >= 0 ? `custom:${i}` : config.level;
  }

  // 只有自定义方案可以修改
  function edit(patch: Partial<Profile>) {
    if (customIndex >= 0) Object.assign(draft.profiles[customIndex], patch);
  }

  // 以当前选中的方案为模板新建
  function addProfile() {
    if (!s) return;
    const names = new Set(draft.profiles.map((p) => p.name.trim()));
    let n = draft.profiles.length + 1;
    while (names.has(s.defaultProfileName(n))) n++;
    const { max_postpones, urgent_mode, meeting_detection, force_rest } = profile;
    draft.profiles.push({
      name: s.defaultProfileName(n),
      max_postpones,
      urgent_mode,
      meeting_detection,
      force_rest,
    });
    selected = `custom:${draft.profiles.length - 1}`;
  }

  function deleteProfile() {
    if (customIndex < 0) return;
    draft.profiles.splice(customIndex, 1);
    selected = DEFAULT_CONFIG.level;
  }

  function applyLang(v: UiLang) {
    s = settingsStrings(v);
    applyDocumentLang(v);
    invoke<LangId>("get_lang").then((id) => (lang = id));
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
      const result = await invoke<Config>("save_config", { config: $state.snapshot(output) });
      // 后端可能修正了越界的值，以返回值为准
      load(result);
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
    invoke<UiLang>("ui_lang").then(applyLang);
    invoke<Config>("get_config").then(load);
    invoke<boolean>("get_autostart").then((v) => (autostart = v));
    invoke<Snapshot>("get_state").then((v) => (phase = v.phase));
    const unlistens = [
      listen<UiLang>(LANG_EVENT, (e) => applyLang(e.payload)),
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
            <span>{s.idleMinutes}</span>
            <input
              type="number"
              min="0"
              step="any"
              class:invalid={errors.idle_minutes}
              bind:value={draft.idle_minutes}
            />
          </label>
          <p class="hint">{s.minutesHint}</p>
          <p class="hint">{s.idleHint}</p>
          {#if saved && draft.work_minutes !== saved.work_minutes}
            <p class="hint accent">{s.workRestartHint}</p>
          {/if}
        </section>

        <!-- 内置等级只读展示，自定义方案可以修改 -->
        <section>
          <h2>{s.enforcement}</h2>
          <label>
            <span>{s.level}</span>
            <select bind:value={selected}>
              {#each BUILTIN_LEVELS as id (id)}
                <option value={id}>{s.levels[id]}</option>
              {/each}
              {#each draft.profiles as p, i (i)}
                <option value={`custom:${i}`}>{p.name.trim() || "…"}</option>
              {/each}
            </select>
          </label>
          {#if customIndex >= 0}
            <label>
              <span>{s.profileName}</span>
              <input
                type="text"
                class:invalid={errors.profile_name}
                bind:value={draft.profiles[customIndex].name}
              />
            </label>
          {/if}
          <label>
            <span>{s.maxPostpones}</span>
            <input
              type="number"
              min="0"
              step="1"
              class:invalid={errors.max_postpones}
              disabled={customIndex < 0}
              value={profile.max_postpones}
              oninput={(e) => edit({ max_postpones: e.currentTarget.valueAsNumber })}
            />
          </label>
          <label class="check">
            <input
              type="checkbox"
              disabled={customIndex < 0}
              checked={profile.force_rest}
              onchange={(e) => edit({ force_rest: e.currentTarget.checked })}
            />
            <span>{s.forceRest}</span>
          </label>
          <label class="check">
            <input
              type="checkbox"
              disabled={customIndex < 0 || !profile.force_rest}
              checked={profile.urgent_mode}
              onchange={(e) => edit({ urgent_mode: e.currentTarget.checked })}
            />
            <span>{s.urgentMode}</span>
          </label>
          <label class="check">
            <input
              type="checkbox"
              disabled={customIndex < 0}
              checked={profile.meeting_detection}
              onchange={(e) => edit({ meeting_detection: e.currentTarget.checked })}
            />
            <span>{s.meetingDetection}</span>
          </label>
          {#if customIndex < 0}
            <p class="hint">{s.builtinHint}</p>
          {/if}
          <div class="actions profile-actions">
            {#if customIndex >= 0}
              <button type="button" class="ghost" onclick={deleteProfile}>
                {s.deleteProfile}
              </button>
            {/if}
            <button type="button" class="ghost" onclick={addProfile}>{s.newProfile}</button>
          </div>
        </section>

        <section>
          <h2>{s.overlay}</h2>
          <label>
            <span>{s.coverageNormal}</span>
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
          <label>
            <span>{s.urgentScale}</span>
            <span class="range">
              <input
                type="range"
                min="0.1"
                max="0.9"
                step="0.05"
                bind:value={draft.urgent_scale}
              />
              <output>{Math.round(draft.urgent_scale * 100)}%</output>
            </span>
          </label>
        </section>

        <div class="actions">
          <button
            type="button"
            class="ghost"
            onclick={() => {
              // 自定义方案是用户自己建的，恢复默认值时保留
              draft = { ...structuredClone(DEFAULT_CONFIG), profiles: $state.snapshot(draft.profiles) };
              selected = DEFAULT_CONFIG.level;
            }}
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
          {#each UI_LANGS as id (id)}
            <option value={id}>{LANG_NAMES[id]}</option>
          {/each}
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
  input[type="text"],
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
  input[type="text"]:focus,
  select:focus {
    outline: 2px solid var(--accent);
    outline-offset: -1px;
  }

  /* 部分语言名称较长（如 Bahasa Indonesia） */
  select {
    width: auto;
    min-width: 7.5rem;
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
    text-align: end;
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

  .profile-actions {
    margin-top: 0.75rem;
  }

  input:disabled {
    opacity: 0.6;
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
    text-align: end;
    font-size: 0.85rem;
    color: var(--accent);
  }

  .message.error {
    color: var(--warn);
  }
</style>
