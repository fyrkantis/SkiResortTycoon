use bevy::prelude::*;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
	}
}

fn setup(
	mut commands: Commands
) {
	commands.spawn((
		PointLight {
			shadows_enabled: true,
			..default()
		},
		Transform::from_xyz(4.0, 8.0, 4.0),
	));
}
