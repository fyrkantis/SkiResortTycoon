use bevy::prelude::*;

use crate::game::object::{
	structure,
	structure::{StructureInstance, StructureTypes},
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LiftNode {
	Station,
	#[default]
	Pillar,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiftInstance {
	pub nodes: Vec<(LiftNode, StructureInstance)>,
	// TODO
}

pub struct LiftPlugin;
impl Plugin for LiftPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PreStartup, setup.after(structure::load_assets)); // TODO: Is this needed?
	}
}

pub fn setup() {

}

#[derive(Event, Debug, Clone)]
/// Spawns entity for object instance and associated instance IDs.
pub struct SpawnLift(pub u32, pub LiftInstance);
