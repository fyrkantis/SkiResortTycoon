use bevy::{
	prelude::*,
	render::{render_asset::RenderAssetUsages, primitives::Aabb, mesh::MeshAabb}
};
use hexx::Hex;

use crate::util::{
	hex::{axial_to_xz, cell_slope},
	hex_mesh::cell_sharp_mesh,
};
use crate::game::{
	placement::{grid::Grid, cursor::{Cursor, Tool}},
	materials::Materials,
	surface::Surface,
};

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CellHex(Hex);

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	materials: Res<Materials>,
	grid: Res<Grid>,
	
) {
	for (pos, _) in &grid.cells {
		let [x, z] = axial_to_xz(pos);
		commands.spawn((
			CellHex(*pos),
			Mesh3d(meshes.add(cell_sharp_mesh(&grid, pos, RenderAssetUsages::all()))),
			MeshMaterial3d(cell_material(&materials, &grid, pos).clone()),
			Transform::from_xyz(x, 0., z),
		))
		.observe(|
			trigger: Trigger<Pointer<Pressed>>,
			cells: Query<&CellHex>,
			mut grid: ResMut<Grid>,
			cursor: Res<Cursor>,
			mut query_meshes: Query<(&CellHex, &mut Mesh3d, &mut Aabb)>,
			mut query_materials: Query<(&CellHex, &mut MeshMaterial3d<StandardMaterial>)>,
			mut meshes: ResMut<Assets<Mesh>>,
			materials: Res<Materials>,
		| {
			let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse clicked cell, but it's missing a CellHex position: {}", e); return}};
			println!("Hex Click: {:?}", pos);

			let cell = match grid.cells.get_mut(&pos) {Some(cell) => cell, None => {error!("Mouse clicked cell, but it's CellHex position could not be found in grid."); return}};

			if cursor.tool == Some(Tool::Terrain) {
				if trigger.button == PointerButton::Primary {
					cell.height += 1;
					update_meshes(query_meshes, &mut meshes, &grid);
					update_materials(query_materials, &materials, &grid);
				} else if trigger.button == PointerButton::Secondary {
					if cell.height <= 0 {
						warn!("Can't lower cell {:?} because it's already at height {}.", pos, cell.height);
					} else {
						cell.height -= 1;
						update_meshes(query_meshes, &mut meshes, &grid);
						update_materials(query_materials, &materials, &grid);
					}
				}
			} else if cursor.tool == Some(Tool::Surface) {
				if trigger.button == PointerButton::Primary {
					if cell.surface != Surface::Normal {
						warn!("Can't add piste because the surface is not normal.");
					} else {
						cell.surface = Surface::Piste;
						update_materials(query_materials, &materials, &grid)
					}
				} else if trigger.button == PointerButton::Secondary {
					if cell.surface != Surface::Piste {
						warn!("Can't remove piste because the surface is already not piste.");
					} else {
						cell.surface = Surface::Normal;
						update_materials(query_materials, &materials, &grid);
					}
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

/// This is not a system.
pub fn update_meshes(
	mut query: Query<(&CellHex, &mut Mesh3d, &mut Aabb)>,
	meshes: &mut ResMut<Assets<Mesh>>,
	grid: &Grid,
) {
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

/// This is also not a system.
pub fn update_materials(
	mut query: Query<(&CellHex, &mut MeshMaterial3d<StandardMaterial>)>,
	materials: &Materials,
	grid: &Grid,
) {
	for (cell, mut material) in query.iter_mut() {
		let pos = cell.0;
		let new_material = cell_material(&materials, grid, &pos);
		if material.0.id() != new_material.id() {
			*material = MeshMaterial3d(new_material.clone());
		}
	}
}

pub const SNOW_MAX_SLOPE: u16 = 3;
pub const DIRT_MAX_SLOPE: u16 = 4;

 fn cell_material<'a>(materials: &'a Materials, grid: &Grid, pos: &Hex) -> &'a Handle<StandardMaterial> {
	let cell = grid.cells.get(pos).unwrap();
	match cell.surface {
		Surface::Piste => &materials.piste,
		Surface::Water => &materials.water,
		Surface::Normal => {
			let slope = cell_slope(grid, pos);
			if slope > DIRT_MAX_SLOPE {return &materials.rock}
			else if slope > SNOW_MAX_SLOPE {return &materials.dirt}
			else {return &materials.snow}
		}
	}
}
