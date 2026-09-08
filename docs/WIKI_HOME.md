# 🔐 traces-sm Wiki — Complete Navigation Hub

Welcome to the **traces-sm** Wiki! This is your central hub for technical documentation, compliance specifications, architecture guides, and community resources for the 100% Rust-native SGX secrets and key management framework.

---

## 📋 Quick Navigation

### 🎯 Getting Started
- **[Quick Start Guide](Quick-Start)** — CLI, Desktop App, and Web GUI installation
- **[Installation & Setup](Installation)** — Multi-OS build instructions
- **[FAQ — Frequently Asked Questions](FAQ)** — Common questions about SGX, FIPS, and Remote Attestation

### 🏗️ Architecture & Design
- **[Technical Specification](Technical-Specification)** — Complete 10-page engineering specification
- **[Architecture Walkthrough](Architecture-Walkthrough)** — System design, crate layout, and data flow
- **[NIST Compliance Matrix](NIST-Compliance-Matrix)** — NIST SP 800-57, 800-90A, 800-38F alignment

### 🔑 Core Features
- **[Key Generation Catalog](Key-Generation-Catalog)** — All supported algorithms (RSA, ECDSA, Ed25519, ML-KEM, ML-DSA, SLH-DSA)
- **[NIST SP 800-57 Key Lifecycle](Key-Lifecycle)** — PreOperational, Operational, Deactivated, Destroyed states
- **[Remote Attestation & RA-TLS](Attestation)** — Intel DCAP Quote verification and mTLS peer authentication
- **[Distributed Key Generation (DKG)](DKG)** — Shamir Secret Sharing, Pedersen VSS, FROST threshold signatures

### 🛠️ User Guides
- **[CLI Reference](CLI-Reference)** — Command-line interface operations and examples
- **[Desktop App Guide](Desktop-App-Guide)** — Cross-platform GUI usage (Ubuntu, Windows, macOS)
- **[Web GUI / WASM Documentation](Web-GUI-Guide)** — Yew 0.21 WebAssembly interface
- **[Policy Engine & Safe Mode](Policy-Engine)** — Mandatory Security Policy (MSP) configuration

### 🔒 Security & Compliance
- **[Conformance Report](Conformance-Report)** — FIPS 140-3, NIST, and standards verification
- **[Threat Model & Security Assumptions](Threat-Model)** — Hardware boundary assumptions, attack surfaces
- **[Vulnerability Disclosure Policy](Security-Policy)** — Reporting security vulnerabilities responsibly

### 📊 Performance & Benchmarks
- **[Performance Benchmarks](Benchmarks)** — Throughput, latency, and SLO verification
- **[Hardware Requirements](Hardware-Requirements)** — CPU, RAM, and Intel SGX platform support

### 🚀 Advanced Topics
- **[Post-Quantum Cryptography (PQC)](Post-Quantum-Crypto)** — ML-KEM, ML-DSA, SLH-DSA implementation
- **[Zero-Knowledge Proofs](Zero-Knowledge-Proofs)** — Schnorr PoK, Bulletproof range proofs, commitments
- **[Homomorphic Encryption](Homomorphic-Encryption)** — Paillier additively homomorphic encryption
- **[Safe Mode & Network Integrity](Safe-Mode)** — Entri DNS verification and quarantine procedures

### 👥 Community & Contributing
- **[Contributing Guidelines](Contributing)** — How to contribute code, tests, and documentation
- **[Code of Conduct](Code-of-Conduct)** — Community standards and expectations
- **[Monthly Contributor Reward](Contributor-Reward)** — $100 BTC reward program details

### 📁 Multi-Crate Workspace
- **[Enclave Crate (`enclave/`)](Enclave-Crate)** — SGX EPC cryptographic engine
- **[Host Crate (`host/`)](Host-Crate)** — Axum REST proxy and SQLite audit logs
- **[GUI Crate (`gui/`)](GUI-Crate)** — Yew WebAssembly web interface
- **[CLI Crate (`cli/`)](CLI-Crate)** — Command-line tool reference
- **[Desktop Crate (`desktop/`)](Desktop-Crate)** — Native desktop application (Ubuntu, Windows, macOS)
- **[Bounty Bot (`bounty_bot/`)](Bounty-Bot)** — Security research and vulnerability aggregation

