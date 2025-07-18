use bevy::prelude::*;

use crate::game::{
	placement::{
		cursor::{Cursor, Tool},
		grid::{Grid, CellPos, CellMesh},
		grid_update::{UpdateMeshes, UpdateMaterials},
		item_update::{UpdateItemHeights, SpawnItem, ItemSpawn, DespawnItem},
		gizmo_update::{UpdateHoverGizmo, SetHoverGizmo, RemoveHoverGizmo},
	},
	surface::Surface,
};

pub fn handle_hover_start(
	trigger: Trigger<Pointer<Over>>,
	mut commands: Commands,
	mut cursor: ResMut<Cursor>,
	grid: Res<Grid>,
	cells: Query<&CellPos, With<CellMesh>>,
) {
	let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse hovered over cell, but it's missing a CellPos position: {}", e); return}};
	let cell = match grid.cells.get(&pos) {Some(cell) => cell, None => {error!("Mouse hovered over cell, but it's CellPos position could not be found in grid."); return}};
	cursor.hover_cell = Some((pos, *cell));

	commands.trigger(SetHoverGizmo(pos));
}

pub fn handle_hover_end(
	trigger: Trigger<Pointer<Out>>,
	mut commands: Commands,
	mut cursor: ResMut<Cursor>,
	cells: Query<&CellPos, With<CellMesh>>,
) {
	let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse hovered over cell, but it's missing a CellPos position: {}", e); return}};
	let current_pos = match cursor.hover_cell {Some((current_pos, _current_cursor)) => current_pos, None => {return}};
	if current_pos == pos {
		cursor.hover_cell = None;
	}
	commands.trigger(RemoveHoverGizmo(pos));
}

pub fn handle_click(
	trigger: Trigger<Pointer<Pressed>>,
	mut commands: Commands,
	cursor: Res<Cursor>,
	mut grid: ResMut<Grid>,
	cells: Query<&CellPos, With<CellMesh>>,
) {
	let pos = match cells.get(trigger.target()) {Ok(pos) => pos.0, Err(e) => {error!("Mouse clicked cell, but it's missing an entity with CellPos and CellMesh components: {}", e); return}};
	let cell = match grid.cells.get_mut(&pos) {Some(cell) => cell, None => {error!("Mouse clicked cell, but it's CellPos position could not be found in grid."); return}};
	
	if cursor.tool == Some(Tool::Terrain) {
		if trigger.button == PointerButton::Primary {
			cell.height += 1;
			commands.trigger(UpdateMeshes);
			commands.trigger(UpdateMaterials);
			commands.trigger(UpdateItemHeights);
			commands.trigger(UpdateHoverGizmo);
			
		} else if trigger.button == PointerButton::Secondary {
			if cell.height <= 0 {
				warn!("Can't lower cell {:?} because it's already at height {}.", pos, cell.height);
			} else {
				cell.height -= 1;
				commands.trigger(UpdateMeshes);
				commands.trigger(UpdateMaterials);
				commands.trigger(UpdateItemHeights);
				commands.trigger(UpdateHoverGizmo);
			}
		}
	} else if cursor.tool == Some(Tool::Surface) {
		if trigger.button == PointerButton::Primary {
			if cell.surface != Surface::Normal {
				warn!("Can't add piste because the surface is not normal.");
			} else {
				cell.surface = Surface::Piste;
				commands.trigger(UpdateMaterials);
			}
		} else if trigger.button == PointerButton::Secondary {
			if cell.surface != Surface::Piste {
				warn!("Can't remove piste because the surface is already not piste.");
			} else {
				cell.surface = Surface::Normal;
				commands.trigger(UpdateMaterials);
			}
		}
		commands.trigger(UpdateMaterials);
	} else if cursor.tool == Some(Tool::Item) {
		if trigger.button == PointerButton::Primary {
			let item_id = match cursor.selected_item_id {Some(id) => id, None => {warn!("Can't place because no item is selected."); return}};
			if cell.item_id != None {warn!("Can't place because cell {:?} is already occupied: {:?}", pos, cell); return}
			cell.item_id = Some(item_id);
			commands.trigger(SpawnItem(ItemSpawn::new(item_id, pos, cell.height)));
		}
	} else if cursor.tool == Some(Tool::Remove) {
		if trigger.button == PointerButton::Primary {
			if cell.item_id != None {warn!("Can't remove item because cell {:?} is already empty", pos); return};
			cell.item_id = None;
			//despawn_item();
		}
	}
}
