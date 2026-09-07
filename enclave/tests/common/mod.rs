//! Shared test helpers.

use std::path::PathBuf;
use traces_sm_enclave::error::EnclaveError;
use traces_sm_enclave::sealing::SealingKeyProvider;
use zeroize::Zeroizing;

/// Deterministic sealing provider so tests never touch the filesystem and
/// never depend on `SimSealingProvider`'s on-disk key cache.
pub struct FixedKeyProvider(pub [u8; 32]);

impl FixedKeyProvider {
    pub fn new(byte: u8) -> Self {
        Self([byte; 32])
    }
}

impl SealingKeyProvider for FixedKeyProvider {
    fn master_key(&self) -> Result<Zeroizing<[u8; 32]>, EnclaveError> {
        Ok(Zeroizing::new(self.0))
    }
}

/// A unique temp directory per test, cleaned up on drop.
pub struct TempStore(pub PathBuf);

impl TempStore {
    pub fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!(
            "traces-sm-test-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp store");
        Self(dir)
    }

    pub fn path(&self) -> &str {
        self.0.to_str().expect("utf-8 temp path")
    }
}

impl Drop for TempStore {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Shannon entropy in bits per byte. Used to assert that a claimed entropy
/// source is not, in fact, a counter or a constant.
pub fn shannon_entropy_bits_per_byte(data: &[u8]) -> f64 {
    let mut counts = [0usize; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let n = data.len() as f64;
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.log2()
        })
        .sum()
}

/// Count of distinct byte values present.
pub fn distinct_bytes(data: &[u8]) -> usize {
    let mut seen = [false; 256];
    for &b in data {
        seen[b as usize] = true;
    }
    seen.iter().filter(|&&s| s).count()
}
