use bevy::{
	prelude::*,
	ecs::system::SystemId,
	render::{render_asset::RenderAssetUsages, primitives::Aabb, mesh::MeshAabb},
};

use crate::util::hex_mesh::cell_sharp_mesh;
use crate::game::{
	placement::grid::{Grid, CellPos, CellMesh},
	materials::{Materials, cell_material},
};

#[derive(Resource, Debug, Clone, Copy)]
pub struct GridSystems {
	pub update_meshes: SystemId,
	pub update_materials: SystemId,
}

pub struct GridUpdatePlugin;
impl Plugin for GridUpdatePlugin {
	fn build(&self, app: &mut App) {
		let system_ids = GridSystems {
			update_meshes: app.register_system(update_meshes),
			update_materials: app.register_system(update_materials),
		};
		app.insert_resource(system_ids);
		
	}
}

fn update_meshes(
	mut meshes: ResMut<Assets<Mesh>>,
	grid: Res<Grid>,
	mut query: Query<(&CellPos, &mut Mesh3d, &mut Aabb), With<CellMesh>>,
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

fn update_materials(
	materials: Res<Materials>,
	grid: Res<Grid>,
	mut query: Query<(&CellPos, &mut MeshMaterial3d<StandardMaterial>), With<CellMesh>>,
) {
	for (cell, mut material) in query.iter_mut() {
		let pos = cell.0;
		let new_material = cell_material(&materials, &grid, &pos);
		if material.0.id() != new_material.id() {
			*material = MeshMaterial3d(new_material.clone());
		}
	}
}

