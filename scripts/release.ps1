# Usage: pnpm release 0.2.0
# Bump versions -> commit -> tag -> push; .github/workflows/release.yml does the build and release
param(
    [Parameter(Mandatory)]
    [string]$Version
)

$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)

function Invoke-Git {
    git @args
    if ($LASTEXITCODE -ne 0) { throw "git $args failed" }
}

$Version = $Version.TrimStart("v")
if ($Version -notmatch '^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$') {
    throw "Invalid version '$Version', expected e.g. 0.2.0"
}
$tag = "v$Version"

$branch = git rev-parse --abbrev-ref HEAD
if ($branch -ne "dev") { throw "Releases must be made from dev (current: $branch)" }
if (git status --porcelain) { throw "Working tree is not clean" }

Invoke-Git fetch origin --tags
Invoke-Git pull --ff-only
if (git tag --list $tag) { throw "Tag $tag already exists" }

# Replace only the version field text, preserving the file's formatting
$targets = @(
    @{ Path = "package.json";              Pattern = '(?m)^(\s*"version":\s*")[^"]+' }
    @{ Path = "src-tauri/tauri.conf.json"; Pattern = '(?m)^(\s*"version":\s*")[^"]+' }
    @{ Path = "src-tauri/Cargo.toml";      Pattern = '(?m)^(version\s*=\s*")[^"]+' }
    @{ Path = "src-tauri/Cargo.lock";      Pattern = '(name = "restguard"\r?\nversion = ")[^"]+' }
)
foreach ($t in $targets) {
    $content = Get-Content $t.Path -Raw
    $regex = [regex]$t.Pattern
    if (-not $regex.IsMatch($content)) { throw "Version field not found in $($t.Path)" }
    $regex.Replace($content, "`${1}$Version", 1) | Set-Content $t.Path -NoNewline
    Write-Host "Updated $($t.Path)"
}

Invoke-Git add $targets.Path
Invoke-Git commit -m "release: $tag"
Invoke-Git tag $tag
Invoke-Git push --atomic origin HEAD $tag

Write-Host ""
Write-Host "Pushed $tag. GitHub Actions will build the release, and update HansTanzi/scoop-bucket and HansTanzi/homebrew-tap (and open a winget-pkgs PR for stable versions)."
