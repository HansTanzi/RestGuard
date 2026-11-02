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
  ja: {
    "nav.levels": "強制レベル",
    "nav.features": "機能",
    "nav.install": "インストール",
    "hero.tagline": "本当に休ませてくれる休憩リマインダー。",
    "hero.lead": "3 段階の強制レベルを内蔵：休憩を促すことも、休憩させることもできます。",
    "hero.download": "ダウンロード",
    "hero.downloadFor": "{os} 版をダウンロード",
    "hero.other": "ほかのプラットフォームとパッケージマネージャー",
    "hero.free": "無料・オープンソース · MIT / Apache-2.0",
    "hero.version": "最新バージョン {version}",
    "levels.title": "3 段階の強制レベル",
    "levels.lead": "どこまで厳しくするかを選べます。この強制力こそが、ほかの休憩リマインダーとの一番の違いです。",
    "levels.high": "高",
    "levels.highDesc": "あらゆる手段で休憩させます。",
    "levels.medium": "中（デフォルト）",
    "levels.mediumDesc": "作業は続けられますが、作業しづらくして休憩へと誘導します。",
    "levels.low": "低",
    "levels.lowDesc": "ほかの休憩リマインダーと同じく、主にお知らせだけです。",
    "table.postpones": "延期回数",
    "table.urgent": "緊急モード（オーバーレイ縮小）",
    "table.meeting": "会議アプリの検出",
    "table.skip": "今回の休憩を閉じる／スキップ",
    "table.coverage": "画面の覆う割合",
    "table.drag": "オーバーレイのドラッグ",
    "features.title": "休憩を確実に取るために",
    "f.force.t": "休憩を確実に取らせる",
    "f.force.d": "60 分作業するごとに 10 分の休憩を、すべての画面で強制します。Alt+F4 は効かず、休憩中はトレイから終了することもできません。",
    "f.custom.t": "細かくカスタマイズ可能",
    "f.custom.d": "任意のプランをもとにカスタムプランを作り、延期、スキップ、緊急モード、覆う割合、ドラッグを調整できます。いくつでも作成できます。",
    "f.meeting.t": "大事なときは邪魔をしない",
    "f.meeting.d": "Zoom、Teams、Webex、Tencent Meeting の使用中は会議が終わるまで待ち、席を離れている間はカウントダウンを一時停止します。",
    "f.notify.t": "休憩の終わりを見逃さない",
    "f.notify.d": "休憩が終わるとサウンドが鳴り、ntfy でスマホやスマートウォッチにも通知できるので、戻るタイミングがわかります。",
    "f.small.t": "小さくて軽い",
    "f.small.d": "Rust + Tauri 2 で OS 標準の WebView を使用。ダウンロードはわずか 2.5 ~ 4.6 MB で、バックグラウンドでもリソースをほとんど使いません。",
    "f.lang.t": "あなたの言語で",
    "f.lang.d": "13 の表示言語に対応し、デフォルトでシステムに従います。トレイメニューからいつでも切り替えられます。",
    "shot.back": "休憩が終わったら「仕事に戻る」をクリック",
    "shot.tray": "トレイメニュー",
    "install.title": "インストール",
    "install.lead": "パッケージマネージャーならアップデートも自動です。直接ダウンロード版はポータブルで、自動アップデートはしません。",
    "install.direct": "直接ダウンロード",
    "install.winNote": "インストーラーのないポータブルな単一 exe：好きな場所に置き、削除すればアンインストールできます。WebView2 ランタイムが必要です（Windows 11 にはプリインストール済み）。",
    "install.macNote": "Apple Silicon と Intel のユニバーサルビルド。解凍して RestGuard.app を /Applications に移動します。アプリは Apple の公証を受けていないため、初回起動前に検疫属性を削除してください：",
    "install.apt": "Debian / Ubuntu（APT リポジトリ）",
    "install.linuxNote": "x86_64 のみ。オーバーレイには X11 が必要です。Linux での Homebrew にも対応しています。詳しくは README を参照してください。",
    "verify.title": "ダウンロードの検証",
    "verify.attest": "リリースファイルは GitHub Actions でビルドされ、署名付きのビルド来歴証明が付いています。GitHub CLI で検証するには：",
    "verify.sums": "またはリリースの SHA256SUMS.txt と照合します：",
    "footer.releases": "すべてのリリース",
    "footer.issues": "問題を報告",
    "footer.privacy": "プライバシーポリシー",
    "footer.license": "MIT / Apache-2.0 のデュアルライセンス。",
    copy: "コピー",
    copied: "コピーしました",
  },
  es: {
    "nav.levels": "Niveles",
    "nav.features": "Funciones",
    "nav.install": "Instalación",
    "hero.tagline": "Un recordatorio de descansos que de verdad te hace descansar.",
    "hero.lead": "Tres niveles de rigor integrados: puede recordarte que tomes un descanso o hacer que lo tomes.",
    "hero.download": "Descargar",
    "hero.downloadFor": "Descargar para {os}",
    "hero.other": "Otras plataformas y gestores de paquetes",
    "hero.free": "Gratis y de código abierto · MIT / Apache-2.0",
    "hero.version": "Última versión {version}",
    "levels.title": "Tres niveles de rigor",
    "levels.lead": "Elige qué tan estricto debe ser. Ese rigor es lo que distingue a RestGuard de otros recordatorios de descansos.",
    "levels.high": "Alto",
    "levels.highDesc": "Hace todo lo posible para que descanses de verdad.",
    "levels.medium": "Medio (predeterminado)",
    "levels.mediumDesc": "Puedes seguir trabajando, pero se vuelve tan incómodo que acabas descansando.",
    "levels.low": "Bajo",
    "levels.lowDesc": "Como otros recordatorios de descansos, sobre todo te avisa.",
    "table.postpones": "Aplazamientos",
    "table.urgent": "Modo urgente (reducir la superposición)",
    "table.meeting": "Detección de apps de reuniones",
    "table.skip": "Cerrar / saltar este descanso",
    "table.coverage": "Cobertura de pantalla",
    "table.drag": "Superposición arrastrable",
    "features.title": "Hecho para que los descansos ocurran",
    "f.force.t": "Descansos que de verdad ocurren",
    "f.force.d": "Un descanso de 10 minutos cada 60 minutos de trabajo, en todos los monitores. Alt+F4 no hace nada y no se puede salir desde la bandeja durante el descanso.",
    "f.custom.t": "Muy personalizable",
    "f.custom.d": "Crea un perfil personalizado a partir de cualquier nivel y ajusta aplazamientos, saltos, modo urgente, cobertura y arrastre. Crea tantos como quieras.",
    "f.meeting.t": "No estorba cuando importa",
    "f.meeting.d": "Espera mientras usas Zoom, Teams, Webex o Tencent Meeting, y pausa la cuenta atrás cuando no estás frente al teclado.",
    "f.notify.t": "Nunca te pierdas el final del descanso",
    "f.notify.d": "Suena un aviso al terminar el descanso, y ntfy puede enviarlo a tu móvil y tu reloj inteligente para que sepas cuándo volver.",
    "f.small.t": "Pequeño y rápido",
    "f.small.d": "Rust + Tauri 2 sobre el WebView del sistema. Las descargas ocupan solo 2,5 ~ 4,6 MB y apenas consume recursos en segundo plano.",
    "f.lang.t": "Habla tu idioma",
    "f.lang.d": "13 idiomas de interfaz; por defecto sigue al sistema. Cámbialo cuando quieras desde el menú de la bandeja.",
    "shot.back": "Cuando termine el descanso, haz clic en «Volver al trabajo»",
    "shot.tray": "El menú de la bandeja",
    "install.title": "Instalación",
    "install.lead": "Los gestores de paquetes mantienen RestGuard actualizado. Las descargas directas son portátiles y no se actualizan solas.",
    "install.direct": "Descarga directa",
    "install.winNote": "Un único exe portátil sin instalador: ponlo donde quieras y bórralo para desinstalarlo. Requiere el runtime de WebView2 (preinstalado en Windows 11).",
    "install.macNote": "Compilación universal para Apple Silicon e Intel. Descomprime y mueve RestGuard.app a /Applications. La app no está notarizada, así que elimina el atributo de cuarentena antes del primer inicio:",
    "install.apt": "Debian / Ubuntu (repositorio APT)",
    "install.linuxNote": "Solo x86_64. La superposición necesita X11. También se admite Homebrew en Linux; consulta el README.",
    "verify.title": "Verifica tu descarga",
    "verify.attest": "Los archivos de cada versión se compilan con GitHub Actions y llevan una atestación firmada de procedencia de compilación. Verifícalos con GitHub CLI:",
    "verify.sums": "O compáralos con el SHA256SUMS.txt de la versión:",
    "footer.releases": "Todas las versiones",
    "footer.issues": "Informar de un problema",
    "footer.privacy": "Política de privacidad",
    "footer.license": "Con doble licencia MIT / Apache-2.0.",
    copy: "Copiar",
    copied: "Copiado",
  },
  "pt-BR": {
    "nav.levels": "Níveis",
    "nav.features": "Recursos",
    "nav.install": "Instalação",
    "hero.tagline": "Um lembrete de pausas que realmente faz você descansar.",
    "hero.lead": "Três níveis de rigor embutidos: ele pode lembrar você de fazer uma pausa ou fazer você fazê-la.",
    "hero.download": "Baixar",
    "hero.downloadFor": "Baixar para {os}",
    "hero.other": "Outras plataformas e gerenciadores de pacotes",
    "hero.free": "Gratuito e de código aberto · MIT / Apache-2.0",
    "hero.version": "Versão mais recente {version}",
    "levels.title": "Três níveis de rigor",
    "levels.lead": "Escolha o quão rigoroso ele deve ser. Esse rigor é o que diferencia o RestGuard de outros lembretes de pausa.",
    "levels.high": "Alto",
    "levels.highDesc": "Faz de tudo para você realmente fazer a pausa.",
    "levels.medium": "Médio (padrão)",
    "levels.mediumDesc": "Você ainda pode trabalhar, mas fica tão desconfortável que você acaba descansando.",
    "levels.low": "Baixo",
    "levels.lowDesc": "Como outros lembretes de pausa, principalmente avisa.",
    "table.postpones": "Adiamentos",
    "table.urgent": "Modo urgente (reduzir a sobreposição)",
    "table.meeting": "Detecção de apps de reunião",
    "table.skip": "Fechar / pular esta pausa",
    "table.coverage": "Cobertura da tela",
    "table.drag": "Sobreposição arrastável",
    "features.title": "Feito para as pausas acontecerem",
    "f.force.t": "Pausas que realmente acontecem",
    "f.force.d": "Uma pausa de 10 minutos a cada 60 minutos de trabalho, em todos os monitores. Alt+F4 não faz nada e não dá para sair pela bandeja durante a pausa.",
    "f.custom.t": "Altamente personalizável",
    "f.custom.d": "Crie um perfil personalizado a partir de qualquer nível e ajuste adiamentos, pular, modo urgente, cobertura e arrastar. Crie quantos quiser.",
    "f.meeting.t": "Não atrapalha quando importa",
    "f.meeting.d": "Espera enquanto Zoom, Teams, Webex ou Tencent Meeting estão em uso e pausa a contagem regressiva quando você se afasta do teclado.",
    "f.notify.t": "Nunca perca o fim da pausa",
    "f.notify.d": "Um som toca quando a pausa termina, e o ntfy pode enviar o aviso para o seu celular e smartwatch, para você saber quando voltar.",
    "f.small.t": "Pequeno e rápido",
    "f.small.d": "Rust + Tauri 2 sobre o WebView do sistema. Os downloads têm só 2,5 ~ 4,6 MB e quase não usa recursos em segundo plano.",
    "f.lang.t": "Fala o seu idioma",
    "f.lang.d": "13 idiomas de interface, seguindo o sistema por padrão. Troque a qualquer momento pelo menu da bandeja.",
    "shot.back": "Quando a pausa acabar, clique em \"Voltar ao trabalho\"",
    "shot.tray": "O menu da bandeja",
    "install.title": "Instalação",
    "install.lead": "Gerenciadores de pacotes mantêm o RestGuard atualizado. Downloads diretos são portáteis e não se atualizam sozinhos.",
    "install.direct": "Download direto",
    "install.winNote": "Um único exe portátil, sem instalador: coloque onde quiser e apague para desinstalar. Requer o runtime do WebView2 (pré-instalado no Windows 11).",
    "install.macNote": "Build universal para Apple Silicon e Intel. Descompacte e mova o RestGuard.app para /Applications. O app não é notarizado, então remova o atributo de quarentena antes da primeira execução:",
    "install.apt": "Debian / Ubuntu (repositório APT)",
    "install.linuxNote": "Somente x86_64. A sobreposição precisa do X11. O Homebrew no Linux também é suportado; veja o README.",
    "verify.title": "Verifique seu download",
    "verify.attest": "Os arquivos de cada versão são compilados pelo GitHub Actions com um atestado assinado de procedência da build. Verifique com o GitHub CLI:",
    "verify.sums": "Ou compare com o SHA256SUMS.txt da versão:",
    "footer.releases": "Todas as versões",
    "footer.issues": "Relatar um problema",
    "footer.privacy": "Política de privacidade",
    "footer.license": "Licença dupla MIT / Apache-2.0.",
    copy: "Copiar",
    copied: "Copiado",
  },
  ko: {
    "nav.levels": "강제 수준",
    "nav.features": "기능",
    "nav.install": "설치",
    "hero.tagline": "진짜로 쉬게 만드는 휴식 알리미.",
    "hero.lead": "세 가지 강제 수준 내장: 휴식을 알려 주기만 할 수도 있고, 실제로 쉬게 만들 수도 있습니다.",
    "hero.download": "다운로드",
    "hero.downloadFor": "{os}용 다운로드",
    "hero.other": "다른 플랫폼 및 패키지 관리자",
    "hero.free": "무료 오픈 소스 · MIT / Apache-2.0",
    "hero.version": "최신 버전 {version}",
    "levels.title": "세 가지 강제 수준",
    "levels.lead": "얼마나 엄격할지 고르세요. 이 강제력이 RestGuard가 다른 휴식 알리미와 가장 다른 점입니다.",
    "levels.high": "높음",
    "levels.highDesc": "휴식을 하도록 할 수 있는 모든 것을 합니다.",
    "levels.medium": "보통(기본값)",
    "levels.mediumDesc": "일은 계속할 수 있지만, 불편하게 만들어 자연스럽게 쉬도록 유도합니다.",
    "levels.low": "낮음",
    "levels.lowDesc": "다른 휴식 알리미처럼 주로 알려 주기만 합니다.",
    "table.postpones": "미루기 횟수",
    "table.urgent": "긴급 모드(오버레이 축소)",
    "table.meeting": "회의 앱 감지",
    "table.skip": "이번 휴식 닫기/건너뛰기",
    "table.coverage": "화면 덮는 비율",
    "table.drag": "오버레이 드래그",
    "features.title": "진짜 휴식을 위해 만들었습니다",
    "f.force.t": "진짜로 쉬게 만드는 휴식",
    "f.force.d": "60분 일할 때마다 모든 모니터에서 10분 휴식을 강제합니다. Alt+F4는 통하지 않고, 휴식 중에는 트레이에서 종료할 수도 없습니다.",
    "f.custom.t": "세세한 사용자 지정",
    "f.custom.d": "아무 수준에서 시작해 사용자 지정 프로필을 만들고 미루기, 건너뛰기, 긴급 모드, 덮는 비율, 드래그를 조정하세요. 원하는 만큼 만들 수 있습니다.",
    "f.meeting.t": "중요한 순간에는 방해하지 않음",
    "f.meeting.d": "Zoom, Teams, Webex, Tencent Meeting을 사용하는 동안에는 기다리고, 자리를 비우면 카운트다운을 멈춥니다.",
    "f.notify.t": "휴식이 끝나는 순간을 놓치지 않음",
    "f.notify.d": "휴식이 끝나면 소리가 나고, ntfy로 휴대폰과 스마트워치에도 알려 주므로 돌아갈 때를 알 수 있습니다.",
    "f.small.t": "작고 가벼움",
    "f.small.d": "Rust + Tauri 2로 시스템 WebView를 사용합니다. 다운로드 크기는 2.5 ~ 4.6MB에 불과하고 백그라운드에서 자원을 거의 쓰지 않습니다.",
    "f.lang.t": "당신의 언어로",
    "f.lang.d": "13개 UI 언어를 지원하며 기본적으로 시스템 언어를 따릅니다. 트레이 메뉴에서 언제든 바꿀 수 있습니다.",
    "shot.back": "휴식이 끝나면 \"업무로 돌아가기\"를 클릭",
    "shot.tray": "트레이 메뉴",
    "install.title": "설치",
    "install.lead": "패키지 관리자로 설치하면 RestGuard가 자동으로 업데이트됩니다. 직접 다운로드한 버전은 포터블이며 스스로 업데이트하지 않습니다.",
    "install.direct": "직접 다운로드",
    "install.winNote": "설치 프로그램 없는 포터블 exe 하나: 아무 곳에나 두고, 지우면 제거됩니다. WebView2 런타임이 필요합니다(Windows 11에는 기본 설치).",
    "install.macNote": "Apple Silicon과 Intel용 유니버설 빌드. 압축을 풀고 RestGuard.app을 /Applications로 옮기세요. 앱이 Apple 공증을 받지 않았으므로 처음 실행하기 전에 격리 속성을 제거하세요:",
    "install.apt": "Debian / Ubuntu(APT 저장소)",
    "install.linuxNote": "x86_64 전용. 오버레이에는 X11이 필요합니다. Linux용 Homebrew도 지원하며, 자세한 내용은 README를 참고하세요.",
    "verify.title": "다운로드 검증",
    "verify.attest": "릴리스 파일은 GitHub Actions로 빌드되며 서명된 빌드 출처 증명이 함께 제공됩니다. GitHub CLI로 검증하려면:",
    "verify.sums": "또는 릴리스의 SHA256SUMS.txt와 비교하세요:",
    "footer.releases": "모든 릴리스",
    "footer.issues": "문제 신고",
    "footer.privacy": "개인정보 처리방침",
    "footer.license": "MIT / Apache-2.0 이중 라이선스.",
    copy: "복사",
    copied: "복사됨",
  },
};

// Screenshot folder under images/ for each language
const SHOT_DIRS = { en: "EN", zh: "CN", de: "DE", ja: "JA", es: "ES", "pt-BR": "PT-BR", ko: "KO" };
// <html lang> for languages whose key isn't a full tag
const HTML_LANGS = { zh: "zh-CN" };

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
    if (base === "pt") return "pt-BR"; // The only Portuguese translation
  }
  return "en";
}

function shotSrc(name) {
  return `images/${SHOT_DIRS[lang] ?? "EN"}/${name}.webp`;
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
  document.documentElement.lang = HTML_LANGS[lang] ?? lang;
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    el.textContent = t(el.dataset.i18n);
  });
  document.querySelectorAll(".copy").forEach((btn) => (btn.textContent = t("copy")));

  document.querySelectorAll("img[data-shot]").forEach((img) => {
    img.src = shotSrc(img.dataset.shot);
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
    levelImg.src = shotSrc(`${level}Enforcement`);
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
