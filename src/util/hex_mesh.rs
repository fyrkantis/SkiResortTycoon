use std::collections::HashMap;
use bevy::{
	prelude::*,
	render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}
};
use hexx::{Hex, MeshInfo, HexLayout, ColumnMeshBuilder, HeightMapMeshBuilder};

use crate::util::hex::{axial_to_xz, HexCorner, corner_height};
use crate::game::placement::grid::Grid; // TODO: Replace with trait describing HashMap<Hex, u16>.

/// Converts hexx MeshInfo into bevy Mesh.
/// From hexx docs example: https://docs.rs/hexx/latest/hexx/index.html#usage-in-bevy
fn hexagonal_mesh(mesh_info: MeshInfo, asset_usage: RenderAssetUsages) -> Mesh {
	Mesh::new(PrimitiveTopology::TriangleList, asset_usage)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
	.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
	.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
	.with_inserted_indices(Indices::U16(mesh_info.indices))
}

#[allow(dead_code)] // TODO: Remove this function if still unused.
pub fn cell_column_mesh(height: f32, asset_usage: RenderAssetUsages) -> Mesh {
	hexagonal_mesh(ColumnMeshBuilder::new(
		&HexLayout::flat(),
		height,
	).build(), asset_usage)
}

#[allow(dead_code)] // TODO: Remove this function if still unused.
pub fn mountain_column_mesh(grid: &Grid, asset_usage: RenderAssetUsages) -> Mesh {
	hexagonal_mesh(HeightMapMeshBuilder::new(
		&HexLayout::flat(),
		&grid.cells.iter().map(|(pos, cell)| (*pos, cell.height as f32)).collect::<HashMap<Hex, f32>>(),
	).build(), asset_usage)
}

fn cell_sharp(grid: &Grid, pos: &Hex, world_transform: bool) -> Vec<Vec3> {
	let center_y = grid.cells.get(pos).unwrap().height as f32;
	let [center_x, center_z] = if world_transform {axial_to_xz(pos)} else {[0., 0.]};
	let center_vertex = Vec3::new(center_x, center_y, center_z);
	let corner_vertices: Vec<Vec3> = HexCorner::get_array().iter().map(|corner| {
		let [x, z] = corner.to_xz();
		Vec3::new(center_x + x, corner_height(grid, pos, *corner), center_z + z)
	}).collect();
	let mut vertices: Vec<Vec3> = Vec::new();
	for i in 0..6 { // TODO: Make this more efficient (it runs a lot).
		vertices.push(center_vertex);
		vertices.push(corner_vertices[(i + 1) % 6]);
		vertices.push(corner_vertices[i]);
	}
	vertices
}

pub fn cell_sharp_mesh(grid: &Grid, pos: &Hex, asset_usage: RenderAssetUsages) -> Mesh {
	Mesh::new(PrimitiveTopology::TriangleList, asset_usage)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, cell_sharp(grid, pos, false))
	.with_inserted_indices(Indices::U16((0..18).collect()))
	.with_computed_smooth_normals()
}

#[allow(dead_code)] // TODO: Remove this function if still unused.
pub fn mountain_sharp_mesh(grid: &Grid, asset_usage: RenderAssetUsages) -> Mesh {
	let mut vertices: Vec<Vec3> = Vec::new();
	for (pos, _cell) in grid.cells.iter() {
		vertices.extend(cell_sharp(grid, pos, true));
	}
	
	Mesh::new(PrimitiveTopology::TriangleList, asset_usage)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
	.with_inserted_indices(Indices::U32((0..grid.cells.keys().count() as u32 * 18).collect()))
	.with_computed_smooth_normals()
}

fn cell_fuzzy(grid: &Grid, pos: &Hex, world_transform: bool) -> Vec<Vec3> {
	let [center_x, center_z] = if world_transform {axial_to_xz(pos)} else {[0., 0.]};
	let cell = grid.cells.get(pos).unwrap();
	vec![Vec3::new(center_x, cell.height as f32, center_z)]
	.into_iter().chain(HexCorner::get_array().iter().map(|corner| {
			let [corner_x, corner_z] = corner.to_xz();
			Vec3::new(center_x + corner_x, corner_height(grid, pos, *corner), center_z + corner_z)
		})
	).collect()
}

