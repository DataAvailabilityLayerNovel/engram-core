use crate::{
	gridgen::core::{Commitment, PolyOrientation, PolynomialGrid},
	pmp::{
		ark_bls12_381::Bls12_381,
		merlin::Transcript,
		method1::{M1NoPrecomp, Proof},
		msm::blst::BlstMSMEngine,
		traits::PolyMultiProofNoPrecomp,
	},
	ArkScalar,
};
use poly_multiproof::ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use thiserror_no_std::Error;

#[derive(Debug, Clone)]
pub struct SegmentMultiProof {
	pub proof: Proof<Bls12_381>,
	pub evals: Vec<ArkScalar>,
	pub column: u16,
}

#[derive(Debug, Error)]
pub enum SegmentProofError {
	#[error("invalid column polynomial grid")]
	InvalidGrid,
	#[error("invalid column index")]
	InvalidColumn,
	#[error("multiproof error")]
	Multiproof(#[from] poly_multiproof::Error),
}

pub fn generate_segment_proof(
	srs: &M1NoPrecomp<Bls12_381, BlstMSMEngine>,
	poly_grid: &PolynomialGrid,
	column: u16,
	points: &[ArkScalar],
) -> Result<SegmentMultiProof, SegmentProofError> {
	if poly_grid.orientation != PolyOrientation::Column {
		return Err(SegmentProofError::InvalidGrid);
	}
	let poly = poly_grid
		.inner
		.get(column as usize)
		.ok_or(SegmentProofError::InvalidColumn)?;
	let evals = points
		.iter()
		.map(|point| {
			let dense = DensePolynomial::from_coefficients_vec(poly.clone());
			Ok::<ArkScalar, poly_multiproof::Error>(dense.evaluate(point))
		})
		.collect::<Result<Vec<_>, _>>()?;
	let mut ts = Transcript::new(b"avail-segment-mp");
	let proof = PolyMultiProofNoPrecomp::open(
		srs,
		&mut ts,
		&[evals.clone()],
		&[poly.clone()],
		points,
	)?;

	Ok(SegmentMultiProof {
		proof,
		evals,
		column,
	})
}

pub fn verify_segment_proof(
	srs: &M1NoPrecomp<Bls12_381, BlstMSMEngine>,
	commitment: &Commitment,
	segment_proof: &SegmentMultiProof,
	points: &[ArkScalar],
) -> Result<bool, SegmentProofError> {
	let mut ts = Transcript::new(b"avail-segment-mp");
	PolyMultiProofNoPrecomp::verify(
		srs,
		&mut ts,
		&[commitment.clone()],
		points,
		&[segment_proof.evals.as_slice()],
		&segment_proof.proof,
	)
	.map_err(Into::into)
}
