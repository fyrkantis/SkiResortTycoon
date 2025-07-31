use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;

mod worldgen;

pub struct GridPlugin;
impl Plugin for GridPlugin {
	fn build(&self, app: &mut App) {
		
	}
}

#[derive(Resource)]
pub struct CellHeights(HashMap<Hex, u16>);

#[derive(Resource)]
pub struct CellSurfaces(HashMap<Hex, Surface>);

#[derive(Resource)]
pub struct PlacedObjects(HashMap<u32, ObjectInstance>);
