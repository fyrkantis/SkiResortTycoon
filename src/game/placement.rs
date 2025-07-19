use bevy::prelude::*;

pub mod grid;
pub mod cursor;

mod grid_entity;
mod gizmo_entity;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(grid::Grid::new(95, 50, Default::default()));
		app.insert_resource(cursor::Cursor::default());
		app.add_plugins((
			grid_entity::GridEntityPlugin,
			gizmo_entity::GizmoEntityPlugin,
		));
	}
}
