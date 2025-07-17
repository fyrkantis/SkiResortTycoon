use bevy::{
	prelude::*,
	render::{render_asset::RenderAssetUsages, primitives::Aabb, mesh::MeshAabb},
	scene::SceneSpawner,
};
use hexx::Hex;

use crate::util::{
	hex::{axial_to_xz, cell_slope},
	hex_mesh::cell_sharp_mesh,
	hex_gizmo::column_sloped,
};
use crate::game::{
	placement::{grid::Grid, cursor::{Cursor, Tool}},
	materials::Materials,
	surface::Surface,
	item::Items,
};

#[derive(Component, Debug)]
/// The visible cell of a mountain.
pub struct CellMesh;

#[derive(Component, Debug)]
/// A placed item.
pub struct CellItem;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// The axial position that corresponds to this mesh/item.
pub struct CellPos(Hex);

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HoverGizmo(Option<Hex>);

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	materials: Res<Materials>,
	grid: Res<Grid>,
) {
	commands.spawn((
		HoverGizmo(None),
		Gizmo {
			handle: gizmo_assets.add(GizmoAsset::new()),
			depth_bias: -0.5,
			..default()
		},
	));
	
	for (pos, _cell) in &grid.cells {
		let [x, z] = axial_to_xz(pos);
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
	}
}

fn handle_hover_start(
	trigger: Trigger<Pointer<Over>>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	grid: Res<Grid>,
	mut cursor: ResMut<Cursor>,
	cells: Query<&CellPos, With<CellMesh>>,
	hover_gizmo: Single<(&mut HoverGizmo, &mut Gizmo)>,
) {
	let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse hovered over cell, but it's missing a CellPos position: {}", e); return}};
	let cell = match grid.cells.get(&pos) {Some(cell) => cell, None => {error!("Mouse hovered over cell, but it's CellPos position could not be found in grid."); return}};
	cursor.hover_cell = Some((pos, *cell));

	let (mut gizmo_pos, gizmo) = hover_gizmo.into_inner();
	gizmo_pos.0 = Some(pos);
	update_hover_gizmo(&grid, gizmo_assets, (&gizmo_pos, gizmo));
}

fn handle_hover_end(
	trigger: Trigger<Pointer<Out>>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	grid: Res<Grid>,
	mut cursor: ResMut<Cursor>,
	cells: Query<&CellPos, With<CellMesh>>,
	hover_gizmo: Single<(&mut HoverGizmo, &mut Gizmo)>,
) {
	let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse hovered over cell, but it's missing a CellPos position: {}", e); return}};
	let current_pos = match cursor.hover_cell {Some((current_pos, _current_cursor)) => current_pos, None => {return}};
	if current_pos == pos {
		cursor.hover_cell = None;
	}
	let (mut gizmo_pos, gizmo) = hover_gizmo.into_inner();
	if gizmo_pos.0 == Some(pos) {
		gizmo_pos.0 = None;
		update_hover_gizmo(&grid, gizmo_assets, (&gizmo_pos, gizmo))
	}
}

