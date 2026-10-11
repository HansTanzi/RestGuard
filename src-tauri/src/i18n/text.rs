//! 后端（托盘、窗口标题、错误提示、配置文件注释）用到的文字。
//! `{app}`、`{e}` 等占位符由调用方替换

use super::Tr;

// 托盘菜单（src-tauri/src/tray.rs）

pub const REST_NOW: Tr = Tr {
    zh: "提前休息",
    en: "Rest now",
    hi: "अभी आराम करें",
    es: "Descansar ahora",
    ar: "استرح الآن",
    fr: "Faire une pause maintenant",
    bn: "এখনই বিশ্রাম নিন",
    pt: "Descansar agora",
    de: "Jetzt Pause machen",
    ja: "今すぐ休憩",
    it: "Fai una pausa ora",
    ko: "지금 휴식",
    id: "Istirahat sekarang",
};

pub const SETTINGS_MENU: Tr = Tr {
    zh: "设置…",
    en: "Settings…",
    hi: "सेटिंग्स…",
    es: "Ajustes…",
    ar: "الإعدادات…",
    fr: "Paramètres…",
    bn: "সেটিংস…",
    pt: "Configurações…",
    de: "Einstellungen…",
    ja: "設定…",
    it: "Impostazioni…",
    ko: "설정…",
    id: "Pengaturan…",
};

pub const AUTOSTART: Tr = Tr {
    zh: "开机自启",
    en: "Start at login",
    hi: "लॉगिन पर शुरू करें",
    es: "Iniciar con el sistema",
    ar: "التشغيل عند تسجيل الدخول",
    fr: "Lancer au démarrage",
    bn: "লগইনের সময় চালু করুন",
    pt: "Iniciar com o sistema",
    de: "Beim Anmelden starten",
    ja: "ログイン時に起動",
    it: "Avvia all'accesso",
    ko: "로그인 시 시작",
    id: "Mulai saat masuk",
};

pub const ABOUT_MENU: Tr = Tr {
    zh: "关于 RestGuard…",
    en: "About RestGuard…",
    hi: "RestGuard के बारे में…",
    es: "Acerca de RestGuard…",
    ar: "حول RestGuard…",
    fr: "À propos de RestGuard…",
    bn: "RestGuard সম্পর্কে…",
    pt: "Sobre o RestGuard…",
    de: "Über RestGuard…",
    ja: "RestGuard について…",
    it: "Informazioni su RestGuard…",
    ko: "RestGuard 정보…",
    id: "Tentang RestGuard…",
};

pub const QUIT: Tr = Tr {
    zh: "退出",
    en: "Quit",
    hi: "बंद करें",
    es: "Salir",
    ar: "خروج",
    fr: "Quitter",
    bn: "প্রস্থান",
    pt: "Sair",
    de: "Beenden",
    ja: "終了",
    it: "Esci",
    ko: "종료",
    id: "Keluar",
};

pub const FOLLOW_SYSTEM: Tr = Tr {
    zh: "跟随系统",
    en: "Follow system",
    hi: "सिस्टम के अनुसार",
    es: "Según el sistema",
    ar: "حسب النظام",
    fr: "Suivre le système",
    bn: "সিস্টেম অনুসারে",
    pt: "Seguir o sistema",
    de: "Systemsprache",
    ja: "システムに従う",
    it: "Segui il sistema",
    ko: "시스템 설정 따르기",
    id: "Ikuti sistem",
};

