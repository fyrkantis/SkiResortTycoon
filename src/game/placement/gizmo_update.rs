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

pub struct GizmoUpdatePlugin;
impl Plugin for GizmoUpdatePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
		app.add_observer(update_hover_gizmo);
		app.add_observer(set_hover_gizmo);
		app.add_observer(remove_hover_gizmo);
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

#[derive(Event, Debug, Clone, Copy)]
pub struct UpdateHoverGizmo;
fn update_hover_gizmo(
	_trigger: Trigger<UpdateHoverGizmo>,
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

#[derive(Event, Debug, Clone, Copy)]
pub struct SetHoverGizmo(pub Hex);
fn set_hover_gizmo(
	trigger: Trigger<SetHoverGizmo>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	grid: Res<Grid>,
	gizmo_entity: Single<(&mut HoverGizmo, &mut Gizmo)>,
) {
	let pos = trigger.0;
	let (mut gizmo_pos, mut gizmo) = gizmo_entity.into_inner();
	gizmo_pos.0 = Some(pos);
	gizmo.handle = gizmo_assets.add(create_hover_gizmo(&grid, pos));
}

#[derive(Event, Debug, Clone, Copy)]
/// Removes the hover gizmo if it is still on input position.
pub struct RemoveHoverGizmo(pub Hex);
fn remove_hover_gizmo(
	trigger: Trigger<RemoveHoverGizmo>,
	mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
	gizmo_entity: Single<(&mut HoverGizmo, &mut Gizmo)>,
) {
	let pos = trigger.0;
	let (mut gizmo_pos, mut gizmo) = gizmo_entity.into_inner();
	if gizmo_pos.0 == Some(pos) {
		gizmo_pos.0 = None;
		gizmo.handle = gizmo_assets.reserve_handle(); // TODO: Find if there's a better way to remove the gizmo.
	}
}
