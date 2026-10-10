// 界面语言由后端决定（src-tauri/src/i18n.rs），与托盘保持一致

// 托盘或页面切换语言时广播，载荷为实际显示的语言（UiLang）
export const LANG_EVENT = "lang:changed";

// 与 src-tauri/src/i18n.rs 的 Lang::id 一致（不含 auto）
export const UI_LANGS = [
  "zh",
  "en",
  "hi",
  "es",
  "ar",
  "fr",
  "bn",
  "pt",
  "de",
  "ja",
  "it",
  "ko",
  "id",
] as const;
export type UiLang = (typeof UI_LANGS)[number];

// 语言名称始终用其本身的文字显示，与托盘菜单（Lang::native_name）一致
export const LANG_NAMES: Record<UiLang, string> = {
  zh: "简体中文",
  en: "English",
  hi: "हिन्दी",
  es: "Español",
  ar: "العربية",
  fr: "Français",
  bn: "বাংলা",
  pt: "Português",
  de: "Deutsch",
  ja: "日本語",
  it: "Italiano",
  ko: "한국어",
  id: "Bahasa Indonesia",
};

// 更新文档的 lang 和书写方向，让浏览器选对字形（中日韩汉字）并让阿拉伯语从右到左排版
export function applyDocumentLang(lang: UiLang) {
  const el = document.documentElement;
  el.lang = lang === "zh" ? "zh-CN" : lang;
  el.dir = lang === "ar" ? "rtl" : "ltr";
}

// 休息遮罩（src/routes/overlay/+page.svelte）
export interface Strings {
  title: string;
  tip: string;
  restOver: string;
  startWork: string;
  postpone: (left: number) => string;
  // 分钟数与 src-tauri/src/timer.rs 的 MEETING_SNOOZE 一致
  inMeeting: (app: string) => string;
  shrink: string;
  unshrink: string;
}