/// 托盘状态行：前台有会议等软件时暂缓休息，`{app}` 为软件名
pub const BREAK_HELD: Tr = Tr {
    zh: "{app} 使用中，休息暂缓",
    en: "Break on hold: {app} in use",
    hi: "{app} उपयोग में है, ब्रेक रुका हुआ है",
    es: "Descanso en pausa: {app} en uso",
    ar: "الاستراحة معلّقة: {app} قيد الاستخدام",
    fr: "Pause suspendue : {app} en cours d'utilisation",
    bn: "{app} ব্যবহৃত হচ্ছে, বিরতি স্থগিত",
    pt: "Pausa adiada: {app} em uso",
    de: "Pause aufgeschoben: {app} wird verwendet",
    ja: "{app} を使用中のため休憩を保留中",
    it: "Pausa sospesa: {app} in uso",
    ko: "{app} 사용 중, 휴식 보류",
    id: "Istirahat ditunda: {app} sedang digunakan",
};

/// 托盘状态行，后接倒计时
pub const BREAK_IN: Tr = Tr {
    zh: "距离休息",
    en: "Break in",
    hi: "अगला ब्रेक",
    es: "Descanso en",
    ar: "الاستراحة بعد",
    fr: "Pause dans",
    bn: "পরবর্তী বিরতি",
    pt: "Pausa em",
    de: "Pause in",
    ja: "休憩まで",
    it: "Pausa tra",
    ko: "휴식까지",
    id: "Istirahat dalam",
};

/// 托盘状态行，后接倒计时
pub const RESTING: Tr = Tr {
    zh: "休息中",
    en: "Resting",
    hi: "ब्रेक जारी",
    es: "Descansando",
    ar: "في استراحة",
    fr: "En pause",
    bn: "বিরতি চলছে",
    pt: "Descansando",
    de: "Pause läuft",
    ja: "休憩中",
    it: "In pausa",
    ko: "휴식 중",
    id: "Sedang istirahat",
};

pub const BREAK_OVER: Tr = Tr {
    zh: "休息结束",
    en: "Break over",
    hi: "ब्रेक समाप्त",
    es: "Descanso terminado",
    ar: "انتهت الاستراحة",
    fr: "Pause terminée",
    bn: "বিরতি শেষ",
    pt: "Pausa encerrada",
    de: "Pause vorbei",
    ja: "休憩終了",
    it: "Pausa terminata",
    ko: "휴식 종료",
    id: "Istirahat selesai",
};

// 窗口标题

pub const SETTINGS_TITLE: Tr = Tr {
    zh: "RestGuard 设置",
    en: "RestGuard Settings",
    hi: "RestGuard सेटिंग्स",
    es: "Ajustes de RestGuard",
    ar: "إعدادات RestGuard",
    fr: "Paramètres de RestGuard",
    bn: "RestGuard সেটিংস",
    pt: "Configurações do RestGuard",
    de: "RestGuard-Einstellungen",
    ja: "RestGuard 設定",
    it: "Impostazioni di RestGuard",
    ko: "RestGuard 설정",
    id: "Pengaturan RestGuard",
};

pub const ABOUT_TITLE: Tr = Tr {
    zh: "关于 RestGuard",
    en: "About RestGuard",
    hi: "RestGuard के बारे में",
    es: "Acerca de RestGuard",
    ar: "حول RestGuard",
    fr: "À propos de RestGuard",
    bn: "RestGuard সম্পর্কে",
    pt: "Sobre o RestGuard",
    de: "Über RestGuard",
    ja: "RestGuard について",
    it: "Informazioni su RestGuard",
    ko: "RestGuard 정보",
    id: "Tentang RestGuard",
};

// 返回给前端的错误提示（src-tauri/src/lib.rs）

pub const SETTINGS_LOCKED: Tr = Tr {
    zh: "休息期间不能修改设置",
    en: "Settings can't be changed during a break",
    hi: "ब्रेक के दौरान सेटिंग्स नहीं बदली जा सकतीं",
    es: "No se pueden cambiar los ajustes durante un descanso",
    ar: "لا يمكن تغيير الإعدادات أثناء الاستراحة",
    fr: "Impossible de modifier les paramètres pendant une pause",
    bn: "বিরতির সময় সেটিংস পরিবর্তন করা যাবে না",
    pt: "Não é possível alterar as configurações durante uma pausa",
    de: "Während einer Pause können die Einstellungen nicht geändert werden",
    ja: "休憩中は設定を変更できません",
    it: "Non puoi modificare le impostazioni durante una pausa",
    ko: "휴식 중에는 설정을 변경할 수 없습니다",
    id: "Pengaturan tidak bisa diubah saat istirahat",
};

