use bevy::prelude::*;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
	}
}

fn setup(
	mut commands: Commands,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
	let mut axes_gizmo = GizmoAsset::new();
	axes_gizmo.axes(Transform::default(), 5.);
	commands.spawn(Gizmo {
		handle: gizmo_assets.add(axes_gizmo),
		..default()
	});
	
	commands.spawn((
		PointLight {
			shadows_enabled: true,
			..default()
		},
		Transform::from_xyz(4.0, 8.0, 4.0),
	));
}
