//! Library surface for the traces-sm enclave.
//!
//! # Why this file exists
//!
//! `enclave/Cargo.toml` declares only a `[[bin]]` target. Rust integration
//! tests under `enclave/tests/` cannot link against a binary-only crate, so
//! before this file was added it was *structurally impossible* to write a
//! single integration test for the enclave — which is why ~5,000 lines of
//! cryptographic code shipped with no test coverage.
//!
//! `main.rs` should be reduced to a thin entrypoint that calls into this
//! library:
//!
//! ```ignore
//! // enclave/src/main.rs
//! use traces_sm_enclave::{config::Config, server, /* ... */};
//! fn main() { /* wiring only */ }
//! ```
//!
//! Until `main.rs` is trimmed, both files declare the modules and the crate
//! will emit duplicate-module warnings. Removing the `pub mod` block from
//! `main.rs` and adding `use traces_sm_enclave::*;` resolves that.

pub mod auth;
pub mod config;
pub mod crypto;
pub mod dkg;
pub mod drbg;
pub mod error;
pub mod frost;
pub mod he;
pub mod keygen;
pub mod models;
pub mod nist;
pub mod policy;
pub mod pqc;
pub mod sealing;
pub mod server;
pub mod store;
pub mod zkp;
