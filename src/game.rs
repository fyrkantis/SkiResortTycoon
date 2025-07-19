use bevy::prelude::*;

mod material;
mod surface;
pub mod item;
mod lift;

mod camera;
mod scene;
pub mod placement; // TODO: Rethink project structure. Which of these should really be public?

pub struct GamePlugin;
impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<material::Materials>();
		app.add_systems(PreStartup, material::setup);
		app.add_systems(PreStartup, item::setup.after(material::setup));
		app.add_systems(PreStartup, lift::setup.after(item::setup));
		app.add_plugins((
			camera::CameraPlugin,
			scene::ScenePlugin,
			placement::PlacementPlugin,
		));
	}
}
