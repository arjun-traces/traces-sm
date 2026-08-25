# `traces-sm` — Page-by-Page UI/UX & System Design Specification
## 100% Rust-Native Multi-OS Console with Integrated Traces AI Assistant

---

## 1. Console Layout Overview

The **`traces-sm`** user console UI design is synchronized 1-to-1 between the **Rust WebAssembly Web GUI (`gui/`)** and the **Cross-Platform Native Desktop App (`desktop/`)**.

```
┌────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ 🔒 traces-sm — SGX Secrets & Key Management Console  Ubuntu 24.04 (GTK3)  SGX HW_ACTIVE  RA-TLS VERIFIED  ✦ Traces AI│
├────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ SIDEBAR               │ MAIN CONTEXT VIEW AREA                                      │ TRACES AI ASSISTANT PANEL │
│ • KEYS & SECRETS      │                                                             │ ✦ Traces AI (Anthropic)   │
│   - Dashboard  [⌘1]   │  • Executive Dashboard / Summary Metric Cards               │ API Key: [sk-ant-api...]  │
│   - Key Lifecycle [⌘2]│  • NIST SP 800-57 Key Lifecycle Matrix & State Machine      │                           │
│   - Vault      [⌘3]   │  • Sealed Secret Vault & Reveal Drawer                      │ Chat Log:                 │
│ • NETWORK             │  • DKG Peer Topology & RA-TLS Attestation Map               │ "Enclave is HW_ACTIVE     │
│   - DKG Topology [⌘4] │  • NIST SP 800-90B DRBG Entropy Health Monitor              │  and 6/6 policy rules     │
│ • CRYPTOGRAPHY        │  • ZKP & Homomorphic Encryption Sandbox                     │  are enforcing..."        │
│   - Entropy    [⌘5]   │  • Mandatory Security Policy Configuration (`policy.rs`)    │                           │
│   - ZKP Sandbox[⌘6]   │  • Non-Repudiable Audit Trail & Intel DCAP Quote Inspector   │ Quick Buttons:            │
│ • GOVERNANCE          │                                                             │ [Why k-104?] [APT cutoff] │
│   - Policy     [⌘7]   │                                                             │ [Quorum health]           │
│   - Audit Logs [⌘8]   │                                                             │ Input: [ Ask question...] │
└───────────────────────┴─────────────────────────────────────────────────────────────┴───────────────────────────┘
```

---

## 2. Integrated Traces AI Assistant Panel (Anthropic API Ready)

The **Traces AI Assistant** is positioned on the right-side drawer across both Web and Desktop applications:

- **Anthropic Claude Integration**: Accepts user-configured Anthropic API Keys (`sk-ant-api...`) for live intelligence on cryptographic state transitions, attestation quote verification, and SP 800-90B entropy statistics.
- **Local Fallback Mode**: When no external API key is passed, Traces AI operates in local mode using in-enclave policy status metrics from `enclave/src/policy.rs`.
- **Quick-Prompt Buttons**: Includes direct diagnostic shortcuts:
  - `[Why k-104?]`: Explains cryptoperiod volume limits on deactivated keys.
  - `[APT cutoff]`: Queries SP 800-90B Adaptive Proportion Test status.
  - `[Quorum health]`: Inspects DKG threshold node RA-TLS certificates.

---

## 3. UI Component Summary Across 8 Main Views

1. **Executive Dashboard**: 4 key indicator cards, Enclave EPC memory gauge (21.2 MB / 64.0 MB), Cryptoperiod volume meter (1.2 GB / 4.2 GB), live Cryptographic Throughput graph (ops/sec), and Architecture Stack card.
2. **NIST SP 800-57 Key Lifecycle Matrix**: State tab filters (`All`, `Pre-Operational`, `Operational`, `Deactivated`, `Revoked`, `Shredded`), key table with `KeyUsage` bitmasks, cryptoperiod volume progress meters, action buttons (`Rotate`, `Activate`, `Shred`), and selected key detail strip.
3. **Sealed Secret Vault**: Type filters (`Opaque`, `SymmetricKey`, `CertBundle`), version counters, owner tags, expiry dates, `AES-256-GCM` sealing badges, `Reveal` button, and `+ Create` modal.
4. **DKG Topology Visualizer**: Node topology diagram, RA-TLS peer topology map, 2-of-3 threshold quorum selector (`Pedersen VSS` / `FROST`).
5. **DRBG & Entropy Health**: SP 800-90B RCT & APT continuous test cards, Reseed counters, DRBG audit log, and `Re-seed DRBG` button.
6. **Mandatory Security Policy**: Governance profile selector (`StrictNistProfile`), 6 active security invariant checkboxes (FIPS 140-3 zeroization, EPC memory encryption, SP 800-38F key wrapping, cryptoperiod volume limits, SP 800-88 crypto-shredding, DCAP attestation quotes).
7. **ZKP & Homomorphic Encryption Sandbox**: Sub-tabs for Schnorr PoK, Bulletproofs 32-bit range proofs, and Paillier PHE calculations.
8. **Audit Logs & Attestation Quotes**: Chronological audit trail table (timestamp, principal, action, resource, IP, policy result) and Intel DCAP quote inspection drawer (`MRENCLAVE`, `MRSIGNER`, `ISVSVN`).
