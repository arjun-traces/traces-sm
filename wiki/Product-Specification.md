# Product Specification Document (PSD) & Execution Framework
## `traces-sm`: Multi-OS Cross-Platform 100% Rust-Native SGX Secrets Manager Framework

---

## 1. Executive Summary & Product Vision

**`traces-sm`** is a **100% Rust-Native**, hardware-enforced Key & Secret Management Platform engineered in strict accordance with **NIST SP 800-57 / SP 800-130 / FIPS 140-3** guidelines.

The framework features an in-enclave **Key Generation & Lifecycle Engine (`keygen.rs`)** that generates, zeroizes, envelope-seals, and manages private key material for:
1. **Classic Asymmetric Keypairs**: RSA-2048, RSA-4096, ECDSA (P-256, P-384, P-521, Secp256k1), Ed25519, X25519.
2. **Post-Quantum Cryptography (PQC)**: ML-KEM-512/768/1024 (Kyber), ML-DSA-3/5 (Dilithium), SLH-DSA (SPHINCS+).
3. **Symmetric & Key Wrapping**: AES-128/256-GCM, AES-128/256-KW (NIST SP 800-38F Key Wrap), HMAC-SHA256/512, ChaCha20-Poly1305.
4. **Threshold DKG Keys**: Shamir Secret Sharing (SSS), Pedersen VSS, FROST Ed25519.
5. **PKI & SSL/TLS Certificates**: X.509 Certificate Bundles (CA Root, Intermediates, Server Certs) & OpenSSH Key Pairs.

All sensitive enclave operations execute strictly inside an **Intel SGX Enclave Page Cache (EPC)** using **Fortanix EDP** (`x86_64-fortanix-unknown-sgx`). Unencrypted keys and secrets **never** leave the EPC memory space.

---

## 2. Supported Key Generation & Management Catalog

| Algorithm Family | Supported Key Generation Schemes | Standard / Specification | Primary Function |
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
| **SSL/TLS & SSH** | X.509 Cert Bundles, OpenSSH KeyPairs | RFC 5280 / OpenSSH protocol | Server/CA certificate issuance & SSH authentication |

---

## 3. Multi-OS Architecture Map

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                               100% RUST-NATIVE `traces-sm` WORKSPACE                             â”‚
â”‚                                                                                                  â”‚
â”‚  User & Client Interfaces                                                                        â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚ Rust WebAssembly Web GUI â”‚  â”‚ Rust Native Desktop App      â”‚  â”‚ Rust Native CLI Tool        â”‚  â”‚
â”‚  â”‚ (`gui/` -> Yew 0.21 WASM)â”‚  â”‚ (`desktop/` -> Ubuntu, Win,  â”‚  â”‚ (`cli/` -> Multi-OS Distros)â”‚  â”‚
â”‚  â”‚                          â”‚  â”‚  macOS via `eframe`/`egui`)  â”‚  â”‚                             â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚               â”‚                               â”‚                                 â”‚                â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ Host Layer (Untrusted Proxy)                  â”‚                                 â”‚                â”‚
â”‚               â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                â”‚
â”‚                                               â”‚ REST / mTLS                                      â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚ Rust Native Host Proxy (`host/` -> Axum 0.7 + Tokio + Rusqlite)                            â”‚  â”‚
â”‚  â”‚ â€¢ Serves WebAssembly Web GUI static assets on port 8080                                    â”‚  â”‚
â”‚  â”‚ â€¢ Manages SQLite metadata DB, audit logs, and DKG topology                                  â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ SGX Enclave Layer (Trusted EPC)               â”‚ Plain TCP / mTLS (Port 8443)                     â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚ Rust SGX Enclave (`enclave/` -> Fortanix EDP `x86_64-fortanix-unknown-sgx`)               â”‚  â”‚
â”‚  â”‚ â€¢ In-Enclave Key Generation Catalog Engine (`keygen.rs`)                                   â”‚  â”‚
â”‚  â”‚ â€¢ Mandatory NIST Security Policy Engine (`policy.rs`)                                      â”‚  â”‚
â”‚  â”‚ â€¢ NIST SP 800-90A/B/C Entropy & DRBG Engine (`drbg.rs`)                                    â”‚  â”‚
â”‚  â”‚ â€¢ NIST SP 800-57 Key Lifecycle State Machine (`nist.rs`)                                   â”‚  â”‚
â”‚  â”‚ â€¢ Classic (RSA/ECDSA/Ed25519) + PQC (ML-KEM/ML-DSA) Engines (`keygen.rs`, `pqc.rs`)        â”‚  â”‚
â”‚  â”‚ â€¢ Threshold DKG (Shamir SSS / Pedersen VSS) & PHE (`dkg.rs`, `paillier.rs`)               â”‚  â”‚
â”‚  â”‚ â€¢ Zero-Knowledge Proofs (Schnorr PoK & Bulletproof Range Proofs) (`zkp/`)                  â”‚  â”‚
â”‚  â”‚ â€¢ FIPS 140-3 Zeroization & NIST SP 800-88 Crypto-Shredding (`store.rs`)                    â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

