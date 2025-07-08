use std::collections::HashMap;
use bevy::{
	prelude::*,
	render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}
};
use hexx::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
	#[default]
	Empty,
	Occupied
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GridCell {
	pub height: u16,
	pub state: CellState,
}
pub const fn empty_cell(height: u16) -> GridCell {GridCell {height: height, state: CellState::Empty}}

#[derive(Resource, Debug, Clone)]
pub struct Grid {
	pub cells: HashMap<Hex, GridCell>,
}
impl Default for Grid {
	fn default() -> Self {Self {
		cells: HashMap::from([
			(hex(0, 0), empty_cell(0)),
			(hex(1, 0), empty_cell(1)),
			(hex(0, 1), empty_cell(2)),
			(hex(1, 1), empty_cell(3)),
			(hex(2, 1), empty_cell(4)),
			(hex(1, 2), empty_cell(5)),
		])
	}}
}

pub struct GridPlugin;
impl Plugin for GridPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Grid::default());
		app.add_systems(Startup, setup);
	}
}

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	grid: Res<Grid>,
) {
	for (pos, cell) in &grid.cells {
		let mesh_info = ColumnMeshBuilder::new(&HexLayout::default(), cell.height as f32).build();
		let [x, z] = crate::util::hex::hex_to_xz(pos);
		
		commands.spawn((
			Mesh3d(meshes.add(hexagonal_mesh(mesh_info))),
			Transform::from_xyz(x, 0., z),
			MeshMaterial3d(materials.add(Color::srgb(pos.x as f32 / 3., pos.y as f32 / 3., pos.z() as f32 / 3.))),
		));
	}
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
