use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const SCALAR_ENCODED_SIZE: usize = 32;
pub const PROOF_ENCODED_SIZE: usize = 48;
pub const CODING_VECTOR_LEN: usize = 16;

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CodingVector(pub [[u8; SCALAR_ENCODED_SIZE]; CODING_VECTOR_LEN]);

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RLNCPiece {
	pub coded_data: Vec<u8>,
	pub coding_vector: CodingVector,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct RLNCProof(pub Vec<u8>);

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SegmentProof {
	pub proof: Vec<u8>,
	pub evals: Vec<[u8; SCALAR_ENCODED_SIZE]>,
	#[codec(compact)]
	pub column: u16,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridPosition {
	#[codec(compact)]
	pub row: u16,
	#[codec(compact)]
	pub col: u16,
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, TypeInfo, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SubnetId {
	Row(#[codec(compact)] u16),
	Column(#[codec(compact)] u16),
}

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PieceIndex(#[codec(compact)] pub u16);

#[derive(PartialEq, Eq, Clone, Copy, Encode, Decode, TypeInfo, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NodeRole {
	Validator,
	FullNode,
	Bootstrap,
	StoreNode,
	FatNode,
}
