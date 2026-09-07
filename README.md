[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/14493/badge)](https://www.bestpractices.dev/projects/14493)

# `traces-sm` — 100% Rust-Native Multi-OS SGX Secrets & Key Management Framework

`traces-sm` is a **100% Rust-Native**, enterprise-grade Key & Secret Management Framework built on **Intel SGX using Fortanix EDP** (`x86_64-fortanix-unknown-sgx`).

It delivers full compliance with **NIST SP 800-57 / SP 800-130 / FIPS 140-3** lifecycle guidelines, featuring an in-enclave Key Generation catalog, Post-Quantum Cryptography (ML-KEM, ML-DSA, SLH-DSA), $M$-of-$N$ Threshold DKG, Zero-Knowledge Proofs (Schnorr PoK, Bulletproofs), Paillier Homomorphic Encryption, and a **Mandatory Security Policy (MSP) Engine** enforcing in-memory and in-storage cryptographic protection.

---

<!-- BOUNTY_BOT_SUMMARY_START -->
## 🛡️ Security Research & Vulnerability Bounties Tracker (AI Bot)

> 🤖 **Automated Live Tracker**: Aggregating, parsing, standardizing, and publishing Bug Bounties & Vulnerability Disclosure Programs across Web2, Web3, and self-hosted security teams.

| Total Tracked Programs | Paid Bug Bounties | Unpaid VDPs | Total Reward Pool | Last Bot Sync |
| :---: | :---: | :---: | :---: | :---: |
| **15** | **10** | **5** | **$2,850,000.00** | `2026-08-26 01:23:49 UTC` |

### 🔗 Direct Data Access
* 📊 **Searchable Bounty Directory**: [`bounty_bot/README.md`](bounty_bot/README.md)
* 📄 **Master JSON Dataset**: [`bounty_bot/data/bounties.json`](bounty_bot/data/bounties.json)
* ⚡ **Minified JSON**: [`bounty_bot/data/bounties.min.json`](bounty_bot/data/bounties.min.json)
* 📂 **By Platform**: [HackerOne](bounty_bot/data/by-platform/hackerone.json) | [Immunefi (Web3)](bounty_bot/data/by-platform/immunefi.json) | [Self-Hosted VDPs](bounty_bot/data/by-platform/self_hosted.json)
<!-- BOUNTY_BOT_SUMMARY_END -->

---

## 🔑 Complete Key Generation & Management Catalog

`traces-sm` provides native in-enclave generation, zeroization, envelope-sealing, and SP 800-57 lifecycle state management for the following cryptographic key algorithms:

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│                             traces-sm KEY GENERATION & ALGORITHM CATALOG                         │
├──────────────────────────┬───────────────────────────────────────────────────────────────────────┤
│ Algorithm Family         │ Supported Key Generation Schemes                                      │
├──────────────────────────┼───────────────────────────────────────────────────────────────────────┤
│ Classic Asymmetric       │ • RSA-2048, RSA-4096 (PKCS#1 v1.5 & OAEP encryption)                 │
│                          │ • ECDSA (P-256, P-384, P-521, Secp256k1 / Koblitz curve)               │
│                          │ • Ed25519 (EdDSA signatures) & X25519 (Diffie-Hellman)                │
├──────────────────────────┼───────────────────────────────────────────────────────────────────────┤
│ Post-Quantum (PQC)       │ • ML-KEM-512, ML-KEM-768, ML-KEM-1024 (NIST FIPS 203 Kyber KEM)        │
│                          │ • ML-DSA-3 (ML-DSA-44), ML-DSA-5 (ML-DSA-87) (NIST FIPS 204 Dilithium) │
│                          │ • SLH-DSA (NIST FIPS 205 SPHINCS+ stateless hash signatures)          │
├──────────────────────────┼───────────────────────────────────────────────────────────────────────┤
│ Symmetric & Key Wrapping │ • AES-128-GCM, AES-256-GCM (Authenticated Envelope Encryption)         │
│                          │ • AES-128-KW, AES-256-KW (NIST SP 800-38F Key Wrap for KEK payloads)   │
│                          │ • HMAC-SHA256, HMAC-SHA512                                            │
│                          │ • ChaCha20-Poly1305                                                   │
├──────────────────────────┼───────────────────────────────────────────────────────────────────────┤
│ Threshold & DKG          │ • Shamir Secret Sharing (SSS over GF(256))                            │
│                          │ • Pedersen Verifiable Secret Sharing (VSS Ristretto255)               │
│                          │ • FROST Ed25519 Threshold Signatures                                  │
├──────────────────────────┼───────────────────────────────────────────────────────────────────────┤
│ SSL/TLS & PKI Certs      │ • X.509 Certificate Bundles (CA Root, Intermediate CAs, Server Certs) │
│                          │ • OpenSSH KeyPairs (RSA-4096, Ed25519, ECDSA-P256)                    │
└──────────────────────────┴───────────────────────────────────────────────────────────────────────┘
```

---

## 🛠️ Multi-Crate Workspace Layout

```
Secrets-Manager/
├── Cargo.toml                  ← Root Workspace Manifest ([workspace] members = ["enclave", "host", "gui", "cli", "desktop"])
├── bounty_bot/                 ← AI Bot for Vulnerability Bounties & Security Research Aggregation
│   ├── data/                   ← Master & platform-partitioned JSON datasets (bounties.json)
│   ├── src/                    ← Discovery engine, Gemini policy parser, deduplicator, generators
│   └── README.md               ← Complete searchable bug bounty directory
├── desktop/                    ← Cross-Platform Native Desktop App (Ubuntu, Windows, macOS via eframe/egui)
├── gui/                        ← Rust WebAssembly Web GUI (Yew 0.21 compiled to `wasm32-unknown-unknown`)
├── host/                       ← Rust Native Host Proxy (Axum 0.7 + Tokio + Rusqlite)
├── cli/                        ← Rust Native Multi-OS CLI Tool (`traces-sm` binary)
└── enclave/                    ← Rust SGX Enclave (Fortanix EDP `x86_64-fortanix-unknown-sgx`)
```

---

## ⚡ Quick Start

```bash
# 1. Run Desktop App (Ubuntu / Windows / macOS)
cd desktop && cargo run --release

# 2. Build WASM GUI & Host
cd gui && trunk build --release
cd ../host && cargo run --release

# 3. CLI Key Generation Command
cd cli && cargo run --release -- key generate --name master-key --algorithm rsa-4096

# 4. Run Security Bounty AI Bot
cd bounty_bot && uv run python -m src.main
```

---

## 🧪 Automated FLOSS Test Suites

`traces-sm` includes comprehensive, publicly accessible automated test suites released under Free/Libre and Open Source Software (FLOSS) licenses (Apache-2.0 / MIT). The complete test suite runs automatically on every push and pull request via GitHub Actions ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)).

### How to Run the Automated Tests

```bash
# 1. Run all Rust workspace unit, integration, and security tests (FLOSS)
cargo test --workspace --locked -- --nocapture

# 2. Run specific crate tests (e.g. SGX enclave cryptographic & zeroization tests)
cargo test -p traces-sm-enclave -- --nocapture

# 3. Run Python Bounty Bot test suite (pytest)
cd bounty_bot && uv sync && uv run pytest

# 4. Run automated code formatting & linter checks
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

📖 For complete build instructions, test target breakdowns, and CI details, see [**`BUILD.md`**](BUILD.md).

---

## 🤝 Contributing & Monthly Contributor Reward

We welcome all contributions to `traces-sm`!

### 🌐 1. Visit ttraces.io & Join Community Discussion
Before writing code or opening PRs, please visit **[ttraces.io](https://ttraces.io)** and join our **[Discord](https://discord.gg/traces)** to participate in community discussions and align on design goals with the team.

### 🎁 2. Monthly $100 in BTC Contributor Reward
To give back to our community, **one lucky GitHub contributor wins $100 in Bitcoin (BTC) every month!**
- **How to Enter**: Submit a Pull Request that gets reviewed and merged into `main` during that month.
- **Selection**: 1 lucky contributor is drawn on the 1st of every month and notified to receive $100 in BTC.
- **Fair Play**: Meaningful contributions (bug fixes, features, docs, tests, cryptographic improvements) qualify.

### 🛠️ 3. Quick Contribution Steps
1. Fork the repo and create your branch (`git checkout -b feat/my-feature`).
2. Adhere to Rust coding standards: run `cargo fmt --all -- --check`, `cargo clippy`, and `cargo test`.
3. Submit a Pull Request referencing the community discussion.
4. **Planned Checkpoints**: At planned checkpoints, the branches are merged to ensure unstable, unpublished versions are accessible for testing and improvements.

📖 For complete details, see [**`CONTRIBUTING.md`**](https://github.com/arjun-traces/traces-sm/CONTRIBUTING.md).

---

## 💖 Open Donation

If you find `traces-sm` useful and want to support ongoing development, research, and infrastructure, open donations are gratefully accepted:

* **Solana (SOL) Address**:
  ```text
  12D1qoP13upaB6AffHhvcpzBMwZkUGDAsYiNmJA5Jqsa
  ```

---

## 🐛 Reporting Issues

If you encounter any bugs, security anomalies, performance bottlenecks, or unexpected behavior, please report them directly on our issue tracker:

👉 **[Submit or Browse Issues on GitHub](https://github.com/arjun-traces/traces-sm/issues)**

When creating an issue:
1. Search existing open and closed issues to avoid duplicates.
2. Provide a clear title, reproduction steps, environment details (OS, Rust version), and relevant error outputs.
3. For confidential or security-critical reports, coordinate via maintainers on [Discord](https://discord.gg/traces).

---

## 📜 Project Badge & License

Project badge entry owned by: [boosters-research](https://www.bestpractices.dev/en/users/56453).Entry created on 2026-09-07 03:59:29 UTC, last updated on 2026-09-07 09:07:03 UTC.
This data is available under the [Community Data License Agreement – Permissive, Version 2.0 (CDLA-Permissive-2.0)](https://cdla.dev/permissive-2-0/). The code is licensed under Apache.
