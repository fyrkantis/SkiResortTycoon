use bevy::prelude::*;
use bevy_egui::*;

mod build_ui;

pub struct UiPlugin;
impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(EguiContextPass, build_ui::ui_system);
	}
}
