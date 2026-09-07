# `traces-sm` — 100% Rust-Native Multi-OS SGX Secrets & Key Management Framework

`traces-sm` is a **100% Rust-Native**, enterprise-grade Key & Secret Management Framework built on **Intel SGX using Fortanix EDP** (`x86_64-fortanix-unknown-sgx`).

It delivers full compliance with **NIST SP 800-57 / SP 800-130 / FIPS 140-3** guidelines, featuring an in-enclave Key Generation catalog, Post-Quantum Cryptography (ML-KEM, ML-DSA, SLH-DSA), $M$-of-$N$ Threshold DKG, Zero-Knowledge Proofs (Schnorr PoK, Bulletproofs), Paillier Homomorphic Encryption, and a **Mandatory Security Policy (MSP) Engine** enforcing in-memory and in-storage cryptographic protection.

---

## 🏛️ 100% Rust-Native 5-Crate Workspace Architecture

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│                               100% RUST-NATIVE `traces-sm` WORKSPACE                             │
│                                                                                                  │
│  User & Client Interfaces                                                                        │
│  ┌──────────────────────────┐  ┌──────────────────────────────┐  ┌─────────────────────────────┐  │
│  │ Rust WebAssembly Web GUI │  │ Rust Native Desktop App      │  │ Rust Native CLI Tool        │  │
│  │ (`gui/` -> Yew 0.21 WASM)│  │ (`desktop/` -> Ubuntu, Win,  │  │ (`cli/` -> Multi-OS Distros)│  │
│  │                          │  │  macOS via `eframe`/`egui`)  │  │                             │  │
│  └────────────┬─────────────┘  └──────────────┬───────────────┘  └──────────────┬──────────────┘  │
│               │                               │                                 │                │
├───────────────┼───────────────────────────────┼─────────────────────────────────┼────────────────┤
│ Host Layer (Untrusted Proxy)                  │                                 │                │
│               └───────────────────────────────┼─────────────────────────────────┘                │
│                                               │ REST / mTLS                                      │
│  ┌────────────────────────────────────────────▼───────────────────────────────────────────────┐  │
│  │ Rust Native Host Proxy (`host/` -> Axum 0.7 + Tokio + Rusqlite)                            │  │
│  │ • Serves WebAssembly Web GUI static assets on port 8080                                    │  │
│  │ • Manages SQLite metadata DB, audit logs, and DKG topology                                  │  │
│  └────────────────────────────────────────────┬───────────────────────────────────────────────┘  │
├───────────────────────────────────────────────┼──────────────────────────────────────────────────┤
│ SGX Enclave Layer (Trusted EPC)               │ Plain TCP / mTLS (Port 8443)                     │
│  ┌────────────────────────────────────────────▼───────────────────────────────────────────────┐  │
│  │ Rust SGX Enclave (`enclave/` -> Fortanix EDP `x86_64-fortanix-unknown-sgx`)               │  │
│  │ • In-Enclave Key Generation Catalog Engine (`keygen.rs`)                                   │  │
│  │ • Mandatory NIST Security Policy Engine (`policy.rs`)                                      │  │
│  │ • NIST SP 800-90A/B/C Entropy & DRBG Engine (`drbg.rs`)                                    │  │
│  │ • NIST SP 800-57 Key Lifecycle State Machine (`nist.rs`)                                   │  │
│  │ • Classic (RSA/ECDSA/Ed25519) + PQC (ML-KEM/ML-DSA) Engines (`keygen.rs`, `pqc.rs`)        │  │
│  │ • Threshold DKG (Shamir SSS / Pedersen VSS) & PHE (`dkg.rs`, `paillier.rs`)               │  │
│  │ • Zero-Knowledge Proofs (Schnorr PoK & Bulletproof Range Proofs) (`zkp/`)                  │  │
│  │ • FIPS 140-3 Zeroization & NIST SP 800-88 Crypto-Shredding (`store.rs`)                    │  │
│  └────────────────────────────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## 🔑 Key Generation & Algorithm Catalog

`traces-sm` provides native in-enclave generation, zeroization, envelope-sealing, and SP 800-57 lifecycle state management for the following cryptographic key algorithms:

