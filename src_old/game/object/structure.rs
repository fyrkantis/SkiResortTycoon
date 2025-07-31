use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

use bevy::prelude::*;
use bevy_mod_outline::{OutlineVolume, OutlineMode, AsyncSceneInheritOutline};
use hexx::Hex;

use crate::util::hex::axial_to_xz;
use crate::game::{
	placement::{
		grid::{Grid, CellPos},
		cursor::{Cursor, Tool, HoverObjects},
	},
	material,
	material::Materials,
	object::ObjectEntity,
	events::{UpdateHoverOutline, UpdateHoverGizmo},
};

mod temp_scenes;
use temp_scenes::*;

#[derive(Debug, Clone)]
pub struct StructureType {
	pub id: u16,
	pub name: &'static str,
	pub scene: Handle<Scene>,
}
impl StructureType {
	pub fn new(id: u16, name: &'static str, scene: Handle<Scene>) -> (u16, Self) {
		(id, Self {id: id, name: name, scene: scene})
	}
}
impl PartialEq for StructureType {fn eq(&self, other: &Self) -> bool {self.id == other.id}}
impl Eq for StructureType {}
impl Ord for StructureType {fn cmp(&self, other: &Self) -> Ordering {self.id.cmp(&other.id)}}
impl PartialOrd for StructureType {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Hash for StructureType {fn hash<H: Hasher>(&self, state: &mut H) {self.id.hash(state)}}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StructureInstance {
	pub structure_id: u16,
	pub pos: Hex, // TODO: Add rotation and height offset.
}
impl StructureInstance {
	pub fn new(structure_id: u16, pos: Hex) -> Self {
		Self {structure_id: structure_id, pos: pos}
	}
}

pub struct StructurePlugin;
impl Plugin for StructurePlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PreStartup, load_assets.after(material::load_assets));

		app.add_observer(spawn_structure);
		app.add_observer(update_structure_outlines);
		app.add_observer(despawn_structure);
		app.add_observer(update_structure_heights);
	}
}

#[derive(Resource, Debug, Clone)]
pub struct StructureTypes(pub HashMap<u16, StructureType>);

/// Loads all structure scenes.
pub fn load_assets(
	mut commands: Commands,
	mut scene_assets: ResMut<Assets<Scene>>,
	mut mesh_assets: ResMut<Assets<Mesh>>,
	mut material_assets: ResMut<Assets<StandardMaterial>>,
	materials: Res<Materials>,
) {
	commands.insert_resource(StructureTypes(HashMap::from([
		StructureType::new(1, "Tree", scene_assets.add(tree_scene(&mut mesh_assets, &mut material_assets))),
		StructureType::new(111, "Red Box", scene_assets.add(red_box_scene(&mut mesh_assets, &materials))),
		StructureType::new(222, "Blue Sphere", scene_assets.add(blue_sphere_scene(&mut mesh_assets, &materials))),
	])));
}

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// The type ID for this structure.
/// All entities with a StructureEntity component will also have an ObjectEntity component for the instance ID.
pub struct StructureEntity(pub u16);

#[derive(Event, Debug, Clone, Copy)]
/// Spawns entity for object instance and associated instance IDs.
pub struct SpawnStructure(pub u32, pub StructureInstance);
fn spawn_structure(
	trigger: Trigger<SpawnStructure>,
	mut commands: Commands,
	structures: Res<StructureTypes>,
	grid: Res<Grid>,
) {
	let (instance_id, instance) = (trigger.0, trigger.1);
	let structure = match structures.0.get(&instance.structure_id) {Some(structure) => structure, None => {error!("Failed to spawn structure with unknown structure type ID {} on cell {:?}", instance.structure_id, instance.pos); return}};
	let [x, z] = axial_to_xz(&instance.pos);
	let y = match grid.heights.get(&instance.pos) {Some(height) => *height as f32, None => {error!("Failed to spawn structure {:?} on cell {:?} because grid is missing cell height.", structure, instance.pos); return}};
	commands.spawn((
		CellPos(instance.pos),
		ObjectEntity(instance_id),
		StructureEntity(instance.structure_id),
		SceneRoot(structure.scene.clone()),
		Transform {
			translation: Vec3::new(x, y, z),
			..default()
		},
		OutlineVolume {
			width: 2.,
			..default()
		},
		OutlineMode::FloodFlat, // TODO: Find a way to make the outline as smooth as the gizmo.
		AsyncSceneInheritOutline::default(),
	))
	.observe(handle_hover_start)
	.observe(handle_hover_end)
	.observe(handle_click);
}

