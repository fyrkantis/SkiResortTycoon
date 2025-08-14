use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;

mod surface;
mod object;
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
pub struct CellSurfaces(HashMap<Hex, surface::Surface>);

#[derive(Resource)]
pub struct PlacedObjects(HashMap<object::ObjectInstanceId, object::ObjectInstance>);
