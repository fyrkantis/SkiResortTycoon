use bevy::{prelude::*, render::render_asset::RenderAssetUsages};

use crate::util::hex_mesh::mountain_mesh_sharp;
use crate::game::placement::grid::Grid;

#[derive(Component)]
pub struct MountainMesh;

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	grid: Res<Grid>,
) {
	commands.spawn((
		MountainMesh,
		Mesh3d(meshes.add(mountain_mesh_sharp(&grid, RenderAssetUsages::all()))),
		MeshMaterial3d(materials.add(Color::WHITE)),
		Pickable {should_block_lower: false, is_hoverable: false},
	));
}

pub fn update(
	mut meshes: ResMut<Assets<Mesh>>,
	grid: Res<Grid>,
	mountain_mesh: Single<&Mesh3d, With<MountainMesh>>,
) {
	let mesh = match meshes.get_mut(&mountain_mesh.0) {Some(mesh) => mesh, None => {error_once!("Failed to update mountain mesh, since it could not be found in asset."); return}};
	*mesh = mountain_mesh_sharp(&grid, RenderAssetUsages::all());
}