/// `{e}` 为具体的错误信息
pub const SAVE_FAILED: Tr = Tr {
    zh: "无法保存配置文件：{e}",
    en: "Couldn't save the config file: {e}",
    hi: "कॉन्फ़िग फ़ाइल सहेजी नहीं जा सकी: {e}",
    es: "No se pudo guardar el archivo de configuración: {e}",
    ar: "تعذّر حفظ ملف الإعدادات: {e}",
    fr: "Impossible d'enregistrer le fichier de configuration : {e}",
    bn: "কনফিগ ফাইল সংরক্ষণ করা যায়নি: {e}",
    pt: "Não foi possível salvar o arquivo de configuração: {e}",
    de: "Die Konfigurationsdatei konnte nicht gespeichert werden: {e}",
    ja: "設定ファイルを保存できませんでした：{e}",
    it: "Impossibile salvare il file di configurazione: {e}",
    ko: "설정 파일을 저장할 수 없습니다: {e}",
    id: "Tidak dapat menyimpan file konfigurasi: {e}",
};

pub const LINKS_LOCKED: Tr = Tr {
    zh: "休息期间不能打开链接",
    en: "Links can't be opened during a break",
    hi: "ब्रेक के दौरान लिंक नहीं खोले जा सकते",
    es: "No se pueden abrir enlaces durante un descanso",
    ar: "لا يمكن فتح الروابط أثناء الاستراحة",
    fr: "Impossible d'ouvrir des liens pendant une pause",
    bn: "বিরতির সময় লিংক খোলা যাবে না",
    pt: "Não é possível abrir links durante uma pausa",
    de: "Während einer Pause können keine Links geöffnet werden",
    ja: "休憩中はリンクを開けません",
    it: "Non puoi aprire link durante una pausa",
    ko: "휴식 중에는 링크를 열 수 없습니다",
    id: "Tautan tidak bisa dibuka saat istirahat",
};

/// 配置文件里的注释（src-tauri/src/config.rs）。每条都是单行，不能含换行
pub mod config {
    use super::Tr;

    pub const HEADER: Tr = Tr {
        zh: "RestGuard 配置文件。可以在托盘菜单的“设置”里修改；手动编辑后需重启生效",
        en: "RestGuard config. Edit it from \"Settings\" in the tray menu, or by hand and then restart the app.",
        hi: "RestGuard कॉन्फ़िग फ़ाइल। ट्रे मेनू की \"सेटिंग्स\" से बदलें, या हाथ से संपादित करके ऐप दोबारा शुरू करें।",
        es: "Configuración de RestGuard. Modifícala desde \"Ajustes\" en el menú de la bandeja, o a mano y luego reinicia la app.",
        ar: "ملف إعدادات RestGuard. عدّله من \"الإعدادات\" في قائمة شريط النظام، أو يدويًا ثم أعد تشغيل التطبيق.",
        fr: "Configuration de RestGuard. Modifiez-la via « Paramètres » dans le menu de la barre d'état, ou à la main puis redémarrez l'application.",
        bn: "RestGuard কনফিগ ফাইল। ট্রে মেনুর \"সেটিংস\" থেকে বদলান, অথবা হাতে সম্পাদনা করে অ্যাপটি আবার চালু করুন।",
        pt: "Configuração do RestGuard. Altere em \"Configurações\" no menu da bandeja, ou edite à mão e reinicie o app.",
        de: "RestGuard-Konfiguration. Über „Einstellungen“ im Tray-Menü ändern oder von Hand bearbeiten und die App neu starten.",
        ja: "RestGuard の設定ファイル。トレイメニューの「設定」から変更できます。手動で編集した場合は再起動後に反映されます",
        it: "Configurazione di RestGuard. Modificala da \"Impostazioni\" nel menu della barra delle applicazioni, oppure a mano e poi riavvia l'app.",
        ko: "RestGuard 설정 파일입니다. 트레이 메뉴의 \"설정\"에서 바꿀 수 있으며, 직접 수정한 경우 앱을 다시 시작해야 적용됩니다.",
        id: "Konfigurasi RestGuard. Ubah lewat \"Pengaturan\" di menu baki, atau edit manual lalu mulai ulang aplikasi.",
    };