### 🔍 External Resources
- **[GitHub Repository](https://github.com/arjun-traces/traces-sm)** — Main source code repository
- **[GitHub Pages / Official Website](https://arjun-traces.github.io/traces-sm/)** — Live documentation site
- **[Discord Community](https://discord.gg/traces)** — Real-time discussion and support
- **[ttraces.io](https://ttraces.io)** — Community hub and announcements

---

## 📚 Documentation Structure Overview

```
arjun-traces/traces-sm/
│
├── README.md                        ← Project overview (source of truth)
├── BUILD.md                         ← Compilation and build instructions
├── CONTRIBUTING.md                  ← Developer contribution guidelines
├── Cargo.toml                       ← Workspace root manifest
│
├── docs/                            ← GitHub Pages / Jekyll site content
│   ├── README.md                    ← Docs directory index
│   ├── TECHNICAL_SPECIFICATION.md   ← 10-page technical spec
│   ├── PRODUCT_SPECIFICATION.md     ← Product features and use cases
│   ├── CONFORMANCE_REPORT.md        ← Standards alignment verification
│   ├── ARCHITECTURE_WALKTHROUGH.md  ← Design and crate interaction
│   ├── PAGE_BY_PAGE_DESIGN.md       ← UI/UX specifications
│   ├── WINDOWS_BUILD_GUIDE.md       ← Windows-specific build steps
│   ├── KNOWLEDGE_BANK.md            ← 100-repo cryptographic systems catalog
│   ├── CLI.md                       ← Command-line interface reference
│   ├── FAQ.md                       ← Frequently asked questions
│   ├── BENCHMARKS.md                ← Performance metrics and SLOs
│   ├── ATTESTATION.md               ← Remote attestation architecture
│   ├── ARCHITECTURE.md              ← System architecture overview
│   ├── SPECIFICATION.md             ← Engineering specifications
│   ├── TASK_CHECKLIST.md            ← Milestone tracking
│   ├── GITHUB_TASKS.md              ← Deployment task list
│   ├── index.html                   ← Main landing page
│   └── images/                      ← Diagrams and screenshots
│
├── enclave/                         ← SGX Enclave crate
├── host/                            ← REST Host Proxy crate
├── gui/                             ← WebAssembly GUI crate
├── cli/                             ← CLI Tool crate
├── desktop/                         ← Desktop App crate
└── bounty_bot/                      ← Security research bot
```

---

## 🎓 Learning Paths

### **Path 1: I want to understand traces-sm**
1. Start with [Quick Start Guide](Quick-Start)
2. Read [Architecture Walkthrough](Architecture-Walkthrough)
3. Explore [Key Generation Catalog](Key-Generation-Catalog)
4. Review [NIST Compliance Matrix](NIST-Compliance-Matrix)

### **Path 2: I want to build and deploy traces-sm**
1. Follow [Installation & Setup](Installation)
2. Choose your platform: [CLI Reference](CLI-Reference), [Desktop App](Desktop-App-Guide), or [Web GUI](Web-GUI-Guide)
3. Review [Performance Benchmarks](Benchmarks) to validate your deployment
4. Check [Conformance Report](Conformance-Report) for compliance details

### **Path 3: I want to contribute to traces-sm**
1. Read [Contributing Guidelines](Contributing)
2. Understand the [Multi-Crate Workspace](Multi-Crate-Workspace)
3. Review [Architecture Walkthrough](Architecture-Walkthrough) to understand crate interactions
4. Check relevant crate documentation ([Enclave](Enclave-Crate), [Host](Host-Crate), [GUI](GUI-Crate), [CLI](CLI-Crate))

### **Path 4: I want to integrate traces-sm into my application**
1. Review [Use Cases & Architecture](Architecture-Walkthrough)
2. Study [Remote Attestation & RA-TLS](Attestation)
3. Implement via [CLI Reference](CLI-Reference) or [REST API Documentation](Host-Crate)
4. Reference [Technical Specification](Technical-Specification) for cryptographic operations

### **Path 5: I want to understand advanced cryptography**
1. Read [Post-Quantum Cryptography (PQC)](Post-Quantum-Crypto)
2. Explore [Zero-Knowledge Proofs](Zero-Knowledge-Proofs)
3. Study [Homomorphic Encryption](Homomorphic-Encryption)
4. Review [Distributed Key Generation (DKG)](DKG)

---

## 📞 Getting Help

### 📖 Documentation Issues
If you find a documentation error or have a suggestion, please:
- File an issue on [GitHub Issues](https://github.com/arjun-traces/traces-sm/issues)
- Label with `documentation`

### 💬 Community Questions
For general questions and discussions:
- Join our **[Discord Community](https://discord.gg/traces)** for real-time chat
- Visit **[ttraces.io](https://ttraces.io)** for announcements

### 🔐 Security Issues
For confidential security reports:
- Email maintainers or reach out on [Discord](https://discord.gg/traces) for secure coordination
- See [Vulnerability Disclosure Policy](Security-Policy)

### 💡 Feature Requests & Bugs
- Create issues on [GitHub Issues](https://github.com/arjun-traces/traces-sm/issues)
- Reference relevant documentation sections in your issue

---

## 🏆 Key Differentiators

✅ **100% Rust-Native** — No FFI, no C/C++ dependencies in the enclave  
✅ **Hardware Isolation** — Keys never exist unencrypted outside Intel SGX EPC  
✅ **NIST Compliance** — NIST SP 800-57, 800-90A, 800-38F, FIPS 140-3 alignment  
✅ **Post-Quantum Ready** — ML-KEM, ML-DSA, SLH-DSA standardized algorithms  
✅ **Threshold Cryptography** — Shamir Secret Sharing, Pedersen VSS, FROST signatures  
✅ **Remote Attestation** — Intel DCAP RA-TLS for peer authentication  
✅ **Multi-Platform** — Ubuntu, Alpine, Fedora, ARM64, Windows, macOS  
✅ **Open Source** — Apache 2.0 licensed, community-driven development  

---

## 📜 License & Attribution

**traces-sm** is licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0). 

Project badge owned by: [boosters-research](https://www.bestpractices.dev/en/users/56453)

---

**Last Updated:** 2026-09-08  
**Wiki Version:** 1.0  
**Maintained by:** arjun-traces community
