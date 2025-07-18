use bevy::prelude::*;

pub mod camera;
pub mod scene;
pub mod placement;
pub mod surface;
pub mod item;

mod materials;

pub struct GamePlugin;
impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<materials::Materials>();
		app.add_systems(PreStartup, materials::setup);
		app.add_systems(PreStartup, item::setup.after(materials::setup));
		app.add_plugins((
			camera::CameraPlugin,
			scene::ScenePlugin,
			placement::PlacementPlugin,
		));
	}
}
