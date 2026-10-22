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

pub const LEVEL_MENU: Tr = Tr {
    zh: "强制方案",
    en: "Enforcement profile",
    hi: "सख़्ती योजना",
    es: "Perfil de rigor",
    ar: "نمط الإلزام",
    fr: "Profil de contrainte",
    bn: "কড়াকড়ির প্রোফাইল",
    pt: "Perfil de rigor",
    de: "Strenge-Profil",
    ja: "強制プラン",
    it: "Profilo di rigore",
    ko: "강제 프로필",
    id: "Profil ketegasan",
};

/// 内置强制等级的名字，与 src/lib/i18n.ts 的 levels 一致
pub const LEVEL_HIGH: Tr = Tr {
    zh: "高等强制",
    en: "High enforcement",
    hi: "उच्च सख़्ती",
    es: "Rigor alto",
    ar: "إلزام مرتفع",
    fr: "Contrainte élevée",
    bn: "উচ্চ কড়াকড়ি",
    pt: "Rigor alto",
    de: "Hohe Strenge",
    ja: "強制：高",
    it: "Rigore alto",
    ko: "강제 높음",
    id: "Ketegasan tinggi",
};

pub const LEVEL_MEDIUM: Tr = Tr {
    zh: "中等强制",
    en: "Medium enforcement",
    hi: "मध्यम सख़्ती",
    es: "Rigor medio",
    ar: "إلزام متوسط",
    fr: "Contrainte moyenne",
    bn: "মাঝারি কড়াকড়ি",
    pt: "Rigor médio",
    de: "Mittlere Strenge",
    ja: "強制：中",
    it: "Rigore medio",
    ko: "강제 보통",
    id: "Ketegasan sedang",
};

