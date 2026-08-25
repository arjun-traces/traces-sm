# Security Policy & Vulnerability Disclosure

## Security Model & SGX Scope
`traces-sm` isolates cryptographic operations within an **Intel SGX Enclave Page Cache (EPC)**.

### What SGX Protects Against:
- Malicious host operating systems and compromised hypervisors.
- Direct host memory dumps (`/dev/mem`) inspecting unencrypted private keys.
- Disk theft (all stored blobs are encrypted via AES-256-GCM hardware sealing).

### Out-of-Scope / Known SGX Limitations:
- Speculative execution side-channel attacks (L1TF, Foreshadow, Spectre-v1/v2) unless Intel Microcode updates and SGX TCB extensions are active.
- Denial of Service (DoS) by untrusted host proxy terminating enclave threads.

## Audit Disclaimer
`traces-sm` has **not** been independently audited by a third-party security firm.

## Reporting Vulnerabilities
Please do **NOT** open public GitHub issues for security vulnerabilities.
Email vulnerability details to: `security@traces.internal` or open a GitHub Private Security Advisory.
