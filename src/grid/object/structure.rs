use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;

use crate::util::rotation::Rotation;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct StructureTypeId(u32);
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct StructureInstance {
	pub type_id: StructureTypeId,
	pub position: Hex,
	pub rotation: Option<Rotation>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructureType {
	pub name: &'static str,
	pub scene: Handle<Scene>,
	pub footprint: Footprint,
	pub has_rotation: bool,
	
}
#[derive(Resource)]
pub struct StructureTypes(HashMap<StructureTypeId, StructureType>);

#[derive(Debug, PartialEq, Eq, Clone)]
/// Each cell has a height, and optionally a bottom (under which there is no collision).
pub struct Footprint(HashMap<Hex, (u16, Option<u16>)>);