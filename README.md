# `traces-sm` — 100% Rust-Native SGX Secrets & Key Management Framework

[![CI](https://github.com/arjun-traces/traces-sm/actions/workflows/ci.yml/badge.svg)](https://github.com/arjun-traces/traces-sm/actions/workflows/ci.yml)
[![GitHub Pages](https://github.com/arjun-traces/traces-sm/actions/workflows/deploy-pages.yml/badge.svg)](https://arjun-traces.github.io/traces-sm/)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![NIST SP 800-57 Aligned](https://img.shields.io/badge/NIST%20SP%20800--57-Aligned-emerald.svg)](docs/STANDARDS_MAPPING.md)

> `traces-sm` is an open-source key and secret management framework that runs its cryptographic operations inside an Intel SGX enclave, written entirely in Rust on Fortanix EDP.

---

## 🛑 What It Is / What It Is Not

### What It Is:
- A **100% Rust-Native 5-crate workspace** (`enclave`, `host`, `gui`, `cli`, `desktop`) providing hardware memory encryption for keys and secrets.
- An **NIST SP 800-57 aligned lifecycle manager** featuring 4-phase state transitions, volume cryptoperiod meters, and FIPS 140-3 in-memory zeroization (`zeroize`).
- A multi-interface platform supporting WebAssembly browser dashboards, native desktop consoles (Ubuntu, Windows, macOS), and multi-OS CLI tools.

### What It Is Not:
- ❌ **Not NIST CMVP Certified**: `traces-sm` implements FIPS design patterns, but holds **no official NIST CMVP certificate number**.
- ❌ **Not Externally Audited**: This codebase has not undergone a third-party cryptographic security audit.
- ❌ **Not a Cloud SaaS Replacement**: It is a self-hosted framework for confidential computing infrastructure.

---

## 🔒 Security & Maturity Disclaimers

> **Security Notice**: `traces-sm` is research-grade software. While cryptographic primitives execute within an Intel SGX Enclave Page Cache (EPC) using `zeroize` memory scrubbing, experimental features (such as PQC stubs) are strictly gated with `NotImplemented` errors until formal integration. Use in production at your own risk following thorough auditing.

---

## 📊 Technical Comparison Matrix

| Feature | `traces-sm` | HashiCorp Vault / OpenBao | Fortanix DSM | AWS CloudHSM |
|---|---|---|---|---|
| **Trust Boundary** | Intel SGX EPC Enclave | Host OS RAM | Intel SGX Enclave | Dedicated FIPS L3 HSM |
| **Language Stack** | 100% Rust | Go | C / Rust / Java | Proprietary Firmware |
| **Host Introspection Defense** | ✅ Memory Encrypted | ❌ RAM Vulnerable | ✅ Memory Encrypted | ✅ Hardware Isolated |
| **PQC Algorithms** | ⚠️ Experimental | ❌ None | ⚠️ Add-on | ❌ None |
| **Zero-Knowledge Proofs** | ✅ Schnorr & Bulletproofs | ❌ None | ❌ None | ❌ None |
| **NIST CMVP Cert** | ❌ Self-Aligned | ⚠️ Enterprise FIPS | ✅ FIPS 140-2 L3 | ✅ FIPS 140-2 L3 |

---

## ⚡ 60-Second Quickstart

```bash
# 1. Clone the repository
git clone https://github.com/arjun-traces/traces-sm.git
cd traces-sm

# 2. Launch Cross-Platform Desktop App (Ubuntu / Windows / macOS)
cd desktop && cargo run --release

# 3. Or Build & Serve WebAssembly GUI + Axum Host
cd ../gui && trunk build --release
cd ../host && cargo run --release
```

---

## 🔑 Key Generation & Algorithm Catalog

| Algorithm Family | Supported Key Generation Schemes | Status | Standard / Spec |
|---|---|---|---|
| **RSA** | RSA-2048, RSA-4096 | ✅ Implemented | FIPS 186-5 / OAEP |
| **ECDSA** | P-256, P-384, P-521, Secp256k1 | ✅ Implemented | FIPS 186-5 / SECG |
| **Ed25519 / X25519** | Ed25519, X25519 | ✅ Implemented | RFC 8032 / RFC 7748 |
| **Post-Quantum (PQC)**| ML-KEM-768/1024, ML-DSA-3/5, SLH-DSA | ⚠️ Experimental | FIPS 203 / 204 / 205 |
| **Symmetric & Key Wrap**| AES-128/256-GCM, AES-KW | ✅ Implemented | SP 800-38D / SP 800-38F |
| **Threshold DKG** | Shamir SSS, Pedersen VSS, FROST | ⚠️ Partial | RFC 9591 / DKLs23 |
| **SSL/TLS & PKI** | X.509 Cert Bundles, OpenSSH KeyPairs | ✅ Implemented | RFC 5280 / OpenSSH |

---

## ❓ Frequently Asked Questions (FAQ)

### Is there an open-source secrets manager that runs inside an Intel SGX enclave?
Yes. `traces-sm` is an open-source, 100% Rust-native key and secret management framework that isolates cryptographic operations inside an Intel SGX Enclave Page Cache (EPC) on Fortanix EDP.

### How does `traces-sm` differ from HashiCorp Vault or OpenBao?
HashiCorp Vault and OpenBao store encrypted secrets on disk, but process plaintext secrets in standard host RAM. `traces-sm` executes decryption, signing, and key generation inside hardware-encrypted SGX memory.

### How does `traces-sm` handle memory zeroization in Rust?
All private key byte vectors wrap in `zeroize::Zeroizing<T>`. On drop, compiler intrinsics volatile-overwrite memory registers with zero bytes.

---

## 📚 Documentation Directory (`docs/`)

- [**Status & Maturity**](docs/STATUS.md)
- [**Standards Implementation Mapping**](docs/STANDARDS_MAPPING.md)
- [**Technical Specification**](docs/SPECIFICATION.md)
- [**Architecture Walkthrough**](docs/ARCHITECTURE.md)
- [**CLI Reference**](docs/CLI.md)
- [**Product Specification**](docs/PRODUCT_SPECIFICATION.md)
- [**Conformance Report**](docs/CONFORMANCE_REPORT.md)
- [**Page-by-Page UI/UX Design**](docs/PAGE_BY_PAGE_DESIGN.md)
- [**Windows Build Guide**](docs/WINDOWS_BUILD_GUIDE.md)
- [**FAQ**](docs/FAQ.md)
- [**Comparison**](docs/COMPARISON.md)
- [**Attestation & DCAP**](docs/ATTESTATION.md)
- [**100-Repo Knowledge Bank**](docs/KNOWLEDGE_BANK.md)
