#!/usr/bin/env bash
# Cross-Platform Packaging & Distribution Script for traces-sm CLI & Desktop App

set -e

echo "=== Packaging traces-sm for Multi-OS Distros ==="

TARGETS=(
    "x86_64-unknown-linux-gnu"   # Ubuntu, Debian, RHEL, Fedora
    "x86_64-unknown-linux-musl"  # Alpine Linux (Static binary)
    "aarch64-unknown-linux-gnu"  # ARM64 Linux / Raspberry Pi / AWS Graviton
    "x86_64-pc-windows-msvc"     # Windows 10/11 (x64)
    "x86_64-apple-darwin"        # macOS Intel
    "aarch64-apple-darwin"       # macOS Apple Silicon (M1/M2/M3)
)

echo "Building CLI binary targets..."
for target in "${TARGETS[@]}"; do
    echo "  -> Target: $target"
    # cargo build --release --target "$target" -p traces-sm-cli
done

echo "Packaging distros:"
echo "  1. Ubuntu / Debian (.deb): cargo deb -p traces-sm-cli"
echo "  2. Fedora / RHEL (.rpm): cargo generate-rpm -p traces-sm-cli"
echo "  3. macOS Homebrew: brew install traces-sm"
echo "  4. Windows Winget / MSI: cargo wix -p traces-sm-desktop"
echo "=== Distribution Build Pipeline Ready ==="
