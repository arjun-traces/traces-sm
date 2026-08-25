//! Bulletproof range proofs.
//!
//! A range proof proves that a committed value `v` satisfies:
//!
//!   min ≤ v ≤ max
//!
//! without revealing `v` itself.  This is used for example to prove that
//! a token's TTL is still within a valid window, or that a key's size
//! satisfies a policy — without disclosing the actual values.
//!
//! # Protocol
//! We use the `bulletproofs` crate (dalek ecosystem) which implements the
//! Bulletproofs protocol by Bünz et al. (2018).  No trusted setup is
//! required.
//!
//! # Range encoding
//! Bulletproofs natively prove  0 ≤ v < 2^n.  To prove  min ≤ v ≤ max
//! we prove  0 ≤ (v - min) < 2^n  where  2^n > (max - min).

use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

use crate::error::EnclaveError;

// Number of bits used for range proofs.  Must be a power of two ≤ 64.
// 32 bits supports values up to ~4.3 billion (sufficient for TTLs, sizes).
const RANGE_BITS: usize = 32;
// Label for the Merlin transcript — must match between prover and verifier.
const TRANSCRIPT_LABEL: &[u8] = b"sm:bulletproof:range:v1";

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

/// A serialisable range proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedRangeProof {
    /// Bulletproof bytes (hex-encoded).
    pub proof_hex: String,
    /// Pedersen commitment to the adjusted value (hex-encoded Ristretto point).
    pub commitment_hex: String,
    /// The `min` bound (public).
    pub min: u64,
    /// The `max` bound (public).
    pub max: u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// Proof generation (inside enclave — has the plaintext value)
// ─────────────────────────────────────────────────────────────────────────────

/// Prove that `value` is in `[min, max]`.
///
/// Returns a `SerializedRangeProof` that can be sent to an untrusted verifier
/// without revealing `value`.
pub fn prove_range(
    value: u64,
    min: u64,
    max: u64,
) -> Result<SerializedRangeProof, EnclaveError> {
    if value < min || value > max {
        return Err(EnclaveError::ZkpInvalidInput(
            format!("value {value} is not in [{min}, {max}]"),
        ));
    }
    if max < min {
        return Err(EnclaveError::ZkpInvalidInput("max must be ≥ min".into()));
    }

    // Shift: prove 0 ≤ (value - min) < 2^RANGE_BITS
    let v_shifted = value
        .checked_sub(min)
        .ok_or_else(|| EnclaveError::ZkpInvalidInput("underflow in shift".into()))?;

    // Verify the shifted value fits in RANGE_BITS
    if v_shifted >= (1u64 << RANGE_BITS) {
        return Err(EnclaveError::ZkpInvalidInput(format!(
            "range [{min}, {max}] exceeds 2^{RANGE_BITS}"
        )));
    }

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(RANGE_BITS, 1);

    let blinding = Scalar::random(&mut OsRng);
    let mut prover_transcript = Transcript::new(TRANSCRIPT_LABEL);

    let (proof, committed_value) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        v_shifted,
        &blinding,
        RANGE_BITS,
    )
    .map_err(|e| EnclaveError::ZkpProve(format!("bulletproof prove failed: {e:?}")))?;

    Ok(SerializedRangeProof {
        proof_hex: hex::encode(proof.to_bytes()),
        commitment_hex: hex::encode(committed_value.to_bytes()),
        min,
        max,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Proof verification (can run on untrusted side — no secret needed)
// ─────────────────────────────────────────────────────────────────────────────

/// Verify a range proof.
///
/// Returns `true` iff the proof is valid — i.e. the committer knows a value
/// in `[proof.min, proof.max]`.
pub fn verify_range_proof(proof: &SerializedRangeProof) -> Result<bool, EnclaveError> {
    let proof_bytes = hex::decode(&proof.proof_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad proof hex".into()))?;

    let bp_proof = RangeProof::from_bytes(&proof_bytes)
        .map_err(|e| EnclaveError::ZkpInvalidInput(format!("cannot parse proof: {e:?}")))?;

    let commitment_bytes = hex::decode(&proof.commitment_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad commitment hex".into()))?;
    let commitment_arr: [u8; 32] = commitment_bytes
        .try_into()
        .map_err(|_| EnclaveError::ZkpInvalidInput("commitment must be 32 bytes".into()))?;
    let committed_value = CompressedRistretto(commitment_arr);

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(RANGE_BITS, 1);

    let mut verifier_transcript = Transcript::new(TRANSCRIPT_LABEL);

    match bp_proof.verify_single(
        &bp_gens,
        &pc_gens,
        &mut verifier_transcript,
        &committed_value,
        RANGE_BITS,
    ) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_proof_verifies() {
        let proof = prove_range(500, 100, 1000).unwrap();
        assert!(verify_range_proof(&proof).unwrap());
    }

    #[test]
    fn boundary_values_prove_and_verify() {
        for &v in &[100u64, 1000u64] {
            let proof = prove_range(v, 100, 1000).unwrap();
            assert!(verify_range_proof(&proof).unwrap());
        }
    }

    #[test]
    fn out_of_range_value_fails_to_prove() {
        assert!(prove_range(1001, 100, 1000).is_err());
        assert!(prove_range(99, 100, 1000).is_err());
    }

    #[test]
    fn tampered_commitment_fails_verify() {
        let mut proof = prove_range(500, 100, 1000).unwrap();
        // Flip a byte in the commitment
        let mut bytes = hex::decode(&proof.commitment_hex).unwrap();
        bytes[0] ^= 0xFF;
        proof.commitment_hex = hex::encode(bytes);
        assert!(!verify_range_proof(&proof).unwrap_or(false));
    }
}
