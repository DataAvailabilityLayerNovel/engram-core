use kate::couscous;
use poly_multiproof::{
	ark_bls12_381::{Bls12_381, Fr},
	ark_poly::{DenseUVPolynomial, EvaluationDomain, GeneralEvaluationDomain, Polynomial},
	merlin::Transcript,
	method1::M1NoPrecomp,
	msm::blst::BlstMSMEngine,
	traits::{Committer, PolyMultiProofNoPrecomp},
};
use rand::{rngs::StdRng, RngCore, SeedableRng};

type E = Bls12_381;
type M = BlstMSMEngine;
type PublicParams = M1NoPrecomp<E, M>;

fn eval_from_coeffs(coeffs: &[Fr], x: Fr) -> Fr {
	let poly = poly_multiproof::ark_poly::univariate::DensePolynomial::from_coefficients_vec(
		coeffs.to_vec(),
	);
	poly.evaluate(&x)
}

#[test]
fn spike_open_single_poly_and_verify_passes() {
	let pp: PublicParams = couscous::multiproof_params();
	let mut rng = StdRng::seed_from_u64(42);
	let poly: Vec<Fr> = (0..256).map(|_| Fr::from(rng.next_u64())).collect();

	let domain = GeneralEvaluationDomain::<Fr>::new(16).expect("valid domain");
	let points: Vec<Fr> = domain.elements().take(4).collect();
	let evals: Vec<Fr> = points.iter().map(|x| eval_from_coeffs(&poly, *x)).collect();

	let mut open_ts = Transcript::new(b"spike-open-1poly");
	let proof = PolyMultiProofNoPrecomp::open(
		&pp,
		&mut open_ts,
		&[evals.clone()],
		&[poly.clone()],
		&points,
	)
	.expect("must open multiproof for single polynomial");

	let commit = pp.commit(&poly).expect("commit should succeed");
	let mut verify_ts = Transcript::new(b"spike-open-1poly");
	let verified = PolyMultiProofNoPrecomp::verify(
		&pp,
		&mut verify_ts,
		&[commit],
		&points,
		&[evals.as_slice()],
		&proof,
	)
	.expect("must verify proof");

	assert!(verified);
}

#[test]
fn spike_open_single_poly_detects_bad_eval() {
	let pp: PublicParams = couscous::multiproof_params();
	let mut rng = StdRng::seed_from_u64(42);
	let poly: Vec<Fr> = (0..256).map(|_| Fr::from(rng.next_u64())).collect();

	let domain = GeneralEvaluationDomain::<Fr>::new(16).expect("valid domain");
	let points: Vec<Fr> = domain.elements().take(4).collect();
	let mut evals: Vec<Fr> = points.iter().map(|x| eval_from_coeffs(&poly, *x)).collect();

	let mut open_ts = Transcript::new(b"spike-open-1poly");
	let proof = PolyMultiProofNoPrecomp::open(
		&pp,
		&mut open_ts,
		&[evals.clone()],
		&[poly.clone()],
		&points,
	)
	.expect("must open multiproof for single polynomial");

	evals[0] += Fr::from(1u64);
	let commit = pp.commit(&poly).expect("commit should succeed");
	let mut verify_ts = Transcript::new(b"spike-open-1poly");
	let verified = PolyMultiProofNoPrecomp::verify(
		&pp,
		&mut verify_ts,
		&[commit],
		&points,
		&[evals.as_slice()],
		&proof,
	)
	.expect("must return verification result");

	assert!(!verified);
}
