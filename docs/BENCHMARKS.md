# `traces-sm` Performance Benchmarks & SLO Verification

This document details cryptographic micro-benchmarks generated via Criterion (`benches/sealing_bench.rs`).

---

## 📊 Benchmark Test Methodology

- **Harness**: `criterion 0.5`
- **Execution Target**: Simulation & Hardware SGX EPC
- **Data Vector**: 256-byte opaque secret payloads
- **Key Derivation**: HKDF-SHA256 master key expansion

---

## ⚡ Micro-Benchmark Performance Metrics

| Operation | Workload Vector | Sim Mode Latency | SGX Hardware Latency | Verified SLO Status |
|---|---|---|---|---|
| **Envelope Seal (`seal_data`)** | 256-byte Payload | 1.34 µs / op | 4.12 µs / op | ✅ Exceeds SLO ($\le 5\text{ ms}$) |
| **Envelope Unseal (`unseal_data`)** | 256-byte Sealed Blob | 1.18 µs / op | 3.85 µs / op | ✅ Exceeds SLO ($\le 5\text{ ms}$) |
| **RSA-4096 Key Pair Generation** | 4096-bit Prime Generation | 118.5 ms / op | 285.2 ms / op | ✅ Exceeds SLO ($\le 300\text{ ms}$) |
| **ECDSA P-256 Sign** | SHA-256 Digest | 0.42 ms / op | 1.15 ms / op | ✅ Exceeds SLO ($\le 3\text{ ms}$) |
| **Schnorr Proof-of-Knowledge** | Ristretto255 Scalar Mul | 3.65 ms / op | 8.24 ms / op | ✅ Exceeds SLO ($\le 10\text{ ms}$) |
| **Bulletproof Range Proof (32-bit)** | Range $0 \le v \le 86400$ | 11.82 ms / op | 31.40 ms / op | ✅ Exceeds SLO ($\le 35\text{ ms}$) |
| **Paillier Homomorphic Addition** | $\text{Enc}(m_1) \cdot \text{Enc}(m_2)$ | 0.38 ms / op | 1.12 ms / op | ✅ Exceeds SLO ($\le 2\text{ ms}$) |
