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

    pub const MAX_POSTPONES: Tr = Tr {
        zh: "完整休息一次之前，最多推迟几次",
        en: "Max postpones before a full break is required",
        hi: "पूरा ब्रेक लेने से पहले अधिकतम कितनी बार टाल सकते हैं",
        es: "Máximo de aplazamientos antes de exigir un descanso completo",
        ar: "أقصى عدد من التأجيلات قبل فرض استراحة كاملة",
        fr: "Nombre maximal de reports avant une pause complète obligatoire",
        bn: "পূর্ণ বিরতির আগে সর্বোচ্চ কতবার পেছানো যাবে",
        pt: "Máximo de adiamentos antes de exigir uma pausa completa",
        de: "Maximale Anzahl an Aufschüben vor einer vollständigen Pause",
        ja: "完全に休憩するまでに延期できる最大回数",
        it: "Numero massimo di rinvii prima di una pausa completa",
        ko: "완전한 휴식 전까지 미룰 수 있는 최대 횟수",
        id: "Jumlah penundaan maksimum sebelum istirahat penuh diwajibkan",
    };

    pub const COVERAGE: Tr = Tr {
        zh: "遮罩覆盖每块屏幕的比例，0.1 ~ 1.0",
        en: "Fraction of each screen covered by the overlay, 0.1 ~ 1.0",
        hi: "ओवरले हर स्क्रीन का कितना हिस्सा ढके, 0.1 ~ 1.0",
        es: "Fracción de cada pantalla cubierta por la superposición, 0.1 ~ 1.0",
        ar: "نسبة ما تغطيه الطبقة من كل شاشة، 0.1 ~ 1.0",
        fr: "Part de chaque écran couverte par le voile, 0.1 ~ 1.0",
        bn: "ওভারলে প্রতিটি স্ক্রিনের কত অংশ ঢাকবে, 0.1 ~ 1.0",
        pt: "Fração de cada tela coberta pela sobreposição, 0.1 ~ 1.0",
        de: "Anteil jedes Bildschirms, den das Overlay abdeckt, 0.1 ~ 1.0",
        ja: "各画面をオーバーレイが覆う割合、0.1 ~ 1.0",
        it: "Frazione di ogni schermo coperta dalla sovrapposizione, 0.1 ~ 1.0",
        ko: "오버레이가 각 화면을 덮는 비율, 0.1 ~ 1.0",
        id: "Bagian setiap layar yang ditutupi lapisan, 0.1 ~ 1.0",
    };

    /// 用于测试：所有注释
    #[cfg(test)]
    pub const ALL: [&Tr; 7] = [
        &HEADER,
        &UNITS,
        &WORK,
        &REST,
        &POSTPONE,
        &MAX_POSTPONES,
        &COVERAGE,
    ];
}
