//! HTTP server — state struct and startup.

use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use crate::auth::EnclaveTokenService;
use crate::config::Config;
use crate::sealing::SealingKeyProvider;
use crate::store::Store;

pub mod router;
pub mod handlers;

// ─────────────────────────────────────────────────────────────────────────────
// Shared enclave state (Arc-cloned into each request handler thread)
// ─────────────────────────────────────────────────────────────────────────────

pub struct EnclaveState {
    pub store:         Arc<Store>,
    pub provider:      Arc<dyn SealingKeyProvider>,
    pub token_service: Arc<EnclaveTokenService>,
    pub config:        Config,
}

// ─────────────────────────────────────────────────────────────────────────────
// Server entry point
// ─────────────────────────────────────────────────────────────────────────────

pub fn start_server(state: Arc<EnclaveState>) {
    let addr = format!("0.0.0.0:{}", state.config.port);
    let listener = TcpListener::bind(&addr)
        .unwrap_or_else(|e| panic!("Cannot bind {addr}: {e}"));

    log::info!("Enclave HTTP server listening on http://{addr}");
    log::warn!("NOTE: TLS should be terminated by a TLS proxy in front of the enclave (e.g. rustls via stunnel or nginx). This server speaks plain HTTP for local dev.");

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                let state = Arc::clone(&state);
                thread::spawn(move || {
                    if let Err(e) = router::handle_connection(stream, state) {
                        log::error!("Request error: {e}");
                    }
                });
            }
            Err(e) => log::error!("Accept error: {e}"),
        }
    }
}
