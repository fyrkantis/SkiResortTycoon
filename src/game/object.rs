use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;

use crate::game::material;

pub mod structure;
pub mod lift;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ObjectType {
	/// Structure with ID.
	Structure(u16),
	Lift,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ObjectInstance {
	object_type: ObjectType,
	pos: Hex,
	// TODO: Add rotation and height offset.
}
impl ObjectInstance {
	pub fn new(object_type: ObjectType, pos: Hex) -> Self {Self {
		object_type: object_type,
		pos: pos,
	}}
	pub fn new_structure(structure_id: u16, pos: Hex) -> Self {Self {
		object_type: ObjectType::Structure(structure_id),
		pos: pos,
	}}
}

pub struct ObjectPlugin;
impl Plugin for ObjectPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			lift::LiftPlugin,
		));

		app.add_systems(PreStartup, structure::load_assets.after(material::load_assets));
	}
}
