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

/// Gossip announcement published by a Store node after RLNC encode completes.
/// Fat nodes decode this (SCALE-first), self-filter column ownership, then pull
/// pieces from the announcing Store via CDA_PIECE_PROTOCOL request_response.
///
/// Wire format: SCALE (`Encode::encode()`), published on the column subnet gossip
/// topic `/engram/cda/subnet/col/{c}/1`. Size ≤ 64 bytes encoded.
#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CdaPieceReadyAnnounce {
	pub block_number: u32,
	#[codec(compact)]
	pub ext_row: u16,
	#[codec(compact)]
	pub ext_col: u16,
	/// Raw libp2p PeerId bytes (`PeerId::to_bytes()`). Avoids stringly-typed wire fields.
	pub store_peer_id: Vec<u8>,
	#[codec(compact)]
	pub piece_count: u16,
	/// FNV-1a-32 over the encoded `RawCellDistribution` payload (same algorithm as
	/// `RawCellDistribution::payload_checksum32`). Lets Fat detect cross-store content
	/// divergence and lets the scorer detect mid-flight bit rot.
	pub payload_checksum32: u32,
}
