# 🏗️ Build & Automated Testing Guide

This document provides complete instructions for compiling `traces-sm` from source and executing its **Free/Libre and Open Source Software (FLOSS)** automated test suites.

---

## 📋 Prerequisites

### 1. Rust Toolchain (FLOSS)
- **Rust Stable** (1.75+): Install via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup component add rustfmt clippy
  ```

### 2. System Dependencies (Linux / Ubuntu)
```bash
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev libx11-dev libxcb-render0-dev \
  libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libgtk-3-dev
```

### 3. Python & uv (for Bounty Bot test suite)
```bash
# Install uv (FLOSS package manager)
curl -LsSf https://astral.sh/uv/install.sh | sh
```

---

## 🧪 Automated FLOSS Test Suites

`traces-sm` employs 100% Free/Libre and Open Source (FLOSS) test harnesses. The complete test suite is automated and runs on every commit and pull request via GitHub Actions ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)).

### 1. Run Complete Rust Workspace Test Suite
To execute all unit, integration, and conformance tests across all crates (`enclave`, `host`, `cli`, `gui`, `desktop`):

```bash
# Run all workspace unit and integration tests
cargo test --workspace --locked

# Run tests with real-time output and logs
cargo test --workspace -- --nocapture
```

### 2. Run Crate-Specific Test Suites

#### A. SGX Enclave Cryptographic & Sealing Tests
```bash
# Tests AES-256-GCM sealing, zeroization, HKDF, FROST threshold DKG, ZKP & Paillier PHE
cargo test -p traces-sm-enclave -- --nocapture
```

#### B. Host Service & SQLite Persistence Tests
```bash
# Tests Axum REST endpoints, WAL mode concurrency, and audit log tables
cargo test -p traces-sm-host -- --nocapture
```

#### C. CLI Tooling Tests
```bash
# Tests argument parsing, subcommands, and client API communication
cargo test -p traces-sm-cli -- --nocapture
```

### 3. Run Python Bounty Bot Test Suite (`pytest`)
The vulnerability discovery bot test suite uses the standard FLOSS `pytest` framework:

```bash
cd bounty_bot
uv sync
uv run pytest -v
```

---

## 🔍 Linting & Code Quality Checks

All code submitted to `traces-sm` must pass standard FLOSS automated static analysis:

```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Clippy static analysis (zero warnings)
cargo clippy --workspace --all-targets --all-features -- -D warnings

# 3. Supply-chain security vulnerability audit
cargo audit --deny warnings
```

---

## ⚙️ Continuous Integration (CI)

Our automated CI pipeline is publicly defined in [`.github/workflows/ci.yml`](.github/workflows/ci.yml) and performs the following automated gates on Ubuntu, Windows, and macOS runners:
- `cargo build --workspace --all-targets --locked`
- `cargo test --workspace --locked -- --nocapture`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `pytest` suite for `bounty_bot`
