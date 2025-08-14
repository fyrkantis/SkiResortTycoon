use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;

use crate::util::rotation::Rotation;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct ObjectInstanceId(u32);

/// A placed object.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjectInstance {
	Structure(StructureInstance),
	Lift(LiftInstance),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Footprint {
	/// Each cell has a height, and optionally a bottom (under which there is no collision).
	Cells(HashMap<Hex, (u16, Option<u16>)>),
	Mesh,
}

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct LiftTypeId(u32);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiftInstance {
	pub type_id: LiftTypeId,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LiftType {
	pub name: &'static str,
}
#[derive(Resource)]
pub struct LiftTypes(HashMap<LiftTypeId, LiftType>);
