use bevy::prelude::{App, DefaultPlugins};
use bevy_egui::EguiPlugin;

use ski_resort_tycoon::game;

fn main() {
	App::new()
	.add_plugins((
		DefaultPlugins,
		EguiPlugin {enable_multipass_for_primary_context: true},
		game::camera::CameraPlugin,
		game::scene::ScenePlugin,
		game::placement::PlacementPlugin,
	))
	.run();
}
