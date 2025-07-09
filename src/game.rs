use bevy::prelude::*;

pub mod camera;
pub mod scene;
pub mod placement;
pub mod surface;
pub mod item;

pub struct GamePlugin;
impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PreStartup, item::load_items_system);
		app.add_plugins((
			camera::CameraPlugin,
			scene::ScenePlugin,
			placement::PlacementPlugin,
		));
	}
}
