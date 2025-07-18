use bevy::{
	prelude::*,
	render::render_asset::RenderAssetUsages,
};

use crate::util::{
	hex::axial_to_xz,
	hex_mesh::cell_sharp_mesh,
};
use crate::game::{
	placement::{
		grid::{Grid, CellPos, CellMesh},
		grid_interaction::*,
		item_update::{ItemSpawn, SpawnItems},
	},
	materials::{Materials, cell_material},
	item::Items,
};

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	materials: Res<Materials>,
	grid: Res<Grid>,
) {
	let mut items: Vec<ItemSpawn> = Vec::new();
	for (pos, cell) in &grid.cells {
		let [x, z] = axial_to_xz(&pos);
		commands.spawn((
			CellMesh,
			CellPos(*pos),
			Mesh3d(meshes.add(cell_sharp_mesh(&grid, pos, RenderAssetUsages::all()))),
			MeshMaterial3d(cell_material(&materials, &grid, pos).clone()),
			Transform::from_xyz(x, 0., z),
		))
		.observe(handle_click)
		.observe(handle_hover_start)
		.observe(handle_hover_end);

		match cell.item_id {
			Some(item_id) => items.push(ItemSpawn::new(item_id, *pos, cell.height)),
			None => (),
		}
	}
	commands.trigger(SpawnItems(items));
}
