// Static landing page: translations, OS detection and download links from the latest GitHub Release
const REPO = "HansTanzi/RestGuard";
const RELEASES_URL = `https://github.com/${REPO}/releases/latest`;

// English text lives in index.html; only the other languages are listed here
const TEXT = {
  zh: {
    "nav.levels": "强制等级",
    "nav.features": "功能",
    "nav.install": "安装",
    "hero.tagline": "真正让你休息的休息提醒工具。",
    "hero.lead": "内置三种强制等级：既可提醒你休息，也可强制你休息。",
    "hero.download": "下载",
    "hero.downloadFor": "下载 {os} 版",
    "hero.other": "其他平台与包管理器",
    "hero.free": "免费开源 · MIT / Apache-2.0",
    "hero.version": "最新版本 {version}",
    "levels.title": "三种强制等级",
    "levels.lead": "选择你需要的严格程度。相比同类软件，强制模式是 RestGuard 最大的特色。",
    "levels.high": "高等强制",
    "levels.highDesc": "会尽可能强迫你休息。",
    "levels.medium": "中等强制（默认）",
    "levels.mediumDesc": "依然可以办公，但会尽可能降低办公体验，半强制你休息。",
    "levels.low": "低等强制",
    "levels.lowDesc": "和其他休息提醒软件一样，主要是提醒。",
    "table.postpones": "推迟次数",
    "table.urgent": "紧急模式（缩小遮罩）",
    "table.meeting": "检测会议软件",
    "table.skip": "允许关闭 / 跳过本次休息",
    "table.coverage": "遮罩覆盖屏幕",
    "table.drag": "遮罩可拖动",
    "features.title": "为真正的休息而生",
    "f.force.t": "真正的强制休息",
    "f.force.d": "每工作 60 分钟强制休息 10 分钟，每块屏幕都会被遮挡。Alt+F4 无效，休息期间也无法从托盘退出。",
    "f.custom.t": "高度自定义",
    "f.custom.d": "以任意方案为模板新建自定义方案，调整推迟次数、跳过、紧急模式、覆盖比例和拖动，数量不限。",
    "f.meeting.t": "该安静时不打扰",
    "f.meeting.d": "使用 Zoom、Teams、Webex 或腾讯会议时会等会议结束；离开键盘时自动暂停计时。",
    "f.notify.t": "不错过休息结束",
    "f.notify.d": "休息结束时播放提示音，还可通过 ntfy 推送到手机和智能手表，走开了也知道何时回来。",
    "f.small.t": "小巧高效",
    "f.small.d": "基于 Rust + Tauri 2 和系统 WebView，安装包仅 2.5 ~ 4.6 MB，后台运行几乎不占资源。",
    "f.lang.t": "说你的语言",
    "f.lang.d": "支持 13 种界面语言，默认跟随系统，可随时在托盘菜单中切换。",
    "shot.back": "休息结束后点击「开始工作」",
    "shot.tray": "托盘菜单",
    "install.title": "安装",
    "install.lead": "通过包管理器安装可以自动更新；直接下载的版本免安装，但不会自动更新。",
    "install.direct": "直接下载",
    "install.winNote": "单个 exe 免安装：放在任意位置即可运行，删除即卸载。需要 WebView2 运行时（Windows 11 已预装）。",
    "install.macNote": "通用版本，同时支持 Apple Silicon 和 Intel。解压后把 RestGuard.app 移到「应用程序」。应用未经 Apple 公证，首次运行前请先移除隔离标记：",
    "install.apt": "Debian / Ubuntu（APT 软件源）",
    "install.linuxNote": "仅支持 x86_64。遮罩需要 X11。也支持在 Linux 上用 Homebrew 安装，详见 README。",
    "verify.title": "校验下载文件",
    "verify.attest": "Release 文件由 GitHub Actions 构建，并附带签名的构建来源证明。可用 GitHub CLI 校验：",
    "verify.sums": "或者对照 Release 中的 SHA256SUMS.txt：",
    "footer.releases": "所有版本",
    "footer.issues": "反馈问题",
    "footer.privacy": "隐私政策",
    "footer.license": "采用 MIT / Apache-2.0 双许可证。",
    copy: "复制",
    copied: "已复制",
  },
  de: {
    "nav.levels": "Stufen",
    "nav.features": "Funktionen",
    "nav.install": "Installation",
    "hero.tagline": "Eine Pausenerinnerung, die dich wirklich Pause machen lässt.",
    "hero.lead": "Drei eingebaute Durchsetzungsstufen: Sie erinnert dich an die Pause oder sorgt dafür, dass du sie machst.",
    "hero.download": "Herunterladen",
    "hero.downloadFor": "Für {os} herunterladen",
    "hero.other": "Andere Plattformen & Paketmanager",
    "hero.free": "Kostenlos und quelloffen · MIT / Apache-2.0",
    "hero.version": "Neueste Version {version}",
    "levels.title": "Drei Durchsetzungsstufen",
    "levels.lead": "Wähle, wie streng es sein soll. Die Durchsetzung unterscheidet RestGuard von anderen Pausenerinnerungen.",
    "levels.high": "Hoch",
    "levels.highDesc": "Tut alles, damit du die Pause wirklich machst.",
    "levels.medium": "Mittel (Standard)",
    "levels.mediumDesc": "Arbeiten geht noch, ist aber unbequem genug, um dich zur Pause zu bewegen.",
    "levels.low": "Niedrig",
    "levels.lowDesc": "Wie andere Pausenerinnerungen: vor allem eine Erinnerung.",
    "table.postpones": "Aufschübe",
    "table.urgent": "Notfallmodus (Overlay verkleinern)",
    "table.meeting": "Erkennung von Meeting-Apps",
    "table.skip": "Diese Pause schließen / überspringen",
    "table.coverage": "Bildschirmabdeckung",
    "table.drag": "Verschiebbares Overlay",
    "features.title": "Damit Pausen wirklich stattfinden",
    "f.force.t": "Pausen, die wirklich stattfinden",
    "f.force.d": "Nach 60 Minuten Arbeit 10 Minuten Pause, auf jedem Bildschirm. Alt+F4 bewirkt nichts, und während der Pause lässt sich die App nicht über das Tray beenden.",
    "f.custom.t": "Weitgehend anpassbar",
    "f.custom.d": "Eigene Profile auf Basis jeder Stufe: Aufschübe, Überspringen, Notfallmodus, Abdeckung und Verschieben einzeln einstellen.",
    "f.meeting.t": "Stört nicht, wenn es darauf ankommt",
    "f.meeting.d": "Wartet, solange Zoom, Teams, Webex oder Tencent Meeting läuft, und pausiert den Countdown, wenn du nicht am Rechner bist.",
    "f.notify.t": "Das Pausenende nie verpassen",
    "f.notify.d": "Ein Ton am Ende der Pause, dazu Push-Benachrichtigungen per ntfy auf Smartphone und Smartwatch.",
    "f.small.t": "Klein und schnell",
    "f.small.d": "Rust + Tauri 2 mit der System-WebView. Downloads sind nur 2,5 ~ 4,6 MB groß, im Hintergrund kaum Ressourcenverbrauch.",
    "f.lang.t": "Spricht deine Sprache",
    "f.lang.d": "13 Oberflächensprachen, standardmäßig wie das System. Jederzeit im Tray-Menü umschaltbar.",
    "shot.back": "Nach der Pause auf „Zurück an die Arbeit“ klicken",
    "shot.tray": "Das Tray-Menü",
    "install.title": "Installation",
    "install.lead": "Paketmanager halten RestGuard aktuell. Direkte Downloads sind portabel und aktualisieren sich nicht selbst.",
    "install.direct": "Direkter Download",
    "install.winNote": "Eine portable exe ohne Installer: irgendwo ablegen und zum Deinstallieren löschen. Benötigt die WebView2-Runtime (unter Windows 11 vorinstalliert).",
    "install.macNote": "Universal-Build für Apple Silicon und Intel. Entpacken und RestGuard.app nach /Programme verschieben. Die App ist nicht notarisiert; entferne vor dem ersten Start das Quarantäne-Flag:",
    "install.apt": "Debian / Ubuntu (APT-Repository)",
    "install.linuxNote": "Nur x86_64. Das Overlay benötigt X11. Homebrew unter Linux wird ebenfalls unterstützt, siehe README.",
    "verify.title": "Download überprüfen",
    "verify.attest": "Release-Dateien werden von GitHub Actions mit signierter Build-Provenienz erstellt. Prüfen mit der GitHub CLI:",
    "verify.sums": "Oder mit der SHA256SUMS.txt aus dem Release vergleichen:",
    "footer.releases": "Alle Releases",
    "footer.issues": "Problem melden",
    "footer.privacy": "Datenschutz",
    "footer.license": "Doppelt lizenziert unter MIT / Apache-2.0.",
    copy: "Kopieren",
    copied: "Kopiert",
  },
};