#[allow(dead_code)] // TODO: Remove this function if still unused.
pub fn cell_fuzzy_mesh(grid: &Grid, pos: &Hex, asset_usage: RenderAssetUsages) -> Mesh {
	Mesh::new(PrimitiveTopology::TriangleList, asset_usage)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, cell_fuzzy(grid, pos, false))
	.with_inserted_indices(Indices::U16(vec![0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 6, 5, 0, 1, 6]))
	.with_computed_smooth_normals()
}

#[allow(dead_code)] // TODO: Remove this function if still unused.
/// Mesh consisting of soft hexagons.
pub fn mountain_fuzzy_mesh(grid: &Grid, asset_usage: RenderAssetUsages) -> Mesh {
	let mut vertices: Vec<Vec3> = Vec::new();
	let mut vertices_count: u16 = 0;
	let mut triangles: Vec<u16> = Vec::new();
	for (pos, cell) in grid.cells.iter() {
		let center_index = vertices_count;
		vertices.extend(cell_fuzzy(grid, pos, true));
		vertices_count += 7;
		for i in 0..6 {
			triangles.push(center_index);
			triangles.push(center_index + 1 + (1 + i) % 6);
			triangles.push(center_index + 1 + i);
		}
	}
	
	Mesh::new(PrimitiveTopology::TriangleList, asset_usage)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
	.with_inserted_indices(Indices::U16(triangles))
	.with_computed_smooth_normals()
}

#[allow(dead_code)] // TODO: Remove this function if still unused.
pub fn mountain_smooth_mesh(grid: &Grid, asset_usage: RenderAssetUsages) -> Mesh { // BUG: Some vertices are displaced because later cells interpret the wrong vertices as existing.
	// Help function that finds the index of the corner vertex with a specific position, or creates one if it doesn't exist.
	let corner_vertex = |
		center_pos: Hex,
		center_cords: Vec3,
		corner: HexCorner,
		grid: &Grid,
		vertices: &mut Vec<Vec3>,
		vertices_count: &mut u16,
		corner_vertex_indices: &mut [HashMap<Hex, u16>; 2]
	| -> u16 {
		let map_index = if corner.is_even() {0} else {1};
		let [edge_1, edge_2] = corner.neighbor_edges();
		let corner_pos = center_pos + edge_1.direction();
		match corner_vertex_indices[map_index].get(&corner_pos) {
			Some(corner_index) => *corner_index, // This edge vertex already exists.
			None => { // This edge vertex doesn't exist yet, needs to be calculated.
				let [corner_x, corner_z] = corner.to_xz();
				let corner_cords = Vec3::new(center_cords.x + corner_x, corner_height(grid, &center_pos, corner), center_cords.z - corner_z);

				let corner_index = *vertices_count;
				vertices.push(corner_cords);
				*vertices_count += 1;
				corner_vertex_indices[map_index].insert(corner_pos, corner_index);
				corner_index
			}
		}
	};

	let mut vertices: Vec<Vec3> = Vec::new();
	let mut vertices_count: u16 = 0;
	// The edge vertices can be mapped on to two hexagonal grids, one for even directions and one for odd.
	let mut corner_vertex_indices: [HashMap<Hex, u16>; 2] = [HashMap::new(), HashMap::new()];
	let mut triangles: Vec<u16> = Vec::new();
	for (pos, cell) in grid.cells.iter() {
		let y = cell.height as f32;
		let [x, z] = axial_to_xz(&pos);
		let vertex_index = vertices_count;
		let cords = Vec3::new(x, y, z);
		vertices.push(cords);
		vertices_count += 1;
		for (corner_i, corner) in HexCorner::get_array().iter().enumerate() {
			let corner_1_index = corner_vertex(*pos, cords, *corner, &grid, &mut vertices, &mut vertices_count, &mut corner_vertex_indices);
			let corner_2_index = corner_vertex(*pos, cords, HexCorner::get_array()[(corner_i + 1) % 6], &grid, &mut vertices, &mut vertices_count, &mut corner_vertex_indices);
			triangles.push(vertex_index);
			triangles.push(corner_2_index);
			triangles.push(corner_1_index);
		}
	}

	Mesh::new(PrimitiveTopology::TriangleList, asset_usage)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
	.with_inserted_indices(Indices::U16(triangles))
	.with_computed_smooth_normals()
}
