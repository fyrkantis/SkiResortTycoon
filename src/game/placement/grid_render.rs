use bevy::prelude::*;

use crate::util::hex_mesh::mountain_mesh_sharp;
use crate::game::placement::grid::Grid;

#[derive(Resource, Debug, Clone)]
pub struct MountainMesh(Handle<Mesh>);

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	grid: Res<Grid>,
) {
	let mountain_mesh_handle = meshes.add(mountain_mesh_sharp(&grid));
	commands.insert_resource(MountainMesh(mountain_mesh_handle.clone()));
	commands.spawn((
		Mesh3d(mountain_mesh_handle.clone()),
		MeshMaterial3d(materials.add(Color::WHITE)),
	));
}

pub fn update(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	grid: Res<Grid>,
	mountain_mesh_handle: Res<MountainMesh>,
) {
	info_once!("{:?}", meshes.add(mountain_mesh_sharp(&grid)));
	info_once!("{:?}", mountain_mesh_handle.0);
	match meshes.get_mut(&mountain_mesh_handle.0) {
		Some(mountain_mesh) => warn!("Yay!"),
		None => error_once!("Damn."),
	}
}

