use bevy::prelude::*;

use crate::util::hex_gizmo::column;
use crate::game::placement::{grid::Grid, cursor::Cursor};

pub fn update(
	mut gizmos: Gizmos,
	cursor: Res<Cursor>,
	grid: Res<Grid>,
) {
	let (pos, _) = match &cursor.hover_cell {Some(hover_cell) => hover_cell, None => return};
	
	column(
		&mut gizmos,
		pos,
		&grid,
		Some(Color::srgb(1., 0., 0.)),
		Some(Color::srgb(0., 0., 1.)),
	);
}
