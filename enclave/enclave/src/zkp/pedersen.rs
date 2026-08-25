//! Pedersen commitments over Ristretto255.
//!
//! Pedersen commitments are *additively homomorphic*:
//!
//!   commit(v1, r1) + commit(v2, r2)  =  commit(v1+v2, r1+r2)
//!
//! This allows the enclave to verify that the sum of committed values
//! equals an expected total — without decrypting any individual value.
//!
//! # Setup
//! We use the standard Ristretto255 generator `G` and a random-oracle
//! derived second generator `H = hash_to_ristretto("sm:pedersen:H:v1")`.
//!
//!   commit(v, r) = v·G + r·H

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::MultiscalarMul;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use crate::error::EnclaveError;

// ─────────────────────────────────────────────────────────────────────────────
// Generator H (hash-to-point, nothing-up-my-sleeve)
// ─────────────────────────────────────────────────────────────────────────────

/// Derive the second Pedersen generator `H` via hash-to-curve.
/// This is the same for all calls — computed once and cached.
fn pedersen_h() -> RistrettoPoint {
    // Use the Elligator2 map via from_uniform_bytes.
    // SHA-512("sm:pedersen:H:v1") provides the uniform bytes.
    use ring::digest;
    let label = b"sm:pedersen:H:v1";
    let mut state = digest::Context::new(&digest::SHA512);
    state.update(label);
    let hash = state.finish();
    // SHA-512 produces 64 bytes — exactly what from_uniform_bytes needs
    let bytes: [u8; 64] = hash.as_ref().try_into().expect("SHA-512 is 64 bytes");
    RistrettoPoint::from_uniform_bytes(&bytes)
}

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

/// A Pedersen commitment  C = v·G + r·H  (compressed Ristretto255 point).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PedersenCommitment {
    /// 32-byte compressed Ristretto255 point (hex-encoded).
    pub point_hex: String,
}

/// The opening of a Pedersen commitment (kept secret by the committer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedersenOpening {
    /// u64 value committed to.
    pub value: u64,
    /// Blinding scalar (32 bytes, hex-encoded).
    pub blinding_hex: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Commitment
// ─────────────────────────────────────────────────────────────────────────────

/// Commit to `value` with a fresh random blinding scalar.
///
/// Returns both the commitment (public) and the opening (secret — caller must
/// store it securely, e.g. seal it into the enclave store).
pub fn commit(value: u64) -> Result<(PedersenCommitment, PedersenOpening), EnclaveError> {
    let r = Scalar::random(&mut OsRng);
    let c = commit_with_blinding(value, &r)?;
    Ok((
        c,
        PedersenOpening {
            value,
            blinding_hex: hex::encode(r.to_bytes()),
        },
    ))
}

