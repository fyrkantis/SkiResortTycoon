use std::collections::HashMap;
use bevy::{
	prelude::*,
	render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}
};
use hexx::*;

#[derive(Resource)]
pub struct Grid {
	pub hex_map: HashMap<Hex, f32>,
}
impl Default for Grid {
	fn default() -> Self {Self {
		hex_map: HashMap::from([
			(hex(0, 0), 0.),
			(hex(1, 0), 1.),
			(hex(0, 1), 2.),
			(hex(1, 1), 3.),
			(hex(2, 1), 4.),
			(hex(1, 2), 5.),
		])
	}}
}

/// Converts hexx MeshInfo into bevy Mesh.
/// From hexx docs example: https://docs.rs/hexx/latest/hexx/index.html#usage-in-bevy
pub fn hexagonal_mesh(mesh_info: MeshInfo) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD, // Won't interact with the mesh on the CPU afterwards
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	grid: Res<Grid>,
) {
	let mesh_info = HeightMapMeshBuilder::new(&HexLayout::default(), &grid.hex_map).build();
	
	commands.spawn((
		Mesh3d(meshes.add(hexagonal_mesh(mesh_info))),
		MeshMaterial3d(materials.add(Color::WHITE)),
	));
}
