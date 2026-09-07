//! NIST SP 800-90A/B/C conformance tests for `enclave/src/drbg.rs`.
//!
//! Spec references (docs/TECHNICAL_SPECIFICATION.md):
//!   §1.2  "HMAC_DRBG seeded via SGX RDRAND/RDSEED hardware entropy with
//!          continuous Repetition Count Test (RCT) and Adaptive Proportion
//!          Test (APT) health testing."
//!   §6.2  "Initialized with 32 bytes of SGX RDRAND/RDSEED entropy.
//!          RCT: rejects if identical bytes exceed C = 16.
//!          APT: sliding window W = 512, rejects if base sample count
//!          exceeds C = 13."
//!
//! STATUS: these tests are expected to FAIL against the code as it stands.
//! `HmacDrbg::get_entropy()` fills every byte of the seed with
//! `SystemTime::now().subsec_nanos() % 256`, which is a timestamp, not
//! entropy. See TEST_PLAN_AND_CONFORMANCE.md issue ENC-001.

mod common;

use common::{distinct_bytes, shannon_entropy_bits_per_byte};
use traces_sm_enclave::drbg::HmacDrbg;

/// TC-DRBG-001 — Seed material must be unpredictable.
///
/// Two DRBG instances created back to back must not produce identical
/// output. With a real entropy source the collision probability is ~2^-256.
/// With a timestamp-derived seed, two instances created in the same
/// nanosecond window produce byte-identical streams.
#[test]
fn tc_drbg_001_two_instances_produce_different_streams() {
    let mut a = [0u8; 64];
    let mut b = [0u8; 64];
    HmacDrbg::new().generate(&mut a);
    HmacDrbg::new().generate(&mut b);

    assert_ne!(
        a, b,
        "two freshly seeded DRBG instances produced identical output — \
         the seed is not entropy (SP 800-90A §8.6.1 requires an approved \
         entropy source)"
    );
}

/// TC-DRBG-002 — Seed material must survive a repeat-instantiation loop.
///
/// Stronger form of TC-DRBG-001: across many instantiations, every stream
/// should be distinct. A time-derived seed collides frequently because
/// instantiation is far faster than the clock's effective resolution.
#[test]
fn tc_drbg_002_repeated_instantiation_yields_distinct_streams() {
    const ROUNDS: usize = 64;
    let mut streams = Vec::with_capacity(ROUNDS);
    for _ in 0..ROUNDS {
        let mut out = [0u8; 32];
        HmacDrbg::new().generate(&mut out);
        streams.push(out);
    }

    let mut unique: Vec<[u8; 32]> = streams.clone();
    unique.sort_unstable();
    unique.dedup();

    assert_eq!(
        unique.len(),
        ROUNDS,
        "{} of {ROUNDS} freshly seeded DRBG streams collided — seed entropy \
         is effectively zero",
        ROUNDS - unique.len()
    );
}

/// TC-DRBG-003 — Output must be statistically indistinguishable from random.
///
/// HMAC-SHA256 in counter mode whitens even a bad seed, so this test is
/// expected to PASS today. It is here so that a future refactor of the
/// generate loop cannot silently regress output quality.
#[test]
fn tc_drbg_003_output_has_full_byte_entropy() {
    let mut out = vec![0u8; 8192];
    HmacDrbg::new().generate(&mut out);

    let h = shannon_entropy_bits_per_byte(&out);
    assert!(
        h > 7.8,
        "DRBG output entropy {h:.3} bits/byte is below the 7.8 threshold \
         expected for 8192 uniform samples"
    );
    assert!(
        distinct_bytes(&out) > 250,
        "DRBG output covered only {} of 256 byte values",
        distinct_bytes(&out)
    );
}

/// TC-DRBG-004 — Successive `generate` calls must not repeat.
#[test]
fn tc_drbg_004_successive_generates_differ() {
    let mut drbg = HmacDrbg::new();
    let mut first = [0u8; 48];
    let mut second = [0u8; 48];
    drbg.generate(&mut first);
    drbg.generate(&mut second);
    assert_ne!(first, second, "DRBG repeated its output across two calls");
}

/// TC-DRBG-005 — RCT cutoff is C = 16 per §6.2.
///
/// SP 800-90B §4.4.1: the test fails when the run length *reaches* the
/// cutoff. The current implementation compares `self.rct_count >= 16` while
/// the spec prose says "exceed C = 16" — one of the two must change. This
/// test pins the spec's stated wording so the ambiguity is resolved
/// deliberately rather than by accident.
///
/// It is `#[ignore]`d because `run_rct` is private and cannot be driven
/// directly. Un-ignore after exposing a `#[cfg(test)] pub(crate)` hook or
/// moving the health tests into their own testable type.
#[test]
#[ignore = "run_rct is private; requires a test hook — see issue ENC-002"]
fn tc_drbg_005_rct_rejects_at_documented_cutoff() {
    unimplemented!("expose HmacDrbg::run_rct for testing");
}

/// TC-DRBG-006 — A failed health test must suppress the output.
///
/// SP 800-90B §4.3: on a health-test failure the noise source must enter an
/// error state; the failing output must not be released to consumers.
/// `generate()` currently returns `EntropyHealthStatus` *alongside* the
/// bytes it already wrote into `out`, so every caller in the codebase
/// (`dkg::split_secret_bytes`, `router::dispatch`) consumes unvalidated
/// output. The signature must become `Result<(), EnclaveError>`.
#[test]
#[ignore = "requires generate() to return Result — see issue ENC-003"]
fn tc_drbg_006_health_failure_suppresses_output() {
    unimplemented!("change generate() to return Result<(), EnclaveError>");
}

/// TC-DRBG-007 — Health-test state must be continuous across calls.
///
/// §1.2 requires *continuous* health testing. `router.rs` handles
/// `GET /v1/entropy/health` by constructing a brand-new `HmacDrbg`, so the
/// RCT/APT windows reset on every poll and `reseed_count` is always 2.
/// The health endpoint must report the state of the long-lived DRBG held in
/// `EnclaveState`, not a throwaway instance.
#[test]
fn tc_drbg_007_reseed_counter_advances_within_one_instance() {
    let mut drbg = HmacDrbg::new();
    let mut out = [0u8; 16];
    let first = drbg.generate(&mut out);
    let second = drbg.generate(&mut out);
    assert!(
        second.reseed_count > first.reseed_count,
        "reseed counter did not advance ({} -> {}); health state is not \
         continuous",
        first.reseed_count,
        second.reseed_count
    );
}
