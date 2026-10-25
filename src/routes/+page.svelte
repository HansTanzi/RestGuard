<!-- Settings panel: opened from "Settings…" in the tray menu (src-tauri/src/settings.rs). The break overlay is at /overlay. -->
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
    CONFIG_EVENT,
    BUILTIN_LEVELS,
    BUILTIN_PROFILES,
    DEFAULT_CONFIG,
    NTFY_TOPIC_PATTERN,
    SOUNDS,
    isBuiltinLevel,
    type Config,
    type LangId,
    type Profile,
  } from "$lib/config";

  let s = $state<SettingsStrings | null>(null);
  let lang = $state<LangId>("auto");
  let autostart = $state(false);
  let phase = $state<Snapshot["phase"]>("working");
  let postponed = $state(false);

  // saved is the config currently in effect in the backend; draft holds the values being edited in the form
  let saved = $state<Config | null>(null);
  let draft = $state<Config>(structuredClone(DEFAULT_CONFIG));
  // Selected enforcement profile: a built-in level id, or "custom:<index>".
  // Custom profiles are selected by index, not by name
  let selected = $state<string>(DEFAULT_CONFIG.level);
  // Value of the "New custom…" option in the dropdown; selecting it creates a profile instead of switching
  const NEW_PROFILE = "new";
  // Rename dialog: only written back to draft on confirm
  let renameDialog = $state<HTMLDialogElement>();
  let newName = $state("");
  let saving = $state(false);
  let message = $state<{ text: string; error: boolean } | null>(null);
  let messageTimer: ReturnType<typeof setTimeout> | undefined;

  // Settings are locked during a break and while one is postponed (see settings_unlocked in lib.rs)
  const locked = $derived(phase !== "working" || postponed);
  const customIndex = $derived(selected.startsWith("custom:") ? Number(selected.slice(7)) : -1);
  const profile = $derived<Profile>(
    customIndex >= 0
      ? draft.profiles[customIndex]
      : BUILTIN_PROFILES[isBuiltinLevel(selected) ? selected : "medium"],
  );
  // Config to save: level replaced with the selected profile's name
  const output = $derived<Config>({
    ...draft,
    level: customIndex >= 0 ? draft.profiles[customIndex].name.trim() : selected,
  });

  // bind:value gives null when the input is cleared
  const positive = (v: number | null) => typeof v === "number" && Number.isFinite(v) && v > 0;
  const count = (v: number | null) => Number.isInteger(v) && v! >= 0;
  // Names are used to refer to profiles, so they can't be empty, clash with built-in levels, or duplicate each other (consistent with sanitized in config.rs)
  const nameInvalid = (raw: string, i: number) => {
    const name = raw.trim();
    return (
      !name ||
      isBuiltinLevel(name) ||
      draft.profiles.some((p, j) => j !== i && p.name.trim() === name)
    );
  };
  const badName = (i: number) => nameInvalid(draft.profiles[i].name, i);
  const errors = $derived({
    work_minutes: !positive(draft.work_minutes),
    rest_minutes: !positive(draft.rest_minutes),
    postpone_minutes: !positive(draft.postpone_minutes),
    idle_minutes: !(positive(draft.idle_minutes) || draft.idle_minutes === 0),
    max_postpones: !count(profile.max_postpones),
    // Not checked when the sound is off; the backend replaces file with no path by the default sound
    sound_file: draft.sound_prompt && draft.sound === "file" && !draft.sound_file.trim(),
    // Not checked when push is off
    ntfy_topic: draft.phone_notify && !NTFY_TOPIC_PATTERN.test(draft.ntfy_topic.trim()),
  });
  // Profiles edited earlier must stay valid after switching to another one
  const valid = $derived(
    !Object.values(errors).some(Boolean) &&
      draft.profiles.every((p, i) => !badName(i) && count(p.max_postpones)),
  );
  // Field order always comes from the backend response (new profiles are built in the same order), so JSON can be compared directly
  const dirty = $derived(saved !== null && JSON.stringify(saved) !== JSON.stringify(output));

  function load(config: Config) {
    saved = config;
    draft = structuredClone(config);
    const i = config.profiles.findIndex((p) => p.name === config.level);
    selected = i >= 0 ? `custom:${i}` : config.level;
  }

  // Only custom profiles can be edited
  function edit(patch: Partial<Profile>) {
    if (customIndex >= 0) Object.assign(draft.profiles[customIndex], patch);
  }

  // Create a new profile based on the currently selected one
  function addProfile() {
    if (!s) return;
    const names = new Set(draft.profiles.map((p) => p.name.trim()));
    let n = draft.profiles.length + 1;
    while (names.has(s.defaultProfileName(n))) n++;
    draft.profiles.push({ ...$state.snapshot(profile), name: s.defaultProfileName(n) });
    selected = `custom:${draft.profiles.length - 1}`;
  }

  function openRename() {
    if (customIndex < 0) return;
    newName = draft.profiles[customIndex].name;
    renameDialog?.showModal();
  }

  function rename() {
    if (customIndex < 0 || nameInvalid(newName, customIndex)) return;
    draft.profiles[customIndex].name = newName.trim();
    renameDialog?.close();
  }

  function deleteProfile() {
    if (customIndex < 0) return;
    draft.profiles.splice(customIndex, 1);
    selected = DEFAULT_CONFIG.level;
  }

  function applyState(snap: Snapshot) {
    phase = snap.phase;
    postponed = snap.postponed;
  }

  function applyLang(v: UiLang) {
    s = settingsStrings(v);
    applyDocumentLang(v);
    invoke<LangId>("get_lang").then((id) => (lang = id));
  }

  // Play once using the form's unsaved settings
  function preview() {
    invoke("preview_sound", { config: $state.snapshot(output) }).catch((e) =>
      flash(String(e), true),
    );
  }

  // Generate a hard-to-guess topic the first time push is enabled; anyone who knows the topic can subscribe on the public server
  function changePhoneNotify() {
    if (draft.phone_notify && !draft.ntfy_topic.trim()) {
      const bytes = crypto.getRandomValues(new Uint8Array(6));
      draft.ntfy_topic = "restguard-" + Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
    }
  }

  // Split a pasted full subscription URL (e.g. ntfy.sh/mytopic) into server and topic
  function splitTopicUrl() {
    const value = draft.ntfy_topic.trim().replace(/\/+$/, "");
    const at = value.lastIndexOf("/");
    const server = value.slice(0, at);
    // No path, or just a scheme like https://
    if (at <= 0 || server.endsWith("/")) return;
    draft.ntfy_server = /^[a-z][a-z0-9+.-]*:\/\//i.test(server) ? server : "https://" + server;
    draft.ntfy_topic = value.slice(at + 1);
  }

  // Push a test notification using the form's unsaved settings
  let sendingTest = $state(false);
  function testPhone() {
    if (!s) return;
    const sent = s.testSent;
    sendingTest = true;
    invoke("test_phone_notify", { config: $state.snapshot(output) })
      .then(() => flash(sent))
      .catch((e) => flash(String(e), true))
      .finally(() => (sendingTest = false));
  }

  function pickFile() {
    invoke<string | null>("pick_sound_file")
      .then((path) => path && (draft.sound_file = path))
      .catch((e) => flash(String(e), true));
  }

  const fileName = (path: string) => path.split(/[\\/]/).pop() ?? "";

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
      // The backend may have corrected out-of-range values; trust the returned value
      load(result);
      flash(s.saved);
    } catch (e) {
      flash(String(e), true);
    } finally {
      saving = false;
    }
  }

  function openConfig() {
    invoke("open_config_file").catch((e) => flash(String(e), true));
  }

  // Re-read the config file when the window gains focus, so manual edits apply on switching back.
  // If anything changed the backend emits CONFIG_EVENT, where the form is updated
  function reloadConfig() {
    invoke<Config | null>("reload_config")
      .then((config) => config && s && flash(s.reloaded))
      .catch((e) => flash(String(e), true));
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
    // focus doesn't fire when the window first opens, so load the manually edited config file first, then fetch the current config
    invoke("reload_config")
      .catch((e) => flash(String(e), true))
      .finally(() => invoke<Config>("get_config").then(load));
    invoke<boolean>("get_autostart").then((v) => (autostart = v));
    invoke<Snapshot>("get_state").then(applyState);
    const unlistens = [
      listen<UiLang>(LANG_EVENT, (e) => applyLang(e.payload)),
      listen<boolean>(AUTOSTART_EVENT, (e) => (autostart = e.payload)),
      // With unsaved changes, don't overwrite the form; only update saved, and the form wins on save
      listen<Config>(CONFIG_EVENT, (e) => (dirty ? (saved = e.payload) : load(e.payload))),
      listen<Snapshot>(STATE_EVENT, (e) => applyState(e.payload)),
    ];
    window.addEventListener("focus", reloadConfig);
    return () => {
      window.removeEventListener("focus", reloadConfig);
      unlistens.forEach((p) => p.then((off) => off()));
      clearTimeout(messageTimer);
    };
  });
