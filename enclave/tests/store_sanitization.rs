//! Storage and media-sanitization tests for `enclave/src/store.rs`.
//!
//! Spec references (docs/TECHNICAL_SPECIFICATION.md):
//!   §1.2 "NIST SP 800-88 Rev. 1: Cryptographic Erasure ('Crypto-Shredding')
//!         overwriting storage sectors with random noise prior to unlinking
//!         file descriptors."
//!   §9.1 "store.rs::crypto_shred(id) overwrites storage file sectors with
//!         random bytes before executing filesystem unlinking."

mod common;

use chrono::Utc;
use common::TempStore;
use std::collections::HashMap;
use uuid::Uuid;

use traces_sm_enclave::models::SecretType;
use traces_sm_enclave::store::{SecretRecord, Store};

fn record(name: &str) -> SecretRecord {
    let now = Utc::now();
    SecretRecord {
        id: Uuid::new_v4(),
        name: name.to_string(),
        secret_type: SecretType::Opaque,
        version: 1,
        public_key_pem: None,
        algorithm: None,
        owner: "test".into(),
        tags: HashMap::new(),
        created_at: now,
        updated_at: now,
        expires_at: None,
        deleted_at: None,
        zkp_commitment: None,
    }
}

/// TC-ST-001 — save/load must round-trip.
#[test]
fn tc_st_001_save_load_roundtrip() {
    let tmp = TempStore::new("save");
    let store = Store::new(tmp.path());
    let r = record("alpha");
    let blob = b"sealed-bytes".to_vec();

    store.save(&r, &blob).expect("save");
    let (loaded, got_blob) = store.load(&r.id).expect("load");
    assert_eq!(loaded.id, r.id);
    assert_eq!(got_blob, blob);
}

/// TC-ST-002 — a missing id must return NotFound.
#[test]
fn tc_st_002_missing_id_is_not_found() {
    let tmp = TempStore::new("missing");
    let store = Store::new(tmp.path());
    assert!(store.load(&Uuid::new_v4()).is_err());
}

/// TC-ST-003 — soft-deleted records disappear from load and list.
#[test]
fn tc_st_003_soft_delete_hides_record() {
    let tmp = TempStore::new("soft");
    let store = Store::new(tmp.path());
    let r = record("beta");
    store.save(&r, b"blob").expect("save");

    store.soft_delete(&r.id).expect("soft delete");
    assert!(
        store.load(&r.id).is_err(),
        "soft-deleted record still loadable"
    );
    assert!(
        store.list().expect("list").is_empty(),
        "soft-deleted record still listed"
    );
}

/// TC-ST-004 — CRITICAL: crypto-shred must overwrite with random bytes.
///
/// §9.1 says "random noise". The implementation writes `vec![0xFF; len]` and
/// the code comment concedes it: "In a real SGX enclave we would get random
/// bytes, here we use 0xFF or something similar".
///
/// A constant-fill pattern is a recognisable signature. On any storage medium
/// where the overwrite lands out-of-place — SSD wear-levelling, a
/// copy-on-write filesystem, a journalled filesystem, a snapshotted volume —
/// the original sealed blob survives intact and the 0xFF block marks exactly
/// which extent was meant to be destroyed. See issue ENC-080.
#[test]
fn tc_st_004_crypto_shred_uses_random_fill() {
    let tmp = TempStore::new("shred-pattern");
    let store = Store::new(tmp.path());
    let r = record("gamma");
    let blob = vec![0xAAu8; 4096];
    store.save(&r, &blob).expect("save");

    let blob_path = tmp.0.join(format!("{}.blob", r.id));

    // Capture what crypto_shred writes by intercepting the file before unlink.
    // We re-create the file, shred a *copy* directory, and inspect the pattern
    // by shredding a file we then immediately re-read from a hard link.
    let link_path = tmp.0.join("shred-witness.bin");
    std::fs::hard_link(&blob_path, &link_path).expect("hard link witness");

    store.crypto_shred(&r.id).expect("crypto_shred");

    let witness = std::fs::read(&link_path).expect("read witness through hard link");
    assert_eq!(
        witness.len(),
        blob.len(),
        "overwrite changed the file length"
    );

    let distinct = common::distinct_bytes(&witness);
    assert!(
        distinct > 200,
        "crypto_shred wrote a constant fill ({distinct} distinct byte values \
         across {} bytes; first byte 0x{:02X}). SP 800-88 and spec §9.1 \
         require random noise (issue ENC-080)",
        witness.len(),
        witness.first().copied().unwrap_or(0)
    );
}

