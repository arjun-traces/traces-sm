# Windows Build Script for traces-sm-desktop

Write-Host "=== Building traces-sm Desktop App for Windows (x86_64-pc-windows-msvc) ===" -ForegroundColor Cipher

# 1. Check Rust installation
$rustVersion = rustc --version
Write-Host "Rust Compiler: $rustVersion" -ForegroundColor Green

# 2. Build Release Binary
Set-Location -Path "$PSScriptRoot\..\desktop"
Write-Host "Compiling desktop crate..." -ForegroundColor Yellow
cargo build --release --target x86_64-pc-windows-msvc

$exePath = "$PSScriptRoot\..\target\x86_64-pc-windows-msvc\release\traces-sm-desktop.exe"
if (-not (Test-Path $exePath)) {
    $exePath = "$PSScriptRoot\..\target\release\traces-sm-desktop.exe"
}

if (Test-Path $exePath) {
    Write-Host "SUCCESS! Windows Desktop Executable built at:" -ForegroundColor Green
    Write-Host "  $exePath" -ForegroundColor Cyan
} else {
    Write-Host "Build failed. Ensure Visual Studio C++ Build Tools are installed." -ForegroundColor Red
}
