# Contributing to RestGuard

Thanks for helping out! Bug reports, ideas, translations, docs and code are all welcome.

## Ways to contribute

- **Report a bug**: open a [bug report](https://github.com/HansTanzi/RestGuard/issues/new?template=bug_report.yml). Include your OS, desktop environment on Linux, and RestGuard version (shown in the About window).
- **Suggest a feature**: open a [feature request](https://github.com/HansTanzi/RestGuard/issues/new?template=feature_request.yml) and describe the problem it solves.
- **Improve a translation**: RestGuard ships in 13 languages, and native speakers catch things machine translation misses. See [Translations](#translations).
- **Fix docs or code**: issues labeled [`good first issue`](https://github.com/HansTanzi/RestGuard/labels/good%20first%20issue) are a good place to start. For larger changes, open an issue first so we can agree on the approach before you spend time on it.
- **Report a security issue**: please don't open a public issue; see [SECURITY.md](SECURITY.md).

## Development setup

Requires [Rust](https://rustup.rs), Node.js, pnpm, and [Tauri's system dependencies](https://tauri.app/start/prerequisites/).

```sh
pnpm install
pnpm tauri dev      # run in development
pnpm tauri build    # build installers
cd src-tauri && cargo test
```

Dev builds read `config.dev.toml` and `language.dev` (same directory as `config.toml`), so you can use short durations or switch languages without touching the installed app. Press `Esc` on the overlay to end a break immediately. Quit the installed RestGuard first, otherwise the single-instance check makes the dev build exit right away.

For debugging, set `work_minutes` to `0.1` (6 seconds) in `config.dev.toml` to trigger a break quickly.

Build with `pnpm tauri build`, not `cargo build`: a plain Cargo build is a dev build that loads the UI from `localhost:1420` and shows blank windows.

## Project layout

| Path | What's there |
|---|---|
| `src-tauri/src/` | Rust backend: timer, overlay windows, tray, config, meeting and idle detection |
| `src-tauri/src/i18n/` | Backend text (tray, window titles, errors, config comments) |
| `src/` | Svelte frontend: overlay, settings and about pages |
| `src/lib/i18n.ts` | Frontend text |
| `site/` | The project website on GitHub Pages |
| `scripts/` | Release, MSIX packaging, Arch packaging and lint scripts |
| `docs/` | Screenshots, store listings and [release notes for maintainers](docs/RELEASING.md) |

## Before you open a pull request

- Open PRs against the **`dev`** branch. `main` only receives merges from `dev` at release time.
- Run the checks:

  ```sh
  pnpm check                    # Svelte and TypeScript
  cd src-tauri && cargo test    # Rust tests
  pnpm lint:english             # see below
  ```

- Code comments, log messages and assertion messages must be in English (`scripts/check-english.py` enforces this). User-facing text goes through the i18n tables instead.
- If you add or change user-facing text, add it for every language. If you can't translate one, use the English text there and mention it in the PR; a native speaker can fill it in later.
- If behavior changes, update `README.md` too. The translated READMEs (`README.*.md`) can lag behind; mention it in the PR and they'll be synced.
- CI builds the app on Windows, macOS and Linux for every PR. A PR that only touches docs skips the build.

## Translations

Every piece of UI text lives in one of two tables, with one field per language:

- Backend: `src-tauri/src/i18n/text.rs` (`Tr { zh, en, hi, es, ar, fr, bn, pt, de, ja, it, ko, id }`)
- Frontend: `src/lib/i18n.ts`

To improve a translation, edit the field for your language in both files and open a PR. Screenshots of the result in `pnpm tauri dev` (switch the language from the tray menu) help a lot with review. Adding a new language touches more places; open an issue first.

## License

By contributing, you agree that your contributions are dual-licensed under [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE), like the rest of the project.

Please also follow the [Code of Conduct](CODE_OF_CONDUCT.md).
