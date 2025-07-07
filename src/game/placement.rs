use bevy::prelude::{Plugin, App, Startup};

pub mod grid;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(grid::Grid::default());
		app.add_systems(Startup, grid::setup);
	}
}
