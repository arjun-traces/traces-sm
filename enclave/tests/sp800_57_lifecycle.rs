//! NIST SP 800-57 / SP 800-108 conformance tests for `enclave/src/nist.rs`
//! and `enclave/src/policy.rs`.
//!
//! Spec references (docs/TECHNICAL_SPECIFICATION.md):
//!   §1.2 "4-phase key lifecycle state machine (PreOperational, Operational,
//!         Deactivated, Expired, Revoked, Destroyed), cryptoperiod volume
//!         limits (2^32 bytes for AES-GCM), and explicit KeyUsage bitmask
//!         validation."
//!   §6.3 "Enforces state transitions ... Rejects signing/encryption on
//!         Deactivated keys while permitting historical decryption."
//!   §1.2 "SP 800-108: KDF in Counter Mode."
//!
//! STATUS: the state-machine and KeyUsage tests cannot compile today because
//! no transition or usage-validation function exists — `nist.rs` defines the
//! enums and nothing else. They are written against the API the spec
//! requires and marked `#[ignore]` with the issue that blocks them.

mod common;

use traces_sm_enclave::nist::{sp800_108_kdf, CryptoPeriod, KeyLifecycleState, KeyUsage};
use traces_sm_enclave::policy::{PolicyEngine, SecurityPolicy};

const MAX_BYTES: u64 = 4_294_967_296; // 2^32

// ─────────────────────────────────────────────────────────────────────────────
// Lifecycle state machine (§6.3)
// ─────────────────────────────────────────────────────────────────────────────

/// TC-LC-001 — the six states named in §1.2 must all exist.
///
/// This is the only lifecycle test that passes today: the enum exists, but
/// nothing enforces movement between its variants.
#[test]
fn tc_lc_001_all_six_states_exist() {
    let all = [
        KeyLifecycleState::PreOperational,
        KeyLifecycleState::Operational,
        KeyLifecycleState::Deactivated,
        KeyLifecycleState::Expired,
        KeyLifecycleState::Revoked,
        KeyLifecycleState::Destroyed,
    ];
    assert_eq!(all.len(), 6);
    assert_ne!(all[0], all[1], "states must be distinguishable");
}

/// TC-LC-002 — legal forward transitions must be accepted.
///
/// Required API: `KeyLifecycleState::can_transition_to(&self, next) -> bool`.
#[test]
#[ignore = "no transition function exists in nist.rs — see issue ENC-020"]
fn tc_lc_002_legal_transitions_are_accepted() {
    unimplemented!(
        "implement KeyLifecycleState::can_transition_to and assert: \
         PreOperational->Operational, Operational->Deactivated, \
         Deactivated->Destroyed, Operational->Revoked, Operational->Expired"
    );
}

/// TC-LC-003 — illegal transitions must be rejected.
///
/// Destroyed is terminal; a destroyed key must never return to Operational.
/// Without this the crypto-shred endpoint is reversible in metadata.
#[test]
#[ignore = "no transition function exists in nist.rs — see issue ENC-020"]
fn tc_lc_003_illegal_transitions_are_rejected() {
    unimplemented!(
        "assert Destroyed->Operational, Destroyed->Deactivated and \
         Revoked->Operational are all rejected"
    );
}

/// TC-LC-004 — Deactivated keys must reject sign/encrypt but permit decrypt.
///
/// This is the single behavioural sentence §6.3 commits to, and it is the
/// clause the CONFORMANCE_REPORT cites as evidence for "✅ 100% CONFORMANT".
/// No code implements it.
#[test]
#[ignore = "no usage-validation function exists — see issue ENC-020"]
fn tc_lc_004_deactivated_key_permits_decrypt_only() {
    unimplemented!(
        "implement validate_usage(state, op) and assert Deactivated rejects \
         Sign and Encrypt while allowing Decrypt and Verify"
    );
}

/// TC-LC-005 — the KeyUsage bitmask must actually gate operations.
///
/// §1.2 requires "explicit KeyUsage bitmask validation". `KeyUsage` is a
/// plain struct of seven bools that no code path reads.
#[test]
fn tc_lc_005_default_key_usage_grants_nothing() {
    let u = KeyUsage::default();
    assert!(
        !(u.sign || u.verify || u.encrypt || u.decrypt || u.key_wrap || u.derive_key || u.authenticate),
        "KeyUsage::default() must be deny-by-default"
    );
}

