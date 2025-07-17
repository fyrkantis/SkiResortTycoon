use bevy::{
	prelude::*,
	ecs::system::SystemId,
};

use crate::util::hex::axial_to_xz;
use crate::game::{
	placement::grid::{Grid, CellPos, CellItem},
	item::{Item, Items},
};

#[derive(Resource, Debug, Clone, Copy)]
pub struct ItemSystems {
	pub update_item_heights: SystemId,
	pub spawn_item: SystemId,
	pub despawn_item: SystemId,
}

pub struct ItemUpdatePlugin;
impl Plugin for ItemUpdatePlugin {
	fn build(&self, app: &mut App) {
		let system_ids = ItemSystems {
			update_item_heights: app.register_system(update_item_heights),
			spawn_item: app.register_system(spawn_item),
			despawn_item: app.register_system(despawn_item),
		};
		app.insert_resource(system_ids);
		
	}
}

/// Goes through all item entities and adjusts the heights to match the terrain.
/// Does NOT spawn/despawn item entities.
fn update_item_heights(
	grid: Res<Grid>,
	mut item_entities: Query<(&CellPos, &mut Transform), With<CellItem>>,
) {
	for (cell_pos, mut transform) in item_entities.iter_mut() {
		let pos = cell_pos.0;
		let cell = match grid.cells.get(&pos) {Some(cell) => cell, None => {error!("Attempted to update item entity at {:?} with pos {:?}, which does not point to a valid cell.", transform, pos); return}};
		transform.translation.y = cell.height as f32;
	}
}

fn spawn_item(
	/*mut commands: Commands,
	item_id: u16,
	pos: Hex,
	height: u16,*/
) {
	/*let item = match items.0.get(&item_id) {Some(item) => item, None => {error!("Can't spawn item with invalid ID {} on cell {:?}.", item_id, pos); return}};
	let [x, z] = axial_to_xz(&pos);

	commands.spawn((
		CellItem(item_id),
		CellPos(pos),
		SceneRoot(item.scene.clone()),
		Transform {
			translation: Vec3::new(x, height as f32, z),
			..default()
		},
	));*/
}

fn despawn_item(
	
) {

}
