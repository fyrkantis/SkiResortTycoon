use std::collections::HashMap;
use bevy::{
	prelude::*,
	render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}
};
use hexx::{Hex, MeshInfo};

use crate::util::hex::{axial_to_xz, HexCorner};
use crate::game::placement::grid::Grid;

/// Converts hexx MeshInfo into bevy Mesh.
/// From hexx docs example: https://docs.rs/hexx/latest/hexx/index.html#usage-in-bevy
pub fn hexagonal_mesh(mesh_info: MeshInfo) -> Mesh {
	Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::all(), // Needed for mesh to be pickable.
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
	.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
	.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
	.with_inserted_indices(Indices::U16(mesh_info.indices))
}

pub fn mountain_mesh_sharp(grid: &Grid) -> Mesh {
	let mut vertices: Vec<Vec3> = Vec::new();
	for (pos, cell) in grid.cells.iter() {
		let center_y = cell.height as f32;
		let [center_x, center_z] = axial_to_xz(pos);
		let center_cords = Vec3::new(center_x, center_y, center_z);
		let corner_vertices: Vec<Vec3> = HexCorner::get_array().iter().map(|corner| {
			let [edge_1, edge_2] = corner.neighbor_edges();
			// Positions of vertex neighboring cells.
			let (pos_1, pos_2) = (*pos + edge_1.direction(), *pos + edge_2.direction());
			let (cell_1_opt, cell_2_opt) = (grid.cells.get(&pos_1), grid.cells.get(&pos_2));
			let y_avg = match cell_1_opt {
				Some(cell_1) => match cell_2_opt {
					Some(cell_2) => (center_y + cell_1.height as f32 + cell_2.height as f32) / 3.,
					None => (center_y + cell_1.height as f32) / 2.,
				},
				None => match cell_2_opt {
					Some(cell_2) => (center_y + cell_2.height as f32) / 2.,
					None => center_y,
				}
			};
			let [x, z] = corner.to_xz();
			Vec3::new(center_x + x, y_avg, center_z - z)
		}).collect();
		for i in 0..6 {
			vertices.push(center_cords);
			vertices.push(corner_vertices[(i + 1) % 6]);
			vertices.push(corner_vertices[i]);
		}
	}
	
	Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::RENDER_WORLD // CPU doesn't touch this mesh after render.
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
	.with_inserted_indices(Indices::U32((0..grid.cells.keys().count() as u32 * 18).collect()))
	.with_computed_smooth_normals()
}

#[allow(dead_code)] // TODO: Remove this function if still unused.
/// Mesh consisting of soft hexagons.
pub fn mountain_mesh_fuzzy(grid: &Grid) -> Mesh {
	let mut vertices: Vec<Vec3> = Vec::new();
	let mut vertices_count: u16 = 0;
	let mut triangles: Vec<u16> = Vec::new();
	for (pos, cell) in grid.cells.iter() {
		let center_y = cell.height as f32;
		let [center_x, center_z] = axial_to_xz(pos);
		let center_cords = Vec3::new(center_x, center_y, center_z);
		let center_index = vertices_count;
		vertices.push(center_cords);
		HexCorner::get_array().iter().for_each(|corner| {
			let [edge_1, edge_2] = corner.neighbor_edges();
			// Positions of vertex neighboring cells.
			let (pos_1, pos_2) = (*pos + edge_1.direction(), *pos + edge_2.direction());
			let (cell_1_opt, cell_2_opt) = (grid.cells.get(&pos_1), grid.cells.get(&pos_2));
			let y_avg = match cell_1_opt {
				Some(cell_1) => match cell_2_opt {
					Some(cell_2) => (center_y + cell_1.height as f32 + cell_2.height as f32) / 3.,
					None => (center_y + cell_1.height as f32) / 2.,
				},
				None => match cell_2_opt {
					Some(cell_2) => (center_y + cell_2.height as f32) / 2.,
					None => center_y,
				}
			};
			let [x, z] = corner.to_xz();
			vertices.push(Vec3::new(center_x + x, y_avg, center_z - z));
		});
		vertices_count += 7;
		for i in 0..6 {
			triangles.push(center_index);
			triangles.push(center_index + 1 + (1 + i) % 6);
			triangles.push(center_index + 1 + i);
		}
	}
	
	Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::RENDER_WORLD // CPU doesn't touch this mesh after render.
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
	.with_inserted_indices(Indices::U16(triangles))
	.with_computed_smooth_normals()
}

#[allow(dead_code)] // TODO: Remove this function if still unused.
pub fn mountain_mesh_smooth(grid: &Grid) -> Mesh { // BUG: Some vertices are displaced because later cells interpret the wrong vertices as existing.
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
					
				// These and pos are the 3 hex cells surrounding this edge vertex.
				let (cell_1_pos, cell_2_pos) = (corner_pos, center_pos + edge_2.direction());
				let cell_1_y = match grid.cells.get(&cell_1_pos) {Some(cell) => cell.height as f32, None => center_cords.y};
				let cell_2_y = match grid.cells.get(&cell_2_pos) {Some(cell) => cell.height as f32, None => center_cords.y};
				let y = (center_cords.y + cell_1_y + cell_2_y) / 3.;
				
				let [corner_x, corner_z] = corner.to_xz();
				let corner_cords = Vec3::new(center_cords.x + corner_x, y, center_cords.z - corner_z);

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

	Mesh::new(
		PrimitiveTopology::TriangleList,
		RenderAssetUsages::RENDER_WORLD // CPU doesn't touch this mesh after render.
	)
	.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
	.with_inserted_indices(Indices::U16(triangles))
	.with_computed_smooth_normals()
}