fn handle_click(
	trigger: Trigger<Pointer<Pressed>>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	materials: Res<Materials>,
	items: Res<Items>,
	mut grid: ResMut<Grid>,
	cursor: Res<Cursor>,
	mut query_meshes: Query<(&CellPos, &mut Mesh3d, &mut Aabb), With<CellMesh>>,
	mut query_materials: Query<(&CellPos, &mut MeshMaterial3d<StandardMaterial>), With<CellMesh>>,
	cells: Query<&CellPos, With<CellMesh>>,
	item_entities: Query<(&CellPos, &mut Transform), With<CellItem>>,
	hover_gizmo: Single<'_, (&HoverGizmo, &mut Gizmo)>,
) {
	let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse clicked cell, but it's missing a CellPos position: {}", e); return}};
	let cell = match grid.cells.get_mut(&pos) {Some(cell) => cell, None => {error!("Mouse clicked cell, but it's CellPos position could not be found in grid."); return}};

	if cursor.tool == Some(Tool::Terrain) {
		if trigger.button == PointerButton::Primary {
			cell.height += 1;
			update_meshes(&mut meshes, &grid, query_meshes);
			update_materials(&materials, &grid, query_materials);
			update_item_heights(&grid, item_entities);
			update_hover_gizmo(&grid, gizmo_assets, hover_gizmo.into_inner());
		} else if trigger.button == PointerButton::Secondary {
			if cell.height <= 0 {
				warn!("Can't lower cell {:?} because it's already at height {}.", pos, cell.height);
			} else {
				cell.height -= 1;
				update_meshes(&mut meshes, &grid, query_meshes);
				update_materials(&materials, &grid, query_materials);
				update_item_heights(&grid, item_entities);
				update_hover_gizmo(&grid, gizmo_assets, hover_gizmo.into_inner());
			}
		}
	} else if cursor.tool == Some(Tool::Surface) {
		if trigger.button == PointerButton::Primary {
			if cell.surface != Surface::Normal {
				warn!("Can't add piste because the surface is not normal.");
			} else {
				cell.surface = Surface::Piste;
				update_materials(&materials, &grid, query_materials)
			}
		} else if trigger.button == PointerButton::Secondary {
			if cell.surface != Surface::Piste {
				warn!("Can't remove piste because the surface is already not piste.");
			} else {
				cell.surface = Surface::Normal;
				update_materials(&materials, &grid, query_materials);
			}
		}
	} else if cursor.tool == Some(Tool::Item) {
		let item_id = match cursor.selected_item_id {Some(id) => id, None => {warn!("Can't place because no item is selected."); return}};
		let item = match items.0.get(&item_id) {Some(item) => item, None => {error!("Can't place because selected item ID is invalid."); return}};
		if cell.item_id != None {warn!("Can't place because cell {:?} is already occupied: {:?}", pos, cell); return}
		cell.item_id = Some(item_id);
		let [x, z] = axial_to_xz(&pos);

		commands.spawn((
			CellItem,
			CellPos(pos),
			SceneRoot(item.scene.clone()),
			Transform {
				translation: Vec3::new(x, cell.height as f32, z),
				..default()
			},
		));
	}
}

fn update_meshes(
	meshes: &mut ResMut<Assets<Mesh>>,
	grid: &Grid,
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
	materials: &Materials,
	grid: &Grid,
	mut query: Query<(&CellPos, &mut MeshMaterial3d<StandardMaterial>), With<CellMesh>>,
) {
	for (cell, mut material) in query.iter_mut() {
		let pos = cell.0;
		let new_material = cell_material(&materials, grid, &pos);
		if material.0.id() != new_material.id() {
			*material = MeshMaterial3d(new_material.clone());
		}
	}
}

pub const SNOW_MAX_SLOPE: u16 = 3;
pub const DIRT_MAX_SLOPE: u16 = 4;

fn cell_material<'a>(materials: &'a Materials, grid: &Grid, pos: &Hex) -> &'a Handle<StandardMaterial> {
	let cell = grid.cells.get(pos).unwrap();
	match cell.surface {
		Surface::Piste => &materials.piste,
		Surface::Water => &materials.water,
		Surface::Normal => {
			let slope = cell_slope(grid, pos);
			if slope > DIRT_MAX_SLOPE {return &materials.rock}
			else if slope > SNOW_MAX_SLOPE {return &materials.dirt}
			else {return &materials.snow}
		}
	}
}

fn update_item_heights(
	grid: &Grid,
	mut item_entities: Query<(&CellPos, &mut Transform), With<CellItem>>,
) {
	for (cell_pos, mut transform) in item_entities.iter_mut() {
		let pos = cell_pos.0;
		let cell = match grid.cells.get(&pos) {Some(cell) => cell, None => {error!("Attempted to update item entity at {:?} with pos {:?}, which does not point to a valid cell.", transform, pos); return}};
		transform.translation.y = cell.height as f32;
	}
}

fn update_hover_gizmo(
	grid: &Grid,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	(gizmo_pos, mut gizmo): (&HoverGizmo, Mut<Gizmo>),
) {
	gizmo.handle = gizmo_assets.add(match gizmo_pos.0 {
		Some(pos) => {
			let mut new_gizmo = GizmoAsset::new();
			column_sloped(
				&mut new_gizmo,
				&pos,
				grid,
				Some(Color::srgb(1., 0., 0.)),
				None,
				Some(Color::srgb(0.8, 0., 1.)),
			);
			new_gizmo
		},
		None => GizmoAsset::new(), // TODO: Find better way to remove gizmo.
	});
}
