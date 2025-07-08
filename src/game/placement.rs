use bevy::prelude::{Plugin, App};

pub mod grid;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(grid::GridPlugin);
	}
}
