// Disable console on windows for release builds.
// https://github.com/bevyengine/bevy_github_ci_template/issues/55#issue-2391468497
#![cfg_attr(not(feature = "fast-compile"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_mod_outline::OutlinePlugin;
use bevy_egui::EguiPlugin;

mod grid;
mod scene;
mod camera;

fn main() {
	App::new()
	.insert_resource(MeshPickingSettings {require_markers: true, ..default()})
	.add_plugins((
		DefaultPlugins,
		OutlinePlugin,
		MeshPickingPlugin,
		EguiPlugin {enable_multipass_for_primary_context: true},
		grid::GridPlugin,
		camera::CameraPlugin,
	))
	.add_systems(Startup, scene::setup)
	.run();
}
