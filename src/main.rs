// Disable console on windows for release builds.
// https://github.com/bevyengine/bevy_github_ci_template/issues/55#issue-2391468497
#![cfg_attr(not(feature = "fast-compile"), windows_subsystem = "windows")]

use bevy::prelude::{App, DefaultPlugins, MeshPickingPlugin, PickingPlugin};
use bevy_egui::EguiPlugin;

mod game;
mod ui;
mod util;

fn main() {
	App::new()
	.add_plugins((
		DefaultPlugins,
		MeshPickingPlugin,
		EguiPlugin {enable_multipass_for_primary_context: true},
		game::GamePlugin,
		ui::UiPlugin,
	))
	.run();
}
