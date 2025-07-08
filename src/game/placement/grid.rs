use std::collections::HashMap;
use bevy::{
	prelude::*,
	render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}
};
use hexx::*;
use noise::{Perlin, NoiseFn};

use crate::util::hex::{axial_to_xz, offset_to_axial};

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WorldGenSettings {
	/// 1-50
	pub peak_height: f64,
	/// 1-50
	pub peak_width: f64,
	/// 1-50
	pub slope_height: f64, // TODO: Add more parameters.
}
impl Default for WorldGenSettings {
	fn default() -> Self {Self {
		peak_height: 10., peak_width: 30., slope_height: 40.
	}}
}

#[derive(Resource, Debug, Clone)]
pub struct Grid {
	pub cells: HashMap<Hex, GridCell>,
	pub width: u16,
	pub length: u16,
	pub settings: WorldGenSettings
}
impl Grid {
	pub fn new(width: u16, length: u16, settings: WorldGenSettings) -> Self {
		let mut cells: HashMap<Hex, GridCell> = HashMap::new();
		let perlin = Perlin::new(0);
		let max_z = length as f64 * f64::sqrt(3.); // TODO: Use fancy new std::f32::consts::SQRT_3 when available. https://github.com/rust-lang/rust/issues/103883
		for col in 0..width as i32 {
			for row in 0..length as i32 {
				let pos_axial = offset_to_axial(col, row);
				
				let [x, z] = axial_to_xz(&pos_axial); // NOTE: xz are pixel coordinates, not hexagonal.
				let height = perlin.get([x as f64 / settings.peak_width, z as f64 / settings.peak_width])
				* settings.peak_height + (z as f64 / max_z) * settings.slope_height;

				cells.insert(pos_axial, empty_cell(height as u16));
			}
		}
		Grid {
			cells: cells,
			width: width,
			length: length,
			settings: settings,
		}
	}
}

pub struct GridPlugin;
impl Plugin for GridPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Grid::new(75, 50, WorldGenSettings::default()));
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
		let mesh_info = ColumnMeshBuilder::new(&HexLayout::flat(), cell.height as f32).build();
		let [x, z] = axial_to_xz(pos);
		
		commands.spawn((
			Mesh3d(meshes.add(hexagonal_mesh(mesh_info))),
			Transform::from_xyz(x, 0., z),
			MeshMaterial3d(materials.add(Color::hsv((360. * cell.height as f64 / (grid.settings.peak_height + grid.settings.slope_height)) as f32, 1., 0.8))),
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
