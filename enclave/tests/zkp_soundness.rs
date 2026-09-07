//! Zero-knowledge proof conformance tests for `enclave/src/zkp/`.
//!
//! Spec references (docs/TECHNICAL_SPECIFICATION.md):
//!   §4.2 "Schnorr Proof-of-Knowledge (PoK) ... Proves knowledge of secret
//!         token s such that C = Hash(s)·G without disclosing s."
//!   §4.2 "Bulletproofs Range Proofs ... Proves numerical values satisfy
//!         min ≤ v ≤ max (e.g. 0 ≤ TTL ≤ 86400) over a 32-bit range."
//!
//! The Schnorr module is the healthiest cryptographic code in the crate.
//! The Bulletproof module has a soundness gap: it proves the lower bound but
//! not the upper bound. TC-BP-005 is the test that demonstrates it.

mod common;

use traces_sm_enclave::zkp::bulletproof::{prove_range, verify_range_proof};
use traces_sm_enclave::zkp::schnorr::{generate_commitment, prove_knowledge, verify_proof};

const SECRET: &[u8] = b"my-super-secret-api-token-value-xyz";
const NONCE: &[u8] = b"random-challenge-nonce-12345";

// ─────────────────────────────────────────────────────────────────────────────
// Schnorr PoK
// ─────────────────────────────────────────────────────────────────────────────

/// TC-ZKP-001 — commitments must be deterministic for a given secret.
#[test]
fn tc_zkp_001_commitment_is_deterministic() {
    let a = generate_commitment(SECRET).expect("commit");
    let b = generate_commitment(SECRET).expect("commit");
    assert_eq!(a.point_hex, b.point_hex);
}

/// TC-ZKP-002 — distinct secrets must yield distinct commitments.
#[test]
fn tc_zkp_002_distinct_secrets_distinct_commitments() {
    let a = generate_commitment(b"secret-one").expect("commit");
    let b = generate_commitment(b"secret-two").expect("commit");
    assert_ne!(a.point_hex, b.point_hex);
}

/// TC-ZKP-003 — completeness: an honest prover always verifies.
#[test]
fn tc_zkp_003_honest_proof_verifies() {
    let c = generate_commitment(SECRET).expect("commit");
    let p = prove_knowledge(SECRET, NONCE).expect("prove");
    assert!(verify_proof(&c, &p, NONCE).expect("verify"));
}

/// TC-ZKP-004 — soundness: a prover without the secret must fail.
#[test]
fn tc_zkp_004_wrong_secret_fails() {
    let c = generate_commitment(SECRET).expect("commit");
    let p = prove_knowledge(b"a-different-secret", NONCE).expect("prove");
    assert!(!verify_proof(&c, &p, NONCE).expect("verify"));
}

/// TC-ZKP-005 — freshness: the nonce must be bound into the transcript.
#[test]
fn tc_zkp_005_nonce_is_bound() {
    let c = generate_commitment(SECRET).expect("commit");
    let p = prove_knowledge(SECRET, NONCE).expect("prove");
    assert!(!verify_proof(&c, &p, b"a-different-nonce").expect("verify"));
}

/// TC-ZKP-006 — zero-knowledge: the proof must not contain the secret.
#[test]
fn tc_zkp_006_proof_does_not_leak_the_secret() {
    let p = prove_knowledge(SECRET, NONCE).expect("prove");
    let bytes = hex::decode(&p.signature_hex).expect("hex");
    assert!(
        !bytes.windows(SECRET.len()).any(|w| w == SECRET),
        "the secret appears verbatim inside the proof"
    );
}

/// TC-ZKP-007 — malformed proofs must be rejected, not panic.
#[test]
fn tc_zkp_007_malformed_input_is_rejected() {
    use traces_sm_enclave::zkp::schnorr::{SchnorrCommitment, SchnorrProof};
    let good_c = generate_commitment(SECRET).expect("commit");
    let good_p = prove_knowledge(SECRET, NONCE).expect("prove");

    let bad_c = SchnorrCommitment { point_hex: "zzzz".into() };
    assert!(verify_proof(&bad_c, &good_p, NONCE).is_err(), "bad commitment hex");

    let short_c = SchnorrCommitment { point_hex: "aabb".into() };
    assert!(verify_proof(&short_c, &good_p, NONCE).is_err(), "short commitment");

    let bad_p = SchnorrProof { signature_hex: "zzzz".into(), context: good_p.context.clone() };
    assert!(verify_proof(&good_c, &bad_p, NONCE).is_err(), "bad proof hex");
}

