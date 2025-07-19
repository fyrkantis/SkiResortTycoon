use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use bevy::prelude::*;

use crate::game::material::Materials;

mod temp_scenes;
use temp_scenes::*;

#[derive(Debug, Clone)]
pub struct Structure {
	pub id: u16,
	pub name: &'static str,
	pub scene: Handle<Scene>,
}
/// Defines new structure.
pub const fn structure(id: u16, name: &'static str, scene: Handle<Scene>) -> (u16, Structure) {(id, Structure {id: id, name: name, scene: scene})}
impl PartialEq for Structure {fn eq(&self, other: &Self) -> bool {self.id == other.id}}
impl Eq for Structure {}
impl Ord for Structure {fn cmp(&self, other: &Self) -> Ordering {self.id.cmp(&other.id)}}
impl PartialOrd for Structure {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Hash for Structure {fn hash<H: Hasher>(&self, state: &mut H) {self.id.hash(state)}}

#[derive(Resource, Debug, Clone)]
pub struct Structures(pub HashMap<u16, Structure>);

/// Loads all structure scenes.
pub fn load_assets(
	mut commands: Commands,
	mut scene_assets: ResMut<Assets<Scene>>,
	mut mesh_assets: ResMut<Assets<Mesh>>,
	mut material_assets: ResMut<Assets<StandardMaterial>>,
	materials: Res<Materials>,
) {
	commands.insert_resource(Structures(HashMap::from([
		structure(1, "Tree", scene_assets.add(tree_scene(&mut mesh_assets, &mut material_assets))),
		structure(111, "Red Box", scene_assets.add(red_box_scene(&mut mesh_assets, &materials))),
		structure(222, "Blue Sphere", scene_assets.add(blue_sphere_scene(&mut mesh_assets, &materials))),
	])));
}

