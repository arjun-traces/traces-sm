# `traces-sm` — 100% Rust-Native SGX Secrets Manager

Welcome to the official documentation portal for **`traces-sm`**, a 100% Rust-Native Key & Secret Management Framework built on **Intel SGX using Fortanix EDP** (`x86_64-fortanix-unknown-sgx`).

---

## 📚 Core Specifications & Guides

- [**10-Page Technical Specification**](TECHNICAL_SPECIFICATION.html): Detailed engineering specification covering enclave trust boundaries, RA-TLS inter-node mTLS networking, DKG/MPC protocols, module data flows, threat model, and Docker deployment.
- [**Product Specification Document (PSD)**](PRODUCT_SPECIFICATION.html): High-level product vision, supported key generation catalog, NIST SP 800-57 4-phase lifecycle rules, and performance SLOs.
- [**Product Conformance & Compliance Report**](CONFORMANCE_REPORT.html): Formal verification report proving 100% compliance across all PSD requirement domains.
- [**Page-by-Page UI/UX Design Specification**](PAGE_BY_PAGE_DESIGN.html): Detailed UI component layout for all 8 main console views + Traces AI Assistant Panel (Anthropic API ready).
- [**Windows Desktop Application Build Guide**](WINDOWS_BUILD_GUIDE.md): Step-by-step Windows 10/11 build instructions, Visual Studio prerequisites, PowerShell commands, and cargo-wix MSI packaging.
- [**Architecture Walkthrough**](ARCHITECTURE_WALKTHROUGH.html): 5-crate architectural walkthrough, file responsibilities, and end-to-end request data flows.
- [**100-Repository Cryptographic Systems Catalog**](KNOWLEDGE_BANK.html): Architectural comparison of 100 open-source TEE, DKG, DPKI, secrets engines, and post-quantum ZK systems.

---

## 💻 Multi-OS Distribution Downloads

- **Ubuntu / Debian**: `.deb` installer package
- **Fedora / RHEL**: `.rpm` installer package
- **Windows 10 / 11**: `.msi` installer / Winget
- **macOS**: Homebrew formula (`brew install traces-sm`)
