#!/usr/bin/env bash
# Repackages the .deb as an Arch package using src-tauri/arch/PKGBUILD, written to the current directory as
# RestGuard_<version>_x86_64.pkg.tar.zst. Usage: bash scripts/arch-package.sh <version>
# Requires running pnpm tauri build --bundles deb first, plus Docker (makepkg runs in an archlinux container)
set -euo pipefail

version="$1"
root=$(cd "$(dirname "$0")/.." && pwd)
dir=$(mktemp -d)

cp "$root/src-tauri/arch/PKGBUILD" "$root/LICENSE-MIT" "$root/LICENSE-APACHE" "$dir/"
cp "$root/src-tauri/target/release/bundle/deb/RestGuard_${version}_amd64.deb" "$dir/restguard.deb"
# pkgver can't contain -. Removing it turns 0.2.0-beta.1 into 0.2.0beta.1, which pacman considers older than 0.2.0
sed -i "s/^pkgver=.*/pkgver=${version//-/}/" "$dir/PKGBUILD"

# makepkg refuses to run as root, so use a regular user, builder; --nodeps skips installing dependencies and only packages.
# Afterwards hand the directory back to the current user, otherwise the output can't be read outside the container
docker run --rm -v "$dir:/build" -e HOST_UID="$(id -u)" archlinux:base-devel bash -euc '
  useradd -m builder
  chown -R builder /build
  cd /build
  sudo -u builder makepkg --nodeps
  chown -R "$HOST_UID" /build
'

pkgs=("$dir"/*.pkg.tar.zst)
if [ "${#pkgs[@]}" != 1 ]; then
  echo "::error::Expected exactly one Arch package, found: ${pkgs[*]}"
  exit 1
fi
cp "${pkgs[0]}" "RestGuard_${version}_x86_64.pkg.tar.zst"