    pub const UNITS: Tr = Tr {
        zh: "时间单位均为分钟，可以写小数（例如 0.5 表示 30 秒）",
        en: "All durations are in minutes and may be fractional (e.g. 0.5 = 30 seconds).",
        hi: "सभी समय मिनट में हैं और दशमलव में हो सकते हैं (जैसे 0.5 = 30 सेकंड)।",
        es: "Todas las duraciones son en minutos y admiten decimales (p. ej. 0.5 = 30 segundos).",
        ar: "جميع المدد بالدقائق ويمكن أن تكون كسرية (مثلًا 0.5 = 30 ثانية).",
        fr: "Toutes les durées sont en minutes et peuvent être décimales (ex. 0.5 = 30 secondes).",
        bn: "সব সময় মিনিটে, দশমিকও লেখা যায় (যেমন 0.5 = 30 সেকেন্ড)।",
        pt: "Todas as durações são em minutos e aceitam decimais (ex.: 0.5 = 30 segundos).",
        de: "Alle Zeiten in Minuten, Dezimalwerte erlaubt (z. B. 0.5 = 30 Sekunden).",
        ja: "時間の単位はすべて分で、小数も使えます（例：0.5 は 30 秒）",
        it: "Tutte le durate sono in minuti e possono essere decimali (es. 0.5 = 30 secondi).",
        ko: "모든 시간은 분 단위이며 소수도 쓸 수 있습니다(예: 0.5 = 30초).",
        id: "Semua durasi dalam menit dan boleh desimal (mis. 0.5 = 30 detik).",
    };

    pub const WORK: Tr = Tr {
        zh: "连续工作多久后强制休息",
        en: "How long to work before a forced break",
        hi: "अनिवार्य ब्रेक से पहले कितनी देर काम करें",
        es: "Tiempo de trabajo antes de un descanso obligatorio",
        ar: "مدة العمل قبل استراحة إلزامية",
        fr: "Durée de travail avant une pause obligatoire",
        bn: "বাধ্যতামূলক বিরতির আগে কতক্ষণ কাজ করবেন",
        pt: "Tempo de trabalho antes de uma pausa obrigatória",
        de: "Arbeitszeit bis zur erzwungenen Pause",
        ja: "強制休憩までの作業時間",
        it: "Quanto lavorare prima di una pausa obbligatoria",
        ko: "강제 휴식 전까지 일하는 시간",
        id: "Lama bekerja sebelum istirahat wajib",
    };

    pub const REST: Tr = Tr {
        zh: "每次休息多久",
        en: "How long each break lasts",
        hi: "हर ब्रेक कितनी देर का हो",
        es: "Duración de cada descanso",
        ar: "مدة كل استراحة",
        fr: "Durée de chaque pause",
        bn: "প্রতিটি বিরতি কতক্ষণ",
        pt: "Duração de cada pausa",
        de: "Dauer jeder Pause",
        ja: "1 回の休憩の長さ",
        it: "Durata di ogni pausa",
        ko: "휴식 1회의 길이",
        id: "Lama setiap istirahat",
    };

