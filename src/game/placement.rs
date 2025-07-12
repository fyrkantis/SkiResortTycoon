use bevy::prelude::*;

pub mod grid;
pub mod cursor;

mod grid_mesh;
mod grid_highlight;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(grid::Grid::new(95, 50, Default::default()));
		app.insert_resource(cursor::Cursor::default());
		app.add_systems(Startup, grid_mesh::setup);
		app.add_systems(Update, (grid_highlight::update, temp));
		app.add_systems(FixedUpdate, grid_mesh::update);
	}
}

fn temp(
	mut gizmos: Gizmos,
	grid: Res<grid::Grid>,
) {
	for (pos, _) in grid.cells.iter() {
		crate::util::hex_gizmo::column(
			&mut gizmos,
			pos,
			&grid,
			Some(Color::srgba(0., 0.8, 0., 0.25)),
			None,
			None,
		);
	}
}
