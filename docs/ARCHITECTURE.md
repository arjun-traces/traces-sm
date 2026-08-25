# `traces-sm` — 100% Rust-Native Architecture Walkthrough

## 1. Architectural Principles & Overview

The **`traces-sm`** framework is engineered 100% in Rust across all 5 workspace crates (`enclave`, `host`, `gui`, `cli`, `desktop`). It enforces an absolute boundary between untrusted host environments and the hardware-encrypted **Intel SGX Enclave Page Cache (EPC)**.

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

## 2. Detailed Crate-by-Crate Breakdown

### 2.1 `enclave/` (`traces-sm-enclave`)
- **Target**: `x86_64-fortanix-unknown-sgx`
- **Key Modules**:
  - `sealing.rs`: HKDF-SHA256 + AES-256-GCM hardware sealing (`EGETKEY`).
  - `drbg.rs`: NIST SP 800-90A HMAC_DRBG + SP 800-90B continuous Repetition Count Test (RCT) & Adaptive Proportion Test (APT).
  - `nist.rs`: NIST SP 800-57 4-phase key lifecycle state machine (`PreOperational`, `Operational`, `Deactivated`, `Expired`, `Revoked`, `Destroyed`).
  - `policy.rs`: Mandatory Security Policy (MSP) Engine enforcing in-memory zeroization, EPC page encryption, SP 800-38F key wrapping, and SP 800-88 crypto-shredding.
  - `keygen.rs`: Full key generation catalog (RSA-2048/4096, ECDSA, Ed25519, ML-KEM, ML-DSA, AES-KW).
  - `pqc.rs`: Post-Quantum Cryptography (ML-KEM-512/768/1024, ML-DSA-3/5, SLH-DSA).
  - `dkg.rs`: Shamir Secret Sharing (SSS) & Pedersen Verifiable Secret Sharing (VSS).
  - `zkp/`: Schnorr Proof-of-Knowledge (Ristretto255) and Bulletproofs 32-bit range proofs.
  - `he/paillier.rs`: Paillier 2048-bit partially homomorphic encryption.
  - `store.rs`: Sealed payload store and NIST SP 800-88 random noise sector overwriting.

### 2.2 `host/` (`traces-sm-host`)
- **Target**: `x86_64-unknown-linux-gnu` / Native Host
- **Key Modules**:
  - `main.rs`: Axum 0.7 HTTP server with Tokio runtime, CORS middleware, and static asset serving (`/` $\to$ `gui/dist`).
  - `db.rs`: Rusqlite SQLite database initializer managing `secrets_metadata`, `audit_logs`, `dkg_nodes`, `entropy_audits`.
  - `routes/`: REST proxy routers forwarding `/v1/...` requests to enclave HTTPS port `8443`.

### 2.3 `gui/` (`traces-sm-gui`)
- **Target**: `wasm32-unknown-unknown`
- **Key Modules**:
  - `lib.rs`, `main.rs`: Yew 0.21 single-page WebAssembly application.
  - `api.rs`: `gloo-net` async fetch client for REST communication.
  - `components/`: Yew component views (`header.rs`, `dashboard.rs`, `lifecycle.rs`, `topology.rs`, `entropy.rs`, `zkp_sandbox.rs`, `traces_ai.rs`).

### 2.4 `desktop/` (`traces-sm-desktop`)
- **Target**: Native Desktop Executables (`Ubuntu GTK`, `Windows 10/11 MSVC`, `macOS WKWebView`)
- **Key Modules**:
  - `main.rs`: `eframe 0.28` / `egui 0.28` desktop application rendering sidebar navigation, status bars, and the **Traces AI Assistant Panel**.

### 2.5 `cli/` (`traces-sm`)
- **Target**: Multi-OS Cross-Compiled CLI Binaries
- **Key Modules**:
  - `main.rs`: `clap 4.5` derive parser with subcommands `secret`, `key`, `lifecycle`, `dkg`, `entropy`, `zkp`, `attest`.
  - `client.rs`: `reqwest 0.12` HTTP API client.

---

## 3. End-to-End Request Data Flow

```
User Action (CLI / Web / Desktop)
       │
       ▼
Axum Host Proxy (Port 8080) ──► Validate Rate Limits & Audit Log
       │
       ▼ (mTLS / Port 8443)
SGX Enclave EPC Boundary ────► Mandatory Policy Engine (`policy.rs`)
       │                         • Check FIPS Zeroization & Cryptoperiod Limits
       ▼
Enclave Cryptographic Core ──► Generate Key / Perform Crypto / Seal Payload
       │                         • Memory Zeroization (`zeroize::Zeroizing<T>`)
       ▼
Sealed File Storage ──────────► Write {id}.meta.json & {id}.blob (AES-256-GCM)
```
