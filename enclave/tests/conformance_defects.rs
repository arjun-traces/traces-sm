//! Conformance Defect Proving & Regression Test Suite
//!
//! Enforces spec clauses from `docs/TECHNICAL_SPECIFICATION.md` against `traces-sm-enclave`.

use traces_sm_enclave::drbg::HmacDrbg;
use traces_sm_enclave::nist::Zeroizing;
use traces_sm_enclave::sealing::{seal, unseal, SealingKeyProvider};
use traces_sm_enclave::zkp::bulletproof::{prove_range, verify_range_proof, SerializedRangeProof};
use traces_sm_enclave::error::EnclaveError;

struct TestSealingProvider([u8; 32]);
impl SealingKeyProvider for TestSealingProvider {
    fn master_key(&self) -> Result<Zeroizing<[u8; 32]>, EnclaveError> {
        Ok(Zeroizing::new(self.0))
    }
}

/// TC-SEAL-001: Enforces that `unseal()` returns only the plaintext slice and strips the 16-byte tag.
/// Spec Clause: TECHNICAL_SPECIFICATION.md §6.1
#[test]
fn tc_seal_001_unseal_returns_exact_plaintext_slice() {
    let provider = TestSealingProvider([0x77; 32]);
    let plaintext = b"test secret payload 12345";
    let sealed_blob = seal(plaintext, "seal:test:tc_seal_001", &provider)
        .expect("sealing plaintext should succeed");

    let unsealed_bytes = unseal(&sealed_blob, "seal:test:tc_seal_001", &provider)
        .expect("unsealing blob should succeed");

    assert_eq!(
        unsealed_bytes.as_slice(),
        plaintext,
        "TC-SEAL-001: unsealed bytes must exactly match plaintext without trailing tag"
    );
}

/// TC-DRBG-001: Enforces that HMAC_DRBG generates non-deterministic output not tied to subsec_nanos step.
/// Spec Clause: TECHNICAL_SPECIFICATION.md §6.2
#[test]
fn tc_drbg_001_drbg_randomness_generation() {
    let mut drbg = HmacDrbg::new();
    let mut buf1 = [0u8; 32];
    let mut buf2 = [0u8; 32];

    drbg.generate(&mut buf1);
    drbg.generate(&mut buf2);

    assert_ne!(
        buf1, buf2,
        "TC-DRBG-001: consecutive DRBG outputs must be non-deterministic"
    );
}

/// TC-ZKP-001: Enforces that Bulletproof range proofs reject upper bound overflow / invalid range span.
/// Spec Clause: TECHNICAL_SPECIFICATION.md §4.2
#[test]
fn tc_zkp_001_range_proof_bounds_validation() {
    // Valid proof
    let valid_proof = prove_range(500, 100, 1000).expect("prove range 500 in [100, 1000]");
    let verified = verify_range_proof(&valid_proof).expect("verification result");
    assert!(verified, "TC-ZKP-001: valid proof must verify");

    // Hostile proof with min > max
    let invalid_span_proof = SerializedRangeProof {
        proof_hex: valid_proof.proof_hex.clone(),
        commitment_hex: valid_proof.commitment_hex.clone(),
        min: 1000,
        max: 100,
    };
    assert!(
        verify_range_proof(&invalid_span_proof).is_err(),
        "TC-ZKP-001: range proof with min > max must return invalid input error"
    );
}
