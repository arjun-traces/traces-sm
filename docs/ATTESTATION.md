# Intel SGX Remote Attestation & RA-TLS Specification

This document details the Remote Attestation architecture used by `traces-sm` for verifying enclave identity and establishing peer-to-peer mTLS network sessions.

---

## 🛠️ Intel DCAP Attestation Flow

```
+---------------------+           +----------------------+           +---------------------+
|  SGX Enclave Node   |           | Untrusted Host Proxy |           | Intel PCCS Service  |
+----------+----------+           +----------+-----------+           +----------+----------+
           |                                 |                                  |
           | 1. EREPORT (Target Info)        |                                  |
           +-------------------------------->|                                  |
           |                                 | 2. Fetch PCK Cert & Collateral   |
           |                                 +--------------------------------->|
           |                                 |<---------------------------------+
           | 3. Generate DCAP Quote          |                                  |
           |<--------------------------------+                                  |
           |                                                                    |
           | 4. Bind Quote to X.509 TLS Cert Extension (1.2.840.113741.1.13.1)  |
           +--------------------------------------------------------------------+
```

## 🔍 RA-TLS Certificate Extension Structure
The enclave embeds its raw DCAP Quote inside the X.509 Certificate extension `1.2.840.113741.1.13.1`:

```text
Extension: 1.2.840.113741.1.13.1 (Intel SGX Attestation Quote)
  Header: DCAP Quote Header (Version 3/4, Attestation Key Type)
  ISV Enclave Report:
    MRENCLAVE:  8a91f4b3... (32-byte SHA-256 Enclave Code Hash)
    MRSIGNER:   41f89a12... (32-byte SHA-256 Enclave Signer Hash)
    ISVSVN:     0001        (Enclave Security Version Number)
    Attributes: 0000000000000006 (Debug: False, Mode: 64-bit)
```
