use bevy::prelude::*;

use crate::util::hex::{axial_to_xz, HexCorner};
use crate::game::placement::cursor::Cursor;

pub struct GridSelectPlugin;
impl Plugin for GridSelectPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, cell_highlight_system);
	}
}

pub fn cell_highlight_system(
	mut gizmos: Gizmos,
	cursor: Res<Cursor>,
) {
	let (pos, cell) = match &cursor.hover_cell {Some(hover_cell) => hover_cell, None => return};
	let height = cell.height as f32;
	let [x, z] = axial_to_xz(&pos);

	for (i, c) in HexCorner::get_array().iter().enumerate() {
		let cn = HexCorner::get_array()[(i + 1) % 6];
		let [cx, cz] = c.to_xz();
		let [cnx, cnz] = cn.to_xz();

		let v = Vec3::new(x + cx, height, z + cz);
		let vn = Vec3::new(x + cnx, height, z + cnz);
		let vb = Vec3::new(x + cx, 0., z + cz);
		gizmos.line(v, vn, Color::srgb(1., 0., 0.));
		gizmos.line(v, vb, Color::srgb(0., 0., 1.));
	}
}
