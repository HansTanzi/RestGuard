# Changelog

All notable changes to RestGuard are listed here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/).

Downloads for every version are on the [Releases](https://github.com/HansTanzi/RestGuard/releases) page.

## [Unreleased]

### Added

- Tray menu: pause reminders for 30 minutes, 1 or 2 hours, or until the next morning, then they resume on their own, so stopping for a while no longer means quitting and forgetting to start RestGuard again. Like Quit, it's unavailable during a break or while a postponed break is owed

## [0.1.22] - 2026-11-11

### Added

- README and website: install from the Microsoft Store on Windows

### Changed

- Meeting detection: "I'm in a meeting" now reminds you again in an hour instead of 15 minutes
- Tray menu: the language submenu is labelled "Language" followed by the word in the current language, instead of always "语言 / Language"

## [0.1.21] - 2026-11-05

### Added

- README and website in French, Italian, Indonesian, Hindi, Bengali and Arabic, so both now cover all 13 UI languages; the website lays out right to left in Arabic

### Changed

- Fixed breaks: only a regular break's length is forced; after that, "Keep using the computer" closes the overlay, and work time starts counting when the fixed break ends, which the sound and phone notification still announce
- Break overlay: the title names the fixed break under way, such as "Time for lunch!"
- Break overlay: the Chinese title is now a plain translation of the English one

### Fixed

- Tray menu: when the next break is a fixed break, the countdown now names it and counts down to it, such as "Lunch in 10:00", instead of showing the time to the next regular break

## [0.1.20] - 2026-11-02

### Added

- Light theme alongside the existing dark one (the break overlay always stays dark)

## [0.1.19] - 2026-11-01

### Added

- Auto-lock: optionally lock the screen once a break is under way, so you have to get up and unlock it; the overlay shows the rest of the break after unlocking

### Changed

- Urgent mode: a shrunk overlay no longer counts as rest time
- Urgent mode: the overlay returns to its normal size after 2 minutes

## [0.1.18] - 2026-10-30

### Added

- Fixed breaks: a built-in lunch break, plus your own breaks at set times (e.g. a tea break)
- Work-end window: holds off breaks that fall due just before the end of the working day
- Screenshots in every supported language
- Microsoft Store listing in every supported language

### Security

- Dependabot with a one-week cooldown, to reduce the risk of pulling in a hijacked dependency release

## [0.1.17] - 2026-10-27

### Added

- Website, license and privacy policy links in the About window
- Privacy policy

## [0.1.16] - 2026-10-25

### Added

- Project website on GitHub Pages

### Changed

- Clearer "Enforcement profile" block in Settings
- Settings can't be changed while a break is postponed
- Smaller binary by using the operating system's TLS library

## [0.1.15] - 2026-10-25

### Added

- `SHA256SUMS.txt` in every release
- Signed build provenance attestations for release files
- Build-from-source instructions

## [0.1.14] - 2026-10-24

### Added

- Meeting detection on macOS checks microphone use

### Fixed

- Copy, paste and select-all didn't work on macOS

## [0.1.13] - 2026-10-22

### Added

- German README
- A friendly hint when an invalid character is entered

## [0.1.12] - 2026-10-22

### Added

- Phone and smartwatch notifications via ntfy when a break ends

## [0.1.11] - 2026-10-21

### Added

- A sound when a break ends

## [0.1.10] - 2026-10-18

### Added

- Homebrew on Linux

## [0.1.9] - 2026-10-16

### Added

- Settings button to open the configuration file

### Changed

- High enforcement allows one postpone

## [0.1.8] - 2026-10-15

### Added

- Tray option to switch the enforcement profile

## [0.1.7] - 2026-10-11

### Added

- Enforcement levels: High, Medium and Low
- Custom profiles, with rename
- Per-profile options: allow exit, allow skip, overlay coverage, urgent-mode scale, movable overlay

### Fixed

- In Low enforcement, the overlay was too small and only appeared on the main screen

## [0.1.6] - 2026-10-11

### Added

- Idle detection
- APT repository for Debian/Ubuntu

## [0.1.5] - 2026-10-10

### Added

- Arch Linux package

### Changed

- `.rpm` replaces AppImage

## [0.1.4] - 2026-10-10

### Changed

- Scoop bucket and Homebrew tap moved to their own repositories

## [0.1.3] - 2026-10-10

### Changed

- Releases are built from the `dev` branch

## [0.1.2] - 2026-10-10

### Added

- Linux release
- 11 more UI languages
- Meeting detection: breaks wait while a meeting app is in use

## [0.1.1] - 2026-10-10

### Added

- macOS support and Homebrew
- WinGet
- Settings window
- About window
- Urgent mode: shrink and move the overlay to save your work
- Language switching
- The break overlay looks the same on every screen

## [0.1.0] - 2026-10-09

### Added

- First release: a rewrite of [ProtectEyes](https://github.com/HansTanzi/ProtectEyes) in Rust + Tauri 2 + Svelte
- English and Simplified Chinese
- Scoop

[Unreleased]: https://github.com/HansTanzi/RestGuard/compare/v0.1.22...HEAD
[0.1.22]: https://github.com/HansTanzi/RestGuard/compare/v0.1.21...v0.1.22
[0.1.21]: https://github.com/HansTanzi/RestGuard/compare/v0.1.20...v0.1.21
[0.1.20]: https://github.com/HansTanzi/RestGuard/compare/v0.1.19...v0.1.20
[0.1.19]: https://github.com/HansTanzi/RestGuard/compare/v0.1.18...v0.1.19
[0.1.18]: https://github.com/HansTanzi/RestGuard/compare/v0.1.17...v0.1.18
[0.1.17]: https://github.com/HansTanzi/RestGuard/compare/v0.1.16...v0.1.17
[0.1.16]: https://github.com/HansTanzi/RestGuard/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/HansTanzi/RestGuard/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/HansTanzi/RestGuard/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/HansTanzi/RestGuard/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/HansTanzi/RestGuard/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/HansTanzi/RestGuard/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/HansTanzi/RestGuard/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/HansTanzi/RestGuard/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/HansTanzi/RestGuard/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/HansTanzi/RestGuard/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/HansTanzi/RestGuard/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/HansTanzi/RestGuard/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/HansTanzi/RestGuard/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/HansTanzi/RestGuard/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/HansTanzi/RestGuard/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/HansTanzi/RestGuard/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/HansTanzi/RestGuard/releases/tag/v0.1.0
