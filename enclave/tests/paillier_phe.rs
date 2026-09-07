//! Paillier partially-homomorphic encryption tests for `enclave/src/he/paillier.rs`.
//!
//! Spec references (docs/TECHNICAL_SPECIFICATION.md):
//!   §4.3 "Paillier Cryptosystem (2048-bit):
//!         Enc(m1)·Enc(m2) mod n² = Enc(m1 + m2 mod n)
//!         Enc(m)^k mod n² = Enc(k·m mod n)"
//!
//! The module doc says the homomorphic operations "run on untrusted
//! ciphertexts". None of them validate their inputs, and `decrypt` panics on
//! a hostile ciphertext. Those are the tests that matter here.

mod common;

use num_bigint::BigUint;
use traces_sm_enclave::he::paillier::{
    add_ciphertexts, decrypt, encrypt, generate_keypair, multiply_ciphertext_by_scalar,
    rerandomize, PaillierKeyPair,
};

/// 1024-bit keys keep the suite fast. TC-PHE-010 covers the spec'd 2048.
fn kp() -> PaillierKeyPair {
    generate_keypair(1024).expect("keygen")
}

/// TC-PHE-001 — encrypt/decrypt must round-trip.
#[test]
fn tc_phe_001_roundtrip() {
    let k = kp();
    let m = BigUint::from(42u64);
    let c = encrypt(&k.public, &m).expect("encrypt");
    assert_eq!(decrypt(&k.private, &c).expect("decrypt"), m);
}

/// TC-PHE-002 — additive homomorphism (§4.3, first identity).
#[test]
fn tc_phe_002_additive_homomorphism() {
    let k = kp();
    let (m1, m2) = (BigUint::from(100u64), BigUint::from(200u64));
    let c = add_ciphertexts(
        &k.public,
        &encrypt(&k.public, &m1).expect("enc"),
        &encrypt(&k.public, &m2).expect("enc"),
    );
    assert_eq!(decrypt(&k.private, &c).expect("dec"), BigUint::from(300u64));
}

/// TC-PHE-003 — scalar multiplication (§4.3, second identity).
#[test]
fn tc_phe_003_scalar_multiplication() {
    let k = kp();
    let c = encrypt(&k.public, &BigUint::from(7u64)).expect("enc");
    let scaled = multiply_ciphertext_by_scalar(&k.public, &c, &BigUint::from(5u64));
    assert_eq!(decrypt(&k.private, &scaled).expect("dec"), BigUint::from(35u64));
}

/// TC-PHE-004 — addition must be associative across three operands.
#[test]
fn tc_phe_004_addition_is_associative() {
    let k = kp();
    let enc = |v: u64| encrypt(&k.public, &BigUint::from(v)).expect("enc");
    let (a, b, c) = (enc(11), enc(22), enc(33));
    let left = add_ciphertexts(&k.public, &add_ciphertexts(&k.public, &a, &b), &c);
    let right = add_ciphertexts(&k.public, &a, &add_ciphertexts(&k.public, &b, &c));
    assert_eq!(
        decrypt(&k.private, &left).expect("dec"),
        decrypt(&k.private, &right).expect("dec")
    );
}

/// TC-PHE-005 — semantic security: the same plaintext must encrypt differently.
#[test]
fn tc_phe_005_encryption_is_randomized() {
    let k = kp();
    let m = BigUint::from(12345u64);
    let a = encrypt(&k.public, &m).expect("enc");
    let b = encrypt(&k.public, &m).expect("enc");
    assert_ne!(a, b, "Paillier encryption is deterministic — no semantic security");
}

/// TC-PHE-006 — re-randomization must preserve the plaintext.
#[test]
fn tc_phe_006_rerandomize_preserves_plaintext() {
    let k = kp();
    let m = BigUint::from(99u64);
    let c = encrypt(&k.public, &m).expect("enc");
    let c2 = rerandomize(&k.public, &c);
    assert_ne!(c, c2);
    assert_eq!(decrypt(&k.private, &c2).expect("dec"), m);
}