    pub const POSTPONE: Tr = Tr {
        zh: "每次推迟多久",
        en: "How long each postpone lasts",
        hi: "हर बार कितनी देर टालें",
        es: "Duración de cada aplazamiento",
        ar: "مدة كل تأجيل",
        fr: "Durée de chaque report",
        bn: "প্রতিবার কতক্ষণ পিছিয়ে দেওয়া হবে",
        pt: "Duração de cada adiamento",
        de: "Dauer jedes Aufschubs",
        ja: "1 回の延期の長さ",
        it: "Durata di ogni rinvio",
        ko: "미루기 1회의 길이",
        id: "Lama setiap penundaan",
    };

    pub const IDLE: Tr = Tr {
        zh: "无操作超过多久算离开电脑，离开期间不计工作时间；离开满一次休息时长就当已休息。0 表示关闭",
        en: "Inactivity after which you count as away; away time isn't work, and being away for a full break counts as one. 0 = off",
        hi: "कितनी देर निष्क्रिय रहने पर आपको दूर माना जाए; दूर का समय काम नहीं गिना जाता, और पूरे ब्रेक जितनी देर दूर रहना ब्रेक माना जाता है। 0 = बंद",
        es: "Inactividad tras la que cuentas como ausente; ese tiempo no es trabajo, y ausentarte lo que dura un descanso cuenta como uno. 0 = desactivado",
        ar: "مدة الخمول التي تُعدّ بعدها بعيدًا؛ وقت الغياب لا يُحسب عملًا، والغياب بطول استراحة كاملة يُحسب استراحة. 0 = إيقاف",
        fr: "Inactivité au-delà de laquelle vous êtes considéré absent ; ce temps ne compte pas comme travail, et une absence aussi longue qu'une pause compte comme une pause. 0 = désactivé",
        bn: "কতক্ষণ নিষ্ক্রিয় থাকলে আপনাকে দূরে ধরা হবে; দূরে থাকার সময় কাজ নয়, আর পুরো বিরতির সমান সময় দূরে থাকলে বিরতি ধরা হয়। 0 = বন্ধ",
        pt: "Inatividade após a qual você conta como ausente; esse tempo não é trabalho, e ficar ausente por uma pausa inteira conta como pausa. 0 = desativado",
        de: "Inaktivität, ab der Sie als abwesend gelten; Abwesenheit zählt nicht als Arbeit, und so lange weg wie eine Pause zählt als Pause. 0 = aus",
        ja: "操作がないまま何分たつと離席とみなすか。離席中は作業時間に数えず、休憩時間分離席したら休憩済みとみなします。0 で無効",
        it: "Inattività dopo cui risulti assente; quel tempo non conta come lavoro, e un'assenza lunga quanto una pausa conta come pausa. 0 = disattivato",
        ko: "입력이 없을 때 자리 비움으로 보는 시간입니다. 자리 비운 시간은 작업으로 치지 않고, 휴식 시간만큼 비우면 휴식한 것으로 봅니다. 0 = 끄기",
        id: "Lama tanpa aktivitas hingga Anda dianggap pergi; waktu pergi tidak dihitung kerja, dan pergi selama satu istirahat penuh dihitung istirahat. 0 = mati",
    };

    pub const LEVEL: Tr = Tr {
        zh: "强制等级：high（高）、medium（中）、low（低），或下面某个自定义方案的名字",
        en: "Enforcement level: high, medium, low, or the name of a custom profile below",
        hi: "सख़्ती का स्तर: high (उच्च), medium (मध्यम), low (निम्न), या नीचे दी गई किसी कस्टम योजना का नाम",
        es: "Nivel de rigor: high (alto), medium (medio), low (bajo) o el nombre de un perfil personalizado de abajo",
        ar: "مستوى الإلزام: high (مرتفع) أو medium (متوسط) أو low (منخفض)، أو اسم أحد الأنماط المخصّصة أدناه",
        fr: "Niveau de contrainte : high (élevé), medium (moyen), low (faible), ou le nom d'un profil personnalisé ci-dessous",
        bn: "কড়াকড়ির মাত্রা: high (উচ্চ), medium (মাঝারি), low (নিম্ন), অথবা নিচের কোনো কাস্টম প্রোফাইলের নাম",
        pt: "Nível de rigor: high (alto), medium (médio), low (baixo) ou o nome de um perfil personalizado abaixo",
        de: "Strenge: high (hoch), medium (mittel), low (niedrig) oder der Name eines eigenen Profils unten",
        ja: "強制レベル：high（高）、medium（中）、low（低）、または下にあるカスタムプランの名前",
        it: "Livello di rigore: high (alto), medium (medio), low (basso) o il nome di un profilo personalizzato qui sotto",
        ko: "강제 수준: high(높음), medium(보통), low(낮음) 또는 아래 사용자 지정 프로필의 이름",
        id: "Tingkat ketegasan: high (tinggi), medium (sedang), low (rendah), atau nama profil kustom di bawah",
    };

