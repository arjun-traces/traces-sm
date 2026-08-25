# `traces-sm` — Project Status & Maturity Disclosure

> **Security Disclosure**: `traces-sm` is an open-source research and engineering framework. It has **not** undergone an independent third-party cryptographic security audit or NIST CMVP certification. Do not use for high-assurance production workloads without conducting your own threat modeling and security audits.

---

## Implementation Status Breakdown

| Subsystem / Feature | Implementation Status | Maturity Level | Notes / Dependencies |
|---|---|---|---|
| **Fortanix EDP SGX Enclave** | Implemented | Alpha | Targets `x86_64-fortanix-unknown-sgx`. Supports SIM and HW modes. |
| **AES-256-GCM Envelope Sealing** | Implemented | Beta | Ephemeral per-secret DEKs sealed via HKDF-SHA256 master keys (`ring` crate). |
| **SP 800-90A/B HMAC_DRBG** | Implemented | Beta | Continuous Repetition Count Test (RCT) & Adaptive Proportion Test (APT) running inside `drbg.rs`. |
| **SP 800-57 Key Lifecycle** | Implemented | Beta | 4-phase state machine (`PreOperational` -> `Destroyed`) with cryptoperiod volume meters. |
| **Classic Asymmetric (RSA/ECDSA/Ed25519)** | Implemented | Beta | In-enclave keypair generation, zeroization (`zeroize`), signing, verification, and OAEP encryption. |
| **Paillier Homomorphic Encryption** | Implemented | Alpha | In-enclave 2048-bit Paillier PHE additive addition & scalar multiplication (`num-bigint`). |
| **Schnorr Proof-of-Knowledge** | Implemented | Alpha | Ristretto255 token possession proof generation & verification (`schnorrkel`). |
| **Bulletproofs Range Proofs** | Implemented | Alpha | 32-bit non-interactive range proofs ($0 \le v \le 86400$) (`bulletproofs`). |
| **Post-Quantum Cryptography (PQC)** | Experimental | Stubbed | ML-KEM and ML-DSA API hooks return `EnclaveError::NotImplemented` until pure-Rust `ml-kem` crates are integrated. |
| **Threshold DKG (FROST / VSS)** | Experimental | Partial | Shamir SSS byte splitting implemented; FROST protocol requires multi-party transport setup. |

---

## Non-Claims & Audit Transparency
- **FIPS Certification**: `traces-sm` complies with FIPS 140-3 design patterns (RAM zeroization, key isolation), but holds **no NIST CMVP certificate number**.
- **Audit Status**: Unaudited code.
