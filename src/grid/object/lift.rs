use std::collections::HashMap;
use bevy::prelude::*;

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