use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;

use crate::util::hex::axial_to_xz;
use crate::game::{
	placement::grid::{Grid, CellPos},
	material,
	object::{structure::Structures, ObjectInstance, ObjectType},
};

pub struct ObjectEntityPlugin;
impl Plugin for ObjectEntityPlugin {
	fn build(&self, app: &mut App) {
		app.add_observer(spawn_object);
		app.add_observer(spawn_objects);
		app.add_observer(despawn_object);
		app.add_observer(despawn_objects);
	}
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// The associated instance ID of this entity.
pub struct ObjectEntity(pub u32);


#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// The type ID for this structure.
/// All entities with a StructureEntity component will also have an ObjectEntity component for the instance ID.
pub struct StructureEntity(pub u16);

/// This is not a system.
fn spawn_structure_entity(
	commands: &mut Commands,
	structures: &Structures,
	instance_id: u32,
	structure_id: u16,
	pos: Hex,
	height: u16,
) {
	let [x, z] = axial_to_xz(&pos);
	match structures.0.get(&structure_id) {
		Some(structure) => {
			commands.spawn((
				CellPos(pos),
				ObjectEntity(instance_id),
				StructureEntity(structure_id),
				SceneRoot(structure.scene.clone()),
				Transform {
					translation: Vec3::new(x, height as f32, z),
					..default()
				},
			));
		},
		None => error!("Failed to spawn structure with unknown structure type ID {} on cell {:?}", structure_id, pos),
	}
}

fn spawn_object_entity(

) {

}

#[derive(Event, Debug, Clone, Copy)]
/// Spawns entity for object instance and associated instance IDs.
pub struct SpawnObject(pub u32, pub ObjectInstance);
fn spawn_object(
	trigger: Trigger<SpawnObject>,
	mut commands: Commands,
	structures: Res<Structures>,
) {
	//spawn_structure_entity(&mut commands, &structures, );
}

#[derive(Event, Debug, Clone)]
/// Spawns entities for multiple object instances and their associated instance IDs.
pub struct SpawnObjects(pub HashMap<u32, ObjectInstance>);
fn spawn_objects(
	trigger: Trigger<SpawnObjects>,
	mut commands: Commands,
	structures: Res<Structures>,
) {
	//trigger.0.iter().for_each(|spawn| {
	//	spawn_structure_entity(&mut commands, &structures, *spawn);
	//});
}

#[derive(Event, Debug, Clone, Copy)]
/// Despawn entity with specified instance ID.
pub struct DespawnObject(pub u32);
fn despawn_object(
	trigger: Trigger<DespawnObject>,
	mut commands: Commands,
	entities: Query<(&CellPos, Entity), With<StructureEntity>>,
) {
	//for (structure_pos, structure_entity) in structure_entities.iter() {
	//	if structure_pos.0 == trigger.0 {
	//		commands.entity(structure_entity).despawn();
	//	}
	//}
}

#[derive(Event, Debug, Clone)]
/// Despawn entities with specified instance IDs.
pub struct DespawnObjects(pub Vec<u32>);
fn despawn_objects(
	trigger: Trigger<DespawnObjects>,
	mut commands: Commands,
	entities: Query<(&CellPos, Entity), With<StructureEntity>>,
) {
	//for (structure_pos, structure_entity) in structure_entities.iter() {
	//	if structure_pos.0 == trigger.0 {
	//		commands.entity(structure_entity).despawn();
	//	}
	//}
}

/// Goes through all structure entities and adjusts the heights to match the terrain.
/// Does NOT spawn/despawn structure entities.
#[derive(Event, Debug, Clone, Copy)]
pub struct UpdateStructureHeights;
fn update_structure_heights( // TODO: Improve this method for structures spanning many cells.
	_trigger: Trigger<UpdateStructureHeights>,
	grid: Res<Grid>,
	mut structure_entities: Query<(&CellPos, &mut Transform), With<StructureEntity>>,
) {
	for (cell_pos, mut transform) in structure_entities.iter_mut() {
		let pos = cell_pos.0;
		let height = match grid.heights.get(&pos) {Some(height) => height, None => {error!("Attempted to update structure entity at {:?} with pos {:?}, which does not point to a valid cell.", transform, pos); return}};
		transform.translation.y = *height as f32;
	}
}
