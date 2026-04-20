use crate::{cda::constants, kate_commitment::v4::KateCommitment, DataLookup};
use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;
use sp_std::vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "runtime")]
use sp_debug_derive::RuntimeDebug;

#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, TypeInfo)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "runtime", derive(RuntimeDebug))]
pub struct HeaderExtension {
	pub app_lookup: DataLookup,
	pub commitment: KateCommitment,
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 {
		self.commitment.data_root
	}

	pub fn app_lookup(&self) -> &DataLookup {
		&self.app_lookup
	}

	pub fn rows(&self) -> u16 {
		constants::GRID_ROWS_EXTENDED
	}

	pub fn cols(&self) -> u16 {
		constants::GRID_COLS_EXTENDED
	}

	pub fn get_empty_header(data_root: H256) -> Self {
		let empty_app_lookup = DataLookup::new_empty();
		let commitment = KateCommitment::new(data_root, vec![]);
		HeaderExtension {
			app_lookup: empty_app_lookup,
			commitment,
		}
	}

	pub fn get_faulty_header(data_root: H256) -> Self {
		let error_app_lookup = DataLookup::new_error();
		let commitment = KateCommitment::new(data_root, vec![]);
		HeaderExtension {
			app_lookup: error_app_lookup,
			commitment,
		}
	}
}
