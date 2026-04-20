use avail_core::cda::types::{RLNCPiece, CODING_VECTOR_LEN};
use thiserror_no_std::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("not enough coded pieces to decode")]
	NotEnoughPieces,
}

/// Recovery-side precheck for RLNC decode inputs.
pub fn ensure_decode_ready(coded_pieces: &[RLNCPiece]) -> Result<(), Error> {
	if coded_pieces.len() < CODING_VECTOR_LEN {
		return Err(Error::NotEnoughPieces);
	}
	Ok(())
}
