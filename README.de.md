# RestGuard

[English](README.md) | [简体中文](README.zh-CN.md) | Deutsch

Ein plattformübergreifender Pausen-Erinnerer mit drei eingebauten Strenge-Stufen: Er kann dich an eine Pause erinnern – oder dich dazu bringen, sie wirklich zu machen. Genau diese Durchsetzung unterscheidet ihn von anderen Pausen-Erinnerern.

- Hoch: tut alles, damit du die Pause wirklich machst.
- Mittel: Du kannst weiterarbeiten, aber das Arbeiten wird so unbequem, dass du lieber Pause machst.
- Niedrig: wie andere Pausen-Erinnerer, vor allem eine Erinnerung.

Gebaut mit Rust + Tauri 2 + Svelte. Nachfolger der reinen Windows-App [ProtectEyes](./ProtectEyes) in C#.

![Das Pausen-Overlay (Niedrig)](./docs/images/EN/LowEnforcement.webp)

## Funktionen

### Pausen, die wirklich stattfinden

- Erzwingt nach jeweils 60 Minuten Arbeit eine Pause von 10 Minuten (beide Zeiten einstellbar)
- Mehrere Monitore: Jeder Bildschirm wird abgedeckt
- Das Overlay lässt sich während der Pause nicht schließen (Alt+F4 wirkt nicht), und über das Tray lässt sich die App nicht beenden
- 5 Minuten aufschieben, bis zu 3-mal; der Zähler wird nach einer vollständigen Pause zurückgesetzt
- Dringend-Modus: Etwas kann nicht warten? Verkleinere das Overlay kurz, um deine Arbeit zu speichern – der Countdown läuft weiter

![Nach der Pause auf „Zurück an die Arbeit“ klicken](./docs/images/EN/Back2Work.webp)

### Drei eingebaute Profile für unterschiedliche Bedürfnisse

| | Hoch | Mittel (Standard) | Niedrig |
|---|:---:|:---:|:---:|
| Geeignet für | Strenge Disziplin, gegen „nur noch fünf Minuten“ | Alltägliche Arbeit | Einen sanften Anstoß |
| Aufschieben | 1 | 3 | 3 |
| Dringend-Modus (Overlay verkleinern) | ✗ | ✓ | ✓ |
| Erkennung von Meeting-Apps | ✗ | ✓ | ✓ |
| Diese Pause schließen / überspringen | ✗ | ✗ | ✓ |
| Bildschirmabdeckung | 100 % | 100 % | 80 % |
| Overlay verschiebbar | ✗ | ✓ | ✓ |

![Das Pausen-Overlay (Hoch)](./docs/images/EN/HighEnforcement.webp)

![Das Pausen-Overlay (Mittel)](./docs/images/EN/MediumEnforcement.webp)

![Das Pausen-Overlay (Niedrig)](./docs/images/EN/LowEnforcement.webp)

### Umfassend anpassbar

Wenn die eingebauten Profile nicht passen, wähle in den Einstellungen **Neu (benutzerdefiniert)…**, um von einem beliebigen Profil aus zu starten, gib ihm einen Namen (später umbenennbar oder löschbar) und passe jede Option an:

- Anzahl der Aufschübe, ob Beenden erlaubt ist, ob das Überspringen dieser Pause erlaubt ist
- Ob der Dringend-Modus verfügbar ist und wie stark das Overlay verkleinert wird
- Wie viel jedes Bildschirms das Overlay abdeckt (10 % ~ 100 %) und ob es innerhalb seines Bildschirms verschoben werden kann
- Ob Meeting-Apps erkannt werden

