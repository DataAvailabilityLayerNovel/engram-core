# CDA / avail-core — Implementation progress

This document records changes made in the **avail-core** repository (CDA Phase 0; excludes `avail` / `avail-light`).

---

## Phase 0 — Overview

**Goal:** shared types, header V4, extended `kate` / `kate-recovery` for an 8×8 grid → 16×16 extended grid, per-column commitments, segment multiproof, RLNC, and validation that **forking `poly-multiproof` is not required** (gate: `PolyMultiProofNoPrecomp::open` with a single polynomial).

---

## Completed work

### 1. Multiproof gate (1 polynomial × N points)

- **File:** `kate/tests/spike_segment_proof.rs`
- Validates `PolyMultiProofNoPrecomp::open` with `polys = [one polynomial]`, `evals = [one vector]`, verified via `PolyMultiProofNoPrecomp::verify`.
- **Result:** Pass — no `poly-multiproof` fork needed for the 1×N segment use case.

### 2. CDA module in `avail-core`

- **[core/src/cda/mod.rs](core/src/cda/mod.rs)** — re-exports `constants`, `types`, `protocol`.
- **[core/src/cda/constants.rs](core/src/cda/constants.rs)** — `K_FACTOR = 16`, 8×8 / 16×16 extended grid, P2P 8×8, `ROW_EXTENSION_V4` / `COL_EXTENSION_V4`.
- **[core/src/cda/types.rs](core/src/cda/types.rs)** — `CodingVector`, `RLNCPiece`, `RLNCProof`, `SegmentProof`, `GridPosition`, `SubnetId`, `PieceIndex`, `NodeRole` (SCALE + `TypeInfo`). Proofs use `Vec<u8>` for serde / fixed-array limits.
- **[core/src/cda/protocol.rs](core/src/cda/protocol.rs)** — protocol names and on-wire structs (`PieceRequest`/`Response`, `SegmentProofRequest`/`Response`, `SubnetAnnounce`).
- **[core/src/lib.rs](core/src/lib.rs)** — `pub mod cda`.

### 3. Header extension V4

- **[core/src/header_version/mod.rs](core/src/header_version/mod.rs)** — `HeaderVersion::V4 = 3`.
- **[core/src/kate_commitment.rs](core/src/kate_commitment.rs)** — `v4::KateCommitment` (`column_commitments: Vec<u8>`, `data_root`).
- **[core/src/header/extension/v4.rs](core/src/header/extension/v4.rs)** — V4 `HeaderExtension`; `rows` / `cols` from extended-grid constants.
- **[core/src/header/extension/mod.rs](core/src/header/extension/mod.rs)** — `V4` variant, forward macro, `From<v4::HeaderExtension>`.
- **[core/src/header/mod.rs](core/src/header/mod.rs)** — test helpers: extra `V4` match arm (no-op) for exhaustiveness.

### 4. Kate — grid & column commitments

- **[kate/src/gridgen/core.rs](kate/src/gridgen/core.rs)**
  - `extend_full(row_factor, col_factor)` — RS by rows then by columns.
  - `make_column_polynomial_grid()` — iFFT per column.
  - `PolyOrientation` (`Row` / `Column`) on `PolynomialGrid`.
  - `column_commitments()` when orientation is `Column`.

### 5. Kate — segment proof

- **[kate/src/segment_proof.rs](kate/src/segment_proof.rs)** — `SegmentMultiProof`, `generate_segment_proof`, `verify_segment_proof` (1×N wrapper over `PolyMultiProofNoPrecomp`).
- **[kate/src/lib.rs](kate/src/lib.rs)** — `pub mod segment_proof` (behind `std`).

### 6. Kate — RLNC & homomorphic proofs (single-point KZG)

- **[kate/src/rlnc.rs](kate/src/rlnc.rs)**
  - `encode_rlnc`, `decode_rlnc`, `generate_random_coding_vector`.
  - **`combine_proofs_homomorphic`** — linear combination in G₁ of `Proof<Bls12_381>` from **single-point KZG** at the same evaluation point `z` (does **not** apply to transcript-mixed `PolyMultiProofNoPrecomp` openings).
  - `pub use avail_core::cda::types::CODING_VECTOR_LEN`.
