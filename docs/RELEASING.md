# Releasing RestGuard

Notes for maintainers. Contributors don't need any of this; see [CONTRIBUTING.md](../CONTRIBUTING.md).

## Cutting a release

1. Add the changes to the `Unreleased` section of [CHANGELOG.md](../CHANGELOG.md), then rename it to the new version and date.
2. On a clean `dev` branch, run `pnpm release <version>` (e.g. `pnpm release 0.2.0`).

The script bumps the version in `package.json`, `tauri.conf.json`, `Cargo.toml` and `Cargo.lock`, commits, and pushes a `v<version>` tag. GitHub Actions then:

- builds the exe, a universal macOS `.app`, and the Linux `.deb`, `.rpm` and Arch `.pkg.tar.zst` (repackaged from the `.deb` with `src-tauri/arch/PKGBUILD`), and creates the Release
- pushes the updated `bucket/restguard.json` to [HansTanzi/scoop-bucket](https://github.com/HansTanzi/scoop-bucket)
- pushes `Casks/restguard.rb` (the macOS `.app`, and on Linux the executable from the Arch package) to [HansTanzi/homebrew-tap](https://github.com/HansTanzi/homebrew-tap)
- for stable versions, triggers [HansTanzi/apt](https://github.com/HansTanzi/apt) to rebuild the APT repository from the latest releases
- for stable versions, opens a PR to [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs) via [winget-releaser](https://github.com/vedantmgoyal9/winget-releaser)
- for stable versions, builds the Microsoft Store MSIX (see below)

## Repository secrets

| Secret | Used for |
|---|---|
| `SCOOP_BUCKET_DEPLOY_KEY` | Private half of a deploy key with write access on `HansTanzi/scoop-bucket` |
| `HOMEBREW_TAP_DEPLOY_KEY` | Private half of a deploy key with write access on `HansTanzi/homebrew-tap` |
| `APT_DISPATCH_TOKEN` | Fine-grained PAT limited to `HansTanzi/apt` with only Actions read and write. The signing key and setup are described in that repo |
| `WINGET_TOKEN` | Classic PAT with `public_repo` scope; also needs a fork of `winget-pkgs` under the token owner's account |

The very first WinGet version must be submitted by hand, e.g. `komac new HansTanzi.RestGuard --version <version> --urls <exe release URL>` with [Komac](https://github.com/russellbanks/Komac) (pick the `portable` installer type and add `Microsoft.EdgeWebView2Runtime` as a package dependency).

## Microsoft Store

The Store build is an MSIX package; the Store signs it, so no code-signing certificate is needed. Run `pnpm msix` to build `src-tauri/target/msix/RestGuard_<version>_x64.msix` locally (needs the Windows SDK); the release workflow also builds it for stable versions and attaches it to the workflow run as the `msix` artifact. Upload it to a new submission in [Partner Center](https://partner.microsoft.com/dashboard). Store listing texts are in [`docs/store/`](store/).

One-time setup: reserve the app name in Partner Center, then copy `Package/Identity/Name`, `Package/Identity/Publisher` and `Package/Properties/PublisherDisplayName` from the app's **Product identity** page into `src-tauri/msix/AppxManifest.xml`. The package declares the restricted `runFullTrust` capability (required for desktop apps), so the submission asks for a justification, e.g. "Desktop app built with Tauri; needs full trust to show always-on-top break overlays on every screen".

Inside the package, start-at-login uses the manifest's `StartupTask` instead of the registry, since registry writes are virtualized there. To try the package without the Store, enable Developer Mode and run `Add-AppxPackage -Register src-tauri/target/msix/layout/AppxManifest.xml`.
