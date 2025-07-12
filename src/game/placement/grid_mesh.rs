use bevy::{
	prelude::*,
	render::{render_asset::RenderAssetUsages, primitives::Aabb, mesh::MeshAabb}
};
use hexx::Hex;

use crate::util::{
	hex::axial_to_xz,
	hex_mesh::cell_sharp_mesh,
};
use crate::game::placement::{grid::Grid, cursor::Cursor};

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CellHex(Hex);

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	grid: Res<Grid>,
) {
	let it_snow = materials.add(Color::WHITE);
	for (pos, _) in &grid.cells {
		let [x, z] = axial_to_xz(pos);
		commands.spawn((
			CellHex(*pos),
			Mesh3d(meshes.add(cell_sharp_mesh(&grid, pos, RenderAssetUsages::all()))),
			MeshMaterial3d(it_snow.clone()),
			Transform::from_xyz(x, 0., z),
		))
		.observe(|
			trigger: Trigger<Pointer<Click>>,
			cells: Query<&CellHex>,
			mut grid: ResMut<Grid>,
		| {
			let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse clicked cell, but it's missing a CellHex position: {}", e); return}};
			println!("Hex Click: {:?}", pos);

			let cell = match grid.cells.get_mut(&pos) {Some(cell) => cell, None => {error!("Mouse clicked cell, but it's CellHex position could not be found in grid."); return}};
			if trigger.button == PointerButton::Primary {
				cell.height += 1;
			} else if trigger.button == PointerButton::Secondary {
				if cell.height <= 0 {
					warn!("Can't lower cell {:?} because it's already at height {}.", pos, cell.height);
				} else {
					cell.height -= 1;
				}
			}
		})
		.observe(|
			trigger: Trigger<Pointer<Over>>, // Mouse hovering.
			cells: Query<&CellHex>,
			grid: Res<Grid>,
			mut cursor: ResMut<Cursor>,
		| {
			
			let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse hovered over cell, but it's missing a CellHex position: {}", e); return}};
			let cell = match grid.cells.get(&pos) {Some(cell) => cell, None => {error!("Mouse hovered over cell, but it's CellHex position could not be found in grid."); return}};
			cursor.hover_cell = Some((pos, *cell));
		})
		.observe(|
			trigger: Trigger<Pointer<Out>>, // Mouse no longer hovering.
			cells: Query<&CellHex>,
			mut cursor: ResMut<Cursor>,
		| {
			let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse hovered over cell, but it's missing a CellHex position: {}", e); return}};
			let current_pos = match cursor.hover_cell {Some((current_pos, _current_cursor)) => current_pos, None => {return}};
			if current_pos == pos {
				cursor.hover_cell = None;
			}
		});
	}
}

pub fn update(
	mut meshes: ResMut<Assets<Mesh>>,
	grid: Res<Grid>,
	mut query: Query<(&CellHex, &mut Mesh3d, &mut Aabb)>,
) {
	if grid.is_changed() {
		info_once!("Grid update!");
		for (cell, mut mesh, mut aabb) in query.iter_mut() {
			let pos = cell.0;
			let new_mesh = cell_sharp_mesh(&grid, &pos, RenderAssetUsages::all());
			// TODO: Remove this if mesh picking bug is fixed.
			// Currently, the Axis-Aligned Bounding Box is
			// not updated automatically when the mesh changes.
			// https://github.com/bevyengine/bevy/issues/18221#issuecomment-2746183172
			*aabb = new_mesh.compute_aabb().unwrap();
			*mesh = Mesh3d(meshes.add(new_mesh));
		}
	}
}
