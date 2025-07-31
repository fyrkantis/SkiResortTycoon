// Disable console on windows for release builds.
// https://github.com/bevyengine/bevy_github_ci_template/issues/55#issue-2391468497
#![cfg_attr(not(feature = "fast-compile"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_mod_outline::OutlinePlugin;
use bevy_egui::EguiPlugin;

mod grid;

fn main() {
	App::new()
	.add_plugins((
		DefaultPlugins,
		OutlinePlugin,
		MeshPickingPlugin,
		EguiPlugin {enable_multipass_for_primary_context: true},
		grid::GridPlugin,
	))
	.insert_resource(MeshPickingSettings {require_markers: true, ..default()})
	.init_resource::<GameState>()
	.run();
}

#[derive(Resource, Default)]
#[allow(dead_code)] // TODO: Remove this when in use.
pub enum GameState {
	#[default]
	MainMenu,
	Loading,
	InGame {
		pub current_player: GameClient,

	},
}

#[allow(dead_code)] // TODO: Remove this when in use.
pub struct GameClient {
	pub player_id: u16,
	pub 
}