const overlay: Record<UiLang, Strings> = {
  zh: {
    title: "保护眼睛，小心猝死！",
    tip: "起来走走，看看远处，喝口水。",
    restOver: "休息结束",
    startWork: "开始工作",
    postpone: (left) => `过会儿再休息（还剩 ${left} 次）`,
    inMeeting: (app) => `我在开 ${app} 会议，15 分钟后再提醒`,
    shrink: "有急事，缩小一下",
    unshrink: "恢复大小",
  },
  en: {
    title: "Time for a break!",
    tip: "Stand up, look into the distance, drink some water.",
    restOver: "Break over",
    startWork: "Back to work",
    postpone: (left) => `Not now (${left} left)`,
    inMeeting: (app) => `I'm in a ${app} meeting, remind me in 15 min`,
    shrink: "Urgent? Shrink",
    unshrink: "Restore size",
  },
  hi: {
    title: "ब्रेक का समय!",
    tip: "उठिए, थोड़ा टहलिए, दूर देखिए और पानी पीजिए।",
    restOver: "ब्रेक समाप्त",
    startWork: "काम पर लौटें",
    postpone: (left) => `अभी नहीं (${left} बार बाकी)`,
    inMeeting: (app) => `मैं ${app} मीटिंग में हूँ, 15 मिनट बाद याद दिलाएँ`,
    shrink: "ज़रूरी काम? छोटा करें",
    unshrink: "पूरा आकार",
  },
  es: {
    title: "¡Hora de descansar!",
    tip: "Levántate, mira a lo lejos y bebe agua.",
    restOver: "Descanso terminado",
    startWork: "Volver al trabajo",
    postpone: (left) => `Ahora no (quedan ${left})`,
    inMeeting: (app) => `Estoy en una reunión de ${app}, recuérdamelo en 15 min`,
    shrink: "¿Urgente? Reducir",
    unshrink: "Restaurar tamaño",
  },
  ar: {
    title: "حان وقت الاستراحة!",
    tip: "قم وتمشَّ قليلًا، وانظر إلى البعيد، واشرب بعض الماء.",
    restOver: "انتهت الاستراحة",
    startWork: "العودة إلى العمل",
    postpone: (left) => `ليس الآن (المتبقي ${left})`,
    inMeeting: (app) => `أنا في اجتماع على ${app}، ذكّرني بعد 15 دقيقة`,
    shrink: "أمر عاجل؟ تصغير",
    unshrink: "استعادة الحجم",
  },
  fr: {
    title: "C'est l'heure de la pause !",
    tip: "Levez-vous, regardez au loin, buvez un peu d'eau.",
    restOver: "Pause terminée",
    startWork: "Retour au travail",
    postpone: (left) => `Pas maintenant (encore ${left})`,
    inMeeting: (app) => `Je suis en réunion ${app}, rappel dans 15 min`,
    shrink: "Urgent ? Réduire",
    unshrink: "Taille normale",
  },
  bn: {
    title: "বিরতির সময় হয়েছে!",
    tip: "উঠে একটু হাঁটুন, দূরে তাকান, একটু পানি খান।",
    restOver: "বিরতি শেষ",
    startWork: "কাজে ফিরুন",
    postpone: (left) => `এখন নয় (${left} বার বাকি)`,
    inMeeting: (app) => `আমি ${app} মিটিংয়ে আছি, 15 মিনিট পরে মনে করিয়ে দিন`,
    shrink: "জরুরি? ছোট করুন",
    unshrink: "আগের আকারে ফেরান",
  },
  pt: {
    title: "Hora da pausa!",
    tip: "Levante-se, olhe para longe, beba um pouco de água.",
    restOver: "Pausa encerrada",
    startWork: "Voltar ao trabalho",
    postpone: (left) => `Agora não (restam ${left})`,
    inMeeting: (app) => `Estou em uma reunião do ${app}, lembre-me em 15 min`,
    shrink: "Urgente? Reduzir",
    unshrink: "Restaurar tamanho",
  },
  de: {
    title: "Zeit für eine Pause!",
    tip: "Steh auf, schau in die Ferne, trink etwas Wasser.",
    restOver: "Pause vorbei",
    startWork: "Zurück an die Arbeit",
    postpone: (left) => `Nicht jetzt (noch ${left})`,
    inMeeting: (app) => `Ich bin in einem ${app}-Meeting, in 15 Min. erinnern`,
    shrink: "Dringend? Verkleinern",
    unshrink: "Originalgröße",
  },
  ja: {
    title: "休憩の時間です！",
    tip: "立ち上がって、遠くを眺めて、水を飲みましょう。",
    restOver: "休憩終了",
    startWork: "仕事に戻る",
    postpone: (left) => `あとで（残り ${left} 回）`,
    inMeeting: (app) => `${app} で会議中です。15 分後に通知`,
    shrink: "急用？縮小する",
    unshrink: "元のサイズに戻す",
  },
  it: {
    title: "È ora di una pausa!",
    tip: "Alzati, guarda lontano, bevi un po' d'acqua.",
    restOver: "Pausa terminata",
    startWork: "Torna al lavoro",
    postpone: (left) => `Non ora (ne restano ${left})`,
    inMeeting: (app) => `Sono in una riunione ${app}, ricordamelo tra 15 min`,
    shrink: "Urgente? Riduci",
    unshrink: "Ripristina dimensioni",
  },
  ko: {
    title: "휴식 시간입니다!",
    tip: "일어나서 먼 곳을 바라보고 물을 마시세요.",
    restOver: "휴식 종료",
    startWork: "업무로 돌아가기",
    postpone: (left) => `나중에 (${left}회 남음)`,
    inMeeting: (app) => `${app} 회의 중이에요. 15분 후에 다시 알림`,
    shrink: "급한 일? 축소",
    unshrink: "원래 크기로",
  },
  id: {
    title: "Waktunya istirahat!",
    tip: "Berdiri, lihat ke kejauhan, dan minum air.",
    restOver: "Istirahat selesai",
    startWork: "Kembali bekerja",
    postpone: (left) => `Nanti saja (sisa ${left})`,
    inMeeting: (app) => `Saya sedang rapat ${app}, ingatkan 15 menit lagi`,
    shrink: "Mendesak? Perkecil",
    unshrink: "Kembalikan ukuran",
  },
};

export function strings(lang: UiLang): Strings {
  return overlay[lang] ?? overlay.en;
}

