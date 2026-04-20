use core::num::NonZeroU16;

pub const K_FACTOR: u16 = 16;
pub const MATRIX_CHUNKS: u16 = 16;

pub const GRID_ROWS_ORIGINAL: u16 = 8;
pub const GRID_COLS_ORIGINAL: u16 = 8;
pub const GRID_ROWS_EXTENDED: u16 = GRID_ROWS_ORIGINAL * 2;
pub const GRID_COLS_EXTENDED: u16 = GRID_COLS_ORIGINAL * 2;

pub const P2P_ROWS: u16 = 8;
pub const P2P_COLS: u16 = 8;

pub const fn p2p_row_from_extended(row: u16) -> u16 {
	row / 2
}

pub const fn p2p_col_from_extended(col: u16) -> u16 {
	col / 2
}

pub const ROW_EXTENSION_V4: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(2) };
pub const COL_EXTENSION_V4: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(2) };