/// TC-LC-006 — a key without the `sign` flag must be refused for signing.
#[test]
#[ignore = "KeyUsage is never consulted by any caller — see issue ENC-021"]
fn tc_lc_006_usage_bitmask_gates_signing() {
    unimplemented!("wire KeyUsage into keygen::sign and assert refusal");
}

// ─────────────────────────────────────────────────────────────────────────────
// Cryptoperiod (§1.2)
// ─────────────────────────────────────────────────────────────────────────────

/// TC-CP-001 — usage below the limit must be permitted.
#[test]
fn tc_cp_001_under_limit_is_permitted() {
    let mut cp = CryptoPeriod::default();
    assert_eq!(cp.max_bytes, MAX_BYTES, "AES-GCM limit must be 2^32 bytes");
    assert!(cp.process(1024), "1 KiB must be under the cryptoperiod limit");
}

/// TC-CP-002 — the boundary must be defined identically in both enforcers.
///
/// `CryptoPeriod::process` returns `bytes_processed <= max_bytes` (allows
/// exactly 2^32). `PolicyEngine::validate_cryptoperiod` rejects
/// `bytes_processed >= max_bytes` (denies exactly 2^32). Exactly 2^32 bytes
/// is simultaneously allowed and denied depending on which one you ask.
/// See issue ENC-022.
#[test]
fn tc_cp_002_boundary_is_consistent_across_enforcers() {
    let mut cp = CryptoPeriod::default();
    let counter_allows = cp.process(MAX_BYTES);

    let engine = PolicyEngine::new(SecurityPolicy::default());
    let policy_allows = engine.validate_cryptoperiod(MAX_BYTES).is_ok();

    assert_eq!(
        counter_allows, policy_allows,
        "CryptoPeriod::process and PolicyEngine::validate_cryptoperiod \
         disagree at exactly 2^32 bytes ({counter_allows} vs {policy_allows}) \
         — issue ENC-022"
    );
}

/// TC-CP-003 — exceeding the limit must be refused.
#[test]
fn tc_cp_003_over_limit_is_refused() {
    let mut cp = CryptoPeriod::default();
    assert!(!cp.process(MAX_BYTES + 1), "exceeding 2^32 bytes must be refused");

    let engine = PolicyEngine::new(SecurityPolicy::default());
    assert!(engine.validate_cryptoperiod(MAX_BYTES + 1).is_err());
}

/// TC-CP-004 — a refused operation must not consume budget.
///
/// `process()` adds to `bytes_processed` before checking, so a rejected
/// call still burns the counter. A caller that retries after a refusal
/// permanently inflates the total. See issue ENC-023.
#[test]
fn tc_cp_004_refused_operation_does_not_consume_budget() {
    let mut cp = CryptoPeriod::default();
    assert!(!cp.process(MAX_BYTES + 1), "oversized request must be refused");
    assert_eq!(
        cp.bytes_processed, 0,
        "a refused operation consumed {} bytes of cryptoperiod budget \
         (issue ENC-023)",
        cp.bytes_processed
    );
}

