use bevy::prelude::{Plugin, App, Startup};

mod grid;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, grid::setup_grid);
	}
}
