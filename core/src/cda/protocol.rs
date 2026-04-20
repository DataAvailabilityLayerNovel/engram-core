use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::types::{GridPosition, NodeRole, RLNCPiece, SegmentProof};

pub const CDA_PIECE_PROTOCOL: &str = "/engram/cda/piece/1";
pub const CDA_SEGMENT_PROOF_PROTOCOL: &str = "/engram/cda/segment-proof/1";
pub const CDA_SUBNET_GOSSIP_PROTOCOL: &str = "/engram/cda/subnet/1";

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PieceRequest {
	pub block_number: u32,
	#[codec(compact)]
	pub row: u16,
	#[codec(compact)]
	pub col: u16,
	#[codec(compact)]
	pub piece_index: u16,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PieceResponse {
	pub piece: RLNCPiece,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SegmentProofRequest {
	pub block_number: u32,
	#[codec(compact)]
	pub column: u16,
	pub points: Vec<u16>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SegmentProofResponse {
	pub proof: SegmentProof,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SubnetAnnounce {
	pub peer_position: GridPosition,
	pub roles: Vec<NodeRole>,
}