/// TC-ZKP-008 — replay protection must be enforced by the enclave.
///
/// `prove_knowledge` accepts whatever `challenge_nonce` the caller supplies
/// and the HTTP handler passes it straight through from the request body.
/// A verifier who reuses a nonce accepts a replayed proof. The enclave must
/// issue the challenge itself, bind it to a session, and expire it.
/// See issue ENC-040.
#[test]
#[ignore = "no enclave-issued challenge exists — see issue ENC-040"]
fn tc_zkp_008_replayed_proof_is_rejected() {
    unimplemented!(
        "add a POST /v1/zkp/schnorr/challenge endpoint that mints a \
         single-use nonce, and reject proofs against a spent nonce"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Bulletproof range proofs
// ─────────────────────────────────────────────────────────────────────────────

/// TC-BP-001 — completeness: an in-range value proves and verifies.
#[test]
fn tc_bp_001_in_range_value_verifies() {
    let p = prove_range(500, 100, 1000).expect("prove");
    assert!(verify_range_proof(&p).expect("verify"));
}

/// TC-BP-002 — inclusive boundaries must both prove (§4.2 says min ≤ v ≤ max).
#[test]
fn tc_bp_002_boundaries_are_inclusive() {
    for v in [100u64, 1000u64] {
        let p = prove_range(v, 100, 1000).expect("prove boundary {v}");
        assert!(verify_range_proof(&p).expect("verify"), "boundary {v} failed");
    }
}

/// TC-BP-003 — an out-of-range value must not be provable by the honest API.
#[test]
fn tc_bp_003_out_of_range_cannot_prove() {
    assert!(prove_range(1001, 100, 1000).is_err());
    assert!(prove_range(99, 100, 1000).is_err());
}

/// TC-BP-004 — a tampered commitment must fail verification.
#[test]
fn tc_bp_004_tampered_commitment_fails() {
    let mut p = prove_range(500, 100, 1000).expect("prove");
    let mut bytes = hex::decode(&p.commitment_hex).expect("hex");
    bytes[0] ^= 0xFF;
    p.commitment_hex = hex::encode(bytes);
    assert!(!verify_range_proof(&p).unwrap_or(false));
}

/// TC-BP-005 — SOUNDNESS: the upper bound must actually be enforced.
///
/// `prove_range` proves `0 ≤ (v - min) < 2^32` using a hard-coded
/// `RANGE_BITS = 32`. It never relates `max` to the bit length. `max` is
/// carried in `SerializedRangeProof` as a plain field and
/// `verify_range_proof` never reads it.
///
/// So a prover can pick any `v` in `[min, min + 2^32)` — including values
/// far above `max` — and produce a proof that verifies. The spec's claim
/// ("proves min ≤ v ≤ max") holds only for the lower bound.
///
/// Concretely: the spec's own example is `0 ≤ TTL ≤ 86400`. A prover can
/// commit to a TTL of 4,000,000,000 seconds and pass.
///
/// This test constructs such a proof by calling `prove_range` with a wide
/// range and re-labelling the result with a narrow one — exactly what a
/// malicious client controls, since `min`/`max` travel in the proof body.
/// See issue ENC-041.
#[test]
fn tc_bp_005_upper_bound_is_enforced() {
    // Honest-looking narrow policy window.
    let (policy_min, policy_max) = (0u64, 86_400u64);

    // A value far outside the policy window, but within min + 2^32.
    let cheating_value = 4_000_000_000u64;

    // The prover proves against a range wide enough to accept it...
    let mut proof = prove_range(cheating_value, policy_min, u64::from(u32::MAX))
        .expect("wide-range prove");

    // ...then relabels the public bounds to the narrow policy window.
    proof.max = policy_max;

    let verified = verify_range_proof(&proof).expect("verify");
    assert!(
        !verified,
        "a commitment to {cheating_value} verified against the declared \
         range [{policy_min}, {policy_max}] — verify_range_proof never reads \
         `max`, so the upper bound is unenforced (issue ENC-041)"
    );
}

/// TC-BP-006 — the bit length must be derived from the declared range.
///
/// The fix for ENC-041 is to compute `n = ceil(log2(max - min + 1))`, round
/// it to a power of two supported by the bulletproofs crate, and bind that
/// `n` into the Merlin transcript so prover and verifier cannot disagree
/// about it.
#[test]
#[ignore = "RANGE_BITS is a hard-coded 32 — see issue ENC-041"]
fn tc_bp_006_bit_length_tracks_the_declared_range() {
    unimplemented!("derive n from (max - min) and commit it to the transcript");
}

/// TC-BP-007 — a range wider than 2^32 must be refused, not silently truncated.
#[test]
fn tc_bp_007_oversized_range_is_refused() {
    assert!(
        prove_range(1, 0, u64::MAX).is_err(),
        "a range exceeding 2^32 must be refused rather than proved over a \
         narrower window than the caller asked for"
    );
}

/// TC-BP-008 — an inverted range must be refused.
///
/// `prove_range` checks `value < min || value > max` *before* it checks
/// `max < min`, so an inverted range reports the wrong error. Cosmetic, but
/// the branch is unreachable dead code as written.
#[test]
fn tc_bp_008_inverted_range_is_refused() {
    assert!(prove_range(50, 100, 10).is_err());
}
