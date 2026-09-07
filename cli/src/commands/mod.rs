//! Subcommand Handlers and Argument Definitions for the `traces-sm` CLI.
//!
//! # Module Hierarchy
//! - [`secret`]: Sealed secrets CRUD handler.
//! - [`key`]: In-enclave asymmetric/symmetric keypair generation and signing handler.
//! - [`token`]: Enclave-signed JWT token issuance and verification handler.
//! - [`attest`]: SGX DCAP / Simulation remote attestation inspection handler.
//! - [`zkp`]: Zero-Knowledge Proof (Schnorr) and Homomorphic Encryption handler.
//! - [`lifecycle`]: NIST SP 800-57 lifecycle transition and SP 800-88 crypto-shredding handler.
//! - [`dkg`]: Distributed Key Generation (DKG) threshold peer node handler.
//! - [`entropy`]: NIST SP 800-90B DRBG continuous health telemetry handler.

pub mod attest;
pub mod dkg;
pub mod entropy;
pub mod key;
pub mod lifecycle;
pub mod secret;
pub mod token;
pub mod zkp;
