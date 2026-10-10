#!/usr/bin/env bash
# 用 src-tauri/arch/PKGBUILD 把 .deb 重新打包为 Arch 包，输出到当前目录的
# RestGuard_<版本>_x86_64.pkg.tar.zst。用法：bash scripts/arch-package.sh <版本>
# 需要先运行 pnpm tauri build --bundles deb，以及 Docker（在 archlinux 容器里运行 makepkg）
set -euo pipefail

version="$1"
root=$(cd "$(dirname "$0")/.." && pwd)
dir=$(mktemp -d)

cp "$root/src-tauri/arch/PKGBUILD" "$root/LICENSE-MIT" "$root/LICENSE-APACHE" "$dir/"
cp "$root/src-tauri/target/release/bundle/deb/RestGuard_${version}_amd64.deb" "$dir/restguard.deb"
# pkgver 不能含 -。去掉后 0.2.0-beta.1 变为 0.2.0beta.1，pacman 认为它比 0.2.0 旧
sed -i "s/^pkgver=.*/pkgver=${version//-/}/" "$dir/PKGBUILD"

# makepkg 不允许以 root 运行，用普通用户 builder；--nodeps 不安装依赖，只打包。
# 结束后把目录归还给当前用户，否则容器外读不到产物
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
