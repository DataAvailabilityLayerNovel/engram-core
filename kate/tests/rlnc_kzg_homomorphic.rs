//! KZG single-point opening proofs are linear in G1 for a fixed evaluation point.
//! This matches `kate::rlnc::combine_proofs_homomorphic` (RLNC over proof elements).

use kate::couscous;
use kate::pmp::{
	ark_bls12_381::{Bls12_381, Fr},
	ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial},
	method1::Proof,
	msm::blst::BlstMSMEngine,
	traits::{Committer, KZGProof},
};
use kate::rlnc::{
	combine_proofs_homomorphic, generate_random_coding_vector, CodingVectorFr, CODING_VECTOR_LEN,
};
use poly_multiproof::method1::M1NoPrecomp;
use rand::{rngs::StdRng, RngCore, SeedableRng};

type Srs = M1NoPrecomp<Bls12_381, BlstMSMEngine>;

fn eval_poly(coeffs: &[Fr], z: Fr) -> Fr {
	DensePolynomial::from_coefficients_slice(coeffs).evaluate(&z)
}

#[test]
fn combined_kzg_proof_verifies_against_linear_commitment_and_value() {
	let srs: Srs = couscous::multiproof_params();
	let mut rng = StdRng::seed_from_u64(2026_04_20);

	let polys: Vec<Vec<Fr>> = (0..CODING_VECTOR_LEN)
		.map(|_| (0..24).map(|_| Fr::from(rng.next_u64())).collect())
		.collect();

	let z = Fr::from(12345u64);
	let coding_vector: CodingVectorFr = generate_random_coding_vector(&mut rng);

	let mut proofs: Vec<Proof<Bls12_381>> = Vec::with_capacity(CODING_VECTOR_LEN);
	for p in &polys {
		let witness = KZGProof::compute_witness_polynomial(&srs, p.clone(), z).unwrap();
		proofs.push(KZGProof::open(&srs, witness).unwrap());
	}
	let proofs_arr: [Proof<Bls12_381>; CODING_VECTOR_LEN] = proofs.try_into().expect("len 16");

	let max_len = polys.iter().map(|p| p.len()).max().unwrap();
	let mut combined_coeffs = vec![Fr::from(0u32); max_len];
	for i in 0..CODING_VECTOR_LEN {
		for (j, c) in polys[i].iter().enumerate() {
			combined_coeffs[j] += coding_vector[i] * c;
		}
	}

	let combined_commit = srs.commit(&combined_coeffs).unwrap();
	let combined_value = eval_poly(&combined_coeffs, z);
	let combined_value_alt: Fr = (0..CODING_VECTOR_LEN).fold(Fr::from(0u32), |acc, i| {
		acc + coding_vector[i] * eval_poly(&polys[i], z)
	});
	assert_eq!(combined_value, combined_value_alt);

	let pi = combine_proofs_homomorphic(&proofs_arr, &coding_vector);
	let ok = KZGProof::verify::<BlstMSMEngine>(&srs, &combined_commit, z, combined_value, &pi)
		.unwrap();
	assert!(ok);
}

#[test]
fn combined_kzg_proof_fails_on_wrong_value() {
	let srs: Srs = couscous::multiproof_params();
	let mut rng = StdRng::seed_from_u64(99);

	let polys: Vec<Vec<Fr>> = (0..CODING_VECTOR_LEN)
		.map(|_| vec![Fr::from(rng.next_u64()), Fr::from(rng.next_u64())])
		.collect();

	let z = Fr::from(7u64);
	let coding_vector: CodingVectorFr = generate_random_coding_vector(&mut rng);

	let mut proofs: Vec<Proof<Bls12_381>> = Vec::with_capacity(CODING_VECTOR_LEN);
	for p in &polys {
		let witness = KZGProof::compute_witness_polynomial(&srs, p.clone(), z).unwrap();
		proofs.push(KZGProof::open(&srs, witness).unwrap());
	}
	let proofs_arr: [Proof<Bls12_381>; CODING_VECTOR_LEN] = proofs.try_into().expect("len 16");

	let max_len = polys.iter().map(|p| p.len()).max().unwrap();
	let mut combined_coeffs = vec![Fr::from(0u32); max_len];
	for i in 0..CODING_VECTOR_LEN {
		for (j, c) in polys[i].iter().enumerate() {
			combined_coeffs[j] += coding_vector[i] * c;
		}
	}
	let combined_commit = srs.commit(&combined_coeffs).unwrap();
	let wrong_value = eval_poly(&combined_coeffs, z) + Fr::from(1u64);

	let pi = combine_proofs_homomorphic(&proofs_arr, &coding_vector);
	let ok = KZGProof::verify::<BlstMSMEngine>(&srs, &combined_commit, z, wrong_value, &pi).unwrap();
	assert!(!ok);
}
