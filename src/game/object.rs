use bevy::prelude::*;

pub mod structure;
pub mod lift;

use structure::*;
use lift::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ObjectType {
	/// Structure type ID.
	Structure(u16),
	Lift,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjectInstance {
	Structure(StructureInstance),
	Lift(LiftInstance),
}

pub struct ObjectPlugin;
impl Plugin for ObjectPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			LiftPlugin,
			StructurePlugin,
		));
	}
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// The associated instance ID of this entity.
pub struct ObjectEntity(pub u32);
