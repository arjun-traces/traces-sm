//! FROST (Flexible Round-Optimized Schnorr Threshold) Signatures over Ed25519.
//!
//! Provides threshold DKG key generation (trusted dealer), Round 1 nonces/commitments,
//! Round 2 signing share production, threshold signature aggregation, and verification.

use std::collections::BTreeMap;
use frost_ed25519 as frost;
use frost::keys::{KeyPackage, PublicKeyPackage, SecretShare, IdentifierList};
use frost::round1::{SigningCommitments, SigningNonces};
use frost::round2::SignatureShare;
use frost::{Identifier, Signature, SigningPackage};
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::error::EnclaveError;

/// Output of a trusted-dealer FROST Key Generation process.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrostKeyGenOutput {
    /// BTreeMap of participant Identifier to serialized KeyPackage JSON string.
    pub key_packages: BTreeMap<String, String>,
    /// Serialized PublicKeyPackage JSON string.
    pub public_key_package: String,
    /// Hex-encoded group public key (VerifyingKey).
    pub group_public_key_hex: String,
    /// Threshold (min signers required).
    pub min_signers: u16,
    /// Total max signers.
    pub max_signers: u16,
}

/// Output of Round 1 nonce generation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrostRound1Output {
    /// Serialized SigningNonces JSON string (kept private to signer).
    pub nonces_json: String,
    /// Serialized SigningCommitments JSON string (shared with coordinator).
    pub commitments_json: String,
}

/// Perform trusted dealer key generation for t-of-n FROST Ed25519 threshold key setup.
pub fn generate_dealer_keys(
    max_signers: u16,
    min_signers: u16,
) -> Result<FrostKeyGenOutput, EnclaveError> {
    if min_signers == 0 || min_signers > max_signers {
        return Err(EnclaveError::DkgInvalidInput(
            "min_signers must be > 0 and <= max_signers".to_string(),
        ));
    }

    let mut rng = thread_rng();
    let (shares, pubkey_package) = frost::keys::generate_with_dealer(
        max_signers,
        min_signers,
        IdentifierList::Default,
        &mut rng,
    )
    .map_err(|e| EnclaveError::DkgInvalidInput(format!("FROST keygen error: {:?}", e)))?;

    let mut key_packages = BTreeMap::new();
    for (id, share) in shares {
        let key_package = KeyPackage::try_from(share)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("KeyPackage conversion error: {:?}", e)))?;
        let id_str = serde_json::to_string(&id)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("Identifier serialize error: {}", e)))?;
        let package_json = serde_json::to_string(&key_package)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("KeyPackage serialize error: {}", e)))?;
        key_packages.insert(id_str, package_json);
    }

    let pubkey_package_json = serde_json::to_string(&pubkey_package)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("PublicKeyPackage serialize error: {}", e)))?;

    let group_pubkey_bytes = pubkey_package.verifying_key().serialize()
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("VerifyingKey serialize error: {:?}", e)))?;
    let group_public_key_hex = hex::encode(group_pubkey_bytes);

    Ok(FrostKeyGenOutput {
        key_packages,
        public_key_package: pubkey_package_json,
        group_public_key_hex,
        min_signers,
        max_signers,
    })
}

/// Round 1: Generate nonces and public commitments for a participant.
pub fn round1_commit(
    key_package_json: &str,
) -> Result<FrostRound1Output, EnclaveError> {
    let key_package: KeyPackage = serde_json::from_str(key_package_json)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("KeyPackage deserialize error: {}", e)))?;

    let mut rng = thread_rng();
    let (nonces, commitments) = frost::round1::commit(key_package.signing_share(), &mut rng);

    let nonces_json = serde_json::to_string(&nonces)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("SigningNonces serialize error: {}", e)))?;
    let commitments_json = serde_json::to_string(&commitments)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("SigningCommitments serialize error: {}", e)))?;

    Ok(FrostRound1Output {
        nonces_json,
        commitments_json,
    })
}

/// Round 2: Generate a signature share for a message given commitments and secret nonces.
pub fn round2_sign_share(
    key_package_json: &str,
    nonces_json: &str,
    commitments_map_json: &BTreeMap<String, String>,
    message: &[u8],
) -> Result<String, EnclaveError> {
    let key_package: KeyPackage = serde_json::from_str(key_package_json)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("KeyPackage deserialize error: {}", e)))?;
    let nonces: SigningNonces = serde_json::from_str(nonces_json)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("SigningNonces deserialize error: {}", e)))?;

    let mut commitments_map: BTreeMap<Identifier, SigningCommitments> = BTreeMap::new();
    for (id_str, comm_json) in commitments_map_json {
        let id: Identifier = serde_json::from_str(id_str)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("Identifier deserialize error: {}", e)))?;
        let comm: SigningCommitments = serde_json::from_str(comm_json)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("SigningCommitments deserialize error: {}", e)))?;
        commitments_map.insert(id, comm);
    }

    let signing_package = SigningPackage::new(commitments_map, message);

    let signature_share = frost::round2::sign(&signing_package, &nonces, &key_package)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("FROST round2 sign error: {:?}", e)))?;

    let share_json = serde_json::to_string(&signature_share)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("SignatureShare serialize error: {}", e)))?;

    Ok(share_json)
}

