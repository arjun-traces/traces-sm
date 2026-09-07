//! Threshold secret sharing conformance tests for `enclave/src/dkg.rs`.
//!
//! Spec references (docs/TECHNICAL_SPECIFICATION.md):
//!   §5.2.1 "Secret splitting supports full key payload slices (&[u8]) via
//!           split_secret_bytes() as well as single byte-level split_secret(),
//!           evaluating Shamir's Secret Sharing over GF(256)."
//!   §5.2.2 "Polynomial coefficient generation is strictly driven by the
//!           enclave's NIST SP 800-90A HMAC_DRBG (drbg.rs), eliminating
//!           dependency on OS PRNGs (rand::thread_rng())."
//!   §5.2.5 "Node j verifies received shares against commitments:
//!           s_ij·G + r_ij·H == Σ j^k · C_ik"

mod common;

use traces_sm_enclave::dkg::{
    reconstruct_secret, reconstruct_secret_bytes, split_secret, split_secret_bytes,
    split_secret_vss, verify_vss_commitment, KeyShare,
};

// ─────────────────────────────────────────────────────────────────────────────
// Shamir over GF(256) — correctness
// ─────────────────────────────────────────────────────────────────────────────

/// TC-DKG-001 — any threshold-sized subset must reconstruct the secret.
///
/// The in-file unit test only checks two hand-picked subsets. This checks
/// every C(5,3) combination.
#[test]
fn tc_dkg_001_every_threshold_subset_reconstructs() {
    let secret = b"this_is_a_32_byte_secret_key_123";
    let (t, n) = (3usize, 5usize);
    let shares = split_secret_bytes(secret, t, n);
    assert_eq!(shares.len(), n);

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let subset = vec![shares[i].clone(), shares[j].clone(), shares[k].clone()];
                let got = reconstruct_secret_bytes(&subset, t).expect("reconstruct");
                assert_eq!(
                    got, secret,
                    "subset ({i},{j},{k}) failed to reconstruct the secret"
                );
            }
        }
    }
}

/// TC-DKG-002 — fewer than `threshold` shares must reveal nothing.
///
/// Information-theoretic security is the entire point of Shamir. With t-1
/// shares the reconstruction must fail outright, not return a wrong answer
/// that a caller might mistake for the secret.
#[test]
fn tc_dkg_002_below_threshold_is_refused() {
    let secret = b"secret-payload-16";
    let shares = split_secret_bytes(secret, 3, 5);
    let two = vec![shares[0].clone(), shares[1].clone()];
    assert!(
        reconstruct_secret_bytes(&two, 3).is_err(),
        "reconstruction succeeded with fewer shares than the threshold"
    );
}

/// TC-DKG-003 — duplicate shares must be rejected, not silently mis-solved.
///
/// Lagrange interpolation divides by (x_i - x_j). Two shares with the same x
/// make that term zero; `gf256_inv(0)` returns 0, so the basis polynomial
/// collapses and the function returns a plausible-looking wrong secret with
/// no error. A caller that pads a request with a duplicate share to reach
/// the threshold gets garbage that it will happily treat as key material.
/// See issue ENC-030.
#[test]
fn tc_dkg_003_duplicate_x_coordinates_are_rejected() {
    let secret = b"duplicate-share-probe";
    let shares = split_secret_bytes(secret, 3, 5);
    let padded = vec![shares[0].clone(), shares[1].clone(), shares[1].clone()];

    match reconstruct_secret_bytes(&padded, 3) {
        Err(_) => {} // correct: duplicate detected
        Ok(v) => panic!(
            "duplicate x-coordinates produced a silent wrong answer \
             ({v:?} != {secret:?}) instead of an error — issue ENC-030"
        ),
    }
}

/// TC-DKG-004 — `total` above 255 must be refused.
///
/// x-coordinates live in GF(256) and are cast with `x as u8`. At total=256
/// the last share wraps to x=0, and f(0) *is the secret*. That share alone
/// reconstructs everything, silently reducing an M-of-N scheme to 1-of-N.
/// See issue ENC-031.
#[test]
fn tc_dkg_004_share_count_above_255_is_refused() {
    let secret = b"wraparound-probe";
    let shares = split_secret_bytes(secret, 3, 256);

    if !shares.is_empty() {
        let leaked = shares.iter().any(|s| s.x == 0 && s.y == secret);
        assert!(
            !leaked,
            "share at x=0 equals the plaintext secret — an M-of-N scheme \
             collapsed to 1-of-N (issue ENC-031)"
        );
        assert!(
            shares.len() <= 255,
            "produced {} shares; GF(256) supports at most 255 (issue ENC-031)",
            shares.len()
        );
    }
}

/// TC-DKG-005 — malformed shares must not panic.
///
/// `reconstruct_secret_bytes` reads `shares[i].y[b]` for b in 0..len(shares[0].y)
/// with no check that the other shares are the same length. A short share
/// from a hostile peer indexes out of bounds and panics the enclave thread.
/// See issue ENC-032.
#[test]
fn tc_dkg_005_ragged_shares_do_not_panic() {
    let shares = vec![
        KeyShare { x: 1, y: vec![0xAA; 32] },
        KeyShare { x: 2, y: vec![0xBB; 4] }, // hostile: short
        KeyShare { x: 3, y: vec![0xCC; 32] },
    ];
    let result = std::panic::catch_unwind(|| reconstruct_secret_bytes(&shares, 3));
    assert!(
        result.is_ok(),
        "a short share from a peer panicked the reconstruction path \
         (issue ENC-032)"
    );
}

