# Changelog

All notable changes to the `traces-sm` project will be documented in this file.

## [0.1.0] - 2026-08-25
### Added
- Fortanix EDP Intel SGX Enclave workspace (`enclave/`).
- AES-256-GCM envelope sealing with HKDF-SHA256 key derivation.
- NIST SP 800-90A/B HMAC_DRBG with continuous APT & RCT health testing.
- NIST SP 800-57 4-phase key lifecycle state machine.
- Mandatory Security Policy Engine (`policy.rs`).
- In-enclave Key Generation catalog (RSA, ECDSA, Ed25519).
- Paillier homomorphic encryption & Schnorr PoK / Bulletproof range proofs.
- Rust Native Axum host proxy (`host/`).
- Rust WebAssembly Web GUI (`gui/`) with Traces AI Anthropic chatbot integration.
- Cross-platform Native Desktop App (`desktop/` via eframe/egui).
- Multi-OS Clap CLI (`cli/`) with ASCII Key Art Banner.
- GitHub Pages site and multi-distro build scripts.
