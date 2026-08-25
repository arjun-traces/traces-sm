//! Enclave entrypoint.
//!
//! When compiled for `x86_64-fortanix-unknown-sgx`, this binary runs entirely
//! inside the Intel SGX Enclave Page Cache (EPC).

use std::sync::Arc;

pub mod error;
pub mod models;
pub mod config;
pub mod sealing;
pub mod crypto;
pub mod keygen;
pub mod zkp;
pub mod he;
pub mod store;
pub mod auth;
pub mod server;
pub mod drbg;
pub mod nist;
pub mod dkg;
pub mod pqc;
pub mod policy;

use crate::config::Config;
use crate::sealing::{SimSealingProvider, HwSealingProvider, SealingKeyProvider};
use crate::store::Store;
use crate::auth::EnclaveTokenService;
use crate::server::EnclaveState;
use crate::policy::{PolicyEngine, SecurityPolicy};

fn main() {
    env_logger::init();

    let cfg = Config::load();
    log::info!(
        "Starting traces-sm-enclave v{} in {} mode on port {}",
        env!("CARGO_PKG_VERSION"),
        cfg.sgx_mode,
        cfg.port
    );

    // Initialise NIST SP 800-90B DRBG & Policy Engine
    let drbg_status = drbg::init_drbg_health_check();
    let policy_engine = PolicyEngine::new(SecurityPolicy::default());
    policy_engine.validate_in_memory_protection().expect("In-memory protection validation failed");
    log::info!("NIST SP 800-90B DRBG & Mandatory Security Policy Enforced: APT={}, RCT={}", drbg_status.apt_passed, drbg_status.rct_passed);

    // ── Sealing provider ──────────────────────────────────────────────────────
    let provider: Arc<dyn SealingKeyProvider> = if cfg.sgx_mode == "HW" {
        log::info!("Using real SGX hardware sealing (EGETKEY)");
        Arc::new(HwSealingProvider)
    } else {
        log::info!("Using simulation sealing key (not for production)");
        Arc::new(SimSealingProvider::new(&cfg.store_path))
    };

    // ── Secret store ──────────────────────────────────────────────────────────
    let store = Arc::new(Store::new(&cfg.store_path));

    // ── Token service ─────────────────────────────────────────────────────────
    let token_service = Arc::new(
        EnclaveTokenService::new(&cfg.store_path, provider.as_ref())
            .expect("Failed to initialise token service"),
    );

    // ── Assemble enclave state ────────────────────────────────────────────────
    let state = Arc::new(EnclaveState {
        store,
        provider,
        token_service,
        config: cfg,
    });

    // ── Start HTTP/TLS server ─────────────────────────────────────────────────
    server::start_server(state);
}
