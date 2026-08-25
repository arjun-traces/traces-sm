//! Schnorr Proof-of-Knowledge (PoK).
//!
//! # What this proves
//! A prover demonstrates knowledge of a secret `s` such that:
//!
//!   commitment = Hash(s)  AND  the corresponding Schnorr keypair is valid
//!
//! Concretely, we treat `s` as a 32-byte seed, derive a Ristretto255
//! keypair, and use a Schnorr signature as the PoK.  The *commitment* is
//! the public key (compressed Ristretto point), which is stored in the
//! enclave alongside the sealed secret.
//!
//! # Verification
//! The verifier holds only the commitment (public key) and the proof
//! (Schnorr signature over the transcript).  It can verify without ever
//! seeing `s`.
//!
//! # Crate
//! Uses `schnorrkel` (dalek ecosystem — Ristretto255, uniform-random
//! signing, batch verification).

use schnorrkel::{ExpansionMode, MiniSecretKey, PublicKey, Signature};
use serde::{Deserialize, Serialize};

use crate::error::EnclaveError;

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

/// The commitment stored alongside a secret in the enclave store.
/// It is the compressed Ristretto255 public key derived from the secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchnorrCommitment {
    /// 32-byte compressed Ristretto255 point (hex-encoded for JSON).
    pub point_hex: String,
}

/// A Schnorr proof-of-knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchnorrProof {
    /// 64-byte Schnorr signature bytes (hex-encoded).
    pub signature_hex: String,
    /// The context label used when generating the proof.
    pub context: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// Commitment generation (stored when a secret is created)
// ─────────────────────────────────────────────────────────────────────────────

/// Derive a Ristretto255 commitment from `secret_bytes`.
///
/// The commitment is stable for a given secret and serves as the public
/// binding that a ZKP will later be checked against.
pub fn generate_commitment(secret_bytes: &[u8]) -> Result<SchnorrCommitment, EnclaveError> {
    let mini = derive_mini_secret(secret_bytes)?;
    let kp = mini.expand_to_keypair(ExpansionMode::Ed25519);
    Ok(SchnorrCommitment {
        point_hex: hex::encode(kp.public.to_bytes()),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Proof generation (runs INSIDE the enclave — has access to the secret)
// ─────────────────────────────────────────────────────────────────────────────

/// Generate a Schnorr PoK for the given `secret_bytes`.
///
/// `challenge_nonce` is a fresh random value supplied by the verifier
/// (or the enclave itself for non-interactive proofs).  It is bound into
/// the transcript to prevent replay.
pub fn prove_knowledge(
    secret_bytes: &[u8],
    challenge_nonce: &[u8],
) -> Result<SchnorrProof, EnclaveError> {
    let mini = derive_mini_secret(secret_bytes)?;
    let kp = mini.expand_to_keypair(ExpansionMode::Ed25519);

    // Sign the challenge nonce with the key derived from the secret.
    // The signature IS the proof: it can only be created by someone who
    // knows the secret (and hence the private key).
    let ctx = schnorrkel::signing_context(b"sm:zkp:schnorr:v1");
    let sig = kp.sign(ctx.bytes(challenge_nonce));

    Ok(SchnorrProof {
        signature_hex: hex::encode(sig.to_bytes()),
        context: "sm:zkp:schnorr:v1".to_string(),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Proof verification (verifier only needs the commitment)
// ─────────────────────────────────────────────────────────────────────────────

/// Verify that a `proof` was produced by someone who knows the secret
/// corresponding to `commitment`.
///
/// `challenge_nonce` must be the same value used in `prove_knowledge`.
pub fn verify_proof(
    commitment: &SchnorrCommitment,
    proof: &SchnorrProof,
    challenge_nonce: &[u8],
) -> Result<bool, EnclaveError> {
    let pubkey_bytes = hex::decode(&commitment.point_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad commitment hex".into()))?;

    let pubkey = PublicKey::from_bytes(&pubkey_bytes)
        .map_err(|_| EnclaveError::ZkpInvalidInput("cannot parse commitment as Ristretto point".into()))?;

    let sig_bytes = hex::decode(&proof.signature_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad proof hex".into()))?;

    let sig = Signature::from_bytes(&sig_bytes)
        .map_err(|_| EnclaveError::ZkpInvalidInput("cannot parse Schnorr signature".into()))?;

    let ctx = schnorrkel::signing_context(b"sm:zkp:schnorr:v1");
    Ok(pubkey.verify(ctx.bytes(challenge_nonce), &sig).is_ok())
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Deterministically derive a `MiniSecretKey` from arbitrary-length bytes
/// via SHA-512 hashing (schnorrkel requires exactly 32 bytes).
fn derive_mini_secret(bytes: &[u8]) -> Result<MiniSecretKey, EnclaveError> {
    use ring::digest;
    let hash = digest::digest(&digest::SHA512, bytes);
    // Take the first 32 bytes of SHA-512 output
    let seed: [u8; 32] = hash.as_ref()[..32].try_into()
        .map_err(|_| EnclaveError::ZkpProve("SHA-512 output too short".into()))?;
    MiniSecretKey::from_bytes(&seed)
        .map_err(|_| EnclaveError::ZkpProve("cannot create MiniSecretKey from seed".into()))
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"my-super-secret-api-token-value-xyz";
    const NONCE:  &[u8] = b"random-challenge-nonce-12345";

    #[test]
    fn commitment_is_deterministic() {
        let c1 = generate_commitment(SECRET).unwrap();
        let c2 = generate_commitment(SECRET).unwrap();
        assert_eq!(c1.point_hex, c2.point_hex);
    }

    #[test]
    fn proof_verifies_with_correct_secret() {
        let commitment = generate_commitment(SECRET).unwrap();
        let proof = prove_knowledge(SECRET, NONCE).unwrap();
        assert!(verify_proof(&commitment, &proof, NONCE).unwrap());
    }

    #[test]
    fn proof_fails_with_wrong_secret() {
        let commitment = generate_commitment(SECRET).unwrap();
        let wrong_proof = prove_knowledge(b"different-secret", NONCE).unwrap();
        assert!(!verify_proof(&commitment, &wrong_proof, NONCE).unwrap());
    }

    #[test]
    fn proof_fails_with_wrong_nonce() {
        let commitment = generate_commitment(SECRET).unwrap();
        let proof = prove_knowledge(SECRET, NONCE).unwrap();
        assert!(!verify_proof(&commitment, &proof, b"different-nonce").unwrap());
    }
}
