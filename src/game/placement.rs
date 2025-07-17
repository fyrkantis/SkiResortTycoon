use bevy::prelude::*;

pub mod grid;
pub mod cursor;

mod grid_update;
mod item_update;
mod gizmo_update;
mod grid_interaction;
mod grid_setup;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(grid::Grid::new(95, 50, Default::default()));
		app.insert_resource(cursor::Cursor::default());
		app.add_plugins((
			grid_update::GridUpdatePlugin,
			item_update::ItemUpdatePlugin,
			gizmo_update::GizmoUpdatePlugin,
		));
		app.add_systems(Startup, grid_setup::setup);
	}
}
