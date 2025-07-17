use bevy::{
	prelude::*,
	ecs::system::SystemId,
};
use hexx::Hex;

use crate::util::{
	hex::{axial_to_xz, cell_slope},
	hex_mesh::cell_sharp_mesh,
	hex_gizmo::column_sloped,
};
use crate::game::placement::{grid::Grid, cursor::Cursor};

#[derive(Resource, Debug, Clone, Copy)]
pub struct GizmoSystems {
	pub set_hover_gizmo: SystemId,
	pub remove_hover_gizmo: SystemId,
	pub update_hover_gizmo: SystemId,
}

pub struct GizmoUpdatePlugin;
impl Plugin for GizmoUpdatePlugin {
	fn build(&self, app: &mut App) {
		let system_ids = GizmoSystems {
			set_hover_gizmo: app.register_system(set_hover_gizmo),
			remove_hover_gizmo: app.register_system(remove_hover_gizmo),
			update_hover_gizmo: app.register_system(update_hover_gizmo),
		};
		app.insert_resource(system_ids);
		
	}
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HoverGizmo(Option<Hex>);

pub fn setup(
	mut commands: Commands,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
	commands.spawn((
		HoverGizmo(None),
		Gizmo {
			handle: gizmo_assets.add(GizmoAsset::new()),
			depth_bias: -0.5,
			..default()
		},
	));
}

fn update_hover_gizmo(
	grid: Res<Grid>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	gizmo_query: Query<(&HoverGizmo, Mut<Gizmo>)>,
) {
	/*gizmo.handle = gizmo_assets.add(match gizmo_pos.0 {
		Some(pos) => {
			let mut new_gizmo = GizmoAsset::new();
			column_sloped(
				&mut new_gizmo,
				&pos,
				grid,
				Some(Color::srgb(1., 0., 0.)),
				None,
				Some(Color::srgb(0.8, 0., 1.)),
			);
			new_gizmo
		},
		None => GizmoAsset::new(), // TODO: Find better way to remove gizmo.
	});*/
}

fn set_hover_gizmo(

) {

}

fn remove_hover_gizmo(

) {

}
