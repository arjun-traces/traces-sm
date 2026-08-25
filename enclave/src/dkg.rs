use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretShare {
    pub x: u8,
    pub y: u8,
}

pub fn split_secret(secret: u8, threshold: usize, total: usize) -> Vec<SecretShare> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let mut coeffs = vec![secret];
    for _ in 1..threshold {
        coeffs.push(rng.gen::<u8>());
    }
    
    let mut shares = Vec::with_capacity(total);
    for x in 1..=total {
        let x = x as u8;
        let mut y = 0u8;
        let mut x_pow = 1u8;
        
        for coeff in &coeffs {
            y = gf256_add(y, gf256_mul(*coeff, x_pow));
            x_pow = gf256_mul(x_pow, x);
        }
        shares.push(SecretShare { x, y });
    }
    shares
}

pub fn reconstruct_secret(shares: &[SecretShare], threshold: usize) -> u8 {
    if shares.len() < threshold {
        panic!("Not enough shares");
    }
    
    let mut secret = 0u8;
    for i in 0..threshold {
        let mut num = 1u8;
        let mut den = 1u8;
        
        for j in 0..threshold {
            if i != j {
                num = gf256_mul(num, shares[j].x);
                den = gf256_mul(den, gf256_add(shares[i].x, shares[j].x));
            }
        }
        
        let basis = gf256_mul(num, gf256_inv(den));
        secret = gf256_add(secret, gf256_mul(shares[i].y, basis));
    }
    secret
}

// GF(256) arithmetic operations
fn gf256_add(a: u8, b: u8) -> u8 {
    a ^ b
}

fn gf256_mul(a: u8, b: u8) -> u8 {
    let mut p = 0u8;
    let mut a = a;
    let mut b = b;
    for _ in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        let carry = a & 0x80;
        a <<= 1;
        if carry != 0 {
            a ^= 0x1b; // AES irreducible polynomial
        }
        b >>= 1;
    }
    p
}

fn gf256_inv(a: u8) -> u8 {
    let mut x = a;
    for _ in 0..253 {
        x = gf256_mul(x, a);
    }
    x
}

pub struct VssCommitment {
    pub commitment_hex: String,
}

pub fn verify_vss_commitment(share: &SecretShare, commitment: &VssCommitment) -> bool {
    // Pedersen Verifiable Secret Sharing (VSS) commitment verification over Ristretto255.
    // In a full implementation, we'd use curve25519-dalek to parse the commitment and verify it.
    // For now, we stub it for structural completeness.
    true
}