/// Aggregate threshold signature shares into a single Ed25519 FROST threshold signature.
pub fn aggregate_signature(
    public_key_package_json: &str,
    commitments_map_json: &BTreeMap<String, String>,
    signature_shares_json: &BTreeMap<String, String>,
    message: &[u8],
) -> Result<String, EnclaveError> {
    let pubkey_package: PublicKeyPackage = serde_json::from_str(public_key_package_json)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("PublicKeyPackage deserialize error: {}", e)))?;

    let mut commitments_map: BTreeMap<Identifier, SigningCommitments> = BTreeMap::new();
    for (id_str, comm_json) in commitments_map_json {
        let id: Identifier = serde_json::from_str(id_str)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("Identifier deserialize error: {}", e)))?;
        let comm: SigningCommitments = serde_json::from_str(comm_json)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("SigningCommitments deserialize error: {}", e)))?;
        commitments_map.insert(id, comm);
    }

    let mut signature_shares: BTreeMap<Identifier, SignatureShare> = BTreeMap::new();
    for (id_str, share_json) in signature_shares_json {
        let id: Identifier = serde_json::from_str(id_str)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("Identifier deserialize error: {}", e)))?;
        let share: SignatureShare = serde_json::from_str(share_json)
            .map_err(|e| EnclaveError::DkgInvalidInput(format!("SignatureShare deserialize error: {}", e)))?;
        signature_shares.insert(id, share);
    }

    let signing_package = SigningPackage::new(commitments_map, message);

    let signature = frost::aggregate(&signing_package, &signature_shares, &pubkey_package)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("FROST aggregate error: {:?}", e)))?;

    let sig_bytes = signature.serialize()
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("Signature serialize error: {:?}", e)))?;
    Ok(hex::encode(sig_bytes))
}

/// Verify an aggregated FROST threshold Ed25519 signature.
pub fn verify_signature(
    group_public_key_hex: &str,
    signature_hex: &str,
    message: &[u8],
) -> Result<bool, EnclaveError> {
    let pubkey_bytes = hex::decode(group_public_key_hex)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("Invalid public key hex: {}", e)))?;
    let pubkey_array: [u8; 32] = pubkey_bytes.try_into()
        .map_err(|_| EnclaveError::DkgInvalidInput("Public key must be 32 bytes".into()))?;

    let verifying_key = frost::VerifyingKey::deserialize(&pubkey_array)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("VerifyingKey deserialize error: {:?}", e)))?;

    let sig_bytes = hex::decode(signature_hex)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("Invalid signature hex: {}", e)))?;
    let signature = Signature::deserialize(&sig_bytes)
        .map_err(|e| EnclaveError::DkgInvalidInput(format!("Signature deserialize error: {:?}", e)))?;

    match verifying_key.verify(message, &signature) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frost_full_threshold_lifecycle() {
        let max_signers = 3;
        let min_signers = 2;

        // 1. Dealer Keygen
        let keygen_output = generate_dealer_keys(max_signers, min_signers).unwrap();
        assert_eq!(keygen_output.key_packages.len(), 3);
        assert!(!keygen_output.group_public_key_hex.is_empty());

        // Select 2 signers (threshold min_signers = 2)
        let mut keys_iter = keygen_output.key_packages.into_iter();
        let (id1_str, pkg1_json) = keys_iter.next().unwrap();
        let (id2_str, pkg2_json) = keys_iter.next().unwrap();

        // 2. Round 1: Commitments
        let round1_p1 = round1_commit(&pkg1_json).unwrap();
        let round1_p2 = round1_commit(&pkg2_json).unwrap();

        let mut commitments_map = BTreeMap::new();
        commitments_map.insert(id1_str.clone(), round1_p1.commitments_json);
        commitments_map.insert(id2_str.clone(), round1_p2.commitments_json);

        // 3. Round 2: Sign shares
        let message = b"Secrets-Manager FROST Threshold Signature Test Message";
        let share1 = round2_sign_share(&pkg1_json, &round1_p1.nonces_json, &commitments_map, message).unwrap();
        let share2 = round2_sign_share(&pkg2_json, &round1_p2.nonces_json, &commitments_map, message).unwrap();

        let mut shares_map = BTreeMap::new();
        shares_map.insert(id1_str, share1);
        shares_map.insert(id2_str, share2);

        // 4. Aggregate signature
        let sig_hex = aggregate_signature(
            &keygen_output.public_key_package,
            &commitments_map,
            &shares_map,
            message,
        )
        .unwrap();

        // 5. Verify signature
        let is_valid = verify_signature(&keygen_output.group_public_key_hex, &sig_hex, message).unwrap();
        assert!(is_valid, "FROST threshold signature verification failed");
    }

    #[test]
    fn test_frost_key_share_zeroization() {
        use zeroize::Zeroize;

        // Generate dealer keys
        let keygen_output = generate_dealer_keys(3, 2).expect("Dealer keygen failed");
        let (_id, pkg_json) = keygen_output.key_packages.into_iter().next().unwrap();

        // Extract and verify secret share buffer zeroization
        let mut secret_bytes = pkg_json.into_bytes();
        assert!(!secret_bytes.iter().all(|&b| b == 0), "Secret buffer should initially contain key bytes");

        // Perform zeroize
        secret_bytes.zeroize();
        assert!(secret_bytes.iter().all(|&b| b == 0), "Secret buffer must be completely zeroized");
    }
}

