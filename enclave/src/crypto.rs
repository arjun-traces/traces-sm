//! Envelope encryption helpers.
//!
//! Each secret gets its own random 256-bit DEK.
//! That DEK is sealed (AES-256-GCM) using the master sealing key
//! so only the enclave can recover it later.
//!
//! Wire format of an EncryptedSecret blob (all concatenated):
//!   sealed_dek  : 12 (nonce) + 32 (DEK) + 16 (tag) = 60 bytes
//!   separator   : 0x00 byte
//!   ciphertext  : 12 (nonce) + payload + 16 (tag)
//!
//! The separator is a fixed marker to make parsing unambiguous.

use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey,
                 UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};
use zeroize::Zeroizing;

use crate::error::EnclaveError;
use crate::sealing::{seal, unseal, SealingKeyProvider};

const SEALED_DEK_LEN: usize = NONCE_LEN + 32 + 16; // 60 bytes
const SEPARATOR: u8 = 0x00;

struct OneTimeNonce(Option<[u8; NONCE_LEN]>);
impl NonceSequence for OneTimeNonce {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        self.0.take().map(Nonce::assume_unique_for_key).ok_or(ring::error::Unspecified)
    }
}

/// Encrypt `plaintext` with a fresh random DEK, then seal the DEK.
///
/// Returns a self-contained blob that can only be decrypted inside the
/// enclave (because only the enclave can unseal the DEK).
pub fn encrypt_secret(
    plaintext: &[u8],
    purpose: &str,
    provider: &dyn SealingKeyProvider,
) -> Result<Vec<u8>, EnclaveError> {
    let rng = SystemRandom::new();

    // 1. Generate a fresh random 256-bit DEK
    let mut dek = Zeroizing::new([0u8; 32]);
    rng.fill(dek.as_mut()).map_err(|_| EnclaveError::AesGcmEncrypt)?;

    // 2. Seal the DEK using the master key
    let sealed_dek = seal(dek.as_ref(), purpose, provider)?;
    debug_assert_eq!(sealed_dek.len(), SEALED_DEK_LEN, "sealed DEK size mismatch");

    // 3. Encrypt the plaintext with the DEK
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill(&mut nonce_bytes).map_err(|_| EnclaveError::AesGcmEncrypt)?;

    let unbound = UnboundKey::new(&AES_256_GCM, dek.as_ref())
        .map_err(|_| EnclaveError::AesGcmEncrypt)?;
    let mut sk = SealingKey::new(unbound, OneTimeNonce(Some(nonce_bytes)));

    let mut ciphertext = plaintext.to_vec();
    sk.seal_in_place_append_tag(Aad::empty(), &mut ciphertext)
        .map_err(|_| EnclaveError::AesGcmEncrypt)?;

    // 4. Pack: sealed_dek || 0x00 || nonce || ciphertext_with_tag
    let mut blob = Vec::with_capacity(SEALED_DEK_LEN + 1 + NONCE_LEN + ciphertext.len());
    blob.extend_from_slice(&sealed_dek);
    blob.push(SEPARATOR);
    blob.extend_from_slice(&nonce_bytes);
    blob.extend(ciphertext);
    Ok(blob)
}

/// Decrypt a blob produced by `encrypt_secret`.
pub fn decrypt_secret(
    blob: &[u8],
    purpose: &str,
    provider: &dyn SealingKeyProvider,
) -> Result<Vec<u8>, EnclaveError> {
    if blob.len() < SEALED_DEK_LEN + 1 + NONCE_LEN + 16 {
        return Err(EnclaveError::AesGcmDecrypt);
    }

    // 1. Split the blob
    let sealed_dek = &blob[..SEALED_DEK_LEN];
    debug_assert_eq!(blob[SEALED_DEK_LEN], SEPARATOR);
    let rest = &blob[SEALED_DEK_LEN + 1..];
    let (nonce_bytes_slice, ciphertext_with_tag) = rest.split_at(NONCE_LEN);

    // 2. Unseal the DEK
    let dek_bytes = unseal(sealed_dek, purpose, provider)?;
    if dek_bytes.len() != 32 {
        return Err(EnclaveError::Unsealing { msg: "DEK wrong length after unseal" });
    }
    let dek = Zeroizing::new({
        let mut a = [0u8; 32];
        a.copy_from_slice(&dek_bytes);
        a
    });

    // 3. Decrypt with the DEK
    let mut nonce_bytes = [0u8; NONCE_LEN];
    nonce_bytes.copy_from_slice(nonce_bytes_slice);

    let unbound = UnboundKey::new(&AES_256_GCM, dek.as_ref())
        .map_err(|_| EnclaveError::AesGcmDecrypt)?;
    let mut ok = OpeningKey::new(unbound, OneTimeNonce(Some(nonce_bytes)));

    let mut in_out = ciphertext_with_tag.to_vec();
    let decrypted = ok
        .open_in_place(Aad::empty(), &mut in_out)
        .map_err(|_| EnclaveError::AesGcmDecrypt)?;
    Ok(decrypted.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sealing::SimSealingProvider;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let provider = SimSealingProvider::new("/tmp/sm-test-crypto");
        let plaintext = b"very secret password 1234";
        let blob = encrypt_secret(plaintext, "test:crypto", &provider).unwrap();
        let recovered = decrypt_secret(&blob, "test:crypto", &provider).unwrap();
        assert_eq!(&recovered, plaintext);
    }

    #[test]
    fn wrong_purpose_fails() {
        let provider = SimSealingProvider::new("/tmp/sm-test-crypto-b");
        let blob = encrypt_secret(b"data", "purpose:A", &provider).unwrap();
        assert!(decrypt_secret(&blob, "purpose:B", &provider).is_err());
    }
}