/// TC-CP-005 — the counter must not overflow into a panic or a wrap.
///
/// `bytes_processed += bytes` panics in debug and wraps in release. A wrap
/// resets the counter to near zero and silently re-grants an exhausted key
/// an unlimited cryptoperiod.
#[test]
fn tc_cp_005_counter_saturates_instead_of_wrapping() {
    let mut cp = CryptoPeriod::default();
    cp.bytes_processed = u64::MAX - 1;
    let allowed = cp.process(1000); // must not panic, must not wrap
    assert!(!allowed, "an exhausted key must stay exhausted");
    assert!(
        cp.bytes_processed >= u64::MAX - 1,
        "counter wrapped to {} — an exhausted key was silently reset \
         (issue ENC-024)",
        cp.bytes_processed
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// SP 800-108 KDF in Counter Mode
// ─────────────────────────────────────────────────────────────────────────────

/// TC-KDF-001 — output length must be honoured exactly.
#[test]
fn tc_kdf_001_output_length_is_exact() {
    for len in [1usize, 16, 32, 33, 64, 100] {
        let okm = sp800_108_kdf(b"key-material", b"label", b"context", len);
        assert_eq!(okm.as_ref().len(), len, "KDF returned wrong length for L={len}");
    }
}

/// TC-KDF-002 — derivation must be deterministic.
#[test]
fn tc_kdf_002_is_deterministic() {
    let a = sp800_108_kdf(b"ki", b"label", b"ctx", 32);
    let b = sp800_108_kdf(b"ki", b"label", b"ctx", 32);
    assert_eq!(a.as_ref(), b.as_ref(), "KDF is not deterministic");
}

/// TC-KDF-003 — label and context must be domain-separating.
#[test]
fn tc_kdf_003_label_and_context_separate_domains() {
    let base = sp800_108_kdf(b"ki", b"label", b"ctx", 32).as_ref().clone();
    let other_label = sp800_108_kdf(b"ki", b"LABEL", b"ctx", 32).as_ref().clone();
    let other_ctx = sp800_108_kdf(b"ki", b"label", b"CTX", 32).as_ref().clone();
    let other_ki = sp800_108_kdf(b"KI", b"label", b"ctx", 32).as_ref().clone();

    assert_ne!(base, other_label, "changing Label did not change the output");
    assert_ne!(base, other_ctx, "changing Context did not change the output");
    assert_ne!(base, other_ki, "changing Ki did not change the output");
}

/// TC-KDF-004 — L must be bound into the PRF input (SP 800-108 §5.1).
///
/// The fixed input string is [i]₂ || Label || 0x00 || Context || [L]₂.
/// Because L is included, a 32-byte derivation and the first 32 bytes of a
/// 64-byte derivation must differ. If they match, L is not bound and two
/// different requests can produce overlapping keying material.
#[test]
fn tc_kdf_004_length_is_bound_into_the_prf() {
    let short = sp800_108_kdf(b"ki", b"label", b"ctx", 32).as_ref().clone();
    let long = sp800_108_kdf(b"ki", b"label", b"ctx", 64).as_ref().clone();
    assert_ne!(
        short[..],
        long[..32],
        "L is not bound into the PRF input — derivations of different \
         lengths share a prefix (SP 800-108 §5.1)"
    );
}

/// TC-KDF-005 — the returned wrapper must actually zeroize.
///
/// `nist.rs` defines its own `Zeroizing<T>` whose `Drop` impl is an empty
/// body with the comment "In a real implementation this would securely wipe
/// the memory". It shadows `zeroize::Zeroizing` at every call site in that
/// module, so the SP 800-108 KDF output — key material — is left in freed
/// heap memory. This is the direct contradiction of CONFORMANCE_REPORT §2.5.
/// See issue ENC-025.
#[test]
#[ignore = "nist::Zeroizing::drop is an empty stub — see issue ENC-025"]
fn tc_kdf_005_derived_material_is_zeroized_on_drop() {
    unimplemented!(
        "delete nist::Zeroizing and return zeroize::Zeroizing<Vec<u8>>, \
         then assert the buffer is cleared"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Policy engine (§2.4 of the conformance report)
// ─────────────────────────────────────────────────────────────────────────────

/// TC-POL-001 — unencrypted storage must be refused.
#[test]
fn tc_pol_001_unencrypted_storage_is_refused() {
    let engine = PolicyEngine::new(SecurityPolicy::default());
    assert!(engine.validate_in_storage_protection(false).is_err());
    assert!(engine.validate_in_storage_protection(true).is_ok());
}

/// TC-POL-002 — `validate_in_memory_protection` must verify something.
///
/// It currently reads its own configuration flag and returns `Ok` if the flag
/// is `true`. It takes no argument, inspects no state, and cannot fail for
/// any reason other than being switched off. It is a tautology presented as
/// an enforcement control. See issue ENC-026.
#[test]
#[ignore = "validate_in_memory_protection is a tautology — see issue ENC-026"]
fn tc_pol_002_memory_protection_check_is_substantive() {
    unimplemented!(
        "give validate_in_memory_protection an observable input (e.g. the \
         SGX mode + a zeroization self-test result) so it can return Err"
    );
}

/// TC-POL-003 — every declared policy flag must have an enforcement path.
///
/// `enforce_dkg_threshold` and `enforce_attestation_check` are declared,
/// defaulted to `true`, and never read anywhere in the crate. A policy that
/// nothing consults is documentation, not enforcement. See issue ENC-027.
#[test]
#[ignore = "enforce_dkg_threshold / enforce_attestation_check are never read — issue ENC-027"]
fn tc_pol_003_all_policy_flags_are_enforced() {
    unimplemented!("add validate_dkg_threshold and validate_attestation to PolicyEngine");
}
