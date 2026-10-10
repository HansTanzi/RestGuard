# 用法: pnpm msix [-SkipBuild]
# 生成提交微软商店用的 MSIX 包 src-tauri/target/msix/RestGuard_<version>_x64.msix。
# 包不需要自己签名，上传到合作伙伴中心后由商店签名
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)

$version = (Get-Content src-tauri/tauri.conf.json -Raw | ConvertFrom-Json).version
# 商店要求四段版本号且最后一段为 0，不支持预发布后缀
if ($version -notmatch '^\d+\.\d+\.\d+$') {
    throw "MSIX packages need a stable version, got '$version'"
}

# 取已安装的最新版 Windows SDK 中的 makeappx.exe
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
