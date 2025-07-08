use bevy::{
	prelude::*,
	input::mouse::{MouseButton, AccumulatedMouseMotion},
	window::{CursorGrabMode, PrimaryWindow}
};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, Resource)]
pub struct CameraSettings {
	/// Vertical field of view in degrees.
	pub fov: f32,
	pub movement_speed: f32,
	pub sensitivity: f32
}
impl Default for CameraSettings {
	fn default() -> Self {
		Self {
			fov: 70.,
			movement_speed: 40.,
			sensitivity: 0.001
		}
	}
}

/// Adds a camera that moves around on user input.
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(CameraSettings::default());
		app.add_systems(Startup, setup);
		app.add_systems(Update, (movement_system, rotation_system, fov_system));
	}
}

pub fn setup(
	mut commands: Commands
) {
	commands.spawn((
		Camera3d::default(),
		Projection::from(PerspectiveProjection::default()),
		Transform {
			translation: Vec3::new(-15., 30., -15.),
			rotation: Quat::from_euler(EulerRot::YXZ, 1.25 * PI, PI / -8., 0.),
			..default()
		}
	));
}

/// Sets Field of view according to settings.
fn fov_system(
	projection: Single<&mut Projection, With<Camera3d>>,
	settings: Res<CameraSettings>
) {
	match *projection.into_inner() {
		Projection::Perspective(ref mut perspective) => {
			perspective.fov = settings.fov * PI / 180.;
		},
		_ => error_once!("Attempted to change FOV, but camera does not use Projection::Perspective.")
	}
}

/// WASD (+QE) moves the camera around.
fn movement_system(
	input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
	settings: Res<CameraSettings>,
	mut camera: Single<&mut Transform, With<Camera3d>>
) {
	let mut vec = Vec3::ZERO;
	if input.pressed(KeyCode::KeyW) {vec += camera.forward().as_vec3()}
	if input.pressed(KeyCode::KeyA) {vec += camera.left().as_vec3()}
	if input.pressed(KeyCode::KeyS) {vec += camera.back().as_vec3()}
	if input.pressed(KeyCode::KeyD) {vec += camera.right().as_vec3()}
	if input.pressed(KeyCode::KeyE) {vec += camera.up().as_vec3()}
	if input.pressed(KeyCode::KeyQ) {vec += camera.down().as_vec3()}

	match vec.try_normalize() {
		Some(vec_norm) => camera.translation += vec_norm * time.delta_secs() * settings.movement_speed,
		None => {}
	}
}

/// Right click hides and locks the cursor, and rotates the camera with mouse movement.
fn rotation_system(
	settings: Res<CameraSettings>,
	mut camera: Single<&mut Transform, With<Camera3d>>,
	mouse_button: Res<ButtonInput<MouseButton>>,
	mouse_motion: Res<AccumulatedMouseMotion>,
	mut window: Single<&mut Window, With<PrimaryWindow>>
) {
	if mouse_button.just_pressed(MouseButton::Right) {
		window.cursor_options.grab_mode = CursorGrabMode::Locked;
		window.cursor_options.visible = false;
	}
	if mouse_button.just_released(MouseButton::Right) {
		window.cursor_options.grab_mode = CursorGrabMode::None;
		window.cursor_options.visible = true;
	}

	if mouse_button.pressed(MouseButton::Right) &&  mouse_motion.delta != Vec2::ZERO {
		let delta = -mouse_motion.delta * settings.sensitivity;
		let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

		// If the window is more flat rather than tall, the vertical mouse sensitivity is decreased.
		let aspect_ratio = window.width() / window.height(); // TODO: Find a way to avoid this calculation on every single frame.

		/// A pitch of +- pi/2 radians means looking straight up or down, which causes issues. This value clamps it 0.01 to radians before that.
		const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
		camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw + delta.x, (pitch + delta.y / aspect_ratio).clamp(-PITCH_LIMIT, PITCH_LIMIT), roll)
	}
}
