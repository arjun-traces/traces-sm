//! WebAssembly Yew View Components.
//!
//! # Component Hierarchy
//! - [`dashboard`]: System overview card deck.
//! - [`entropy`]: NIST SP 800-90B DRBG continuous health monitor.
//! - [`header`]: Top navigation bar and status pills.
//! - [`lifecycle`]: NIST SP 800-57 key matrix and SP 800-88 crypto-shredding controls.
//! - [`topology`]: DKG threshold cluster node topology view.
//! - [`traces_ai`]: Interactive AI intelligence drawer.
//! - [`webauthn`]: FIDO2 / WebAuthn hardware token ceremony simulator.
//! - [`zkp_sandbox`]: Interactive zero-knowledge proof & homomorphic encryption playground.

pub mod dashboard;
pub mod entropy;
pub mod header;
pub mod lifecycle;
pub mod topology;
pub mod traces_ai;
pub mod webauthn;
pub mod zkp_sandbox;
