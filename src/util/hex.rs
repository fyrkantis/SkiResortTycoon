use hexx::Hex;

use crate::game::placement::grid::Grid; // TODO: Replace with trait describing HashMap<Hex, u16>.

// TODO: Use fancy new std::f32::consts::SQRT_3 when available. https://github.com/rust-lang/rust/issues/103883
const SQRT_3: f32 = 1.732050807568877293527446341505872367;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HexEdge {
	TopCenter,
	TopRight,
	BottomRight,
	BottomCenter,
	BottomLeft,
	TopLeft,
}
impl HexEdge {
	pub const fn direction(&self) -> Hex {
		match self {
			Self::TopCenter => Hex::new(0, -1),
			Self::TopRight => Hex::new(1, -1),
			Self::BottomRight => Hex::new(1, 0),
			Self::BottomCenter => Hex::new(0, 1),
			Self::BottomLeft => Hex::new(-1, 1),
			Self::TopLeft => Hex::new(-1, 0),
		}
	}
	pub const fn get_array() -> [HexEdge; 6] {
		[Self::TopCenter, Self::TopRight, Self::BottomRight, Self::BottomCenter, Self::BottomLeft, Self::TopLeft]
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HexCorner {
	TopRight,
	MiddleRight,
	BottomRight,
	BottomLeft,
	MiddleLeft,
	TopLeft,
}
impl HexCorner {
	pub const fn is_even(&self) -> bool {
		match self {
			Self::TopRight => true,
			Self::MiddleRight => false,
			Self::BottomRight => true,
			Self::BottomLeft => false,
			Self::MiddleLeft => true,
			Self::TopLeft => false,
		}
	}
	pub const fn to_xz(&self) -> [f32; 2] {
		const SQRT_3_DIV_2: f32 = SQRT_3 / 2.;
		match self {
			Self::TopRight => [0.5, -SQRT_3_DIV_2],
			Self::MiddleRight => [1., 0.],
			Self::BottomRight => [0.5, SQRT_3_DIV_2],
			Self::BottomLeft => [-0.5, SQRT_3_DIV_2],
			Self::MiddleLeft => [-1., 0.],
			Self::TopLeft => [-0.5, -SQRT_3_DIV_2],
		}
	}
	pub const fn neighbor_edges(&self) -> [HexEdge; 2] {
		match self {
			Self::TopRight => [HexEdge::TopCenter, HexEdge::TopRight],
			Self::MiddleRight => [HexEdge::TopRight, HexEdge::BottomRight],
			Self::BottomRight => [HexEdge::BottomRight, HexEdge::BottomCenter],
			Self::BottomLeft => [HexEdge::BottomCenter, HexEdge::BottomLeft],
			Self::MiddleLeft => [HexEdge::BottomLeft, HexEdge::TopLeft],
			Self::TopLeft => [HexEdge::TopLeft, HexEdge::TopCenter],
		}
	}
	pub const fn get_array() -> [HexCorner; 6] {
		[Self::TopRight, Self::MiddleRight, Self::BottomRight, Self::BottomLeft, Self::MiddleLeft, Self::TopLeft]
	}
}

/// Calculates the height of a corner of a hex cell in a grid (which is an average of all surounding cell heights.
/// Panics if pos is not in grid.cells.
pub fn corner_height(grid: &Grid, pos: &Hex, corner: HexCorner) -> f32 {
	let [edge_1, edge_2] = corner.neighbor_edges();
	// Positions of vertex neighboring cells.
	let (pos_1, pos_2) = (*pos + edge_1.direction(), *pos + edge_2.direction());
	let (cell_1_opt, cell_2_opt) = (grid.cells.get(&pos_1), grid.cells.get(&pos_2));

	let center_y = grid.cells.get(pos).unwrap().height as f32;

	// Average height of all 1-3 cells.
	match cell_1_opt {
		Some(cell_1) => match cell_2_opt {
			Some(cell_2) => (center_y + cell_1.height as f32 + cell_2.height as f32) / 3.,
			None => (center_y + cell_1.height as f32) / 2.,
		},
		None => match cell_2_opt {
			Some(cell_2) => (center_y + cell_2.height as f32) / 2.,
			None => center_y,
		}
	}
}

/// Converts axial hex coordinates to xz pixel coordinates.
pub const fn axial_to_xz(pos: &Hex) -> [f32; 2] {
	[pos.x as f32 * 3. / 2., pos.x as f32 * SQRT_3 / 2. + pos.y as f32 * SQRT_3]
}

/// Converts odd-q vertical layout hexagonal coordinates to axial hexagonal coordinates.
/// https://www.redblobgames.com/grids/hexagons/#coordinates-offset
pub fn offset_to_axial(col: i32, row: i32) -> Hex {
	Hex {x: col, y: row - (col + 1) / 2}
}
