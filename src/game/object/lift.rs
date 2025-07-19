use bevy::prelude::*;

use crate::game::object::structure::Structure;

pub struct LiftPlugin;
impl Plugin for LiftPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PreStartup, setup.after(crate::game::object::structure::load_assets)); // TODO: Is this needed?
	}
}

pub fn setup() {

}
