use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;

mod worldgen;

pub struct GridPlugin;
impl Plugin for GridPlugin {
	fn build(&self, app: &mut App) {
		let test = ObjectInstance::Structure
	}
}

#[derive(Resource)]
pub struct CellHeights(HashMap<Hex, u16>);

#[derive(Resource)]
pub struct CellSurfaces(HashMap<Hex, Surface>);

#[derive(Resource)]
pub struct PlacedObjects(HashMap<u32, ObjectInstance>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjetInstance {
	Structure(StrctureInstance),
	Lift(LiftInstance),
}
impl Placeable for ObjectInstance {
	fn spawn() {
		
	}
}

pub trait Placeable {
	fn spawn() {}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructureInstance {
	pub name: &'static str,
	
}

impl Placeable for StructureInstance {
	fn spawn() {
		println!("Hello World!");
	}
}