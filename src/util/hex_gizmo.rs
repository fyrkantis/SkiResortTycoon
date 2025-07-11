use bevy::{prelude::*, gizmos::gizmos::GizmoBuffer};
use hexx::Hex;

use crate::util::hex::{axial_to_xz, HexCorner};

pub fn column_level<Config: GizmoConfigGroup, Clear: 'static + Send + Sync>( // TODO: Clean up. Idk what this is, GizmoBuffer needs it.
	gizmos: &mut GizmoBuffer<Config, Clear>,
	pos: &Hex,
	height: f32,
	c_top: Option<Color>,
	c_sides: Option<Color>,
) {
	let [x, z] = axial_to_xz(pos);
	for (i, c) in HexCorner::get_array().iter().enumerate() {
		let cn = HexCorner::get_array()[(i + 1) % 6];
		let [cx, cz] = c.to_xz();
		let [cnx, cnz] = cn.to_xz();

		let v = Vec3::new(x + cx, height, z + cz);
		let vn = Vec3::new(x + cnx, height, z + cnz);
		let vb = Vec3::new(x + cx, 0., z + cz);
		match c_top {Some(color) => gizmos.line(v, vn, color), None => ()}
		match c_sides {Some(color) => gizmos.line(v, vb, color), None => ()}
	}
}