// 设置面板（src/routes/+page.svelte）
export interface SettingsStrings {
  title: string;
  timing: string;
  workMinutes: string;
  restMinutes: string;
  postponeMinutes: string;
  maxPostpones: string;
  minutesHint: string;
  workRestartHint: string;
  overlay: string;
  coverage: string;
  general: string;
  language: string;
  followSystem: string;
  autostart: string;
  save: string;
  saved: string;
  resetDefaults: string;
  invalid: string;
  lockedDuringBreak: string;
}

const settings: Record<UiLang, SettingsStrings> = {
  zh: {
    title: "设置",
    timing: "时间",
    workMinutes: "工作时长（分钟）",
    restMinutes: "休息时长（分钟）",
    postponeMinutes: "每次推迟（分钟）",
    maxPostpones: "最多推迟次数",
    minutesHint: "可以写小数，例如 0.5 表示 30 秒",
    workRestartHint: "修改工作时长后，当前的工作倒计时会按新时长重新开始。",
    overlay: "遮罩",
    coverage: "覆盖屏幕比例",
    general: "通用",
    language: "语言",
    followSystem: "跟随系统",
    autostart: "开机自启",
    save: "保存",
    saved: "已保存",
    resetDefaults: "恢复默认值",
    invalid: "请检查标红的输入项",
    lockedDuringBreak: "休息期间不能修改设置",
  },
  en: {
    title: "Settings",
    timing: "Timing",
    workMinutes: "Work duration (minutes)",
    restMinutes: "Break duration (minutes)",
    postponeMinutes: "Postpone length (minutes)",
    maxPostpones: "Max postpones",
    minutesHint: "Fractions are allowed, e.g. 0.5 = 30 seconds",
    workRestartHint: "Changing the work duration restarts the current work countdown.",
    overlay: "Overlay",
    coverage: "Screen coverage",
    general: "General",
    language: "Language",
    followSystem: "Follow system",
    autostart: "Start at login",
    save: "Save",
    saved: "Saved",
    resetDefaults: "Restore defaults",
    invalid: "Please fix the highlighted fields",
    lockedDuringBreak: "Settings can't be changed during a break",
  },
  hi: {
    title: "सेटिंग्स",
    timing: "समय",
    workMinutes: "काम की अवधि (मिनट)",
    restMinutes: "ब्रेक की अवधि (मिनट)",
    postponeMinutes: "टालने की अवधि (मिनट)",
    maxPostpones: "अधिकतम बार टालना",
    minutesHint: "दशमलव भी चलेगा, जैसे 0.5 = 30 सेकंड",
    workRestartHint: "काम की अवधि बदलने पर मौजूदा काउंटडाउन नई अवधि से फिर शुरू होगा।",
    overlay: "ओवरले",
    coverage: "स्क्रीन कवरेज",
    general: "सामान्य",
    language: "भाषा",
    followSystem: "सिस्टम के अनुसार",
    autostart: "लॉगिन पर शुरू करें",
    save: "सहेजें",
    saved: "सहेजा गया",
    resetDefaults: "डिफ़ॉल्ट पर लौटें",
    invalid: "कृपया लाल रंग वाले फ़ील्ड ठीक करें",
    lockedDuringBreak: "ब्रेक के दौरान सेटिंग्स नहीं बदली जा सकतीं",
  },
  es: {
    title: "Ajustes",
    timing: "Tiempos",
    workMinutes: "Duración del trabajo (minutos)",
    restMinutes: "Duración del descanso (minutos)",
    postponeMinutes: "Duración del aplazamiento (minutos)",
    maxPostpones: "Aplazamientos máximos",
    minutesHint: "Se admiten decimales, p. ej. 0.5 = 30 segundos",
    workRestartHint: "Cambiar la duración del trabajo reinicia la cuenta atrás actual.",
    overlay: "Superposición",
    coverage: "Cobertura de pantalla",
    general: "General",
    language: "Idioma",
    followSystem: "Según el sistema",
    autostart: "Iniciar con el sistema",
    save: "Guardar",
    saved: "Guardado",
    resetDefaults: "Restaurar valores predeterminados",
    invalid: "Corrige los campos resaltados",
    lockedDuringBreak: "No se pueden cambiar los ajustes durante un descanso",
  },
  ar: {
    title: "الإعدادات",
    timing: "التوقيت",
    workMinutes: "مدة العمل (بالدقائق)",
    restMinutes: "مدة الاستراحة (بالدقائق)",
    postponeMinutes: "مدة التأجيل (بالدقائق)",
    maxPostpones: "أقصى عدد للتأجيل",
    minutesHint: "يُسمح بالكسور، مثلًا 0.5 = 30 ثانية",
    workRestartHint: "تغيير مدة العمل يعيد تشغيل العد التنازلي الحالي.",
    overlay: "الطبقة",
    coverage: "نسبة تغطية الشاشة",
    general: "عام",
    language: "اللغة",
    followSystem: "حسب النظام",
    autostart: "التشغيل عند تسجيل الدخول",
    save: "حفظ",
    saved: "تم الحفظ",
    resetDefaults: "استعادة الافتراضيات",
    invalid: "يرجى تصحيح الحقول المميّزة",
    lockedDuringBreak: "لا يمكن تغيير الإعدادات أثناء الاستراحة",
  },
  fr: {
    title: "Paramètres",
    timing: "Durées",
    workMinutes: "Durée de travail (minutes)",
    restMinutes: "Durée de pause (minutes)",
    postponeMinutes: "Durée d'un report (minutes)",
    maxPostpones: "Reports maximum",
    minutesHint: "Décimales acceptées, ex. 0.5 = 30 secondes",
    workRestartHint: "Modifier la durée de travail relance le compte à rebours actuel.",
    overlay: "Voile",
    coverage: "Couverture de l'écran",
    general: "Général",
    language: "Langue",
    followSystem: "Suivre le système",
    autostart: "Lancer au démarrage",
    save: "Enregistrer",
    saved: "Enregistré",
    resetDefaults: "Valeurs par défaut",
    invalid: "Corrigez les champs en rouge",
    lockedDuringBreak: "Impossible de modifier les paramètres pendant une pause",
  },
  bn: {
    title: "সেটিংস",
    timing: "সময়",
    workMinutes: "কাজের সময় (মিনিট)",
    restMinutes: "বিরতির সময় (মিনিট)",
    postponeMinutes: "পেছানোর সময় (মিনিট)",
    maxPostpones: "সর্বোচ্চ পেছানোর সংখ্যা",
    minutesHint: "দশমিক লেখা যায়, যেমন 0.5 = 30 সেকেন্ড",
    workRestartHint: "কাজের সময় বদলালে বর্তমান কাউন্টডাউন নতুন করে শুরু হবে।",
    overlay: "ওভারলে",
    coverage: "স্ক্রিন কভারেজ",
    general: "সাধারণ",
    language: "ভাষা",
    followSystem: "সিস্টেম অনুসারে",
    autostart: "লগইনের সময় চালু করুন",
    save: "সংরক্ষণ",
    saved: "সংরক্ষিত হয়েছে",
    resetDefaults: "ডিফল্টে ফিরুন",
    invalid: "লাল চিহ্নিত ঘরগুলো ঠিক করুন",
    lockedDuringBreak: "বিরতির সময় সেটিংস পরিবর্তন করা যাবে না",
  },
  pt: {
    title: "Configurações",
    timing: "Tempos",
    workMinutes: "Duração do trabalho (minutos)",
    restMinutes: "Duração da pausa (minutos)",
    postponeMinutes: "Duração do adiamento (minutos)",
    maxPostpones: "Máximo de adiamentos",
    minutesHint: "Aceita decimais, ex.: 0.5 = 30 segundos",
    workRestartHint: "Alterar a duração do trabalho reinicia a contagem regressiva atual.",
    overlay: "Sobreposição",
    coverage: "Cobertura da tela",
    general: "Geral",
    language: "Idioma",
    followSystem: "Seguir o sistema",
    autostart: "Iniciar com o sistema",
    save: "Salvar",
    saved: "Salvo",
    resetDefaults: "Restaurar padrões",
    invalid: "Corrija os campos destacados",
    lockedDuringBreak: "Não é possível alterar as configurações durante uma pausa",
  },
  de: {
    title: "Einstellungen",
    timing: "Zeiten",
    workMinutes: "Arbeitsdauer (Minuten)",
    restMinutes: "Pausendauer (Minuten)",
    postponeMinutes: "Aufschubdauer (Minuten)",
    maxPostpones: "Max. Aufschübe",
    minutesHint: "Dezimalwerte erlaubt, z. B. 0.5 = 30 Sekunden",
    workRestartHint: "Eine Änderung der Arbeitsdauer startet den aktuellen Countdown neu.",
    overlay: "Overlay",
    coverage: "Bildschirmabdeckung",
    general: "Allgemein",
    language: "Sprache",
    followSystem: "Systemsprache",
    autostart: "Beim Anmelden starten",
    save: "Speichern",
    saved: "Gespeichert",
    resetDefaults: "Standardwerte",
    invalid: "Bitte die markierten Felder korrigieren",
    lockedDuringBreak: "Während einer Pause können die Einstellungen nicht geändert werden",
  },
  ja: {
    title: "設定",
    timing: "時間",
    workMinutes: "作業時間（分）",
    restMinutes: "休憩時間（分）",
    postponeMinutes: "延期時間（分）",
    maxPostpones: "最大延期回数",
    minutesHint: "小数も使えます（例：0.5 は 30 秒）",
    workRestartHint: "作業時間を変更すると、現在のカウントダウンは新しい時間で再開されます。",
    overlay: "オーバーレイ",
    coverage: "画面カバー率",
    general: "一般",
    language: "言語",
    followSystem: "システムに従う",
    autostart: "ログイン時に起動",
    save: "保存",
    saved: "保存しました",
    resetDefaults: "デフォルトに戻す",
    invalid: "赤く表示された項目を確認してください",
    lockedDuringBreak: "休憩中は設定を変更できません",
  },
  it: {
    title: "Impostazioni",
    timing: "Tempi",
    workMinutes: "Durata del lavoro (minuti)",
    restMinutes: "Durata della pausa (minuti)",
    postponeMinutes: "Durata del rinvio (minuti)",
    maxPostpones: "Rinvii massimi",
    minutesHint: "Sono ammessi decimali, es. 0.5 = 30 secondi",
    workRestartHint: "Modificare la durata del lavoro riavvia il conto alla rovescia attuale.",
    overlay: "Sovrapposizione",
    coverage: "Copertura dello schermo",
    general: "Generale",
    language: "Lingua",
    followSystem: "Segui il sistema",
    autostart: "Avvia all'accesso",
    save: "Salva",
    saved: "Salvato",
    resetDefaults: "Ripristina predefiniti",
    invalid: "Correggi i campi evidenziati",
    lockedDuringBreak: "Non puoi modificare le impostazioni durante una pausa",
  },
  ko: {
    title: "설정",
    timing: "시간",
    workMinutes: "작업 시간(분)",
    restMinutes: "휴식 시간(분)",
    postponeMinutes: "미루기 시간(분)",
    maxPostpones: "최대 미루기 횟수",
    minutesHint: "소수도 입력할 수 있습니다(예: 0.5 = 30초)",
    workRestartHint: "작업 시간을 바꾸면 현재 카운트다운이 새 시간으로 다시 시작됩니다.",
    overlay: "오버레이",
    coverage: "화면 덮는 비율",
    general: "일반",
    language: "언어",
    followSystem: "시스템 설정 따르기",
    autostart: "로그인 시 시작",
    save: "저장",
    saved: "저장됨",
    resetDefaults: "기본값 복원",
    invalid: "빨간색으로 표시된 항목을 확인하세요",
    lockedDuringBreak: "휴식 중에는 설정을 변경할 수 없습니다",
  },
  id: {
    title: "Pengaturan",
    timing: "Waktu",
    workMinutes: "Durasi kerja (menit)",
    restMinutes: "Durasi istirahat (menit)",
    postponeMinutes: "Durasi penundaan (menit)",
    maxPostpones: "Maks. penundaan",
    minutesHint: "Boleh desimal, mis. 0.5 = 30 detik",
    workRestartHint: "Mengubah durasi kerja akan memulai ulang hitung mundur saat ini.",
    overlay: "Lapisan",
    coverage: "Cakupan layar",
    general: "Umum",
    language: "Bahasa",
    followSystem: "Ikuti sistem",
    autostart: "Mulai saat masuk",
    save: "Simpan",
    saved: "Tersimpan",
    resetDefaults: "Kembalikan default",
    invalid: "Periksa kolom yang ditandai merah",
    lockedDuringBreak: "Pengaturan tidak bisa diubah saat istirahat",
  },
};