const EN_EXTRA = {
  "hero.downloadFor": "Download for {os}",
  "hero.version": "Latest version {version}",
  copy: "Copy",
  copied: "Copied",
};

const OS_NAMES = { windows: "Windows", macos: "macOS", linux: "Linux" };

// Release file name suffixes per platform; the first one is the primary download
const ASSET_PATTERNS = {
  windows: [["_x64.exe", "Portable .exe"]],
  macos: [["_universal.zip", "Universal .zip"]],
  linux: [
    ["_amd64.deb", ".deb (Debian/Ubuntu)"],
    ["_x86_64.rpm", ".rpm (Fedora/openSUSE)"],
    ["_x86_64.pkg.tar.zst", ".pkg.tar.zst (Arch)"],
  ],
};

const english = new Map();
let lang = "en";
let release = null;

function t(key, vars = {}) {
  const raw = TEXT[lang]?.[key] ?? english.get(key) ?? EN_EXTRA[key] ?? key;
  return raw.replace(/\{(\w+)\}/g, (_, name) => vars[name] ?? "");
}

function detectLang() {
  const param = new URLSearchParams(location.search).get("lang");
  if (param && (param === "en" || TEXT[param])) return param;
  try {
    const saved = localStorage.getItem("lang");
    if (saved && (saved === "en" || TEXT[saved])) return saved;
  } catch {}
  for (const tag of navigator.languages || [navigator.language]) {
    const base = (tag || "").toLowerCase().split("-")[0];
    if (base === "en" || TEXT[base]) return base;
  }
  return "en";
}

