# CDA / avail-core — Tiến độ triển khai

Tài liệu ghi lại các thay đổi đã thực hiện trong repo **avail-core** (Phase 0 CDA, không gồm `avail` / `avail-light`).

---

## Phase 0 — Tổng quan

Mục tiêu: nền tảng types, header V4, mở rộng `kate` / `kate-recovery` cho lưới 8×8 → extended 16×16, commitment theo cột, segment multiproof, RLNC, và kiểm chứng **không cần fork `poly-multiproof`** (gate `PolyMultiProofNoPrecomp::open` với 1 polynomial).

---

## Đã hoàn thành

### 1. Gate multiproof (1 polynomial × N điểm)

- **File:** `kate/tests/spike_segment_proof.rs`
- Kiểm chứng `PolyMultiProofNoPrecomp::open` với `polys = [một đa thức]`, `evals = [một vector]`, verify qua `PolyMultiProofNoPrecomp::verify`.
- **Kết luận:** Pass — không cần fork `poly-multiproof` cho use-case segment 1×N.

### 2. Module CDA trong `avail-core`

- **`core/src/cda/mod.rs`** — gom `constants`, `types`, `protocol`.
- **`core/src/cda/constants.rs`** — `K_FACTOR = 16`, grid 8×8 / extended 16×16, P2P 8×8, `ROW_EXTENSION_V4` / `COL_EXTENSION_V4`.
- **`core/src/cda/types.rs`** — `CodingVector`, `RLNCPiece`, `RLNCProof`, `SegmentProof`, `GridPosition`, `SubnetId`, `PieceIndex`, `NodeRole` (SCALE + `TypeInfo`). Proof dùng `Vec<u8>` để tương thích serde/array bounds.
- **`core/src/cda/protocol.rs`** — tên protocol và struct wire (`PieceRequest`/`Response`, `SegmentProofRequest`/`Response`, `SubnetAnnounce`).
- **`core/src/lib.rs`** — `pub mod cda`.

### 3. Header extension V4

- **`core/src/header_version/mod.rs`** — thêm `HeaderVersion::V4 = 3`.
- **`core/src/kate_commitment.rs`** — module `v4::KateCommitment` (`column_commitments: Vec<u8>`, `data_root`).
- **`core/src/header/extension/v4.rs`** — `HeaderExtension` V4; `rows`/`cols` từ constants extended.
- **`core/src/header/extension/mod.rs`** — variant `V4`, macro forward, `From<v4::HeaderExtension>`.
- **`core/src/header/mod.rs`** — test helper: match thêm arm `V4` (no-op) cho exhaustiveness.

### 4. Kate — lưới & commitment cột

- **`kate/src/gridgen/core.rs`**
  - `extend_full(row_factor, col_factor)` — RS theo hàng rồi theo cột.
  - `make_column_polynomial_grid()` — iFFT theo cột.
  - `PolyOrientation` (`Row` / `Column`) trên `PolynomialGrid`.
  - `column_commitments()` khi orientation = `Column`.

### 5. Kate — segment proof

- **`kate/src/segment_proof.rs`** — `SegmentMultiProof`, `generate_segment_proof`, `verify_segment_proof` (wrapper 1×N trên `PolyMultiProofNoPrecomp`).
- **`kate/src/lib.rs`** — `pub mod segment_proof` (feature `std`).

### 6. Kate — RLNC & proof homomorphic (KZG một điểm)

- **`kate/src/rlnc.rs`**
  - `encode_rlnc`, `decode_rlnc`, `generate_random_coding_vector`.
  - **`combine_proofs_homomorphic`** — tổ hợp tuyến tính trong G₁ các `Proof<Bls12_381>` của **KZG single-point cùng điểm `z`** (không áp dụng cho multiproof có transcript γ).
  - `pub use avail_core::cda::types::CODING_VECTOR_LEN`.
- **Dependency:** `ark-ec` trong `kate/Cargo.toml` (và `ark-ec/std` trong feature `std`).

### 7. Kate — test RLNC + KZG homomorphic

- **`kate/tests/rlnc_kzg_homomorphic.rs`**
  - Verify proof tổ hợp khớp commitment + giá trị tại `z`.
  - Negative test với giá trị sai.

### 8. kate-recovery

- **`kate/recovery/src/proof.rs`** — `verify_segment_proof`, `verify_column_commitment` (so sánh bytes commitment).
- **`kate/recovery/src/rlnc.rs`** — `ensure_decode_ready` (precheck số piece).
- **`kate/recovery/src/lib.rs`** — `pub mod rlnc`.

### 9. CI / script build & test

- **`build_test.sh`**
  - `cd` về thư mục chứa script (repo root).
  - Sau check `core`: **`cargo test -q`** (tests `avail-core`).
  - Trong `kate`: **`cargo test --test spike_segment_proof`**, **`cargo test --test rlnc_kzg_homomorphic`**.
  - Sửa đường dẫn **Kate Recovery:** `cd recovery` (từ thư mục `kate/`), thay cho `cd ../recovery` (sai).

---

## Kiểm thử đã chạy (tham chiếu)

- `cargo test -p kate --test spike_segment_proof`
- `cargo test -p kate --test rlnc_kzg_homomorphic`
- `cargo test -p kate` (toàn bộ, ~80+ tests)
- `cargo test -p avail-core` / `cargo build` các feature như trong `build_test.sh` (nên chạy lại sau thay đổi).

---

## Việc còn lại / gợi ý (ngoài scope đã làm xong)

- `combine_proofs_homomorphic` **không** kết hợp được các `PolyMultiProofNoPrecomp` proof đa đa thức có cùng transcript — chỉ dùng cho **KZG `KZGProof::open`** cùng `z`.
- Mở rộng test: `extend_full` 8→16, `column_commitments` + segment proof end-to-end trên grid nhỏ.
- `kate-recovery::rlnc`: có thể bổ sung re-export decode từ `kate::rlnc` hoặc test verify column/segment mới.

---

## Ghi chú phiên bản

- Cập nhật log: **2026-04-20** — Phase 0 cốt lõi (CDA types, V4, kate grid/segment/RLNC, gate multiproof, test KZG homomorphic, `build_test.sh` + `progress.md`).