/// TC-ST-005 — crypto-shred must remove both files.
#[test]
fn tc_st_005_crypto_shred_unlinks_both_files() {
    let tmp = TempStore::new("shred-unlink");
    let store = Store::new(tmp.path());
    let r = record("delta");
    store.save(&r, b"blob").expect("save");

    store.crypto_shred(&r.id).expect("crypto_shred");

    assert!(
        !tmp.0.join(format!("{}.blob", r.id)).exists(),
        ".blob survived"
    );
    assert!(
        !tmp.0.join(format!("{}.meta.json", r.id)).exists(),
        ".meta.json survived"
    );
    assert!(!store.exists(&r.id));
}

/// TC-ST-006 — shredding an unknown id must report failure.
///
/// `crypto_shred` iterates the two paths, skips them if absent, and returns
/// `Ok(())` unconditionally. An operator who shreds the wrong UUID — or whose
/// shred silently no-ops because of a permissions error, since every write is
/// discarded with `let _ =` — receives a success response for a destruction
/// that never happened. For a NIST SP 800-88 control the outcome must be
/// reported truthfully. See issue ENC-081.
#[test]
fn tc_st_006_shredding_unknown_id_reports_failure() {
    let tmp = TempStore::new("shred-absent");
    let store = Store::new(tmp.path());
    assert!(
        store.crypto_shred(&Uuid::new_v4()).is_err(),
        "crypto_shred reported success for an id that does not exist \
         (issue ENC-081)"
    );
}

/// TC-ST-007 — stored files must not be world-readable.
///
/// `.meta.json` and `.blob` are written with `fs::write`, i.e. mode 0644.
/// The blob is sealed, but the metadata is plaintext by design: names,
/// owners, tags, algorithms and timestamps of every secret are readable by
/// any local user. See issue ENC-082.
#[cfg(unix)]
#[test]
fn tc_st_007_stored_files_are_owner_only() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = TempStore::new("perms");
    let store = Store::new(tmp.path());
    let r = record("epsilon");
    store.save(&r, b"blob").expect("save");

    for suffix in ["meta.json", "blob"] {
        let path = tmp.0.join(format!("{}.{suffix}", r.id));
        let mode = std::fs::metadata(&path).expect("stat").permissions().mode() & 0o777;
        assert_eq!(
            mode, 0o600,
            "{suffix} is mode {mode:o}; secret metadata must be 0600 \
             (issue ENC-082)"
        );
    }
}

/// TC-ST-008 — a corrupt metadata file must not silently vanish from listings.
///
/// `list_all` uses `if let Ok(record) = serde_json::from_slice(...)` and drops
/// anything that fails to parse. A truncated or corrupted `.meta.json` makes
/// the secret disappear from `GET /v1/secrets` with no error anywhere — it
/// looks deleted while its sealed blob is still on disk. See issue ENC-083.
#[test]
fn tc_st_008_corrupt_metadata_is_surfaced_not_hidden() {
    let tmp = TempStore::new("corrupt");
    let store = Store::new(tmp.path());
    let r = record("zeta");
    store.save(&r, b"blob").expect("save");

    let meta_path = tmp.0.join(format!("{}.meta.json", r.id));
    std::fs::write(&meta_path, b"{ this is not valid json").expect("corrupt the file");

    match store.list() {
        Err(_) => {} // correct: corruption surfaced
        Ok(list) => panic!(
            "a corrupted metadata file was silently skipped; list() returned \
             {} records and reported no error (issue ENC-083)",
            list.len()
        ),
    }
}

/// TC-ST-009 — duplicate names must be detectable.
#[test]
fn tc_st_009_name_lookup_finds_live_records_only() {
    let tmp = TempStore::new("names");
    let store = Store::new(tmp.path());
    let r = record("eta");
    store.save(&r, b"blob").expect("save");

    assert!(store.name_exists("eta"));
    store.soft_delete(&r.id).expect("soft delete");
    assert!(
        !store.name_exists("eta"),
        "a soft-deleted name is still claimed"
    );
}
