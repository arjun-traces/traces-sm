use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use serde::{Deserialize, Serialize};
use bulletproofs::PedersenGens;
use crate::error::EnclaveError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedersenCommitment {
    pub point_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PedersenOpening {
    pub value: u64,
    pub blinding_hex: String,
}

pub fn commit(value: u64) -> Result<(PedersenCommitment, PedersenOpening), EnclaveError> {
    let mut rnd = [0u8; 32];
    ring::rand::SystemRandom::new()
        .fill(&mut rnd)
        .map_err(|_| EnclaveError::CryptoError("RNG failure".into()))?;
    let blinding = Scalar::from_bytes_mod_order(rnd);
    let commitment = commit_with_blinding(value, &blinding)?;
    let opening = PedersenOpening {
        value,
        blinding_hex: hex::encode(blinding.as_bytes()),
    };
    Ok((commitment, opening))
}

pub fn commit_with_blinding(
    value: u64,
    blinding: &Scalar,
) -> Result<PedersenCommitment, EnclaveError> {
    let pc_gens = PedersenGens::default();
    let point = pc_gens.commit(Scalar::from(value), *blinding);
    Ok(PedersenCommitment {
        point_hex: hex::encode(point.compress().as_bytes()),
    })
}

pub fn verify_opening(
    commitment: &PedersenCommitment,
    opening: &PedersenOpening,
) -> Result<bool, EnclaveError> {
    let b_bytes = hex::decode(&opening.blinding_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad blinding hex".into()))?;
    let b_arr: [u8; 32] = b_bytes
        .try_into()
        .map_err(|_| EnclaveError::ZkpInvalidInput("blinding must be 32 bytes".into()))?;
    let blinding = Scalar::from_bytes_mod_order(b_arr);
    let expected = commit_with_blinding(opening.value, &blinding)?;
    Ok(commitment.point_hex == expected.point_hex)
}

pub fn add_commitments(
    c1: &PedersenCommitment,
    c2: &PedersenCommitment,
) -> Result<PedersenCommitment, EnclaveError> {
    let p1 = decompress(c1)?;
    let p2 = decompress(c2)?;
    let sum = p1 + p2;
    Ok(PedersenCommitment {
        point_hex: hex::encode(sum.compress().as_bytes()),
    })
}

pub fn verify_homomorphic_sum(
    commitments: &[PedersenCommitment],
    claimed_total: &PedersenCommitment,
    total_val: u64,
    combined_blinding_hex: &str,
) -> Result<bool, EnclaveError> {
    let homomorphic_sum = commitments
        .iter()
        .try_fold(None::<RistrettoPoint>, |acc, c| {
            let p = decompress(c)?;
            Ok::<_, EnclaveError>(Some(acc.map(|a| a + p).unwrap_or(p)))
        })?
        .ok_or_else(|| EnclaveError::ZkpInvalidInput("empty commitment list".into()))?;

    let claimed = decompress(claimed_total)?;
    if homomorphic_sum.compress() != claimed.compress() {
        return Ok(false);
    }

    let blinding_bytes = hex::decode(combined_blinding_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad combined blinding hex".into()))?;
    let b_arr: [u8; 32] = blinding_bytes
        .try_into()
        .map_err(|_| EnclaveError::ZkpInvalidInput("blinding must be 32 bytes".into()))?;
    let blinding = Scalar::from_bytes_mod_order(b_arr);

    let expected = commit_with_blinding(total_val, &blinding)?;
    Ok(claimed_total.point_hex == expected.point_hex)
}

fn decompress(c: &PedersenCommitment) -> Result<RistrettoPoint, EnclaveError> {
    let bytes = hex::decode(&c.point_hex)
        .map_err(|_| EnclaveError::ZkpInvalidInput("bad commitment hex".into()))?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| EnclaveError::ZkpInvalidInput("commitment must be 32 bytes".into()))?;
    CompressedRistretto(arr)
        .decompress()
        .ok_or_else(|| EnclaveError::ZkpInvalidInput("commitment is not a valid Ristretto point".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_and_verify() {
        let (c, o) = commit(42).unwrap();
        assert!(verify_opening(&c, &o).unwrap());
    }

    #[test]
    fn wrong_value_fails() {
        let (c, o) = commit(42).unwrap();
        let bad_opening = PedersenOpening { value: 99, blinding_hex: o.blinding_hex };
        assert!(!verify_opening(&c, &bad_opening).unwrap());
    }

    #[test]
    fn homomorphic_addition() {
        let r1 = Scalar::from(7u64);
        let r2 = Scalar::from(11u64);
        let c1 = commit_with_blinding(10, &r1).unwrap();
        let c2 = commit_with_blinding(20, &r2).unwrap();
        let c_sum = add_commitments(&c1, &c2).unwrap();
        let r_sum = r1 + r2;
        let expected = commit_with_blinding(30, &r_sum).unwrap();
        assert_eq!(c_sum.point_hex, expected.point_hex);
    }
}
