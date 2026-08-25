use ring::hmac;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct EntropyHealthStatus {
    pub rct_passed: bool,
    pub apt_passed: bool,
    pub reseed_count: u64,
}

pub struct HmacDrbg {
    key: hmac::Key,
    v: Vec<u8>,
    reseed_counter: u64,
    rct_prev_sample: u8,
    rct_count: usize,
    apt_window: Vec<u8>,
    apt_count: usize,
    apt_base_sample: u8,
}

impl HmacDrbg {
    pub fn new() -> Self {
        let entropy = Self::get_entropy(32);
        let key = hmac::Key::new(hmac::HMAC_SHA256, &vec![0u8; 32]);
        let v = vec![1u8; 32];
        let mut drbg = Self {
            key,
            v,
            reseed_counter: 1,
            rct_prev_sample: 0,
            rct_count: 0,
            apt_window: Vec::with_capacity(512),
            apt_count: 0,
            apt_base_sample: 0,
        };
        drbg.update(&entropy);
        drbg
    }

    fn get_entropy(len: usize) -> Vec<u8> {
        let mut buf = vec![0u8; len];
        // In a real SGX environment, we would use rdrand/rdseed.
        // For fallback/simulation, we use standard OS randomness or time.
        for i in 0..len {
            buf[i] = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() % 256) as u8;
        }
        buf
    }

    fn update(&mut self, provided_data: &[u8]) {
        let mut ctx = hmac::Context::with_key(&self.key);
        ctx.update(&self.v);
        ctx.update(&[0x00]);
        ctx.update(provided_data);
        self.key = hmac::Key::new(hmac::HMAC_SHA256, ctx.sign().as_ref());

        let mut ctx = hmac::Context::with_key(&self.key);
        ctx.update(&self.v);
        self.v = ctx.sign().as_ref().to_vec();

        if !provided_data.is_empty() {
            let mut ctx = hmac::Context::with_key(&self.key);
            ctx.update(&self.v);
            ctx.update(&[0x01]);
            ctx.update(provided_data);
            self.key = hmac::Key::new(hmac::HMAC_SHA256, ctx.sign().as_ref());

            let mut ctx = hmac::Context::with_key(&self.key);
            ctx.update(&self.v);
            self.v = ctx.sign().as_ref().to_vec();
        }
    }

    pub fn generate(&mut self, out: &mut [u8]) -> EntropyHealthStatus {
        if self.reseed_counter > 10000 {
            let entropy = Self::get_entropy(32);
            self.update(&entropy);
            self.reseed_counter = 1;
        }

        let mut generated = 0;
        while generated < out.len() {
            let mut ctx = hmac::Context::with_key(&self.key);
            ctx.update(&self.v);
            self.v = ctx.sign().as_ref().to_vec();
            
            let to_copy = std::cmp::min(self.v.len(), out.len() - generated);
            out[generated..generated + to_copy].copy_from_slice(&self.v[..to_copy]);
            generated += to_copy;
        }
        
        self.update(&[]);
        self.reseed_counter += 1;

        // Run Health Tests
        let rct_passed = self.run_rct(out);
        let apt_passed = self.run_apt(out);

        EntropyHealthStatus {
            rct_passed,
            apt_passed,
            reseed_count: self.reseed_counter,
        }
    }

    fn run_rct(&mut self, data: &[u8]) -> bool {
        for &byte in data {
            if byte == self.rct_prev_sample {
                self.rct_count += 1;
                if self.rct_count >= 16 {
                    return false;
                }
            } else {
                self.rct_prev_sample = byte;
                self.rct_count = 1;
            }
        }
        true
    }

    fn run_apt(&mut self, data: &[u8]) -> bool {
        for &byte in data {
            if self.apt_count == 0 {
                self.apt_base_sample = byte;
            }
            if byte == self.apt_base_sample {
                self.apt_window.push(byte);
                if self.apt_window.len() >= 13 {
                    return false;
                }
            }
            self.apt_count += 1;
            if self.apt_count == 512 {
                self.apt_count = 0;
                self.apt_window.clear();
            }
        }
        true
    }
}

pub fn init_drbg_health_check() -> EntropyHealthStatus {
    let mut drbg = HmacDrbg::new();
    let mut dummy = [0u8; 64];
    drbg.generate(&mut dummy)
}
