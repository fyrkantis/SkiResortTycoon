use bevy::prelude::{Plugin, App, Startup, Update};

mod grid;
mod cursor;
mod grid_select;
mod grid_render;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(grid::Grid::new(95, 50, Default::default()));
		app.insert_resource(cursor::Cursor::default());
		app.add_systems(Startup, grid_render::setup);
		app.add_systems(Update, grid_select::cell_highlight_system);
	}
}
