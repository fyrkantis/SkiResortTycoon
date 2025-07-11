use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use hexx::Hex;

use crate::util::{
	hex::axial_to_xz,
	hex_mesh::cell_mesh,
};
use crate::game::placement::{grid::Grid, cursor::Cursor};

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CellHex(Hex);

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	grid: Res<Grid>,
) {
	for (pos, _) in &grid.cells {
		let [x, z] = axial_to_xz(pos);
		commands.spawn((
			CellHex(*pos),
			Mesh3d(meshes.add(cell_mesh(&grid, pos, RenderAssetUsages::MAIN_WORLD))),
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
	query: Query<(&CellHex, &Mesh3d)>,
) {
	for (cell, mesh_handle) in query.iter() {
		let pos = cell.0;
		match meshes.get_mut(&mesh_handle.0) {
			Some(mesh) => *mesh = cell_mesh(&grid, &pos, RenderAssetUsages::MAIN_WORLD),
			None => warn_once!("Failed to update selection mesh for cell {:?} because its mesh asset could not be found.", pos),
		}
	}
}
