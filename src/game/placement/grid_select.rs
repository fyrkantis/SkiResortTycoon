use bevy::prelude::*;

use crate::util::hex::axial_to_xz;
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
	let [x, y] = axial_to_xz(&pos);

	// TODO: Use fancy new std::f32::consts::SQRT_3 when available. https://github.com/rust-lang/rust/issues/103883
	const SQRT_3: f32 = 1.732050807568877293527446341505872367;
	const SQRT_3_DIV_2: f32 = SQRT_3 / 2.;
	let corners2d = [
		Vec2::new(x + 0.5, y + SQRT_3_DIV_2), Vec2::new(x + 1., y), Vec2::new(x + 0.5, y - SQRT_3_DIV_2),
		Vec2::new(x - 0.5, y - SQRT_3_DIV_2), Vec2::new(x - 1., y), Vec2::new(x - 0.5, y + SQRT_3_DIV_2)
	];

	for (i, c) in corners2d.iter().enumerate() {
		let cn = corners2d[(i + 1) % 6];
		let v = Vec3::new(c.x, height, c.y);
		let vn = Vec3::new(cn.x, height, cn.y);
		let vb = Vec3::new(c.x, 0., c.y);
		gizmos.line(v, vn, Color::srgb(1., 0., 0.));
		gizmos.line(v, vb, Color::srgb(0., 0., 1.));
	}
}