    pub const PROFILES: Tr = Tr {
        zh: "自定义方案，每个 [[profiles]] 一个。max_postpones 完整休息前最多推迟几次；urgent_mode 允许缩小遮罩；meeting_detection 检测会议软件；force_rest 为 false 时只弹出可关闭的窗口；overlay_coverage 遮罩覆盖屏幕的比例 0.1 ~ 1.0；urgent_scale 紧急模式时相对 overlay_coverage 的缩放因子 0.1 ~ 0.9",
        en: "Custom profiles, one [[profiles]] each. max_postpones: postpones before a full break; urgent_mode: allow shrinking the overlay; meeting_detection: detect meeting apps; force_rest = false only shows a closable window; overlay_coverage: fraction of each screen covered, 0.1 ~ 1.0; urgent_scale: urgent mode scale relative to overlay_coverage, 0.1 ~ 0.9",
        hi: "कस्टम योजनाएँ, हर एक के लिए एक [[profiles]]। max_postpones: पूरे ब्रेक से पहले कितनी बार टालें; urgent_mode: ओवरले छोटा करने दें; meeting_detection: मीटिंग ऐप पहचानें; force_rest = false होने पर केवल बंद की जा सकने वाली विंडो दिखती है; overlay_coverage: ओवरले हर स्क्रीन का कितना हिस्सा ढके, 0.1 ~ 1.0; urgent_scale: आपात मोड में overlay_coverage के सापेक्ष स्केल, 0.1 ~ 0.9",
        es: "Perfiles personalizados, un [[profiles]] cada uno. max_postpones: aplazamientos antes de un descanso completo; urgent_mode: permitir reducir la superposición; meeting_detection: detectar apps de reuniones; force_rest = false solo muestra una ventana que se puede cerrar; overlay_coverage: fracción de cada pantalla cubierta, 0.1 ~ 1.0; urgent_scale: escala del modo urgente respecto a overlay_coverage, 0.1 ~ 0.9",
        ar: "الأنماط المخصّصة، لكلٍّ منها [[profiles]]. max_postpones: عدد التأجيلات قبل استراحة كاملة؛ urgent_mode: السماح بتصغير الطبقة؛ meeting_detection: اكتشاف تطبيقات الاجتماعات؛ force_rest = false يعرض نافذة قابلة للإغلاق فقط؛ overlay_coverage: نسبة ما تغطيه الطبقة من كل شاشة، 0.1 ~ 1.0؛ urgent_scale: معامل التصغير في وضع الطوارئ نسبةً إلى overlay_coverage، 0.1 ~ 0.9",
        fr: "Profils personnalisés, un [[profiles]] chacun. max_postpones : reports avant une pause complète ; urgent_mode : autoriser la réduction du voile ; meeting_detection : détecter les applis de réunion ; force_rest = false affiche seulement une fenêtre qu'on peut fermer ; overlay_coverage : part de chaque écran couverte, 0.1 ~ 1.0 ; urgent_scale : échelle du mode urgence par rapport à overlay_coverage, 0.1 ~ 0.9",
        bn: "কাস্টম প্রোফাইল, প্রতিটির জন্য একটি [[profiles]]। max_postpones: পূর্ণ বিরতির আগে কতবার পেছানো যাবে; urgent_mode: ওভারলে ছোট করার অনুমতি; meeting_detection: মিটিং অ্যাপ শনাক্ত করা; force_rest = false হলে শুধু বন্ধ করা যায় এমন উইন্ডো দেখায়; overlay_coverage: ওভারলে প্রতিটি স্ক্রিনের কত অংশ ঢাকবে, 0.1 ~ 1.0; urgent_scale: জরুরি মোডে overlay_coverage-এর তুলনায় স্কেল, 0.1 ~ 0.9",
        pt: "Perfis personalizados, um [[profiles]] cada. max_postpones: adiamentos antes de uma pausa completa; urgent_mode: permitir reduzir a sobreposição; meeting_detection: detectar apps de reunião; force_rest = false só mostra uma janela que pode ser fechada; overlay_coverage: fração de cada tela coberta, 0.1 ~ 1.0; urgent_scale: escala do modo urgente em relação a overlay_coverage, 0.1 ~ 0.9",
        de: "Eigene Profile, je ein [[profiles]]. max_postpones: Aufschübe vor einer vollständigen Pause; urgent_mode: Verkleinern des Overlays erlauben; meeting_detection: Meeting-Apps erkennen; force_rest = false zeigt nur ein schließbares Fenster; overlay_coverage: Anteil jedes Bildschirms, der abgedeckt wird, 0.1 ~ 1.0; urgent_scale: Skalierung im Notfallmodus relativ zu overlay_coverage, 0.1 ~ 0.9",
        ja: "カスタムプラン。1 つにつき [[profiles]] を 1 つ。max_postpones は完全に休憩するまでの最大延期回数、urgent_mode はオーバーレイの縮小を許可、meeting_detection は会議アプリを検出、force_rest = false なら閉じられるウィンドウを表示するだけ、overlay_coverage は各画面を覆う割合 0.1 ~ 1.0、urgent_scale は緊急モード時の overlay_coverage に対する縮小率 0.1 ~ 0.9",
        it: "Profili personalizzati, un [[profiles]] ciascuno. max_postpones: rinvii prima di una pausa completa; urgent_mode: consenti di ridurre la sovrapposizione; meeting_detection: rileva le app per riunioni; force_rest = false mostra solo una finestra chiudibile; overlay_coverage: frazione di ogni schermo coperta, 0.1 ~ 1.0; urgent_scale: scala in modalità urgente rispetto a overlay_coverage, 0.1 ~ 0.9",
        ko: "사용자 지정 프로필, 하나당 [[profiles]] 하나. max_postpones: 완전한 휴식 전 최대 미루기 횟수, urgent_mode: 오버레이 축소 허용, meeting_detection: 회의 앱 감지, force_rest = false면 닫을 수 있는 창만 표시, overlay_coverage: 각 화면을 덮는 비율 0.1 ~ 1.0, urgent_scale: 긴급 모드 때 overlay_coverage에 곱하는 축소 비율 0.1 ~ 0.9",
        id: "Profil kustom, satu [[profiles]] untuk masing-masing. max_postpones: penundaan sebelum istirahat penuh; urgent_mode: izinkan memperkecil lapisan; meeting_detection: deteksi aplikasi rapat; force_rest = false hanya menampilkan jendela yang bisa ditutup; overlay_coverage: bagian setiap layar yang ditutupi, 0.1 ~ 1.0; urgent_scale: skala mode darurat relatif terhadap overlay_coverage, 0.1 ~ 0.9",
    };

    /// 用于测试：所有注释
    #[cfg(test)]
    pub const ALL: [&Tr; 8] = [
        &HEADER,
        &UNITS,
        &WORK,
        &REST,
        &POSTPONE,
        &IDLE,
        &LEVEL,
        &PROFILES,
    ];
}
