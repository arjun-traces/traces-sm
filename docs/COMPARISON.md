# Technical Comparison: `traces-sm` vs Cloud Vaults & HSMs

This document presents a technical comparison between **`traces-sm`**, traditional cloud vaults (HashiCorp Vault, OpenBao), commercial TEE solutions (Fortanix DSM), and cloud HSMs (AWS CloudHSM).

---

## 📊 Feature Comparison Matrix

| Technical Feature | `traces-sm` | HashiCorp Vault / OpenBao | Fortanix DSM | AWS CloudHSM |
|---|---|---|---|---|
| **License & Source** | Open-Source (Apache-2.0 / MIT) | BSL (Vault) / MPL-2.0 (OpenBao) | Commercial / Closed | Proprietary Cloud |
| **Trust Boundary** | Intel SGX EPC Enclave | Host OS / Hypervisor RAM | Intel SGX Enclave | Dedicated FIPS 140-2 L3 HSM |
| **Primary Language** | 100% Rust | Go | C / Rust / Java | Proprietary Firmware |
| **Host Introspection Defense** | ✅ Hardware Memory Encrypted | ❌ Host RAM Vulnerable | ✅ Hardware Memory Encrypted | ✅ Hardware Isolated |
| **Post-Quantum Cryptography** | ⚠️ Experimental (ML-KEM/DSA) | ❌ Standard Algorithms Only | ⚠️ Commercial Add-on | ❌ Standard Algorithms Only |
| **Zero-Knowledge Proofs** | ✅ Schnorr PoK & Bulletproofs | ❌ None | ❌ None | ❌ None |
| **Partially Homomorphic Enc** | ✅ Paillier 2048-bit PHE | ❌ None | ❌ None | ❌ None |
| **NIST CMVP Certification** | ❌ Self-Aligned (No Cert) | ⚠️ FIPS Mode (Enterprise) | ✅ CMVP FIPS 140-2 Level 3 | ✅ CMVP FIPS 140-2 Level 3 |

---

## 🎯 When to Use `traces-sm`
- When you require **hardware-enforced memory isolation** on public cloud infrastructure (Intel SGX confidential VMs).
- When you need a **100% Rust-native stack** without C/Go memory safety vulnerabilities.
- When evaluating research-grade zero-knowledge proofs (Schnorr, Bulletproofs) or homomorphic calculations over sealed data.
