# Secrets Manager — Task List (100% Rust-Native Multi-OS)

## Phase 1: Scaffold & Workspace Structure
- [x] Multi-crate root Cargo.toml (`enclave`, `host`, `gui`, `cli`, `desktop`)
- [x] enclave/Cargo.toml (Fortanix EDP target `x86_64-fortanix-unknown-sgx`)
- [x] host/Cargo.toml (Axum 0.7 + Tokio + Rusqlite)
- [x] gui/Cargo.toml (Yew 0.21 WebAssembly `wasm32-unknown-unknown`)
- [x] cli/Cargo.toml (Clap 4.5 derive CLI binary `traces-sm`)
- [x] desktop/Cargo.toml (Cross-platform native GUI for Ubuntu, Windows, macOS via `eframe`/`egui`)

## Phase 2: Cryptographic Core & NIST Policy Framework (Rust Enclave)
- [x] enclave/src/policy.rs         — Mandatory Security Policy Engine (In-Memory & In-Storage NIST enforcement)
- [x] enclave/src/sealing.rs        — EGETKEY + HKDF-SHA256 + AES-256-GCM seal/unseal
- [x] enclave/src/crypto.rs         — Envelope encryption (per-secret random DEK)
- [x] enclave/src/keygen.rs         — RSA-4096, ECDSA P-256/P-384/Secp256k1, Ed25519 gen+sign+verify
- [x] enclave/src/drbg.rs           — NIST SP 800-90A HMAC_DRBG + SP 800-90B APT & RCT health tests
- [x] enclave/src/nist.rs           — NIST SP 800-57 Key Lifecycle State Machine & SP 800-108 PRF KDF
- [x] enclave/src/pqc.rs            — Post-Quantum Cryptography (ML-KEM-768/1024 & ML-DSA-3/5)
- [x] enclave/src/dkg.rs            — Shamir Secret Sharing (SSS) & Pedersen Verifiable Secret Sharing ($M$-of-$N$)

## Phase 3: ZKP & Homomorphic Encryption Engine (Rust Enclave)
- [x] enclave/src/zkp/schnorr.rs    — Schnorr PoK (Ristretto255, schnorrkel crate)
- [x] enclave/src/zkp/pedersen.rs   — Pedersen commitments (additive HE, curve25519-dalek)
- [x] enclave/src/zkp/bulletproof.rs — Range proofs (bulletproofs crate, no trusted setup)
- [x] enclave/src/he/paillier.rs    — Full Paillier PHE: keygen, enc, dec, add, mul, rerandomize

## Phase 4: Rust Native Host Proxy (`host/`)
- [x] host/src/main.rs         — Axum 0.7 server with Tokio, CORS, static WASM file serving
- [x] host/src/db.rs           — Rusqlite SQLite metadata DB
- [x] host/src/routes/         — Router modules for secrets, keys, lifecycle, dkg, entropy

## Phase 5: Rust WebAssembly Web GUI (`gui/`)
- [x] gui/Cargo.toml, Trunk.toml, index.html
- [x] gui/src/lib.rs, main.rs  — Yew 0.21 application
- [x] gui/src/api.rs         — gloo-net WASM HTTP fetch client
- [x] gui/src/components/    — Yew components: header, dashboard, lifecycle, topology, entropy, zkp_sandbox

## Phase 6: Cross-Platform Native Desktop App (`desktop/`)
- [x] desktop/Cargo.toml, src/main.rs — eframe/egui native Desktop UI for Ubuntu, Windows, and macOS

## Phase 7: Multi-OS Native CLI (`cli/`)
- [x] cli/Cargo.toml          — Clap 4.5 derive binary `traces-sm`
- [x] cli/src/main.rs         — Subcommands: `secret`, `key`, `token`, `attest`, `zkp`, `lifecycle`, `dkg`, `entropy`
- [x] cli/src/client.rs       — Reqwest HTTP client
- [x] scripts/package_distros.sh — Multi-OS packaging script (.deb, .rpm, brew, winget, musl)
