# Builds release binaries into .\dist
# Needs: rustup (x86_64-pc-windows-gnu host) and MinGW-w64 (WinLibs) in %USERPROFILE%\mingw64.
param([switch]$Debug, [switch]$Test)
$ErrorActionPreference = "Stop"

$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:USERPROFILE\mingw64\bin;$env:PATH"
foreach ($tool in "cargo", "gcc", "dlltool") {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) { throw "$tool not found on PATH; see README 'Building'" }
}
Set-Location $PSScriptRoot

if ($Test) { cargo test --workspace; if ($LASTEXITCODE) { exit $LASTEXITCODE } }

$profile = if ($Debug) { "debug" } else { "release" }
$flag = if ($Debug) { @() } else { @("--release") }
cargo build --workspace @flag
if ($LASTEXITCODE) { exit $LASTEXITCODE }

New-Item -ItemType Directory -Force dist | Out-Null
foreach ($bin in "cyberspyde-agent", "cyberspyde-relay", "cyberspyde-sim") {
    Copy-Item "target\$profile\$bin.exe" "dist\$bin.exe" -Force
}
Copy-Item "examples\builds.override.example.json" "dist\builds.override.example.json" -Force
Get-ChildItem dist | Format-Table Name, Length
