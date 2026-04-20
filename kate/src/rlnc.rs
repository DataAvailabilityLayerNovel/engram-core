pub use avail_core::cda::types::CODING_VECTOR_LEN;
use avail_core::cda::types::{CodingVector, RLNCPiece, SCALAR_ENCODED_SIZE};
use ark_ec::{AffineRepr, CurveGroup};
use poly_multiproof::ark_bls12_381::{Bls12_381, G1Affine};
use poly_multiproof::ark_ff::Field;
use poly_multiproof::method1::Proof;
use poly_multiproof::traits::AsBytes;
use rand::Rng;
use thiserror_no_std::Error;

use crate::ArkScalar;

pub type CodingVectorFr = [ArkScalar; CODING_VECTOR_LEN];

#[derive(Debug, Error)]
pub enum RlncError {
	#[error("invalid number of pieces")]
	InvalidPieceCount,
	#[error("piece size mismatch")]
	PieceSizeMismatch,
	#[error("piece bytes are not valid scalars")]
	InvalidScalarEncoding,
	#[error("coding matrix is rank deficient")]
	RankDeficient,
}

pub fn generate_random_coding_vector<R: Rng>(rng: &mut R) -> CodingVectorFr {
	core::array::from_fn(|_| ArkScalar::from(rng.next_u64()))
}

/// Linearly combine KZG opening proofs (method1 `Proof` = single G1 point) using the RLNC coefficients.
///
/// This is valid **only** when every input proof is a standard single-point KZG opening for the **same**
/// evaluation point `z` (same divisor \(X-z\)). Then
/// \(\pi_{\sum_i c_i f_i} = \sum_i c_i \pi_{f_i}\) in \(\mathbb{G}_1\).
///
/// It does **not** apply to `PolyMultiProofNoPrecomp` openings, which mix polynomials via a transcript challenge.
pub fn combine_proofs_homomorphic(
	proofs: &[Proof<Bls12_381>; CODING_VECTOR_LEN],
	coding_vector: &CodingVectorFr,
) -> Proof<Bls12_381> {
	let mut acc = G1Affine::zero().into_group();
	for i in 0..CODING_VECTOR_LEN {
		acc += proofs[i].0.into_group() * coding_vector[i];
	}
	Proof(acc.into_affine())
}

pub fn encode_rlnc(
	pieces: &[Vec<u8>; CODING_VECTOR_LEN],
	coding_vector: &CodingVectorFr,
) -> Result<RLNCPiece, RlncError> {
	let piece_len = pieces[0].len();
	if pieces.iter().any(|p| p.len() != piece_len) {
		return Err(RlncError::PieceSizeMismatch);
	}
	if piece_len % SCALAR_ENCODED_SIZE != 0 {
		return Err(RlncError::PieceSizeMismatch);
	}

	let scalar_count = piece_len / SCALAR_ENCODED_SIZE;
	let mut out = vec![0u8; piece_len];
	for scalar_idx in 0..scalar_count {
		let mut acc = ArkScalar::from(0u32);
		for piece_idx in 0..CODING_VECTOR_LEN {
			let from = scalar_idx * SCALAR_ENCODED_SIZE;
			let to = from + SCALAR_ENCODED_SIZE;
			let chunk: [u8; SCALAR_ENCODED_SIZE] = pieces[piece_idx][from..to]
				.try_into()
				.map_err(|_| RlncError::InvalidScalarEncoding)?;
			let scalar =
				ArkScalar::from_bytes(&chunk).map_err(|_| RlncError::InvalidScalarEncoding)?;
			acc += coding_vector[piece_idx] * scalar;
		}
		let encoded = acc.to_bytes().map_err(|_| RlncError::InvalidScalarEncoding)?;
		let from = scalar_idx * SCALAR_ENCODED_SIZE;
		let to = from + SCALAR_ENCODED_SIZE;
		out[from..to].copy_from_slice(encoded.as_ref());
	}

	let coding_vector = CodingVector(core::array::from_fn(|idx| {
		coding_vector[idx]
			.to_bytes()
			.map(|bytes| bytes.as_ref().try_into().unwrap_or([0u8; SCALAR_ENCODED_SIZE]))
			.unwrap_or([0u8; SCALAR_ENCODED_SIZE])
	}));

	Ok(RLNCPiece {
		coded_data: out,
		coding_vector,
	})
}

