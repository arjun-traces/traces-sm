# NIST SP 800-57 & FIPS 140-3 Standards Mapping

This document provides a line-item status mapping of `traces-sm` against National Institute of Standards and Technology (NIST) guidelines.

---

## 📋 Standards Implementation Table

| NIST / FIPS Standard | Feature / Mechanism | Status | Implementation Source |
|---|---|---|---|
| **NIST SP 800-57 Part 1** | Key Lifecycle State Machine | Implemented | `enclave/src/nist.rs` |
| **NIST SP 800-57 Part 1** | `KeyUsage` Bitmask Validation | Implemented | `enclave/src/nist.rs` |
| **NIST SP 800-57 Part 1** | Cryptoperiod Byte Limits ($2^{32}$ bytes) | Implemented | `enclave/src/policy.rs` |
| **NIST SP 800-90A** | `HMAC_DRBG` Seeded via `RDRAND` | Implemented | `enclave/src/drbg.rs` |
| **NIST SP 800-90B** | Continuous APT ($W=512, C=13$) & RCT ($C=16$) | Implemented | `enclave/src/drbg.rs` |
| **NIST SP 800-108** | Counter Mode PRF Key Derivation | Implemented | `enclave/src/nist.rs` |
| **NIST SP 800-38F** | AES Key Wrap (`AES-KW`) | Implemented | `enclave/src/keygen.rs` |
| **NIST SP 800-88 Rev 1** | Crypto-Shredding (Random Sector Overwrite) | Implemented | `enclave/src/store.rs` |
| **FIPS 140-3** | In-Memory RAM Zeroization (`zeroize`) | Implemented | `enclave/src/keygen.rs` |
| **FIPS 203 (PQC KEM)** | ML-KEM-768 / ML-KEM-1024 | Experimental | `enclave/src/pqc.rs` (Stubs return `NotImplemented`) |
| **FIPS 204 (PQC DSA)** | ML-DSA-3 / ML-DSA-5 | Experimental | `enclave/src/pqc.rs` (Stubs return `NotImplemented`) |