fn handle_hover_start(
	mut trigger: Trigger<Pointer<Over>>,
	mut commands: Commands,
	mut cursor: ResMut<Cursor>,
	structures: Query<&ObjectEntity, With<StructureEntity>>,
) {
	if matches!(cursor.tool, Tool::None) || matches!(cursor.tool, Tool::Select(_)) || matches!(cursor.tool, Tool::Remove) {
		let instance_id = match structures.get(trigger.target()) {Ok(instance) => instance.0, Err(e) => {error!("Mouse started hovering unknown structure: {}", e); return}};
		trigger.propagate(false);
		cursor.hover_objects = HoverObjects::Single(instance_id);
		commands.trigger(UpdateHoverOutline);
	}
}

fn handle_hover_end(
	trigger: Trigger<Pointer<Out>>,
	mut commands: Commands,
	mut cursor: ResMut<Cursor>,
	structures: Query<&ObjectEntity, With<StructureEntity>>,
) {
	let instance_id = match structures.get(trigger.target()) {Ok(instance) => instance.0, Err(e) => {error!("Mouse stopped hovering unknown structure: {}", e); return}};
	
	if cursor.hover_objects == HoverObjects::Single(instance_id) {
		cursor.hover_objects = HoverObjects::None;
		commands.trigger(UpdateHoverOutline);
	}
}

fn handle_click(
	mut trigger: Trigger<Pointer<Pressed>>,
	mut commands: Commands,
	mut cursor: ResMut<Cursor>,
	mut grid: ResMut<Grid>,
) {
	if matches!(cursor.tool, Tool::None) || matches!(cursor.tool, Tool::Select(_)) {
		trigger.propagate(false); // TODO: Double-check and test if this is the best way to handle overlap.
		if matches!(trigger.button, PointerButton::Primary) {
			let instance_id = match cursor.hover_object() {Some(instance_id) => instance_id, None => {error!("Mouse clicked structure before it was hovered."); return}};
			cursor.tool = Tool::Select(instance_id);
			commands.trigger(UpdateHoverOutline);
		}
	} else if matches!(cursor.tool, Tool::Remove) {
		trigger.propagate(false);
		if matches!(trigger.button, PointerButton::Primary) {
			let instance_id = match cursor.hover_object() {Some(instance_id) => instance_id, None => {error!("Mouse clicked structure before it was hovered."); return}};
			match grid.objects.remove(&instance_id) {Some(_) => (), None => {error!("Attempted to remove unknown structure with instance ID {}.", instance_id); return}}
			commands.trigger(DespawnStructure(instance_id));
		}
	}
}

fn update_structure_outlines(
	_trigger: Trigger<UpdateHoverOutline>,
	cursor: Res<Cursor>,
	mut entities: Query<(&ObjectEntity, &mut OutlineVolume), With<StructureEntity>>,
) {
	for (instance_id, mut outline) in entities.iter_mut() { // TODO: Find less of a brute-force solution for updating structure outlines.
		if cursor.tool == Tool::Select(instance_id.0) {
			outline.colour = Color::srgb(1., 0., 0.);
			outline.visible = true;
			continue
		}
		match &cursor.hover_objects {
			HoverObjects::None => outline.visible = false,
			HoverObjects::Single(hover_id) => {
				if *hover_id == instance_id.0 {outline.colour = Color::srgb(0., 0., 1.); outline.visible = true}
				else {outline.visible = false}
			},
			HoverObjects::Many(hover_ids, hover_index) => 'many: {
				for (index, id) in hover_ids.iter().enumerate() {
					if *id == instance_id.0 {
						outline.colour = if index == *hover_index {Color::srgb(0., 0., 1.)} else {Color::srgb(0., 1., 1.)};
						outline.visible = true;
						break 'many
					}
				}
				outline.visible = false; // Only gets here if for loop doesn't break.
			}
		}
	}
}

#[derive(Event, Debug, Clone, Copy)]
/// Despawn entity with specified instance ID.
pub struct DespawnStructure(pub u32);
fn despawn_structure(
	trigger: Trigger<DespawnStructure>,
	mut commands: Commands,
	entities: Query<(&ObjectEntity, Entity), With<StructureEntity>>,
) {
	for (instance_id, entity) in entities.iter() {
		if instance_id.0 == trigger.0 {
			commands.entity(entity).despawn();
		}
	}
}

/// Goes through all structure entities and adjusts the heights to match the terrain.
/// Does NOT spawn/despawn structure entities.
#[derive(Event, Debug, Clone, Copy)]
pub struct UpdateStructureHeights;
fn update_structure_heights( // TODO: Improve this method for structures spanning many cells.
	_trigger: Trigger<UpdateStructureHeights>,
	grid: Res<Grid>,
	mut structure_entities: Query<(&CellPos, &mut Transform), With<StructureEntity>>,
) {
	for (cell_pos, mut transform) in structure_entities.iter_mut() {
		let pos = cell_pos.0;
		let height = match grid.heights.get(&pos) {Some(height) => height, None => {error!("Attempted to update structure entity at {:?} with pos {:?}, which does not point to a valid cell.", transform, pos); return}};
		transform.translation.y = *height as f32;
	}
}
