use ring::hmac;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyLifecycleState {
    PreOperational,
    Operational,
    Deactivated,
    Expired,
    Revoked,
    Destroyed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KeyUsage {
    pub sign: bool,
    pub verify: bool,
    pub encrypt: bool,
    pub decrypt: bool,
    pub key_wrap: bool,
    pub derive_key: bool,
    pub authenticate: bool,
}

impl Default for KeyUsage {
    fn default() -> Self {
        Self {
            sign: false,
            verify: false,
            encrypt: false,
            decrypt: false,
            key_wrap: false,
            derive_key: false,
            authenticate: false,
        }
    }
}

pub struct Zeroizing<T> {
    data: T,
}

impl<T> Zeroizing<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
    pub fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> Drop for Zeroizing<T> {
    fn drop(&mut self) {
        // In a real implementation this would securely wipe the memory
    }
}

pub fn sp800_108_kdf(ki: &[u8], label: &[u8], context: &[u8], l: usize) -> Zeroizing<Vec<u8>> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, ki);
    let mut okm = Vec::with_capacity(l);
    let mut counter = 1u32;
    
    while okm.len() < l {
        let mut ctx = hmac::Context::with_key(&key);
        ctx.update(&counter.to_be_bytes());
        ctx.update(label);
        ctx.update(&[0x00]);
        ctx.update(context);
        ctx.update(&(l as u32 * 8).to_be_bytes());
        
        let tag = ctx.sign();
        let chunk = tag.as_ref();
        let to_copy = std::cmp::min(chunk.len(), l - okm.len());
        okm.extend_from_slice(&chunk[..to_copy]);
        counter += 1;
    }
    Zeroizing::new(okm)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CryptoPeriod {
    pub bytes_processed: u64,
    pub max_bytes: u64,
}

impl Default for CryptoPeriod {
    fn default() -> Self {
        Self {
            bytes_processed: 0,
            max_bytes: 4_294_967_296, // 2^32
        }
    }
}

impl CryptoPeriod {
    pub fn process(&mut self, bytes: u64) -> bool {
        self.bytes_processed += bytes;
        self.bytes_processed <= self.max_bytes
    }
}