| Algorithm Family | Supported Key Generation Schemes | Standard / Spec | Primary Function |
|---|---|---|---|
| **RSA** | RSA-2048, RSA-4096 | FIPS 186-5 / PKCS#1 v1.5 / OAEP | Asymmetric signing, PKCS#8 PEM export, OAEP encryption |
| **ECDSA** | P-256, P-384, P-521, Secp256k1 | FIPS 186-5 / SECG / Bitcoin-Ethereum | Elliptic curve digital signatures, ECDH key agreement |
| **Ed25519 / X25519** | Ed25519, X25519 | RFC 8032 / RFC 7748 | Fast EdDSA signatures & Montgomery curve Diffie-Hellman |
| **ML-KEM (Kyber)** | ML-KEM-512, ML-KEM-768, ML-KEM-1024 | NIST FIPS 203 (PQC KEM) | Quantum-resistant key encapsulation mechanism |
| **ML-DSA (Dilithium)**| ML-DSA-3 (ML-DSA-44), ML-DSA-5 (ML-DSA-87) | NIST FIPS 204 (PQC Signatures) | Quantum-resistant lattice digital signatures |
| **SLH-DSA (SPHINCS+)**| SLH-DSA-SHA2-128f / 256f | NIST FIPS 205 (PQC Hash-Sign) | Stateless hash-based post-quantum digital signatures |
| **AES & Key Wrap** | AES-128-GCM, AES-256-GCM, AES-KW | NIST SP 800-38D / SP 800-38F | Envelope encryption & exportable KEK payload wrapping |
| **HMAC & Stream** | HMAC-SHA256, HMAC-SHA512, ChaCha20 | FIPS 198-1 / RFC 8439 | Message authentication codes & authenticated stream encryption |
| **Threshold DKG** | Shamir SSS, Pedersen VSS, FROST | RFC 9591 / DKLs23 | $M$-of-$N$ threshold secret sharing & threshold Ed25519 sign |
| **SSL/TLS & PKI** | X.509 Cert Bundles, OpenSSH KeyPairs | RFC 5280 / OpenSSH protocol | Server/CA certificate issuance & SSH authentication |

---

## 💻 Multi-OS & Multi-Distro Distribution Matrix

| Platform / Distro | CLI Binary Target | Desktop App Target | Package Format |
|---|---|---|---|
| **Ubuntu / Debian** | `x86_64-unknown-linux-gnu` | Native GTK / `eframe` | `.deb` package (`cargo deb`) |
| **Alpine Linux** | `x86_64-unknown-linux-musl` | Static musl binary | `.apk` / Standalone binary |
| **Fedora / RHEL** | `x86_64-unknown-linux-gnu` | Native GTK / `eframe` | `.rpm` package (`cargo generate-rpm`) |
| **ARM64 / Graviton** | `aarch64-unknown-linux-gnu` | Native ARM64 GTK | Tarball archive |
| **Windows 10 / 11** | `x86_64-pc-windows-msvc` | WebView2 / `eframe` | `.msi` installer, Winget package |
| **macOS (Intel & M1/2/3)** | `x86_64-apple-darwin` / `aarch64-apple-darwin` | Native macOS App | Homebrew (`brew install traces-sm`) / `.dmg` |

---

## ⚡ Quick Start Commands

```bash
# 1. Launch Cross-Platform Native Desktop App (Ubuntu / Windows / macOS)
cd desktop && cargo run --release

# 2. Build WASM Web GUI & Start Host Proxy
cd gui && trunk build --release
cd ../host && cargo run --release

# 3. CLI Key Generation Command
cd cli && cargo run --release -- key generate --name master-key --algorithm rsa-4096
```

---

## 📚 Complete Documentation Directory (`docs/`)

- [`docs/TECHNICAL_SPECIFICATION.md`](file:///c:/Users/admin/Downloads/Secrets-Manager/docs/TECHNICAL_SPECIFICATION.md) — 10-Page Technical Specification Document
- [`docs/PRODUCT_SPECIFICATION.md`](file:///c:/Users/admin/Downloads/Secrets-Manager/docs/PRODUCT_SPECIFICATION.md) — Product Specification Document (PSD)
- [`docs/CONFORMANCE_REPORT.md`](file:///c:/Users/admin/Downloads/Secrets-Manager/docs/CONFORMANCE_REPORT.md) — Conformance & Compliance Verification Report
- [`docs/PAGE_BY_PAGE_DESIGN.md`](file:///c:/Users/admin/Downloads/Secrets-Manager/docs/PAGE_BY_PAGE_DESIGN.md) — Page-by-Page UI/UX Design Specification
- [`docs/WINDOWS_BUILD_GUIDE.md`](file:///c:/Users/admin/Downloads/Secrets-Manager/docs/WINDOWS_BUILD_GUIDE.md) — Windows Desktop Application Build Guide
- [`docs/KNOWLEDGE_BANK.md`](file:///c:/Users/admin/Downloads/Secrets-Manager/docs/KNOWLEDGE_BANK.md) — 100-Repository Cryptographic Systems Catalog
- [`docs/ARCHITECTURE_WALKTHROUGH.md`](file:///c:/Users/admin/Downloads/Secrets-Manager/docs/ARCHITECTURE_WALKTHROUGH.md) — Architecture Walkthrough
- [`docs/TASK_CHECKLIST.md`](file:///c:/Users/admin/Downloads/Secrets-Manager/docs/TASK_CHECKLIST.md) — Milestone Task Checklist
