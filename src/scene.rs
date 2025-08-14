use std::f32::consts::PI;
use bevy::prelude::*;

pub fn setup(
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
		DirectionalLight {
			illuminance: light_consts::lux::FULL_DAYLIGHT,
			shadows_enabled: true,
			..default()
		},
		Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, -0.75 * PI, PI / -8., 0.))
	));
}