</script>

{#if s}
  <main>
    <!-- One form for the whole page: title and save bar at the top, groups flowing below -->
    <form
      class="flow"
      onsubmit={(e) => {
        e.preventDefault();
        save();
      }}
    >
      <header>
        <h1>{s.title}</h1>
        <p class="message" class:error={message?.error || locked} aria-live="polite">
          {locked
            ? postponed
              ? s.lockedWhilePostponed
              : s.lockedDuringBreak
            : (message?.text ?? "")}
        </p>
        <div class="actions">
          <!-- Show a description below the button on hover (or keyboard focus) -->
          <span class="tip">
            <button
              type="button"
              class="ghost"
              disabled={locked}
              aria-describedby="config-hint"
              onclick={openConfig}
            >
              {s.openConfig}
            </button>
            <span id="config-hint" role="tooltip">{s.configHint}</span>
          </span>
          <button
            type="button"
            class="ghost"
            disabled={locked}
            onclick={() => {
              // Custom profiles are user-created, so they're kept when restoring defaults
              draft = { ...structuredClone(DEFAULT_CONFIG), profiles: $state.snapshot(draft.profiles) };
              selected = DEFAULT_CONFIG.level;
            }}
          >
            {s.resetDefaults}
          </button>
          <button type="submit" class="primary" disabled={locked || !dirty || saving}>
            {s.save}
          </button>
        </div>
      </header>

      <!-- Language and autostart apply immediately without saving; kept outside the fieldset so they can be changed during a break -->
      <section>
        <h2>{s.general}</h2>
        <label>
          <span>{s.language}</span>
          <select bind:value={lang} onchange={changeLang}>
            <option value="auto">{s.followSystem}</option>
            <!-- Language names are always shown in their own language, as in the tray menu -->
            {#each UI_LANGS as id (id)}
              <option value={id}>{LANG_NAMES[id]}</option>
            {/each}
          </select>
        </label>
        <label class="check">
          <input type="checkbox" bind:checked={autostart} onchange={changeAutostart} />
          <span>{s.autostart}</span>
        </label>
        <!-- Break-over alerts only apply after saving and are locked during a break -->
        <fieldset disabled={locked}>
          <label class="check">
            <input type="checkbox" bind:checked={draft.sound_prompt} />
            <span>{s.soundPrompt}</span>
          </label>
          <label class="sub">
            <span>{s.soundKind}</span>
            <select disabled={!draft.sound_prompt} bind:value={draft.sound}>
              {#each SOUNDS as id (id)}
                <option value={id}>{s.sounds[id]}</option>
              {/each}
            </select>
          </label>
          {#if draft.sound === "file"}
            <!-- Chosen only via the dialog; long paths show just the file name, with the full path in the tooltip -->
            <div class="row sub">
              <span>{s.soundFile}</span>
              <span class="file">
                <span class="name" class:invalid={errors.sound_file} title={draft.sound_file}>
                  {fileName(draft.sound_file) || s.noFile}
                </span>
                <button
                  type="button"
                  class="ghost small"
                  disabled={!draft.sound_prompt}
                  onclick={pickFile}
                >
                  {s.chooseFile}
                </button>
              </span>
            </div>
            <p class="hint sub">{s.fileHint}</p>
          {/if}
          <div class="actions profile-actions">
            <button
              type="button"
              class="ghost"
              disabled={!draft.sound_prompt || errors.sound_file}
              onclick={preview}
            >
              {s.preview}
            </button>
          </div>
        </fieldset>
      </section>

      <fieldset disabled={locked}>
        <section>
          <h2>{s.phone}</h2>
          <label class="check">
            <input type="checkbox" bind:checked={draft.phone_notify} onchange={changePhoneNotify} />
            <span>{s.phoneNotify}</span>
          </label>
          <label class="sub">
            <span>{s.ntfyServer}</span>
            <input
              type="text"
              class="wide"
              spellcheck="false"
              placeholder={DEFAULT_CONFIG.ntfy_server}
              disabled={!draft.phone_notify}
              bind:value={draft.ntfy_server}
            />
          </label>
          <label class="sub">
            <span>{s.ntfyTopic}</span>
            <input
              type="text"
              class="wide"
              spellcheck="false"
              class:invalid={errors.ntfy_topic}
              disabled={!draft.phone_notify}
              bind:value={draft.ntfy_topic}
              onchange={splitTopicUrl}
            />
          </label>
          {#if errors.ntfy_topic}
            <p class="hint sub warn">{s.ntfyTopicRule}</p>
          {/if}
          <p class="hint sub">{s.phoneHint}</p>
          <div class="actions profile-actions">
            <button
              type="button"
              class="ghost"
              disabled={!draft.phone_notify || errors.ntfy_topic || sendingTest}
              onclick={testPhone}
            >
              {s.sendTest}
            </button>
          </div>
        </section>
      </fieldset>

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
      </fieldset>

      <fieldset disabled={locked}>
        <!-- Built-in levels are read-only; custom profiles are editable -->
        <section>
          <h2>{s.enforcement}</h2>
          <label>
            <span>{s.level}</span>
            <select
              bind:value={
                () => selected,
                (v) => (v === NEW_PROFILE ? addProfile() : (selected = v))
              }
            >
              {#each BUILTIN_LEVELS as id (id)}
                <option value={id}>{s.levels[id]}</option>
              {/each}
              {#each draft.profiles as p, i (i)}
                <option value={`custom:${i}`}>{p.name.trim() || "…"}</option>
              {/each}
              <option value={NEW_PROFILE}>{s.newProfile}</option>
            </select>
          </label>
          <label>
            <span>{s.coverageNormal}</span>
            <span class="range">
              <input
                type="range"
                min="0.1"
                max="1"
                step="0.05"
                disabled={customIndex < 0}
                value={profile.overlay_coverage}
                oninput={(e) => edit({ overlay_coverage: e.currentTarget.valueAsNumber })}
              />
              <output>{Math.round(profile.overlay_coverage * 100)}%</output>
            </span>
          </label>
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
              checked={!profile.force_rest}
              onchange={(e) => edit({ force_rest: !e.currentTarget.checked })}
            />
            <span>{s.allowExit}</span>
          </label>
          <!-- Unforced breaks can always be skipped -->
          <label class="check">
            <input
              type="checkbox"
              disabled={customIndex < 0 || !profile.force_rest}
              checked={profile.allow_skip || !profile.force_rest}
              onchange={(e) => edit({ allow_skip: e.currentTarget.checked })}
            />
            <span>{s.allowSkip}</span>
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
          <!-- The shrink ratio belongs to urgent mode, shown by indentation -->
          <label class="sub">
            <span>{s.urgentScale}</span>
            <span class="range">
              <input
                type="range"
                min="0.1"
                max="0.9"
                step="0.05"
                disabled={customIndex < 0 || !profile.force_rest || !profile.urgent_mode}
                value={profile.urgent_scale}
                oninput={(e) => edit({ urgent_scale: e.currentTarget.valueAsNumber })}
              />
              <output>{Math.round(profile.urgent_scale * 100)}%</output>
            </span>
          </label>
          <label class="check">
            <input
              type="checkbox"
              disabled={customIndex < 0}
              checked={profile.movable}
              onchange={(e) => edit({ movable: e.currentTarget.checked })}
            />
            <span>{s.movable}</span>
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
          {:else}
            <div class="actions profile-actions">
              <button type="button" class="ghost" onclick={openRename}>
                {s.renameProfile}
              </button>
              <button type="button" class="ghost" onclick={deleteProfile}>
                {s.deleteProfile}
              </button>
            </div>
          {/if}
        </section>
      </fieldset>
    </form>

    <!-- Outside the settings form so Enter only submits the dialog's own form -->
    <dialog bind:this={renameDialog}>
      <form
        onsubmit={(e) => {
          e.preventDefault();
          rename();
        }}
      >
        <h2>{s.renameProfile}</h2>
        <label>
          <span>{s.profileName}</span>
          <!-- svelte-ignore a11y_autofocus -->
          <input
            type="text"
            autofocus
            class:invalid={customIndex >= 0 && nameInvalid(newName, customIndex)}
            bind:value={newName}
          />
        </label>
        <div class="actions profile-actions">
          <button type="button" class="ghost" onclick={() => renameDialog?.close()}>
            {s.cancel}
          </button>
          <button
            type="submit"
            class="primary"
            disabled={customIndex < 0 || nameInvalid(newName, customIndex)}
          >
            {s.confirm}
          </button>
        </div>
      </form>
    </dialog>
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
    /* The window is maximized by default; up to three columns on wide screens (see .flow) */
    max-width: 1200px;
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

  /* Title and save bar: stick to the top when scrolling */
  header {
    position: sticky;
    top: 0;
    z-index: 1;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem 1rem;
    padding: 0.5rem 0;
    background: var(--bg-1);
  }

  h1 {
    margin: 0;
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

  fieldset:disabled section,
  section fieldset:disabled {
    opacity: 0.5;
  }

  /* Groups flow with the window width: one column when narrow, several when wide */
  .flow {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(100%, 22rem), 1fr));
    gap: 1rem;
    align-items: start;
  }

  .flow > header {
    grid-column: 1 / -1;
  }

  section {
    padding: 1rem;
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

  label.sub,
  .row.sub,
  .hint.sub {
    padding-inline-start: 1.6rem;
  }

  /* Same one-row layout as label, but it contains buttons so it can't be a label */
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.35rem 0;
  }

  .file {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .file .name {
    max-width: 10rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--muted);
  }

  .file .name.invalid {
    color: var(--warn);
  }

  button.small {
    padding: 0.25rem 0.9rem;
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

  /* Some language names are long (e.g. Bahasa Indonesia) */
  select {
    width: auto;
    min-width: 7.5rem;
  }

  /* Hint text and sound names are long */
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

  .hint.warn {
    color: var(--warn);
  }

  .actions {
    display: flex;
    /* The title bar has three buttons, which wrap in narrow windows */
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: 0.75rem;
  }

  .tip {
    position: relative;
  }

  /* Left-aligned with the button: the button is leftmost, so it stays inside the window even after wrapping */
  .tip [role="tooltip"] {
    position: absolute;
    top: calc(100% + 0.4rem);
    left: 0;
    width: max-content;
    max-width: min(20rem, calc(100vw - 3rem));
    padding: 0.45rem 0.65rem;
    font-size: 0.8rem;
    color: var(--fg);
    background: var(--bg-2);
    border: 1px solid var(--line);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgb(0 0 0 / 0.4);
    opacity: 0;
    visibility: hidden;
    pointer-events: none;
    transition:
      opacity 0.15s,
      visibility 0.15s;
  }

  .tip:hover [role="tooltip"],
  .tip:has(:focus-visible) [role="tooltip"] {
    opacity: 1;
    visibility: visible;
  }

  .profile-actions {
    margin-top: 0.75rem;
  }

  dialog {
    min-width: min(22rem, calc(100vw - 2rem));
    box-sizing: border-box;
    padding: 1rem;
    color: var(--fg);
    background: var(--bg-2);
    border: 1px solid var(--line);
    border-radius: 10px;
  }

  dialog::backdrop {
    background: rgb(0 0 0 / 0.5);
  }

  dialog input[type="text"],
  input.wide {
    width: 12rem;
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

  /* Sits between the title and buttons, filling the remaining width; wraps to the next line when too narrow */
  .message {
    flex: 1 1 8rem;
    min-height: 1.2em;
    margin: 0;
    text-align: end;
    font-size: 0.85rem;
    color: var(--accent);
  }

  .message.error {
    color: var(--warn);
  }
</style>
