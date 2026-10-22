# Usage: pnpm msix [-SkipBuild]
# Builds the MSIX package for Microsoft Store submission: src-tauri/target/msix/RestGuard_<version>_x64.msix.
# No need to sign it ourselves; the Store signs it after upload to Partner Center
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)

$version = (Get-Content src-tauri/tauri.conf.json -Raw | ConvertFrom-Json).version
# The Store requires a four-part version ending in 0 and doesn't support pre-release suffixes
if ($version -notmatch '^\d+\.\d+\.\d+$') {
    throw "MSIX packages need a stable version, got '$version'"
}

# Use makeappx.exe from the latest installed Windows SDK
$makeappx = Get-ChildItem "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\makeappx.exe" -ErrorAction SilentlyContinue |
    Sort-Object { [version]$_.Directory.Parent.Name } |
    Select-Object -Last 1
if (-not $makeappx) { throw "makeappx.exe not found, install the Windows SDK" }

if (-not $SkipBuild) {
    pnpm tauri build --no-bundle
    if ($LASTEXITCODE -ne 0) { throw "tauri build failed" }
}

$out = "src-tauri/target/msix"
$layout = "$out/layout"
if (Test-Path $layout) { Remove-Item $layout -Recurse -Force }
New-Item -ItemType Directory "$layout/Assets" | Out-Null

Copy-Item src-tauri/target/release/restguard.exe $layout
foreach ($logo in "StoreLogo", "Square44x44Logo", "Square71x71Logo", "Square150x150Logo") {
    Copy-Item "src-tauri/icons/$logo.png" "$layout/Assets"
}
(Get-Content src-tauri/msix/AppxManifest.xml -Raw).Replace('$VERSION$', "$version.0") |
    Set-Content "$layout/AppxManifest.xml" -NoNewline

$msix = "$out/RestGuard_${version}_x64.msix"
& $makeappx.FullName pack /d $layout /p $msix /o
if ($LASTEXITCODE -ne 0) { throw "makeappx failed" }

Write-Host ""
Write-Host "Created $msix"
Write-Host "To test locally (needs Developer Mode): Add-AppxPackage -Register $layout/AppxManifest.xml"
