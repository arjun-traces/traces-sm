use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub store_path: String,
    pub sgx_mode: String,
    pub jwt_validity_secs: u64,
}

impl Config {
    pub fn load() -> Self {
        Self {
            port: env::var("ENCLAVE_PORT").unwrap_or_else(|_| "8443".to_string()).parse().unwrap_or(8443),
            store_path: env::var("ENCLAVE_STORE_PATH").unwrap_or_else(|_| "/tmp/sm-store".to_string()),
            sgx_mode: env::var("SGX_MODE").unwrap_or_else(|_| "SIM".to_string()),
            jwt_validity_secs: env::var("JWT_VALIDITY_SECS").unwrap_or_else(|_| "3600".to_string()).parse().unwrap_or(3600),
        }
    }
}
