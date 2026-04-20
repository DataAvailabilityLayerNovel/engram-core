#!/bin/bash
set -x

# Run from repository root (directory containing core/, kate/).
cd "$(dirname "$0")"

cd core

# Core
cargo check
cargo check --no-default-features
cargo check --no-default-features --features "serde"
cargo check --no-default-features --features "std"
cargo check --no-default-features --features "std, serde"
cargo check --target wasm32-unknown-unknown --no-default-features
cargo check --target wasm32-unknown-unknown --no-default-features --features "serde"
cargo check --target wasm32-unknown-unknown --no-default-features --features "runtime"
cargo check --target wasm32-unknown-unknown --no-default-features --features "runtime, serde"

# Core tests (CDA types, header V3/V4, etc.)
cargo test -q

# Kate
cd ../kate
cargo check
cargo check --no-default-features
cargo check --no-default-features --features "serde"
cargo check --no-default-features --features "std"
cargo check --no-default-features --features "std, serde"
cargo check --target wasm32-unknown-unknown --no-default-features
cargo check --target wasm32-unknown-unknown --no-default-features --features "serde"

# Kate integration tests (CDA gate: 1-polynomial multiproof; RLNC + KZG homomorphic openings)
cargo test --test spike_segment_proof
cargo test --test rlnc_kzg_homomorphic

# Kate Recovery
cd recovery
cargo check
cargo check --no-default-features
cargo check --no-default-features --features "serde"
cargo check --no-default-features --features "std"
cargo check --no-default-features --features "std, serde"
cargo check --target wasm32-unknown-unknown --no-default-features
cargo check --target wasm32-unknown-unknown --no-default-features --features "serde"

cargo test -q