function detectOs() {
  const platform = (navigator.userAgentData?.platform || navigator.platform || "").toLowerCase();
  const ua = navigator.userAgent.toLowerCase();
  if (/android|iphone|ipad|ipod/.test(ua)) return null;
  if (platform.includes("win") || ua.includes("windows")) return "windows";
  if (platform.includes("mac") || ua.includes("mac os")) return "macos";
  if (platform.includes("linux") || ua.includes("linux")) return "linux";
  return null;
}

function formatSize(bytes) {
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

function findAsset(suffix) {
  return release?.assets.find((a) => a.name.endsWith(suffix)) ?? null;
}

function applyLang() {
  document.documentElement.lang = lang === "zh" ? "zh-CN" : lang;
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    el.textContent = t(el.dataset.i18n);
  });
  document.querySelectorAll(".copy").forEach((btn) => (btn.textContent = t("copy")));

  // Screenshots exist in English and Chinese
  const dir = lang === "zh" ? "CN" : "EN";
  document.querySelectorAll("img[data-shot]").forEach((img) => {
    img.src = `images/${dir}/${img.dataset.shot}.webp`;
  });
  renderDownloads();
}

function renderDownloads() {
  const os = detectOs();
  const primary = document.getElementById("primary-download");
  const label = document.getElementById("primary-label");
  const meta = document.getElementById("primary-meta");
  const asset = os && findAsset(ASSET_PATTERNS[os][0][0]);

  primary.href = asset ? asset.browser_download_url : RELEASES_URL;
  label.textContent = os ? t("hero.downloadFor", { os: OS_NAMES[os] }) : t("hero.download");
  meta.textContent = asset ? `${release.tag_name} · ${formatSize(asset.size)}` : "";
  document.getElementById("version-line").textContent = release
    ? `${t("hero.version", { version: release.tag_name })} · `
    : "";

  document.querySelectorAll("[data-assets]").forEach((box) => {
    box.replaceChildren();
    for (const [suffix, name] of ASSET_PATTERNS[box.dataset.assets]) {
      const file = findAsset(suffix);
      const link = document.createElement("a");
      link.className = "asset";
      link.href = file ? file.browser_download_url : RELEASES_URL;
      link.textContent = name;
      if (file) {
        const small = document.createElement("small");
        small.textContent = `${file.name} · ${formatSize(file.size)}`;
        link.append(small);
      }
      box.append(link);
    }
  });
}

function setupTabs(buttonSelector, attr, onSelect) {
  const buttons = document.querySelectorAll(buttonSelector);
  const select = (value) => {
    buttons.forEach((b) => b.setAttribute("aria-selected", String(b.dataset[attr] === value)));
    onSelect(value);
  };
  buttons.forEach((b) => b.addEventListener("click", () => select(b.dataset[attr])));
  return select;
}

function setupCopyButtons() {
  document.querySelectorAll("pre").forEach((pre) => {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = "copy";
    btn.addEventListener("click", async () => {
      // Drop trailing "# comment" annotations so the copied commands run as is
      const text = pre.querySelector("code").textContent
        .split("\n")
        .map((line) => line.replace(/\s+#\s.*$/, ""))
        .join("\n");
      try {
        await navigator.clipboard.writeText(text);
        btn.textContent = t("copied");
        setTimeout(() => (btn.textContent = t("copy")), 1500);
      } catch {}
    });
    pre.append(btn);
  });
}

async function loadRelease() {
  try {
    const res = await fetch(`https://api.github.com/repos/${REPO}/releases/latest`, {
      headers: { Accept: "application/vnd.github+json" },
    });
    if (!res.ok) return;
    release = await res.json();
    renderDownloads();
  } catch {
    // Offline or rate limited: links keep pointing at the Releases page
  }
}

function init() {
  document.querySelectorAll("[data-i18n]").forEach((el) => english.set(el.dataset.i18n, el.textContent));

  const levelImg = document.getElementById("level-img");
  setupTabs(".level", "level", (level) => {
    levelImg.dataset.shot = `${level}Enforcement`;
    levelImg.src = `images/${lang === "zh" ? "CN" : "EN"}/${level}Enforcement.webp`;
  })("Medium");

  setupTabs(".os-tabs button", "os", (os) => {
    document.querySelectorAll(".os-panel").forEach((p) => (p.hidden = p.dataset.os !== os));
  })(detectOs() || "windows");

  setupCopyButtons();

  const picker = document.getElementById("lang");
  lang = detectLang();
  picker.value = lang;
  picker.addEventListener("change", () => {
    lang = picker.value;
    try {
      localStorage.setItem("lang", lang);
    } catch {}
    applyLang();
  });
  applyLang();
  loadRelease();
}

init();