- **Dependency:** `ark-ec` in [kate/Cargo.toml](kate/Cargo.toml) (and `ark-ec/std` under the `std` feature).

### 7. Kate — RLNC + KZG homomorphic tests

- **[kate/tests/rlnc_kzg_homomorphic.rs](kate/tests/rlnc_kzg_homomorphic.rs)**
  - Combined proof verifies against linearized commitment and value at `z`.
  - Negative test with a wrong value.

### 8. kate-recovery

- **[kate/recovery/src/proof.rs](kate/recovery/src/proof.rs)** — `verify_segment_proof`, `verify_column_commitment` (byte comparison of commitments).
- **[kate/recovery/src/rlnc.rs](kate/recovery/src/rlnc.rs)** — `ensure_decode_ready` (piece-count precheck).
- **[kate/recovery/src/lib.rs](kate/recovery/src/lib.rs)** — `pub mod rlnc`.

### 9. Build & test script

- **[build_test.sh](build_test.sh)**
  - `cd` to the script directory (repo root).
  - After `core` checks: **`cargo test -q`** (`avail-core` tests).
  - Under `kate`: **`cargo test --test spike_segment_proof`**, **`cargo test --test rlnc_kzg_homomorphic`**.
  - **Kate Recovery path:** `cd recovery` from `kate/` (replaces incorrect `cd ../recovery`).
  - After recovery checks: **`cargo test -q`**.

### 10. kate-recovery lifetime warning cleanup

- **[kate/recovery/src/com.rs](kate/recovery/src/com.rs)** — fixed explicit lifetime in:
  - `fn extract_encoded_extrinsic(range_data: &[u8]) -> SparseSliceRead<'_>`
- Purpose: remove warning seen from external workspace checks (e.g. when `avail-light` compiles against this `avail-core` path dependency):
  - `warning: hiding a lifetime that's elided elsewhere is confusing`
  - `warning: kate-recovery (lib) generated 1 warning`
- Validation: `cargo check -p kate-recovery` passes after the fix.

---

## Tests run (reference)

- `cargo test -p kate --test spike_segment_proof`
- `cargo test -p kate --test rlnc_kzg_homomorphic`
- `cargo test -p kate` (full suite, ~80+ tests)
- `cargo test -p avail-core` / `cargo build` with feature combinations from `build_test.sh` (re-run after changes).

---

## Follow-ups / suggestions (out of scope for the above)

- `combine_proofs_homomorphic` does **not** combine multi-polynomial `PolyMultiProofNoPrecomp` proofs that share a transcript — only **KZG `KZGProof::open`** at a common `z`.
- Extend tests: `extend_full` 8→16, `column_commitments` + segment proof end-to-end on a small grid.
- `kate-recovery::rlnc`: optional re-export of decode from `kate::rlnc` or dedicated column/segment verification tests.

---

## Proof model clarification (important)

There are **two different proof families** in this codebase:

1. **Single-point KZG proof (`KZGProof::open`)**
   - Used to open one polynomial at one evaluation point `z`.
   - Proof is a single G1 point (`method1::Proof` wrapping one affine point).
   - Linearity holds for the same `z`: this is why `combine_proofs_homomorphic` works here.

2. **BDFG multiproof (`PolyMultiProofNoPrecomp::open`)**
   - Used to open multiple polynomials / evaluations over a point set.
   - Internally depends on transcript challenge `γ` (Fiat-Shamir).
   - You cannot safely reuse the same linear-combination rule from single-point KZG on these proofs.

In short:
- `avail-core` **does have multiproof support** (open/verify paths are present and used).
- `combine_proofs_homomorphic` is intentionally scoped to **single-point KZG proofs**, not transcript-mixed multiproofs.

---

## Version note

- **2026-04-20** — Phase 0 core (CDA types, V4, kate grid/segment/RLNC, multiproof gate, KZG homomorphic tests, `build_test.sh` + `progress.md`).