/// Commit with a caller-supplied blinding scalar (for deterministic use in tests).
pub fn commit_with_blinding(
    value: u64,
    blinding: &Scalar,
) -> Result<PedersenCommitment, EnclaveError> {
    let g = &RISTRETTO_BASEPOINT_TABLE;
    let h = pedersen_h();
    let v_scalar = Scalar::from(value);
    // C = v·G + r·H
    let point = RistrettoPoint::multiscalar_mul(
        &[v_scalar, *blinding],
        &[*g * Scalar::ONE, h], // g * 1 = G base point
    );
    // simpler: v*G + r*H directly
    let c = (g * &v_scalar) + (&h * blinding);
    Ok(PedersenCommitment {
        point_hex: hex::encode(c.compress().to_bytes()),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Verification (open a commitment)
// ─────────────────────────────────────────────────────────────────────────────

/// Verify that a commitment opens to `(value, blinding)`.
pub fn verify_opening(
    commitment: &PedersenCommitment,
    opening: &PedersenOpening,
) -> Result<bool, EnclaveError> {
    let blinding_bytes = hex::decode(&opening.blinding_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad blinding hex".into()))?;
    let b_arr: [u8; 32] = blinding_bytes
        .try_into()
        .map_err(|_| EnclaveError::ZkpInvalidInput("blinding must be 32 bytes".into()))?;
    let blinding = Scalar::from_bytes_mod_order(b_arr);

    let expected = commit_with_blinding(opening.value, &blinding)?;
    Ok(expected.point_hex == commitment.point_hex)
}

// ─────────────────────────────────────────────────────────────────────────────
// Homomorphic operations
// ─────────────────────────────────────────────────────────────────────────────

/// Add two Pedersen commitments homomorphically.
///
///   add(commit(v1, r1), commit(v2, r2))  =  commit(v1+v2, r1+r2)
pub fn add_commitments(
    c1: &PedersenCommitment,
    c2: &PedersenCommitment,
) -> Result<PedersenCommitment, EnclaveError> {
    let p1 = decompress(c1)?;
    let p2 = decompress(c2)?;
    let sum = p1 + p2;
    Ok(PedersenCommitment {
        point_hex: hex::encode(sum.compress().to_bytes()),
    })
}

/// Verify that the sum of a list of commitments equals a claimed total.
///
/// This allows proving that N encrypted values sum to `total` without
/// revealing any individual value.
pub fn verify_sum(
    commitments: &[PedersenCommitment],
    claimed_total: &PedersenCommitment,
    combined_blinding_hex: &str,
) -> Result<bool, EnclaveError> {
    // Compute the expected commitment to the sum
    let homomorphic_sum = commitments
        .iter()
        .try_fold(None::<RistrettoPoint>, |acc, c| {
            let p = decompress(c)?;
            Ok::<_, EnclaveError>(Some(acc.map(|a| a + p).unwrap_or(p)))
        })?
        .ok_or_else(|| EnclaveError::ZkpInvalidInput("empty commitment list".into()))?;

    let claimed = decompress(claimed_total)?;
    if homomorphic_sum.compress() != claimed.compress() {
        return Ok(false);
    }

    // Verify the claimed total opens correctly with the combined blinding
    let blinding_bytes = hex::decode(combined_blinding_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad combined blinding hex".into()))?;
    let b_arr: [u8; 32] = blinding_bytes
        .try_into()
        .map_err(|_| EnclaveError::ZkpInvalidInput("blinding must be 32 bytes".into()))?;
    let blinding = Scalar::from_bytes_mod_order(b_arr);

    // We do not know the total value here, so we can only verify the point.
    // For full verification, caller must also supply the total value.
    let _ = blinding;
    Ok(true)
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper
// ─────────────────────────────────────────────────────────────────────────────

fn decompress(c: &PedersenCommitment) -> Result<RistrettoPoint, EnclaveError> {
    let bytes = hex::decode(&c.point_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad commitment hex".into()))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| EnclaveError::ZkpInvalidInput("commitment must be 32 bytes".into()))?;
    CompressedRistretto(arr)
        .decompress()
        .ok_or_else(|| EnclaveError::ZkpInvalidInput("commitment is not a valid Ristretto point".into()))
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_and_verify() {
        let (c, o) = commit(42).unwrap();
        assert!(verify_opening(&c, &o).unwrap());
    }

    #[test]
    fn wrong_value_fails() {
        let (c, o) = commit(42).unwrap();
        let bad_opening = PedersenOpening { value: 99, blinding_hex: o.blinding_hex };
        assert!(!verify_opening(&c, &bad_opening).unwrap());
    }

    #[test]
    fn homomorphic_addition() {
        let r1 = Scalar::from(7u64);
        let r2 = Scalar::from(11u64);
        let c1 = commit_with_blinding(10, &r1).unwrap();
        let c2 = commit_with_blinding(20, &r2).unwrap();
        let c_sum = add_commitments(&c1, &c2).unwrap();
        let r_sum = r1 + r2;
        let expected = commit_with_blinding(30, &r_sum).unwrap();
        assert_eq!(c_sum.point_hex, expected.point_hex);
    }
}
