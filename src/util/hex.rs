use hexx::Hex;

/// Converts axial hex coordinates to xz pixel coordinates.
pub const fn axial_to_xz(pos: &Hex) -> [f32; 2] {
	// TODO: Use fancy new std::f32::consts::SQRT_3 when available. https://github.com/rust-lang/rust/issues/103883
	const SQRT_3: f32 = 1.732050807568877293527446341505872367;
	[pos.x as f32 * 3. / 2., pos.x as f32 * SQRT_3 / 2. + pos.y as f32 * SQRT_3]
}

/// Converts odd-q vertical layout hexagonal coordinates to axial hexagonal coordinates.
/// https://www.redblobgames.com/grids/hexagons/#coordinates-offset
pub fn offset_to_axial(col: i32, row: i32) -> Hex {
	Hex {x: col, y: row - (col + 1) / 2}
}
