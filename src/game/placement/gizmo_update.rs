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
	pub update_hover_gizmo: SystemId,
	pub set_hover_gizmo: SystemId<In<Hex>>,
	pub remove_hover_gizmo: SystemId<In<Hex>>,
}

pub struct GizmoUpdatePlugin;
impl Plugin for GizmoUpdatePlugin {
	fn build(&self, app: &mut App) {
		let system_ids = GizmoSystems {
			update_hover_gizmo: app.register_system(update_hover_gizmo),
			set_hover_gizmo: app.register_system(set_hover_gizmo),
			remove_hover_gizmo: app.register_system(remove_hover_gizmo),
		};
		app.insert_resource(system_ids);
		app.add_systems(Startup, setup);
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
			handle: gizmo_assets.reserve_handle(),
			depth_bias: -0.5,
			..default()
		},
	));
}

fn create_hover_gizmo(
	grid: &Grid,
	pos: Hex,
) -> GizmoAsset {
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
}

fn update_hover_gizmo(
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	grid: Res<Grid>,
	gizmo_entity: Single<(&HoverGizmo, &mut Gizmo)>,
) {
	let (gizmo_pos, mut gizmo) = gizmo_entity.into_inner();
	match gizmo_pos.0 {
		Some(pos) => gizmo.handle = gizmo_assets.add(create_hover_gizmo(&grid, pos)),
		None => (),
	}
}

fn set_hover_gizmo(
	In(pos): In<Hex>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	grid: Res<Grid>,
	gizmo_entity: Single<(&mut HoverGizmo, &mut Gizmo)>,
) {
	let (mut gizmo_pos, mut gizmo) = gizmo_entity.into_inner();
	gizmo_pos.0 = Some(pos);
	gizmo.handle = gizmo_assets.add(create_hover_gizmo(&grid, pos));
}

/// Removes the hover gizmo if it is still on input position.
fn remove_hover_gizmo(
	In(pos): In<Hex>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	gizmo_entity: Single<(&mut HoverGizmo, &mut Gizmo)>,
) {
	let (mut gizmo_pos, mut gizmo) = gizmo_entity.into_inner();
	if gizmo_pos.0 == Some(pos) {
		gizmo_pos.0 = None;
		gizmo.handle = gizmo_assets.reserve_handle(); // TODO: Find if there's a better way to remove the gizmo.
	}
}