/// TC-DKG-006 — the single-byte API must round-trip.
#[test]
fn tc_dkg_006_single_byte_roundtrip() {
    let shares = split_secret(0xa5, 2, 4);
    assert_eq!(shares.len(), 4);
    assert_eq!(reconstruct_secret(&shares[0..2], 2), 0xa5);
}

/// TC-DKG-007 — degenerate parameters must return empty, not panic.
#[test]
fn tc_dkg_007_degenerate_parameters_are_handled() {
    assert!(split_secret_bytes(b"", 2, 3).is_empty(), "empty secret");
    assert!(split_secret_bytes(b"x", 0, 3).is_empty(), "zero threshold");
    assert!(split_secret_bytes(b"x", 4, 3).is_empty(), "threshold > total");
}

/// TC-DKG-008 — share randomness must come from the SP 800-90A DRBG.
///
/// §5.2.2 is explicit that `rand::thread_rng()` must not be used. Today
/// `split_secret_bytes` correctly uses `HmacDrbg`, but it constructs a *new*
/// one per call — and `HmacDrbg::new()` seeds from a timestamp (issue
/// ENC-001). Two splits of the same secret in the same nanosecond therefore
/// produce identical "random" coefficients, which leaks the polynomial.
#[test]
fn tc_dkg_008_coefficients_differ_across_calls() {
    let secret = b"same-secret-both-times";
    let a = split_secret_bytes(secret, 3, 5);
    let b = split_secret_bytes(secret, 3, 5);

    let same: Vec<bool> = a.iter().zip(&b).map(|(x, y)| x.y == y.y).collect();
    assert!(
        !same.iter().all(|&s| s),
        "two independent splits produced identical shares — the polynomial \
         coefficients are predictable (issue ENC-001 via ENC-033)"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Pedersen VSS (§5.2.3–5.2.5)
// ─────────────────────────────────────────────────────────────────────────────

/// TC-VSS-001 — honestly generated shares must verify.
#[test]
fn tc_vss_001_honest_shares_verify() {
    let (shares, commitment) = split_secret_vss(42, 3, 5);
    assert_eq!(shares.len(), 5);
    assert_eq!(commitment.coefficient_commitments.len(), 3);
    for s in &shares {
        assert!(verify_vss_commitment(s, &commitment), "honest share x={} failed", s.x);
    }
}

/// TC-VSS-002 — a tampered share value must fail verification.
#[test]
fn tc_vss_002_tampered_share_fails() {
    let (shares, commitment) = split_secret_vss(100, 2, 4);
    let mut bad = shares[0].clone();
    bad.scalar_y_hex = Some(hex::encode([99u8; 32]));
    assert!(!verify_vss_commitment(&bad, &commitment));
}

/// TC-VSS-003 — a tampered blinding factor must fail verification.
#[test]
fn tc_vss_003_tampered_blinding_fails() {
    let (shares, commitment) = split_secret_vss(55, 2, 3);
    let mut bad = shares[0].clone();
    bad.blinding_hex = Some(hex::encode([123u8; 32]));
    assert!(!verify_vss_commitment(&bad, &commitment));
}

/// TC-VSS-004 — a share must not verify against another dealer's commitment.
#[test]
fn tc_vss_004_cross_dealer_share_fails() {
    let (shares_a, _) = split_secret_vss(7, 2, 3);
    let (_, commitment_b) = split_secret_vss(7, 2, 3);
    assert!(
        !verify_vss_commitment(&shares_a[0], &commitment_b),
        "a share verified against a commitment from a different dealer"
    );
}

/// TC-VSS-005 — VSS shares must be reconstructible.
///
/// `split_secret_vss` evaluates the polynomial over the Ristretto scalar
/// field, then stores only `s_val.to_bytes()[0]` in the `y: u8` field.
/// `reconstruct_secret` interpolates those bytes over GF(256) — a different
/// algebraic structure entirely. The truncated low byte of a scalar-field
/// evaluation carries no GF(256) Lagrange relationship, so VSS shares can
/// be *verified* but never *recombined*. A verifiable secret sharing scheme
/// that cannot recover the secret is not a sharing scheme.
/// See issue ENC-034.
#[test]
fn tc_vss_005_verified_shares_reconstruct_the_secret() {
    let secret = 42u8;
    let (shares, _) = split_secret_vss(secret, 3, 5);
    let recovered = reconstruct_secret(&shares[0..3], 3);
    assert_eq!(
        recovered, secret,
        "VSS shares verify but reconstruct to {recovered} instead of \
         {secret}: shares are evaluated in the Ristretto scalar field and \
         recombined over GF(256) (issue ENC-034)"
    );
}

/// TC-VSS-006 — an all-zero commitment must be rejected, not decompressed.
#[test]
fn tc_vss_006_invalid_curve_point_is_rejected() {
    use traces_sm_enclave::dkg::VssCommitment;
    let (shares, _) = split_secret_vss(77, 2, 3);
    let bad = VssCommitment {
        commitment_hex: "00".repeat(32),
        coefficient_commitments: vec!["00".repeat(32)],
    };
    assert!(!verify_vss_commitment(&shares[0], &bad));
}

/// TC-VSS-007 — non-canonical scalar encodings must be rejected.
///
/// `verify_vss_commitment` falls back to `Scalar::from_bytes_mod_order` when
/// `from_canonical_bytes` fails, so a share carrying a non-canonical scalar
/// is silently reduced and accepted. That admits multiple distinct wire
/// encodings for one share — malleability in a verification path.
/// See issue ENC-035.
#[test]
#[ignore = "verify_vss_commitment reduces non-canonical scalars — see issue ENC-035"]
fn tc_vss_007_non_canonical_scalars_are_rejected() {
    unimplemented!("remove the from_bytes_mod_order fallback and return false");
}
