use bevy::{prelude::*, gizmos::gizmos::GizmoBuffer};
use hexx::Hex;

use crate::util::hex::{axial_to_xz, HexCorner, corner_height};
use crate::game::placement::grid::Grid;

pub fn column_level<Config: GizmoConfigGroup, Clear: 'static + Send + Sync>( // TODO: Clean up. Idk what this is, GizmoBuffer needs it.
	gizmos: &mut GizmoBuffer<Config, Clear>,
	pos: &Hex,
	height: f32,
	c_top: Option<Color>,
	c_sides: Option<Color>,
	c_star: Option<Color>,
) {
	let [x, z] = axial_to_xz(pos);
	let center = Vec3::new(x, height, z);
	for (i, c1) in HexCorner::get_array().iter().enumerate() {
		let c2 = HexCorner::get_array()[(i + 1) % 6];
		let [c1x, c1z] = c1.to_xz();
		let [c2x, c2z] = c2.to_xz();

		let v1 = Vec3::new(x + c1x, height, z + c1z);
		let v2 = Vec3::new(x + c2x, height, z + c2z);
		let v1b = Vec3::new(x + c1x, 0., z + c1z);
		match c_top {Some(color) => gizmos.line(v1, v2, color), None => ()}
		match c_sides {Some(color) => gizmos.line(v1, v1b, color), None => ()}
		match c_star {Some(color) => gizmos.line(center, v1, color), None => ()}
	}
}

pub fn column<Config: GizmoConfigGroup, Clear: 'static + Send + Sync>( // TODO: Clean up. Idk what this is, GizmoBuffer needs it.
	gizmos: &mut GizmoBuffer<Config, Clear>,
	pos: &Hex,
	grid: &Grid,
	c_top: Option<Color>,
	c_sides: Option<Color>,
	c_star: Option<Color>,
) {
	let [x, z] = axial_to_xz(pos);
	let height = grid.cells.get(pos).unwrap().height as f32;
	let center = Vec3::new(x, height, z);
	for (i, c1) in HexCorner::get_array().iter().enumerate() {
		let c2 = HexCorner::get_array()[(i + 1) % 6];
		let [c1x, c1z] = c1.to_xz();
		let [c2x, c2z] = c2.to_xz();

		let v1 = Vec3::new(x + c1x, corner_height(grid, pos, *c1), z + c1z);
		let v2 = Vec3::new(x + c2x, corner_height(grid, pos, c2), z + c2z);
		let v1b = Vec3::new(x + c1x, 0., z + c1z);
		match c_top {Some(color) => gizmos.line(v1, v2, color), None => ()}
		match c_sides {Some(color) => gizmos.line(v1, v1b, color), None => ()}
		match c_star {Some(color) => gizmos.line(center, v1, color), None => ()}
	}
}
