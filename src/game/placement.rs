use bevy::prelude::*;

pub mod grid;
pub mod cursor;

mod grid_select;
mod grid_render;
mod grid_highlight;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(grid::Grid::new(95, 50, Default::default()));
		app.insert_resource(cursor::Cursor::default());
		app.add_systems(Startup, (grid_render::setup, grid_select::setup, temp));
		app.add_systems(Update, grid_highlight::update);
		app.add_systems(FixedUpdate, grid_render::update);
	}
}

fn temp(
	mut commands: Commands,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	grid: Res<grid::Grid>,
) {
	let mut gizmo = GizmoAsset::new();
	for (pos, cell) in grid.cells.iter() {
		crate::util::hex_gizmo::column_level(
			&mut gizmo,
			pos,
			cell.height as f32,
			Some(Color::srgb(0., 1., 0.)),
			None,
		);
	}
	commands.spawn(Gizmo {handle: gizmo_assets.add(gizmo), ..default()});
}
