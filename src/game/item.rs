use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use bevy::{prelude::*, color::palettes::tailwind};

use crate::game::materials::Materials;

#[derive(Debug, Clone)]
pub struct Item {
	pub id: u16,
	pub name: &'static str,
	pub scene: Handle<Scene>
}
/// Constructs new item.
pub const fn item(id: u16, name: &'static str, scene: Handle<Scene>) -> (u16, Item) {(id, Item {id: id, name: name, scene: scene})}
impl PartialEq for Item {fn eq(&self, other: &Self) -> bool {self.id == other.id}}
impl Eq for Item {}
impl Ord for Item {fn cmp(&self, other: &Self) -> Ordering {self.id.cmp(&other.id)}}
impl PartialOrd for Item {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Hash for Item {fn hash<H: Hasher>(&self, state: &mut H) {self.id.hash(state)}}

#[derive(Debug, Resource, Clone)]
pub struct Items(pub HashMap<u16, Item>);

pub fn load_items_system(
	mut commands: Commands,
	mut scenes: ResMut<Assets<Scene>>,
	mut meshes: ResMut<Assets<Mesh>>,
	materials: Res<Materials>
) {
	commands.insert_resource(Items(HashMap::from([
		item(111, "Red Box", scenes.add(red_box_scene(&mut meshes, &materials))),
		item(222, "Blue Sphere", scenes.add(blue_sphere_scene(&mut meshes, &materials))),
	])));
}

fn red_box_scene(
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &Materials,
) -> Scene {
	let mut world = World::new();
	world.spawn((
		Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
		MeshMaterial3d(materials.red.clone()),
		Transform::from_xyz(0., 0.5, 0.),
	));
	Scene::new(world)
}

fn blue_sphere_scene(
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &Materials,
) -> Scene {
	let mut world = World::new();
	world.spawn((
		Mesh3d(meshes.add(Sphere::new(0.5))),
		MeshMaterial3d(materials.blue.clone()),
		Transform::from_xyz(0., 0.25, 0.),
	));
	Scene::new(world)
}
