# RestGuard

English | [简体中文](README.zh-CN.md)

A cross-platform break reminder that actually makes you rest: after each work session, a full-screen overlay forces you to take a break, and it can't be closed until the countdown ends.

Built with Rust + Tauri 2 + Svelte. Successor to the Windows-only C# app [ProtectEyes](./ProtectEyes).

## Features

- Forces a 10-minute break after every 60 minutes of work (configurable)
- Multi-monitor: every screen is covered
- The overlay can't be closed during a break (Alt+F4 does nothing), and you can't quit from the tray
- Postpone for 5 minutes, up to 3 times; the count resets after a full break
- Tray menu: time remaining, rest now, start at login
- Single instance
- UI language follows the system: Chinese on Chinese systems, English otherwise

## Install

On Windows, install with [Scoop](https://scoop.sh):

```powershell
scoop bucket add hanstanzi https://github.com/HansTanzi/scoop-bucket
scoop install hanstanzi/restguard
```

Requires the WebView2 runtime (preinstalled on Windows 11).

## Configuration

A `config.toml` is created on first run. Restart the app after editing it.

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

Settings are deliberately kept out of the UI; otherwise the breaks wouldn't really be forced.

## Platform limitations

- **macOS**: system gestures such as `Cmd+Q` and Mission Control can't be fully blocked
- **Linux Wayland**: the protocol doesn't let apps keep themselves on top or position their own windows, so the overlay may not work; X11 is fine

## Development

Requires [Rust](https://rustup.rs), Node.js, pnpm, and [Tauri's system dependencies](https://tauri.app/start/prerequisites/).

```sh
pnpm install
pnpm tauri dev      # run in development
pnpm tauri build    # build installers
cd src-tauri && cargo test
```

To release, bump the version in `tauri.conf.json`, `Cargo.toml` and `package.json`, then push a `v<version>` tag. GitHub Actions builds the exe, creates the Release, and updates `bucket/restguard.json`.

For debugging, set `work_minutes` to `0.1` (6 seconds) to trigger a break quickly.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
