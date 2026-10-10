# RestGuard

English | [简体中文](README.zh-CN.md)

A cross-platform break reminder that actually makes you rest: after each work session, a full-screen overlay forces you to take a break, and it can't be closed until the countdown ends.

Built with Rust + Tauri 2 + Svelte. Successor to the Windows-only C# app [ProtectEyes](./ProtectEyes).

![The break overlay - Main Screen](./docs/images/RestUI-MainScreen.webp)

![The break overlay - Other Screens](./docs/images/RestUI-OtherScreen.webp)

## Features

- Forces a 10-minute break after every 60 minutes of work (configurable)
- Multi-monitor: every screen is covered
- The overlay can't be closed during a break (Alt+F4 does nothing), and you can't quit from the tray
- Postpone for 5 minutes, up to 3 times; the count resets after a full break
- Tray menu: time remaining, rest now, settings, start at login
- Single instance
- UI language follows the system by default (Chinese on Chinese systems, English otherwise); switch it any time from the tray menu's "语言 / Language" submenu

## Install

On Windows, install with [WinGet](https://learn.microsoft.com/windows/package-manager/winget/):

```powershell
winget install HansTanzi.RestGuard
```

Or with [Scoop](https://scoop.sh):

```powershell
scoop bucket add hanstanzi https://github.com/HansTanzi/scoop-bucket
scoop install hanstanzi/restguard
```

Requires the WebView2 runtime (preinstalled on Windows 11).

On macOS, install with [Homebrew](https://brew.sh) (universal build for Apple Silicon and Intel):

```sh
brew tap hanstanzi/restguard https://github.com/HansTanzi/RestGuard
brew install --cask hanstanzi/restguard/restguard
```

RestGuard lives in the menu bar and has no Dock icon. The app isn't notarized by Apple; the cask removes the quarantine flag on install. If you download the zip from Releases by hand, run `xattr -dr com.apple.quarantine /Applications/RestGuard.app` before first launch.

## Configuration

Open **Settings…** from the tray menu to change durations, postpone limits, overlay coverage, language and start-at-login. Changes apply immediately (settings are locked during a break). Changing the work duration restarts the current work countdown.

Settings are stored in `config.toml`, created on first run. You can also edit it by hand; restart the app afterwards.

| OS | Path |
|---|---|
| Windows | `%APPDATA%\io.github.restguard\config.toml` |
| macOS | `~/Library/Application Support/io.github.restguard/config.toml` |
| Linux | `~/.config/io.github.restguard/config.toml` |

```toml
work_minutes = 60.0      # how long to work before a forced break
rest_minutes = 10.0      # how long each break lasts
postpone_minutes = 5.0   # how long each postpone lasts
max_postpones = 3        # max postpones before a full break is required
overlay_coverage = 1.0   # fraction of each screen covered, 0.1 ~ 1.0
```

## Platform limitations

- **macOS**: system-level actions such as Force Quit (`Cmd+Option+Esc`) can't be blocked
- **Linux Wayland**: the protocol doesn't let apps keep themselves on top or position their own windows, so the overlay may not work; X11 is fine

## Development

Requires [Rust](https://rustup.rs), Node.js, pnpm, and [Tauri's system dependencies](https://tauri.app/start/prerequisites/).

```sh
pnpm install
pnpm tauri dev      # run in development
pnpm tauri build    # build installers
cd src-tauri && cargo test
```

Dev builds read `config.dev.toml` and `language.dev` (same directory as `config.toml`), so you can use short durations or switch languages without touching the installed app. Press `Esc` on the overlay to end a break immediately. Quit the installed RestGuard first, otherwise the single-instance check makes the dev build exit right away.

To release, run `pnpm release <version>` (e.g. `pnpm release 0.2.0`) on a clean `dev` branch. It bumps the version in `package.json`, `tauri.conf.json`, `Cargo.toml` and `Cargo.lock`, commits, and pushes a `v<version>` tag. GitHub Actions then builds the exe and a universal macOS `.app`, creates the Release, and commits the updated `bucket/restguard.json` and `Casks/restguard.rb` to `dev`, so run `git pull` afterwards. For stable versions it also opens a PR to [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs) via [winget-releaser](https://github.com/vedantmgoyal9/winget-releaser); this needs a `WINGET_TOKEN` repository secret (a classic PAT with `public_repo` scope) and a fork of `winget-pkgs` under the token owner's account. The very first WinGet version must be submitted by hand, e.g. `komac new HansTanzi.RestGuard --version <version> --urls <exe release URL>` with [Komac](https://github.com/russellbanks/Komac) (pick the `portable` installer type and add `Microsoft.EdgeWebView2Runtime` as a package dependency).

For debugging, set `work_minutes` to `0.1` (6 seconds) to trigger a break quickly.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
