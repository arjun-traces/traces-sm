use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use crate::error::EnclaveError;

/// Shamir Secret Sharing (SSS) share representation
#[derive(Debug, Clone)]
pub struct SecretShare {
    pub index: u8,
    pub value: u8,
}

/// Split a secret byte using Shamir Secret Sharing over GF(256)
pub fn split_secret_byte(secret: u8, threshold: usize, total: usize) -> Vec<SecretShare> {
    let mut shares = Vec::with_capacity(total);
    let mut coefficients = vec![secret];
    
    // Draw coefficients using ring CSPRNG
    for _ in 1..threshold {
        let mut rnd = [0u8; 1];
        ring::rand::SystemRandom::new().fill(&mut rnd).unwrap();
        coefficients.push(rnd[0]);
    }

    for i in 1..=(total as u8) {
        let mut val = 0u8;
        let mut x_pow = 1u16;
        for &coeff in &coefficients {
            val ^= (coeff as u16 * x_pow % 255) as u8;
            x_pow = (x_pow * i as u16) % 255;
        }
        shares.push(SecretShare { index: i, value: val });
    }

    shares
}

/// Pedersen Verifiable Secret Sharing (VSS) commitment validation
pub fn verify_vss_commitment(
    share_val: u64,
    commitments: &[RistrettoPoint],
    index: u32,
) -> Result<bool, EnclaveError> {
    if commitments.is_empty() {
        return Err(EnclaveError::BadRequest("Commitments list cannot be empty".into()));
    }

    // Evaluate sum(c_k * index^k)
    let idx_scalar = Scalar::from(index as u64);
    let mut expected_commitment = RistrettoPoint::default();
    let mut current_pow = Scalar::ONE;

    for comm in commitments {
        expected_commitment += comm * current_pow;
        current_pow *= idx_scalar;
    }

    // Compare against value
    let val_scalar = Scalar::from(share_val);
    let actual_commitment = RistrettoPoint::mul_base(&val_scalar);

    if actual_commitment == expected_commitment {
        Ok(true)
    } else {
        Ok(false)
    }
}
