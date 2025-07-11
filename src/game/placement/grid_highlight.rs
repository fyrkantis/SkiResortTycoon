use bevy::prelude::*;

use crate::util::hex_gizmo::column_level;
use crate::game::placement::cursor::Cursor;

pub fn update(
	mut gizmos: Gizmos,
	cursor: Res<Cursor>,
) {
	let (pos, cell) = match &cursor.hover_cell {Some(hover_cell) => hover_cell, None => return};

	column_level(
		&mut gizmos,
		pos,
		cell.height as f32,
		Some(Color::srgb(1., 0., 0.)),
		Some(Color::srgb(0., 0., 1.)),
	);
}