pub fn decode_rlnc(
	coded_pieces: &[RLNCPiece],
) -> Result<[Vec<u8>; CODING_VECTOR_LEN], RlncError> {
	if coded_pieces.len() < CODING_VECTOR_LEN {
		return Err(RlncError::InvalidPieceCount);
	}
	let selected = &coded_pieces[0..CODING_VECTOR_LEN];
	let piece_len = selected[0].coded_data.len();
	if selected.iter().any(|p| p.coded_data.len() != piece_len) {
		return Err(RlncError::PieceSizeMismatch);
	}
	if piece_len % SCALAR_ENCODED_SIZE != 0 {
		return Err(RlncError::PieceSizeMismatch);
	}

	let mut a = [[ArkScalar::from(0u32); CODING_VECTOR_LEN]; CODING_VECTOR_LEN];
	for row in 0..CODING_VECTOR_LEN {
		for col in 0..CODING_VECTOR_LEN {
			a[row][col] = ArkScalar::from_bytes(&selected[row].coding_vector.0[col])
				.map_err(|_| RlncError::InvalidScalarEncoding)?;
		}
	}

	let inv = invert_matrix(a).ok_or(RlncError::RankDeficient)?;
	let scalar_count = piece_len / SCALAR_ENCODED_SIZE;
	let mut original = core::array::from_fn(|_| vec![0u8; piece_len]);

	for scalar_idx in 0..scalar_count {
		let mut y = [ArkScalar::from(0u32); CODING_VECTOR_LEN];
		for row in 0..CODING_VECTOR_LEN {
			let from = scalar_idx * SCALAR_ENCODED_SIZE;
			let to = from + SCALAR_ENCODED_SIZE;
			let chunk: [u8; SCALAR_ENCODED_SIZE] = selected[row].coded_data[from..to]
				.try_into()
				.map_err(|_| RlncError::InvalidScalarEncoding)?;
			y[row] = ArkScalar::from_bytes(&chunk).map_err(|_| RlncError::InvalidScalarEncoding)?;
		}

		for out_row in 0..CODING_VECTOR_LEN {
			let mut acc = ArkScalar::from(0u32);
			for col in 0..CODING_VECTOR_LEN {
				acc += inv[out_row][col] * y[col];
			}
			let bytes = acc.to_bytes().map_err(|_| RlncError::InvalidScalarEncoding)?;
			let from = scalar_idx * SCALAR_ENCODED_SIZE;
			let to = from + SCALAR_ENCODED_SIZE;
			original[out_row][from..to].copy_from_slice(bytes.as_ref());
		}
	}

	Ok(original)
}

fn invert_matrix(
	mut matrix: [[ArkScalar; CODING_VECTOR_LEN]; CODING_VECTOR_LEN],
) -> Option<[[ArkScalar; CODING_VECTOR_LEN]; CODING_VECTOR_LEN]> {
	let mut inv = core::array::from_fn(|row| {
		core::array::from_fn(|col| {
			if row == col {
				ArkScalar::from(1u32)
			} else {
				ArkScalar::from(0u32)
			}
		})
	});

	for pivot_idx in 0..CODING_VECTOR_LEN {
		let mut pivot = pivot_idx;
		while pivot < CODING_VECTOR_LEN && matrix[pivot][pivot_idx] == ArkScalar::from(0u32) {
			pivot += 1;
		}
		if pivot == CODING_VECTOR_LEN {
			return None;
		}
		if pivot != pivot_idx {
			matrix.swap(pivot, pivot_idx);
			inv.swap(pivot, pivot_idx);
		}

		let pivot_inv = matrix[pivot_idx][pivot_idx].inverse()?;
		for col in 0..CODING_VECTOR_LEN {
			matrix[pivot_idx][col] *= pivot_inv;
			inv[pivot_idx][col] *= pivot_inv;
		}

		for row in 0..CODING_VECTOR_LEN {
			if row == pivot_idx {
				continue;
			}
			let factor = matrix[row][pivot_idx];
			if factor == ArkScalar::from(0u32) {
				continue;
			}
			for col in 0..CODING_VECTOR_LEN {
				matrix[row][col] -= factor * matrix[pivot_idx][col];
				inv[row][col] -= factor * inv[pivot_idx][col];
			}
		}
	}

	Some(inv)
}
