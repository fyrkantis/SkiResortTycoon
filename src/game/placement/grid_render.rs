use bevy::{
	prelude::*,
	render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}
};
use hexx::*;

use crate::util::hex::axial_to_xz;
use crate::game::placement::grid::Grid;
use crate::game::placement::cursor::Cursor;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CellHex(Hex);

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	grid: Res<Grid>,
) {
	let snow_material = materials.add(Color::WHITE);
	for (pos, cell) in &grid.cells {
		let mesh_info = ColumnMeshBuilder::new(&HexLayout::flat(), cell.height as f32).build();
		let [x, z] = axial_to_xz(pos);
		
		commands.spawn((
			CellHex(*pos),
			Mesh3d(meshes.add(hexagonal_mesh(mesh_info))),
			MeshMaterial3d(snow_material.clone()),
			Transform::from_xyz(x, 0., z),
		))
		.observe(|
			mut trigger: Trigger<Pointer<Pressed>>,
			cells: Query<&CellHex>,
		| {
			let pos = match cells.get(trigger.target()) {Ok(pos) => pos, Err(e) => {error!("Mouse clicked cell, but it's missing a CellHex position: {}", e); return}};
			println!("Hex Click: {:?}", pos);
		})
		.observe(|
			mut trigger: Trigger<Pointer<Over>>, // Mouse hovering.
			cells: Query<&CellHex>,
			grid: Res<Grid>,
			mut cursor: ResMut<Cursor>,
		| {
			
			let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse hovered over cell, but it's missing a CellHex position: {}", e); return}};
			let cell = match grid.cells.get(&pos) {Some(cell) => cell, None => {error!("Mouse hovered over cell, but it's CellHex position could not be found in grid."); return}};
			cursor.hover_cell = Some((pos, *cell));
		})
		.observe(|
			mut trigger: Trigger<Pointer<Out>>, // Mouse no longer hovering.
			cells: Query<&CellHex>,
			grid: Res<Grid>,
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

/// Converts hexx MeshInfo into bevy Mesh.
/// From hexx docs example: https://docs.rs/hexx/latest/hexx/index.html#usage-in-bevy
fn hexagonal_mesh(mesh_info: MeshInfo) -> Mesh {
	Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::all(), // Needed for mesh to be pickable.
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
	.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
	.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
	.with_inserted_indices(Indices::U16(mesh_info.indices))
}
