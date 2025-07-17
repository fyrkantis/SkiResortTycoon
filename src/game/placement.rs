use bevy::prelude::*;

pub mod grid;
pub mod cursor;

mod grid_mesh;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(grid::Grid::new(95, 50, Default::default()));
		app.insert_resource(cursor::Cursor::default());
		app.add_systems(Startup, grid_mesh::setup);
	}
}
