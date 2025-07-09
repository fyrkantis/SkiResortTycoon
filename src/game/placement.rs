use bevy::prelude::{Plugin, App};

pub mod grid;
pub mod cursor;
pub mod grid_select;
pub mod grid_render;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((grid::GridPlugin, cursor::CursorPlugin, grid_render::GridRenderPlugin, grid_select::GridSelectPlugin));
	}
}