pub const LEVEL_LOW: Tr = Tr {
    zh: "低等强制",
    en: "Low enforcement",
    hi: "निम्न सख़्ती",
    es: "Rigor bajo",
    ar: "إلزام منخفض",
    fr: "Contrainte faible",
    bn: "নিম্ন কড়াকড়ি",
    pt: "Rigor baixo",
    de: "Niedrige Strenge",
    ja: "強制：低",
    it: "Rigore basso",
    ko: "강제 낮음",
    id: "Ketegasan rendah",
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

/// 推送到手机的通知正文，标题用 BREAK_OVER
pub const PHONE_MESSAGE: Tr = Tr {
    zh: "休息时间到了，回到电脑前开始工作吧",
    en: "Your break is over. Time to get back to work.",
    hi: "आपका ब्रेक खत्म हो गया। काम पर लौटने का समय है।",
    es: "Tu descanso ha terminado. Es hora de volver al trabajo.",
    ar: "انتهت استراحتك. حان وقت العودة إلى العمل.",
    fr: "Votre pause est terminée. Il est temps de reprendre le travail.",
    bn: "আপনার বিরতি শেষ। কাজে ফেরার সময় হয়েছে।",
    pt: "Sua pausa terminou. Hora de voltar ao trabalho.",
    de: "Deine Pause ist vorbei. Zeit, wieder an die Arbeit zu gehen.",
    ja: "休憩が終わりました。仕事に戻りましょう",
    it: "La pausa è finita. È ora di tornare al lavoro.",
    ko: "휴식이 끝났습니다. 다시 업무로 돌아갈 시간입니다.",
    id: "Istirahat Anda sudah selesai. Saatnya kembali bekerja.",
};

/// 设置面板发送测试通知失败。`{e}` 为具体的错误信息
pub const PHONE_FAILED: Tr = Tr {
    zh: "推送失败，请检查服务器地址、主题和网络：{e}",
    en: "Couldn't send the notification. Check the server, topic and network: {e}",
    hi: "सूचना नहीं भेजी जा सकी। सर्वर, टॉपिक और नेटवर्क जाँचें: {e}",
    es: "No se pudo enviar la notificación. Revisa el servidor, el tema y la red: {e}",
    ar: "تعذّر إرسال الإشعار. تحقّق من الخادم والموضوع والشبكة: {e}",
    fr: "Impossible d'envoyer la notification. Vérifiez le serveur, le sujet et le réseau : {e}",
    bn: "নোটিফিকেশন পাঠানো যায়নি। সার্ভার, টপিক ও নেটওয়ার্ক পরীক্ষা করুন: {e}",
    pt: "Não foi possível enviar a notificação. Verifique o servidor, o tópico e a rede: {e}",
    de: "Die Benachrichtigung konnte nicht gesendet werden. Server, Thema und Netzwerk prüfen: {e}",
    ja: "通知を送信できませんでした。サーバー、トピック、ネットワークを確認してください：{e}",
    it: "Impossibile inviare la notifica. Controlla server, argomento e rete: {e}",
    ko: "알림을 보낼 수 없습니다. 서버, 토픽, 네트워크를 확인하세요: {e}",
    id: "Notifikasi gagal dikirim. Periksa server, topik, dan jaringan: {e}",
};

/// 设置面板选择提示音文件时，文件对话框里的类型名称
pub const AUDIO_FILES: Tr = Tr {
    zh: "音频文件",
    en: "Audio files",
    hi: "ऑडियो फ़ाइलें",
    es: "Archivos de audio",
    ar: "ملفات صوتية",
    fr: "Fichiers audio",
    bn: "অডিও ফাইল",
    pt: "Arquivos de áudio",
    de: "Audiodateien",
    ja: "音声ファイル",
    it: "File audio",
    ko: "오디오 파일",
    id: "File audio",
};

/// 试听自选文件播放失败时的提示（多半是格式不受支持）
pub const SOUND_FILE_FAILED: Tr = Tr {
    zh: "无法播放这个文件，可能是本系统不支持它的格式，请换一个文件",
    en: "Couldn't play this file. Its format may not be supported on this system; please choose another file.",
    hi: "यह फ़ाइल नहीं चल सकी। हो सकता है इस सिस्टम पर इसका फ़ॉर्मैट समर्थित न हो; कृपया दूसरी फ़ाइल चुनें।",
    es: "No se pudo reproducir este archivo. Puede que su formato no sea compatible con este sistema; elige otro archivo.",
    ar: "تعذّر تشغيل هذا الملف. قد لا تكون صيغته مدعومة على هذا النظام؛ يُرجى اختيار ملف آخر.",
    fr: "Impossible de lire ce fichier. Son format n'est peut-être pas pris en charge sur ce système ; choisissez-en un autre.",
    bn: "এই ফাইলটি চালানো যায়নি। এই সিস্টেমে এর ফরম্যাট সমর্থিত না-ও হতে পারে; অন্য একটি ফাইল বেছে নিন।",
    pt: "Não foi possível reproduzir este arquivo. O formato pode não ser compatível com este sistema; escolha outro arquivo.",
    de: "Diese Datei konnte nicht abgespielt werden. Ihr Format wird auf diesem System eventuell nicht unterstützt; bitte eine andere Datei wählen.",
    ja: "このファイルを再生できません。このシステムでは形式に対応していない可能性があります。別のファイルを選んでください",
    it: "Impossibile riprodurre questo file. Il formato potrebbe non essere supportato su questo sistema; scegli un altro file.",
    ko: "이 파일을 재생할 수 없습니다. 이 시스템에서 지원하지 않는 형식일 수 있으니 다른 파일을 선택하세요.",
    id: "File ini tidak dapat diputar. Formatnya mungkin tidak didukung di sistem ini; silakan pilih file lain.",
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

/// `{e}` 为具体的错误信息
pub const CONFIG_READ_FAILED: Tr = Tr {
    zh: "无法读取配置文件：{e}",
    en: "Couldn't read the config file: {e}",
    hi: "कॉन्फ़िग फ़ाइल पढ़ी नहीं जा सकी: {e}",
    es: "No se pudo leer el archivo de configuración: {e}",
    ar: "تعذّرت قراءة ملف الإعدادات: {e}",
    fr: "Impossible de lire le fichier de configuration : {e}",
    bn: "কনফিগ ফাইল পড়া যায়নি: {e}",
    pt: "Não foi possível ler o arquivo de configuração: {e}",
    de: "Die Konfigurationsdatei konnte nicht gelesen werden: {e}",
    ja: "設定ファイルを読み込めませんでした：{e}",
    it: "Impossibile leggere il file di configurazione: {e}",
    ko: "설정 파일을 읽을 수 없습니다: {e}",
    id: "Tidak dapat membaca file konfigurasi: {e}",
};

/// 手动编辑的配置文件有语法错误。`{line}` 为行号，`{e}` 为具体的错误信息
pub const CONFIG_INVALID: Tr = Tr {
    zh: "配置文件第 {line} 行有误，修改未生效：{e}",
    en: "Config file error on line {line}, changes not applied: {e}",
    hi: "कॉन्फ़िग फ़ाइल की पंक्ति {line} में त्रुटि, बदलाव लागू नहीं हुए: {e}",
    es: "Error en la línea {line} del archivo de configuración, cambios no aplicados: {e}",
    ar: "خطأ في السطر {line} من ملف الإعدادات، لم تُطبَّق التغييرات: {e}",
    fr: "Erreur ligne {line} du fichier de configuration, modifications non appliquées : {e}",
    bn: "কনফিগ ফাইলের {line} নম্বর লাইনে ত্রুটি, পরিবর্তন প্রয়োগ হয়নি: {e}",
    pt: "Erro na linha {line} do arquivo de configuração, alterações não aplicadas: {e}",
    de: "Fehler in Zeile {line} der Konfigurationsdatei, Änderungen nicht übernommen: {e}",
    ja: "設定ファイルの {line} 行目に誤りがあるため、変更は反映されていません：{e}",
    it: "Errore alla riga {line} del file di configurazione, modifiche non applicate: {e}",
    ko: "설정 파일 {line}번째 줄에 오류가 있어 변경 사항이 적용되지 않았습니다: {e}",
    id: "Kesalahan di baris {line} file konfigurasi, perubahan tidak diterapkan: {e}",
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
        zh: "RestGuard 配置文件。可以在托盘菜单的“设置”里修改；手动编辑保存后，打开或回到设置窗口即可生效",
        en: "RestGuard config. Edit it from \"Settings\" in the tray menu, or by hand: after saving, open or switch back to the Settings window to apply it.",
        hi: "RestGuard कॉन्फ़िग फ़ाइल। ट्रे मेनू की \"सेटिंग्स\" से बदलें, या हाथ से संपादित करें: सहेजने के बाद लागू करने के लिए सेटिंग्स विंडो खोलें या उस पर लौटें।",
        es: "Configuración de RestGuard. Modifícala desde \"Ajustes\" en el menú de la bandeja, o a mano: tras guardar, abre o vuelve a la ventana de Ajustes para aplicarla.",
        ar: "ملف إعدادات RestGuard. عدّله من \"الإعدادات\" في قائمة شريط النظام، أو يدويًا: بعد الحفظ، افتح نافذة الإعدادات أو عُد إليها لتطبيقه.",
        fr: "Configuration de RestGuard. Modifiez-la via « Paramètres » dans le menu de la barre d'état, ou à la main : après l'enregistrement, ouvrez la fenêtre Paramètres ou revenez-y pour l'appliquer.",
        bn: "RestGuard কনফিগ ফাইল। ট্রে মেনুর \"সেটিংস\" থেকে বদলান, অথবা হাতে সম্পাদনা করুন: সংরক্ষণের পর প্রয়োগ করতে সেটিংস উইন্ডো খুলুন বা সেখানে ফিরে যান।",
        pt: "Configuração do RestGuard. Altere em \"Configurações\" no menu da bandeja, ou edite à mão: depois de salvar, abra ou volte à janela de Configurações para aplicá-la.",
        de: "RestGuard-Konfiguration. Über „Einstellungen“ im Tray-Menü ändern oder von Hand bearbeiten: Nach dem Speichern das Einstellungsfenster öffnen oder dorthin zurückkehren, um sie zu übernehmen.",
        ja: "RestGuard の設定ファイル。トレイメニューの「設定」から変更できます。手動で編集して保存した場合は、設定ウィンドウを開くか戻ると反映されます",
        it: "Configurazione di RestGuard. Modificala da \"Impostazioni\" nel menu della barra delle applicazioni, oppure a mano: dopo il salvataggio, apri o torna alla finestra Impostazioni per applicarla.",
        ko: "RestGuard 설정 파일입니다. 트레이 메뉴의 \"설정\"에서 바꿀 수 있으며, 직접 수정해 저장한 경우 설정 창을 열거나 설정 창으로 돌아오면 적용됩니다.",
        id: "Konfigurasi RestGuard. Ubah lewat \"Pengaturan\" di menu baki, atau edit manual: setelah disimpan, buka atau kembali ke jendela Pengaturan untuk menerapkannya.",
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
        zh: "自定义方案，每个 [[profiles]] 一个。max_postpones 完整休息前最多推迟几次；urgent_mode 允许缩小遮罩；meeting_detection 检测会议软件；force_rest 为 false 时只弹出可关闭的窗口；allow_skip 强制休息时仍提供“跳过这次休息”；overlay_coverage 遮罩覆盖屏幕的比例 0.1 ~ 1.0；urgent_scale 紧急模式时相对 overlay_coverage 的缩放因子 0.1 ~ 0.9；movable 遮罩未占满屏幕时能否在本屏幕内拖动",
        en: "Custom profiles, one [[profiles]] each. max_postpones: postpones before a full break; urgent_mode: allow shrinking the overlay; meeting_detection: detect meeting apps; force_rest = false only shows a closable window; allow_skip: still offer \"Skip this break\" when breaks are forced;overlay_coverage: fraction of each screen covered, 0.1 ~ 1.0; urgent_scale: urgent mode scale relative to overlay_coverage, 0.1 ~ 0.9; movable: allow dragging the overlay within its screen when not full screen",
        hi: "कस्टम योजनाएँ, हर एक के लिए एक [[profiles]]। max_postpones: पूरे ब्रेक से पहले कितनी बार टालें; urgent_mode: ओवरले छोटा करने दें; meeting_detection: मीटिंग ऐप पहचानें; force_rest = false होने पर केवल बंद की जा सकने वाली विंडो दिखती है; allow_skip: ब्रेक अनिवार्य होने पर भी \"यह ब्रेक छोड़ें\" दिखाएँ;overlay_coverage: ओवरले हर स्क्रीन का कितना हिस्सा ढके, 0.1 ~ 1.0; urgent_scale: आपात मोड में overlay_coverage के सापेक्ष स्केल, 0.1 ~ 0.9; movable: पूरी स्क्रीन न होने पर ओवरले को उसकी स्क्रीन के भीतर खींचने दें",
        es: "Perfiles personalizados, un [[profiles]] cada uno. max_postpones: aplazamientos antes de un descanso completo; urgent_mode: permitir reducir la superposición; meeting_detection: detectar apps de reuniones; force_rest = false solo muestra una ventana que se puede cerrar; allow_skip: ofrecer \"Saltar este descanso\" aunque los descansos sean forzados;overlay_coverage: fracción de cada pantalla cubierta, 0.1 ~ 1.0; urgent_scale: escala del modo urgente respecto a overlay_coverage, 0.1 ~ 0.9; movable: permitir arrastrar la superposición dentro de su pantalla si no la ocupa entera",
        ar: "الأنماط المخصّصة، لكلٍّ منها [[profiles]]. max_postpones: عدد التأجيلات قبل استراحة كاملة؛ urgent_mode: السماح بتصغير الطبقة؛ meeting_detection: اكتشاف تطبيقات الاجتماعات؛ force_rest = false يعرض نافذة قابلة للإغلاق فقط؛ allow_skip: إظهار \"تخطَّ هذه الاستراحة\" حتى عند فرض الاستراحة؛overlay_coverage: نسبة ما تغطيه الطبقة من كل شاشة، 0.1 ~ 1.0؛ urgent_scale: معامل التصغير في وضع الطوارئ نسبةً إلى overlay_coverage، 0.1 ~ 0.9؛ movable: السماح بسحب الطبقة داخل شاشتها عندما لا تملأ الشاشة",
        fr: "Profils personnalisés, un [[profiles]] chacun. max_postpones : reports avant une pause complète ; urgent_mode : autoriser la réduction du voile ; meeting_detection : détecter les applis de réunion ; force_rest = false affiche seulement une fenêtre qu'on peut fermer ; allow_skip : proposer « Passer cette pause » même quand les pauses sont imposées ;overlay_coverage : part de chaque écran couverte, 0.1 ~ 1.0 ; urgent_scale : échelle du mode urgence par rapport à overlay_coverage, 0.1 ~ 0.9 ; movable : autoriser le déplacement du voile sur son écran s'il ne le couvre pas entièrement",
        bn: "কাস্টম প্রোফাইল, প্রতিটির জন্য একটি [[profiles]]। max_postpones: পূর্ণ বিরতির আগে কতবার পেছানো যাবে; urgent_mode: ওভারলে ছোট করার অনুমতি; meeting_detection: মিটিং অ্যাপ শনাক্ত করা; force_rest = false হলে শুধু বন্ধ করা যায় এমন উইন্ডো দেখায়; allow_skip: বিরতি বাধ্যতামূলক হলেও \"এই বিরতি বাদ দিন\" দেখানো;overlay_coverage: ওভারলে প্রতিটি স্ক্রিনের কত অংশ ঢাকবে, 0.1 ~ 1.0; urgent_scale: জরুরি মোডে overlay_coverage-এর তুলনায় স্কেল, 0.1 ~ 0.9; movable: পুরো স্ক্রিন না ঢাকলে ওভারলেটি নিজের স্ক্রিনের মধ্যে টেনে সরানো যাবে",
        pt: "Perfis personalizados, um [[profiles]] cada. max_postpones: adiamentos antes de uma pausa completa; urgent_mode: permitir reduzir a sobreposição; meeting_detection: detectar apps de reunião; force_rest = false só mostra uma janela que pode ser fechada; allow_skip: oferecer \"Pular esta pausa\" mesmo com pausas forçadas;overlay_coverage: fração de cada tela coberta, 0.1 ~ 1.0; urgent_scale: escala do modo urgente em relação a overlay_coverage, 0.1 ~ 0.9; movable: permitir arrastar a sobreposição dentro da tela quando não a ocupa toda",
        de: "Eigene Profile, je ein [[profiles]]. max_postpones: Aufschübe vor einer vollständigen Pause; urgent_mode: Verkleinern des Overlays erlauben; meeting_detection: Meeting-Apps erkennen; force_rest = false zeigt nur ein schließbares Fenster; allow_skip: \"Diese Pause überspringen\" auch bei erzwungenen Pausen anbieten;overlay_coverage: Anteil jedes Bildschirms, der abgedeckt wird, 0.1 ~ 1.0; urgent_scale: Skalierung im Notfallmodus relativ zu overlay_coverage, 0.1 ~ 0.9; movable: Overlay innerhalb seines Bildschirms verschieben erlauben, wenn nicht im Vollbild",
        ja: "カスタムプラン。1 つにつき [[profiles]] を 1 つ。max_postpones は完全に休憩するまでの最大延期回数、urgent_mode はオーバーレイの縮小を許可、meeting_detection は会議アプリを検出、force_rest = false なら閉じられるウィンドウを表示するだけ、allow_skip は休憩を強制していても「今回の休憩をスキップ」を表示、overlay_coverage は各画面を覆う割合 0.1 ~ 1.0、urgent_scale は緊急モード時の overlay_coverage に対する縮小率 0.1 ~ 0.9、movable は全画面でないときに同じ画面内でドラッグ可能にする",
        it: "Profili personalizzati, un [[profiles]] ciascuno. max_postpones: rinvii prima di una pausa completa; urgent_mode: consenti di ridurre la sovrapposizione; meeting_detection: rileva le app per riunioni; force_rest = false mostra solo una finestra chiudibile; allow_skip: offri \"Salta questa pausa\" anche con pause imposte;overlay_coverage: frazione di ogni schermo coperta, 0.1 ~ 1.0; urgent_scale: scala in modalità urgente rispetto a overlay_coverage, 0.1 ~ 0.9; movable: consenti di trascinare la sovrapposizione nel suo schermo se non a schermo intero",
        ko: "사용자 지정 프로필, 하나당 [[profiles]] 하나. max_postpones: 완전한 휴식 전 최대 미루기 횟수, urgent_mode: 오버레이 축소 허용, meeting_detection: 회의 앱 감지, force_rest = false면 닫을 수 있는 창만 표시, allow_skip: 휴식을 강제해도 \"이번 휴식 건너뛰기\" 제공,overlay_coverage: 각 화면을 덮는 비율 0.1 ~ 1.0, urgent_scale: 긴급 모드 때 overlay_coverage에 곱하는 축소 비율 0.1 ~ 0.9, movable: 전체 화면이 아닐 때 오버레이를 해당 화면 안에서 드래그 허용",
        id: "Profil kustom, satu [[profiles]] untuk masing-masing. max_postpones: penundaan sebelum istirahat penuh; urgent_mode: izinkan memperkecil lapisan; meeting_detection: deteksi aplikasi rapat; force_rest = false hanya menampilkan jendela yang bisa ditutup; allow_skip: tetap tampilkan \"Lewati istirahat ini\" saat istirahat dipaksa;overlay_coverage: bagian setiap layar yang ditutupi, 0.1 ~ 1.0; urgent_scale: skala mode darurat relatif terhadap overlay_coverage, 0.1 ~ 0.9; movable: izinkan menyeret lapisan di dalam layarnya saat tidak layar penuh",
    };

    pub const SOUND_PROMPT: Tr = Tr {
        zh: "休息结束时是否播放提示音（true / false）",
        en: "Play a sound when a break ends (true / false)",
        hi: "ब्रेक खत्म होने पर ध्वनि बजाएँ या नहीं (true / false)",
        es: "Reproducir un sonido al terminar el descanso (true / false)",
        ar: "تشغيل صوت عند انتهاء الاستراحة (true / false)",
        fr: "Jouer un son à la fin de la pause (true / false)",
        bn: "বিরতি শেষ হলে শব্দ বাজবে কি না (true / false)",
        pt: "Tocar um som ao fim da pausa (true / false)",
        de: "Am Ende der Pause einen Ton abspielen (true / false)",
        ja: "休憩が終わったときに通知音を鳴らすか（true / false）",
        it: "Riprodurre un suono alla fine della pausa (true / false)",
        ko: "휴식이 끝나면 알림음을 재생할지 여부(true / false)",
        id: "Putar suara saat istirahat selesai (true / false)",
    };

    pub const SOUND: Tr = Tr {
        zh: "提示音：beep（滴滴）、chime（叮咚）、rise（上扬三音）或 file（播放 sound_file）",
        en: "Sound: beep, chime, rise (three rising notes), or file (plays sound_file)",
        hi: "ध्वनि: beep (बीप), chime (घंटी), rise (तीन चढ़ते सुर), या file (sound_file बजाएँ)",
        es: "Sonido: beep (pitido), chime (campanilla), rise (tres notas ascendentes) o file (reproduce sound_file)",
        ar: "الصوت: beep (صفير)، chime (رنين)، rise (ثلاث نغمات صاعدة)، أو file (تشغيل sound_file)",
        fr: "Son : beep (bip), chime (carillon), rise (trois notes montantes) ou file (joue sound_file)",
        bn: "শব্দ: beep (বিপ), chime (ঘণ্টা), rise (তিনটি ঊর্ধ্বমুখী সুর), অথবা file (sound_file বাজায়)",
        pt: "Som: beep (bipe), chime (sininho), rise (três notas ascendentes) ou file (toca sound_file)",
        de: "Ton: beep (Piepton), chime (Gong), rise (drei aufsteigende Töne) oder file (spielt sound_file ab)",
        ja: "通知音：beep（ピピッ）、chime（ピンポン）、rise（上がる 3 音）、file（sound_file を再生）",
        it: "Suono: beep (bip), chime (campanello), rise (tre note ascendenti) o file (riproduce sound_file)",
        ko: "알림음: beep(삐삐), chime(딩동), rise(올라가는 세 음), file(sound_file 재생)",
        id: "Suara: beep (bip), chime (lonceng), rise (tiga nada naik), atau file (memutar sound_file)",
    };

    pub const SOUND_FILE: Tr = Tr {
        zh: "sound 为 \"file\" 时播放的音频文件的完整路径",
        en: "Full path of the audio file played when sound = \"file\"",
        hi: "sound = \"file\" होने पर बजाई जाने वाली ऑडियो फ़ाइल का पूरा पथ",
        es: "Ruta completa del archivo de audio que se reproduce cuando sound = \"file\"",
        ar: "المسار الكامل لملف الصوت الذي يُشغَّل عندما sound = \"file\"",
        fr: "Chemin complet du fichier audio joué quand sound = \"file\"",
        bn: "sound = \"file\" হলে যে অডিও ফাইল বাজবে তার পূর্ণ পথ",
        pt: "Caminho completo do arquivo de áudio tocado quando sound = \"file\"",
        de: "Vollständiger Pfad der Audiodatei, die bei sound = \"file\" abgespielt wird",
        ja: "sound が \"file\" のときに再生する音声ファイルのフルパス",
        it: "Percorso completo del file audio riprodotto quando sound = \"file\"",
        ko: "sound가 \"file\"일 때 재생할 오디오 파일의 전체 경로",
        id: "Path lengkap file audio yang diputar saat sound = \"file\"",
    };

    pub const PHONE_NOTIFY: Tr = Tr {
        zh: "休息结束时是否通过 ntfy 推送到手机（true / false），手机上安装 ntfy 应用并订阅 ntfy_topic",
        en: "Push a notification to your phone via ntfy when a break ends (true / false); install the ntfy app and subscribe to ntfy_topic",
        hi: "ब्रेक खत्म होने पर ntfy से फ़ोन पर सूचना भेजें (true / false); फ़ोन पर ntfy ऐप इंस्टॉल करके ntfy_topic की सदस्यता लें",
        es: "Enviar una notificación al móvil mediante ntfy al terminar el descanso (true / false); instala la app ntfy y suscríbete a ntfy_topic",
        ar: "إرسال إشعار إلى الهاتف عبر ntfy عند انتهاء الاستراحة (true / false)؛ ثبّت تطبيق ntfy واشترك في ntfy_topic",
        fr: "Envoyer une notification au téléphone via ntfy à la fin de la pause (true / false) ; installez l'appli ntfy et abonnez-vous à ntfy_topic",
        bn: "বিরতি শেষ হলে ntfy দিয়ে ফোনে নোটিফিকেশন পাঠানো হবে কি না (true / false); ফোনে ntfy অ্যাপ ইনস্টল করে ntfy_topic-এ সাবস্ক্রাইব করুন",
        pt: "Enviar uma notificação ao celular via ntfy ao fim da pausa (true / false); instale o app ntfy e assine ntfy_topic",
        de: "Am Ende der Pause per ntfy eine Benachrichtigung ans Handy senden (true / false); ntfy-App installieren und ntfy_topic abonnieren",
        ja: "休憩が終わったときに ntfy でスマホに通知するか（true / false）。スマホに ntfy アプリを入れて ntfy_topic を購読してください",
        it: "Inviare una notifica al telefono tramite ntfy alla fine della pausa (true / false); installa l'app ntfy e iscriviti a ntfy_topic",
        ko: "휴식이 끝나면 ntfy로 휴대폰에 알림을 보낼지 여부(true / false). 휴대폰에 ntfy 앱을 설치하고 ntfy_topic을 구독하세요",
        id: "Kirim notifikasi ke ponsel lewat ntfy saat istirahat selesai (true / false); pasang aplikasi ntfy dan berlangganan ntfy_topic",
    };

    pub const NTFY_SERVER: Tr = Tr {
        zh: "ntfy 服务器地址，默认为公共服务器 https://ntfy.sh，也可以填自建的服务器",
        en: "ntfy server URL; defaults to the public https://ntfy.sh, or use your own server",
        hi: "ntfy सर्वर का पता; डिफ़ॉल्ट सार्वजनिक https://ntfy.sh है, या अपना सर्वर इस्तेमाल करें",
        es: "URL del servidor ntfy; por defecto el público https://ntfy.sh, o usa tu propio servidor",
        ar: "عنوان خادم ntfy؛ الافتراضي هو الخادم العام https://ntfy.sh، أو استخدم خادمك الخاص",
        fr: "URL du serveur ntfy ; par défaut le serveur public https://ntfy.sh, ou votre propre serveur",
        bn: "ntfy সার্ভারের ঠিকানা; ডিফল্ট পাবলিক https://ntfy.sh, অথবা নিজের সার্ভার ব্যবহার করুন",
        pt: "URL do servidor ntfy; padrão é o público https://ntfy.sh, ou use seu próprio servidor",
        de: "URL des ntfy-Servers; standardmäßig der öffentliche https://ntfy.sh, oder ein eigener Server",
        ja: "ntfy サーバーの URL。既定は公開サーバー https://ntfy.sh で、自前のサーバーも指定できます",
        it: "URL del server ntfy; predefinito il pubblico https://ntfy.sh, oppure il tuo server",
        ko: "ntfy 서버 주소. 기본값은 공개 서버 https://ntfy.sh이며 직접 운영하는 서버도 쓸 수 있습니다",
        id: "URL server ntfy; bawaan server publik https://ntfy.sh, atau gunakan server sendiri",
    };

    pub const NTFY_TOPIC: Tr = Tr {
        zh: "ntfy 主题，只能用字母、数字、- 和 _。公共服务器上知道主题的人都能订阅，请取一个不易猜到的名字",
        en: "ntfy topic: letters, digits, - and _ only. Anyone who knows it can subscribe on the public server, so pick something hard to guess",
        hi: "ntfy टॉपिक: केवल अक्षर, अंक, - और _। सार्वजनिक सर्वर पर इसे जानने वाला कोई भी सदस्यता ले सकता है, इसलिए अनुमान लगाने में कठिन नाम चुनें",
        es: "Tema de ntfy: solo letras, dígitos, - y _. En el servidor público cualquiera que lo conozca puede suscribirse, así que elige uno difícil de adivinar",
        ar: "موضوع ntfy: حروف وأرقام و- و_ فقط. على الخادم العام يمكن لأي شخص يعرفه الاشتراك فيه، لذا اختر اسمًا يصعب تخمينه",
        fr: "Sujet ntfy : lettres, chiffres, - et _ uniquement. Sur le serveur public, quiconque le connaît peut s'y abonner : choisissez un nom difficile à deviner",
        bn: "ntfy টপিক: শুধু অক্ষর, সংখ্যা, - ও _। পাবলিক সার্ভারে যে কেউ এটি জানলে সাবস্ক্রাইব করতে পারে, তাই অনুমান করা কঠিন এমন নাম দিন",
        pt: "Tópico do ntfy: apenas letras, dígitos, - e _. No servidor público qualquer pessoa que o conheça pode assinar, então escolha algo difícil de adivinhar",
        de: "ntfy-Thema: nur Buchstaben, Ziffern, - und _. Auf dem öffentlichen Server kann es jeder abonnieren, der es kennt – also schwer zu erraten wählen",
        ja: "ntfy のトピック。英数字、-、_ のみ使えます。公開サーバーではトピックを知っていれば誰でも購読できるので、推測されにくい名前にしてください",
        it: "Argomento ntfy: solo lettere, cifre, - e _. Sul server pubblico chiunque lo conosca può iscriversi, quindi scegline uno difficile da indovinare",
        ko: "ntfy 토픽. 영문자, 숫자, -, _만 쓸 수 있습니다. 공개 서버에서는 토픽을 아는 사람 누구나 구독할 수 있으니 추측하기 어려운 이름을 쓰세요",
        id: "Topik ntfy: hanya huruf, angka, - dan _. Di server publik siapa pun yang tahu bisa berlangganan, jadi pilih nama yang sulit ditebak",
    };

    /// 用于测试：所有注释
    #[cfg(test)]
    pub const ALL: [&Tr; 14] = [
        &HEADER,
        &UNITS,
        &WORK,
        &REST,
        &POSTPONE,
        &IDLE,
        &LEVEL,
        &SOUND_PROMPT,
        &SOUND,
        &SOUND_FILE,
        &PHONE_NOTIFY,
        &NTFY_SERVER,
        &NTFY_TOPIC,
        &PROFILES,
    ];
}
