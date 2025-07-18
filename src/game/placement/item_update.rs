use bevy::{
	prelude::*,
	ecs::system::SystemId,
};
use hexx::Hex;

use crate::util::hex::axial_to_xz;
use crate::game::{
	placement::grid::{Grid, CellPos, CellItem},
	item::{Item, Items},
};

pub struct ItemUpdatePlugin;
impl Plugin for ItemUpdatePlugin {
	fn build(&self, app: &mut App) {
		app.add_observer(update_item_heights);
		app.add_observer(spawn_item);
		app.add_observer(despawn_item);
	}
}

/// Goes through all item entities and adjusts the heights to match the terrain.
/// Does NOT spawn/despawn item entities.
#[derive(Event, Debug, Clone, Copy)]
pub struct UpdateItemHeights;
fn update_item_heights(
	_trigger: Trigger<UpdateItemHeights>,
	grid: Res<Grid>,
	mut item_entities: Query<(&CellPos, &mut Transform), With<CellItem>>,
) {
	for (cell_pos, mut transform) in item_entities.iter_mut() {
		let pos = cell_pos.0;
		let cell = match grid.cells.get(&pos) {Some(cell) => cell, None => {error!("Attempted to update item entity at {:?} with pos {:?}, which does not point to a valid cell.", transform, pos); return}};
		transform.translation.y = cell.height as f32;
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// A utility struct used to 
pub struct ItemSpawn { // TODO: Rename this.
	pub item_id: u16,
	pub cell_pos: Hex,
	pub cell_height: u16,
}
impl ItemSpawn {
	pub fn new(item_id: u16, cell_pos: Hex, cell_height: u16) -> Self {Self {
		item_id: item_id, cell_pos: cell_pos, cell_height: cell_height,
	}}
}

/// This is not a system.
fn spawn_item_entity(
	commands: &mut Commands,
	items: &Items,
	spawn: ItemSpawn,
) {
	let [x, z] = axial_to_xz(&spawn.cell_pos);
	match items.0.get(&spawn.item_id) {
		Some(item) => {
			commands.spawn((
				CellItem(spawn.item_id),
				CellPos(spawn.cell_pos),
				SceneRoot(item.scene.clone()),
				Transform {
					translation: Vec3::new(x, spawn.cell_height as f32, z),
					..default()
				},
			));
		},
		None => error!("Failed to spawn item with unknown item ID {} on cell {:?}", spawn.item_id, spawn.cell_pos),
	}
}

#[derive(Event, Debug, Clone, Copy)]
pub struct SpawnItem(pub ItemSpawn);
fn spawn_item(
	trigger: Trigger<SpawnItem>,
	mut commands: Commands,
	items: Res<Items>,
) {
	spawn_item_entity(&mut commands, &items, trigger.0);
}

#[derive(Event, Debug, Clone)]
pub struct SpawnItems(pub Vec<ItemSpawn>);
fn spawn_items(
	trigger: Trigger<SpawnItems>,
	mut commands: Commands,
	items: Res<Items>,
) {
	trigger.0.iter().for_each(|spawn| {
		spawn_item_entity(&mut commands, &items, *spawn);
	});
}

#[derive(Event, Debug, Clone, Copy)]
pub struct DespawnItem;
fn despawn_item(
	trigger: Trigger<DespawnItem>,
	//item_entities: Query<Entity, With<CellItem>>,
) {
	//let items = trigger.0;

}