Lege beliebig viele eigene Profile an und wechsle jederzeit. Arbeits-, Pausen-, Aufschub- und Leerlaufzeiten gelten für alle Profile gemeinsam. Wer Dateien lieber von Hand bearbeitet, kann auch [`config.toml`](#konfiguration) verwenden.

### Hält sich zurück, wenn es darauf ankommt

- Meeting-Erkennung (Windows, macOS): Solange Zoom, Microsoft Teams, Webex oder Tencent Meeting im Vordergrund ist (oder unter Windows das Mikrofon nutzt), wartet die Pause, bis das Meeting vorbei ist; während einer Pause kannst du außerdem auf „Ich bin in einem …-Meeting“ klicken, um in 15 Minuten erneut erinnert zu werden
- Leerlauferkennung: Nach 5 Minuten ohne Tastatur- oder Mauseingabe giltst du als abwesend und der Arbeits-Countdown pausiert; warst du so lange weg wie eine Pause dauert, zählt das als vollständige Pause, sodass du nach der Rückkehr nicht gleich wieder in die Pause geschickt wirst

### Das Pausenende nie verpassen

- Am Ende einer Pause ertönt ein Ton: Wähle einen eingebauten oder deine eigene Audiodatei
- Handy-Benachrichtigungen: Erhalte über [ntfy](https://ntfy.sh) eine Push-Nachricht auf dein Handy (Android, iOS), damit du auch nach dem Weggehen weißt, wann es weitergeht; unter Android kann sie wie ein Wecker klingeln, bis du sie wegwischst
- Smartwatches: Die Handy-Benachrichtigung wird auf eine gekoppelte Apple Watch, Wear OS, Galaxy Watch oder eine Uhr bzw. ein Band von Xiaomi, Huawei, Garmin o. Ä. gespiegelt – ein Blick aufs Handgelenk genügt (siehe [Benachrichtigungen auf Handy und Smartwatch](#benachrichtigungen-auf-handy-und-smartwatch))

### Klein und schnell

- Gebaut mit Rust + Tauri 2 auf der System-WebView, ohne mitgelieferte Browser-Engine – im Hintergrund verbraucht es kaum Ressourcen
- Winzig: Downloads sind nur 2,5 ~ 4,6 MB groß (Windows 4,6 MB, macOS 4,0 MB, Linux 2,5 MB)
- Portabel: unter Windows eine einzelne exe, unter macOS eine `.app` zum Entpacken und Starten; zum Deinstallieren einfach löschen

### Und mehr

- Plattformübergreifend: Windows, macOS und Linux, installier- und aktualisierbar über WinGet, Scoop, Homebrew und APT (siehe [Installation](#installation))
- Tray-Menü: verbleibende Zeit, jetzt Pause machen, Einstellungen, beim Anmelden starten, Strenge-Profil wechseln
- Nur eine Instanz
- Frei und Open Source unter der freizügigen Doppellizenz MIT / Apache-2.0 (siehe [Lizenz](#lizenz))
- Verfügbar auf Vereinfachtem Chinesisch, Englisch, Hindi, Spanisch, Arabisch, Französisch, Bengalisch, Portugiesisch, Deutsch, Japanisch, Italienisch, Koreanisch und Indonesisch. Die Sprache der Oberfläche folgt standardmäßig dem System (Englisch, wenn die Systemsprache nicht unterstützt wird); du kannst sie jederzeit im Untermenü „语言 / Language“ des Tray-Menüs wechseln

![Das Tray-Menü](./docs/images/EN/TrayMenu.webp)

## Installation

### Windows

Installation mit [WinGet](https://learn.microsoft.com/windows/package-manager/winget/):

```powershell
winget install HansTanzi.RestGuard
```

Oder mit [Scoop](https://scoop.sh):

```powershell
scoop bucket add hanstanzi https://github.com/HansTanzi/scoop-bucket
scoop install hanstanzi/restguard
```

Oder lade `RestGuard_<Version>_x64.exe` von den [Releases](https://github.com/HansTanzi/RestGuard/releases/latest) herunter und starte sie. Es ist eine portable Einzeldatei ohne Installer: Lege sie an einen beliebigen Ort und lösche sie zum Deinstallieren. Sie aktualisiert sich nicht selbst; lade für ein Update die neue Version herunter.

Benötigt die WebView2-Laufzeit (unter Windows 11 vorinstalliert).

### macOS

Installation mit [Homebrew](https://brew.sh) (Universal-Build für Apple Silicon und Intel):

```sh
brew install --cask hanstanzi/tap/restguard
```

Falls du dieses Repository früher als Tap hinzugefügt hast (`brew tap hanstanzi/restguard https://github.com/HansTanzi/RestGuard`), wechsle zum neuen Tap, um weiterhin Updates zu erhalten: `brew untap hanstanzi/restguard && brew install --cask hanstanzi/tap/restguard`.

Oder lade `RestGuard_<Version>_universal.zip` von den [Releases](https://github.com/HansTanzi/RestGuard/releases/latest) herunter, entpacke es und verschiebe `RestGuard.app` nach `/Applications`. Die App aktualisiert sich nicht selbst; lade für ein Update die neue Version herunter.

RestGuard sitzt in der Menüleiste und hat kein Dock-Symbol. Die App ist nicht von Apple notarisiert; der Cask entfernt bei der Installation das Quarantäne-Attribut. Wenn du das Zip manuell heruntergeladen hast, führe vor dem ersten Start `xattr -dr com.apple.quarantine /Applications/RestGuard.app` aus, sonst lässt macOS die App nicht starten.

### Linux

Unter Debian/Ubuntu (x86_64) füge das [APT-Repository](https://github.com/HansTanzi/apt) hinzu und installiere bzw. aktualisiere dann mit `apt`:

```sh
sudo install -d /etc/apt/keyrings
curl -fsSL https://hanstanzi.github.io/apt/hanstanzi.gpg | sudo tee /etc/apt/keyrings/hanstanzi.gpg > /dev/null
echo "deb [signed-by=/etc/apt/keyrings/hanstanzi.gpg] https://hanstanzi.github.io/apt stable main" | sudo tee /etc/apt/sources.list.d/hanstanzi.list
sudo apt update && sudo apt install rest-guard
```

Neuere Versionen kommen mit `sudo apt update && sudo apt upgrade`. Du kannst die `.deb` auch direkt von den [Releases](https://github.com/HansTanzi/RestGuard/releases/latest) installieren (`sudo apt install ./RestGuard_*_amd64.deb`), dann wird sie aber nicht aktualisiert.

Für andere Distributionen lade die Datei von den [Releases](https://github.com/HansTanzi/RestGuard/releases/latest) herunter. Unter Fedora/openSUSE installierst du die `.rpm`:

```sh
sudo dnf install ./RestGuard_*_x86_64.rpm      # Fedora
sudo zypper install ./RestGuard_*_x86_64.rpm   # openSUSE
```

Unter Arch installierst du die `.pkg.tar.zst`:

```sh
sudo pacman -U ./RestGuard_*_x86_64.pkg.tar.zst
```

Auf jeder Distribution (x86_64, glibc 2.35+) kannst du auch mit [Homebrew](https://brew.sh) installieren und aktualisieren. Homebrew kann keine Systembibliotheken installieren, installiere daher zuerst WebKitGTK 4.1 und libayatana-appindicator mit dem Paketmanager deiner Distribution:

```sh
sudo apt install libwebkit2gtk-4.1-0 libayatana-appindicator3-1                 # Debian/Ubuntu
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3                     # Fedora
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator                           # Arch
sudo zypper install libwebkit2gtk-4_1-0 libayatana-appindicator3-1              # openSUSE

brew install --cask hanstanzi/tap/restguard
```

Dabei wird nur die ausführbare Datei `restguard` installiert, ohne Eintrag im Anwendungsmenü: Starte `restguard` einmal und aktiviere dann „Beim Anmelden starten“ im Tray-Menü. Neuere Versionen kommen mit `brew upgrade`.

Das Overlay benötigt X11 (siehe [Plattformeinschränkungen](#plattformeinschränkungen)).

### Downloads überprüfen

Release-Dateien nach v0.1.14 werden von GitHub Actions gebaut und tragen eine signierte [Build-Provenance-Attestierung](https://docs.github.com/actions/security-for-github-actions/using-artifact-attestations), die jede Datei mit dem Workflow-Lauf und dem Commit verknüpft, aus dem sie entstanden ist. So prüfst du eine heruntergeladene Datei mit der [GitHub CLI](https://cli.github.com):

```sh
gh attestation verify RestGuard_<Version>_x64.exe --repo HansTanzi/RestGuard
```

Jedes Release enthält außerdem `SHA256SUMS.txt`, damit du Downloads auch ohne GitHub CLI prüfen kannst. Lege die heruntergeladene Datei in dasselbe Verzeichnis wie `SHA256SUMS.txt` und führe aus:

```sh
sha256sum -c SHA256SUMS.txt --ignore-missing          # Linux
shasum -a 256 -c SHA256SUMS.txt --ignore-missing      # macOS
```

Unter Windows vergleiche die Ausgabe von `Get-FileHash .\RestGuard_*_x64.exe` mit der passenden Zeile in `SHA256SUMS.txt` (Groß-/Kleinschreibung spielt keine Rolle).

### Aus dem Quellcode bauen

Installiere die unter [Entwicklung](#entwicklung) genannten Voraussetzungen und baue dann eine getaggte Version:

```sh
git clone https://github.com/HansTanzi/RestGuard
cd RestGuard
git checkout v<Version>
pnpm install --frozen-lockfile
pnpm tauri build --no-bundle
```

Die ausführbare Datei liegt unter `src-tauri/target/release/restguard` (`restguard.exe` unter Windows). Baue mit `pnpm tauri build`, nicht mit `cargo build`: Ein reiner Cargo-Build ist ein Entwicklungs-Build, der die Oberfläche von `localhost:1420` lädt und leere Fenster zeigt.

## Konfiguration

Öffne **Einstellungen…** im Tray-Menü, um Zeiten, Strenge-Stufe, Overlay-Abdeckung, Sprache und den Start beim Anmelden zu ändern. Änderungen gelten sofort (während einer Pause sind die Einstellungen gesperrt). Wenn du die Arbeitszeit änderst, startet der aktuelle Arbeits-Countdown neu.

Die Einstellungen werden in `config.toml` gespeichert, die beim ersten Start angelegt wird. Du kannst sie auch von Hand bearbeiten; starte die App danach neu.

| System | Pfad |
|---|---|
| Windows | `%APPDATA%\io.github.restguard\config.toml` |
| macOS | `~/Library/Application Support/io.github.restguard/config.toml` |
| Linux | `~/.config/io.github.restguard/config.toml` |

```toml
work_minutes = 60.0      # wie lange gearbeitet wird, bevor eine Pause erzwungen wird
rest_minutes = 10.0      # wie lange jede Pause dauert
postpone_minutes = 5.0   # wie lange jeder Aufschub dauert
idle_minutes = 5.0       # so lange ohne Eingabe gilt als abwesend, 0 = aus
level = "medium"         # "high", "medium", "low" oder der Name eines eigenen Profils
sound_prompt = true      # am Ende einer Pause einen Ton abspielen
sound = "beep"           # "beep", "chime", "rise" oder "file" (spielt sound_file ab)
sound_file = ""          # vollständiger Pfad deiner eigenen Audiodatei, verwendet bei sound = "file"
phone_notify = false     # am Ende einer Pause über ntfy eine Benachrichtigung ans Handy senden
ntfy_server = "https://ntfy.sh"  # ntfy-Server, der öffentliche oder dein eigener
ntfy_topic = ""          # Topic, das in der ntfy-App abonniert wird; schwer zu erraten halten

[[fixed_breaks]]         # eine feste Pause; "work_end" (Feierabend) und "lunch" (Mittagspause) sind eingebaut
name = "work_end"
enabled = false          # Pausen, die kurz davor fällig werden, aufschieben
time = "18:00"           # 24-Stunden-Format HH:MM in Ortszeit
quiet_minutes = 30.0     # Pausen, die so lange vor time fällig werden, warten bis time

[[fixed_breaks]]         # Mittagspause: um time beginnt eine Pause von rest_minutes
name = "lunch"
enabled = true
time = "12:00"
quiet_minutes = 30.0
rest_minutes = 30.0      # wie lange die Pause dauert (Mittagspause und eigene feste Pausen)

[[fixed_breaks]]         # eigene hinzufügen, z. B. eine Kaffeepause, je ein [[fixed_breaks]]
name = "Kaffeepause"
enabled = true
time = "15:00"
quiet_minutes = 15.0
rest_minutes = 15.0

[[profiles]]             # ein eigenes Profil; ein [[profiles]] pro Profil
name = "Fokus"
max_postpones = 1        # maximale Aufschübe, bevor eine vollständige Pause nötig ist
urgent_mode = false      # Verkleinern des Overlays erlauben
meeting_detection = true # Pausen zurückhalten, solange Zoom, Teams usw. genutzt werden
force_rest = true        # false = das Overlay kann geschlossen werden
allow_skip = false       # „Diese Pause überspringen“ trotzdem anbieten, wenn Pausen erzwungen werden
overlay_coverage = 1.0   # abgedeckter Anteil jedes Bildschirms, 0.1 ~ 1.0
urgent_scale = 0.5       # Skalierung im Dringend-Modus (verkleinertes Overlay) relativ zum obigen Wert, 0.1 ~ 0.9
movable = false          # Verschieben des Overlays innerhalb seines Bildschirms erlauben, wenn es nicht den ganzen Bildschirm füllt
```

### Benachrichtigungen auf Handy und Smartwatch

1. Installiere die ntfy-App auf deinem Handy ([Android](https://play.google.com/store/apps/details?id=io.heckel.ntfy), [F-Droid](https://f-droid.org/packages/io.heckel.ntfy/), [iOS](https://apps.apple.com/app/ntfy/id1625396347))
2. Aktiviere in den Einstellungen von RestGuard „Am Ende der Pause das Handy benachrichtigen (ntfy)“ und gib ein schwer zu erratendes Topic ein (nur Buchstaben, Ziffern, `-` und `_`). Als Server ist der öffentliche `https://ntfy.sh` voreingestellt; du kannst stattdessen deinen eigenen verwenden
3. Abonniere in der ntfy-App denselben Server und dasselbe Topic und klicke dann auf „Testbenachrichtigung senden“, um zu prüfen, ob sie ankommt

Benachrichtigungen werden mit höchster Priorität gesendet. Unter Android aktivierst du in den ntfy-Einstellungen „Keep alerting for highest priority“, damit sie wie ein Wecker klingelt, bis du sie wegwischst; unter iOS ist es eine zeitkritische Mitteilung, die den Fokus durchbricht (wenn du ntfy zeitkritische Mitteilungen erlaubst). Jeder, der das Topic kennt, kann es auf dem öffentlichen Server abonnieren – halte es also geheim.

Auf der Uhr muss nichts installiert werden; das System spiegelt die Handy-Benachrichtigung dorthin:

| Uhr | Einrichtung |
|---|---|
| Apple Watch | Öffne in der Watch-App auf dem iPhone „Mitteilungen“ und aktiviere für ntfy „iPhone-Hinweise spiegeln“. Mitteilungen gehen nur an die Uhr, während das iPhone gesperrt oder der Bildschirm aus ist |
| Wear OS / Galaxy Watch | Standardmäßig gespiegelt; stelle sicher, dass ntfy in den Benachrichtigungseinstellungen der Uhr nicht deaktiviert ist |
| Xiaomi, Huawei, Garmin, Amazfit usw. | Erlaube ntfy in der App-Benachrichtigungsliste der Begleit-App (Zepp, Huawei Health, Garmin Connect usw.) |

Hinweise:

- Die Uhr vibriert einmal; sie alarmiert nicht dauerhaft wie das Handy
- Das Handy muss online sein und ntfy im Hintergrund laufen, damit die Uhr die Benachrichtigung erhält; eine Uhr, die ohne Handy genutzt wird, erhält sie nicht
- Bei einem selbst gehosteten Server benötigt iOS `upstream-base-url: "https://ntfy.sh"` in der Serverkonfiguration, um Benachrichtigungen sofort zu erhalten (siehe [ntfy-Dokumentation](https://docs.ntfy.sh/config/#ios-instant-notifications))

## Plattformeinschränkungen

- **macOS**: Systemaktionen wie „Sofort beenden“ (`Cmd+Option+Esc`) lassen sich nicht blockieren
- **Linux**: Die Leerlauferkennung benötigt GNOME oder einen Desktop, der `org.freedesktop.ScreenSaver` implementiert (z. B. KDE Plasma), sowie `gdbus`; andernfalls ist sie aus
- **Linux**: Töne werden mit `paplay`, `pw-play` oder `aplay` abgespielt; verwende für eigene Dateien daher wav, ogg oder flac (mp3 wird nicht unterstützt)
- **Linux Wayland**: Das Protokoll erlaubt Apps nicht, sich selbst im Vordergrund zu halten oder ihre Fenster selbst zu positionieren, daher funktioniert das Overlay möglicherweise nicht; unter X11 klappt es

## Entwicklung

Benötigt [Rust](https://rustup.rs), Node.js, pnpm und die [Systemabhängigkeiten von Tauri](https://tauri.app/start/prerequisites/).

```sh
pnpm install
pnpm tauri dev      # im Entwicklungsmodus starten
pnpm tauri build    # Installationspakete bauen
cd src-tauri && cargo test
```

Entwicklungs-Builds lesen `config.dev.toml` und `language.dev` (im selben Verzeichnis wie `config.toml`), sodass du kurze Zeiten verwenden oder die Sprache wechseln kannst, ohne die installierte App zu beeinflussen. Drücke `Esc` auf dem Overlay, um eine Pause sofort zu beenden. Beende vorher das installierte RestGuard, sonst sorgt die Einzelinstanz-Prüfung dafür, dass sich der Entwicklungs-Build sofort beendet.

Für ein Release führe `pnpm release <version>` (z. B. `pnpm release 0.2.0`) auf einem sauberen `dev`-Branch aus. Das Skript erhöht die Version in `package.json`, `tauri.conf.json`, `Cargo.toml` und `Cargo.lock`, committet und pusht ein Tag `v<version>`. GitHub Actions baut dann die exe, eine universelle macOS-`.app` sowie unter Linux die `.deb`, `.rpm` und die Arch-`.pkg.tar.zst` (mit `src-tauri/arch/PKGBUILD` aus der `.deb` neu verpackt), erstellt das Release und pusht die aktualisierte `bucket/restguard.json` nach [HansTanzi/scoop-bucket](https://github.com/HansTanzi/scoop-bucket) sowie `Casks/restguard.rb` (die macOS-`.app` und unter Linux die ausführbare Datei aus dem Arch-Paket) nach [HansTanzi/homebrew-tap](https://github.com/HansTanzi/homebrew-tap). Dafür werden die Repository-Secrets `SCOOP_BUCKET_DEPLOY_KEY` und `HOMEBREW_TAP_DEPLOY_KEY` benötigt: die privaten Hälften von Deploy-Keys mit Schreibzugriff auf diese Repositories. Bei stabilen Versionen wird außerdem [HansTanzi/apt](https://github.com/HansTanzi/apt) angestoßen, um das APT-Repository aus den neuesten Releases neu zu bauen; dafür wird das Repository-Secret `APT_DISPATCH_TOKEN` benötigt (ein auf dieses Repository beschränkter Fine-grained PAT mit ausschließlich Lese- und Schreibrechten für Actions). Signaturschlüssel und Einrichtung sind in jenem Repository beschrieben. Bei stabilen Versionen wird außerdem über [winget-releaser](https://github.com/vedantmgoyal9/winget-releaser) ein PR an [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs) geöffnet; dafür werden das Repository-Secret `WINGET_TOKEN` (ein Classic PAT mit Scope `public_repo`) und ein Fork von `winget-pkgs` im Konto des Token-Inhabers benötigt. Die allererste WinGet-Version muss von Hand eingereicht werden, z. B. mit `komac new HansTanzi.RestGuard --version <version> --urls <exe-Release-URL>` über [Komac](https://github.com/russellbanks/Komac) (Installer-Typ `portable` wählen und `Microsoft.EdgeWebView2Runtime` als Paketabhängigkeit hinzufügen).

### Microsoft Store

Der Store-Build ist ein MSIX-Paket; der Store signiert es, daher wird kein Code-Signing-Zertifikat benötigt. Führe `pnpm msix` aus, um lokal `src-tauri/target/msix/RestGuard_<version>_x64.msix` zu bauen (benötigt das Windows SDK); der Release-Workflow baut es für stabile Versionen ebenfalls und hängt es als Artefakt `msix` an den Workflow-Lauf an. Lade es in [Partner Center](https://partner.microsoft.com/dashboard) in eine neue Einreichung hoch.

Einmalige Einrichtung: Reserviere den App-Namen im Partner Center und kopiere dann `Package/Identity/Name`, `Package/Identity/Publisher` und `Package/Properties/PublisherDisplayName` von der Seite **Product identity** der App in `src-tauri/msix/AppxManifest.xml`. Das Paket deklariert die eingeschränkte Funktion `runFullTrust` (für Desktop-Apps erforderlich), daher verlangt die Einreichung eine Begründung, z. B. „Desktop-App auf Basis von Tauri; benötigt volles Vertrauen, um auf jedem Bildschirm Pausen-Overlays im Vordergrund anzuzeigen“.

Innerhalb des Pakets nutzt der Start beim Anmelden den `StartupTask` aus dem Manifest statt der Registry, da Registry-Schreibzugriffe dort virtualisiert werden. Um das Paket ohne den Store auszuprobieren, aktiviere den Entwicklermodus und führe `Add-AppxPackage -Register src-tauri/target/msix/layout/AppxManifest.xml` aus.

Zum Debuggen setze `work_minutes` auf `0.1` (6 Sekunden), um schnell eine Pause auszulösen.

## Datenschutz

RestGuard erhebt keine Daten: keine Konten, keine Analyse, keine Telemetrie. Online geht es nur, um die optionale ntfy-Benachrichtigung ans Handy zu senden. Details in der [Datenschutzerklärung](https://hanstanzi.github.io/RestGuard/privacy.html) (auf Englisch).

## Lizenz

Lizenziert nach Wahl unter [MIT](LICENSE-MIT) oder [Apache-2.0](LICENSE-APACHE).
