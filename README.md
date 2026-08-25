# `traces-sm` — 100% Rust-Native Multi-OS SGX Secrets & Key Management Framework

`traces-sm` is a **100% Rust-Native**, enterprise-grade Key & Secret Management Framework built on **Intel SGX using Fortanix EDP** (`x86_64-fortanix-unknown-sgx`).

It delivers full compliance with **NIST SP 800-57 / SP 800-130 / FIPS 140-3** lifecycle guidelines, featuring an in-enclave Key Generation catalog, Post-Quantum Cryptography (ML-KEM, ML-DSA, SLH-DSA), $M$-of-$N$ Threshold DKG, Zero-Knowledge Proofs (Schnorr PoK, Bulletproofs), Paillier Homomorphic Encryption, and a **Mandatory Security Policy (MSP) Engine** enforcing in-memory and in-storage cryptographic protection.

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
├── desktop/                    ← Cross-Platform Native Desktop App (Ubuntu, Windows, macOS via eframe/egui)
│   ├── Cargo.toml              ← eframe, egui, tokio, reqwest
│   └── src/main.rs             ← Native Desktop GUI App
├── gui/                        ← Rust WebAssembly Web GUI (Yew 0.21 compiled to `wasm32-unknown-unknown`)
│   ├── Cargo.toml              ← yew, gloo-net, wasm-bindgen, web-sys
│   ├── Trunk.toml              ← WASM bundler
│   └── src/                    ← Rust WASM UI components & fetch client
├── host/                       ← Rust Native Host Proxy (Axum 0.7 + Tokio + Rusqlite)
│   ├── Cargo.toml              ← axum, tokio, tower-http, rusqlite, reqwest
│   └── src/                    ← Axum HTTP server & REST proxy endpoints
├── cli/                        ← Rust Native Multi-OS CLI Tool (`traces-sm` binary)
│   ├── Cargo.toml              ← clap 4.5, tokio, reqwest, comfy-table, colored
│   └── src/                    ← Clap derive CLI parser & API client
└── enclave/                    ← Rust SGX Enclave (Fortanix EDP `x86_64-fortanix-unknown-sgx`)
    ├── Cargo.toml              ← 23 enclave Rust modules
    └── src/                    ← sealing, drbg, nist, pqc, dkg, paillier, zkp, store, auth, policy, keygen
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
```