export function settingsStrings(lang: UiLang): SettingsStrings {
  return settings[lang] ?? settings.en;
}

// 关于页面（src/routes/about/+page.svelte）
export interface AboutStrings {
  tagline: string;
  version: string;
  github: string;
  feedback: string;
  lockedDuringBreak: string;
}

const about: Record<UiLang, AboutStrings> = {
  zh: {
    tagline: "一个真的会让你休息的休息提醒工具",
    version: "版本",
    github: "GitHub 主页",
    feedback: "问题反馈",
    lockedDuringBreak: "休息期间不能打开链接",
  },
  en: {
    tagline: "A break reminder that actually makes you rest",
    version: "Version",
    github: "GitHub",
    feedback: "Report an issue",
    lockedDuringBreak: "Links can't be opened during a break",
  },
  hi: {
    tagline: "एक ब्रेक रिमाइंडर जो सच में आपको आराम करवाता है",
    version: "संस्करण",
    github: "GitHub",
    feedback: "समस्या बताएँ",
    lockedDuringBreak: "ब्रेक के दौरान लिंक नहीं खोले जा सकते",
  },
  es: {
    tagline: "Un recordatorio de descansos que de verdad te hace descansar",
    version: "Versión",
    github: "GitHub",
    feedback: "Informar de un problema",
    lockedDuringBreak: "No se pueden abrir enlaces durante un descanso",
  },
  ar: {
    tagline: "مذكّر استراحة يجعلك تستريح فعلًا",
    version: "الإصدار",
    github: "GitHub",
    feedback: "الإبلاغ عن مشكلة",
    lockedDuringBreak: "لا يمكن فتح الروابط أثناء الاستراحة",
  },
  fr: {
    tagline: "Un rappel de pause qui vous fait vraiment souffler",
    version: "Version",
    github: "GitHub",
    feedback: "Signaler un problème",
    lockedDuringBreak: "Impossible d'ouvrir des liens pendant une pause",
  },
  bn: {
    tagline: "একটি বিরতি রিমাইন্ডার যা সত্যিই আপনাকে বিশ্রাম নেওয়ায়",
    version: "সংস্করণ",
    github: "GitHub",
    feedback: "সমস্যা জানান",
    lockedDuringBreak: "বিরতির সময় লিংক খোলা যাবে না",
  },
  pt: {
    tagline: "Um lembrete de pausas que realmente faz você descansar",
    version: "Versão",
    github: "GitHub",
    feedback: "Relatar um problema",
    lockedDuringBreak: "Não é possível abrir links durante uma pausa",
  },
  de: {
    tagline: "Eine Pausenerinnerung, die dich wirklich Pause machen lässt",
    version: "Version",
    github: "GitHub",
    feedback: "Problem melden",
    lockedDuringBreak: "Während einer Pause können keine Links geöffnet werden",
  },
  ja: {
    tagline: "本当に休ませてくれる休憩リマインダー",
    version: "バージョン",
    github: "GitHub",
    feedback: "問題を報告",
    lockedDuringBreak: "休憩中はリンクを開けません",
  },
  it: {
    tagline: "Un promemoria per le pause che ti fa davvero riposare",
    version: "Versione",
    github: "GitHub",
    feedback: "Segnala un problema",
    lockedDuringBreak: "Non puoi aprire link durante una pausa",
  },
  ko: {
    tagline: "정말로 쉬게 만드는 휴식 알림 도구",
    version: "버전",
    github: "GitHub",
    feedback: "문제 신고",
    lockedDuringBreak: "휴식 중에는 링크를 열 수 없습니다",
  },
  id: {
    tagline: "Pengingat istirahat yang benar-benar membuat Anda beristirahat",
    version: "Versi",
    github: "GitHub",
    feedback: "Laporkan masalah",
    lockedDuringBreak: "Tautan tidak bisa dibuka saat istirahat",
  },
};

export function aboutStrings(lang: UiLang): AboutStrings {
  return about[lang] ?? about.en;
}