/// TC-PHE-007 — plaintexts at or above n must be refused.
#[test]
fn tc_phe_007_oversized_plaintext_is_refused() {
    let k = kp();
    assert!(encrypt(&k.public, &k.public.n).is_err(), "m = n must be refused");
    assert!(
        encrypt(&k.public, &(&k.public.n + BigUint::from(1u64))).is_err(),
        "m > n must be refused"
    );
}

/// TC-PHE-008 — a hostile ciphertext must not panic the enclave.
///
/// `decrypt` computes `L(u) = (u - 1) / n`. For c = 0, `c^λ mod n²` is 0 and
/// `BigUint` subtraction underflows, which panics. The module doc explicitly
/// says these operations run on "ciphertexts supplied by untrusted parties",
/// so this is a one-request denial of service against the enclave.
/// See issue ENC-050.
#[test]
fn tc_phe_008_hostile_ciphertext_does_not_panic() {
    let k = kp();
    for hostile in [BigUint::from(0u64), BigUint::from(1u64)] {
        let outcome = std::panic::catch_unwind(|| decrypt(&k.private, &hostile));
        assert!(
            outcome.is_ok(),
            "decrypt panicked on ciphertext {hostile} — untrusted input must \
             return Err, not unwind (issue ENC-050)"
        );
    }
}

/// TC-PHE-009 — homomorphic operations must validate their operands.
///
/// `add_ciphertexts`, `multiply_ciphertext_by_scalar` and `rerandomize`
/// return `BigUint` rather than `Result` and never check that their inputs
/// are in Z*_{n²}. Garbage in, garbage out — with no signal to the caller.
/// See issue ENC-051.
#[test]
#[ignore = "homomorphic ops return BigUint, not Result — see issue ENC-051"]
fn tc_phe_009_homomorphic_ops_validate_operands() {
    unimplemented!(
        "change add_ciphertexts / multiply_ciphertext_by_scalar / rerandomize \
         to Result<BigUint, EnclaveError> and reject c >= n²"
    );
}

/// TC-PHE-010 — the spec'd 2048-bit modulus must work.
///
/// Marked `#[ignore]` because `gen_prime` rejection-samples 1024-bit
/// candidates with a 20-round Miller-Rabin and no small-prime sieve, which
/// takes minutes. Run with `cargo test -- --ignored` in a nightly job.
/// The absence of a sieve is itself a finding — see issue ENC-052.
#[test]
#[ignore = "slow: no small-prime sieve in gen_prime — see issue ENC-052"]
fn tc_phe_010_2048_bit_key_roundtrips() {
    let k = generate_keypair(2048).expect("2048-bit keygen");
    assert!(k.public.n.bits() >= 2047, "modulus is smaller than requested");
    let m = BigUint::from(123456789u64);
    let c = encrypt(&k.public, &m).expect("enc");
    assert_eq!(decrypt(&k.private, &c).expect("dec"), m);
}

/// TC-PHE-011 — an undersized key request must be refused.
///
/// `generate_keypair` accepts any `bits`. `generate_keypair(16)` produces a
/// toy modulus that is trivially factorable, with no warning to the caller.
#[test]
fn tc_phe_011_undersized_key_is_refused() {
    assert!(
        generate_keypair(64).is_err(),
        "a 64-bit Paillier modulus was accepted; the minimum must be enforced \
         (spec §4.3 requires 2048) — issue ENC-053"
    );
}

/// TC-PHE-012 — the private key must be zeroized on drop.
///
/// `impl Drop for PaillierPrivateKey` assigns `BigUint::zero()` over `lambda`
/// and `mu`. That drops the old `BigUint` — freeing its limb `Vec` without
/// wiping it — and binds a fresh allocation. The secret limbs stay in freed
/// heap memory. The `use zeroize::Zeroize` import in the module is unused,
/// which is the tell. See issue ENC-054.
#[test]
#[ignore = "Drop reassigns instead of wiping; needs zeroize::Zeroize on the limbs — issue ENC-054"]
fn tc_phe_012_private_key_is_zeroized() {
    unimplemented!("wipe lambda/mu limb slices via Zeroize before dropping");
}
