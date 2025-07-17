use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use bevy::{prelude::*, color::palettes::tailwind};

use crate::game::materials::Materials;

#[derive(Debug, Clone)]
pub struct Item {
	pub id: u16,
	pub name: &'static str,
	pub scene: Handle<Scene>,
}
/// Constructs new item.
pub const fn item(id: u16, name: &'static str, scene: Handle<Scene>) -> (u16, Item) {(id, Item {id: id, name: name, scene: scene})}
impl PartialEq for Item {fn eq(&self, other: &Self) -> bool {self.id == other.id}}
impl Eq for Item {}
impl Ord for Item {fn cmp(&self, other: &Self) -> Ordering {self.id.cmp(&other.id)}}
impl PartialOrd for Item {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Hash for Item {fn hash<H: Hasher>(&self, state: &mut H) {self.id.hash(state)}}

#[derive(Resource, Debug, Clone)]
pub struct Items(pub HashMap<u16, Item>);

pub fn load_items_system(
	mut commands: Commands,
	mut scene_assets: ResMut<Assets<Scene>>,
	mut mesh_assets: ResMut<Assets<Mesh>>,
	mut material_assets: ResMut<Assets<StandardMaterial>>,
	materials: Res<Materials>,
) {
	commands.insert_resource(Items(HashMap::from([
		item(1, "Tree", scene_assets.add(tree_scene(&mut mesh_assets, &mut material_assets))),
		item(111, "Red Box", scene_assets.add(red_box_scene(&mut mesh_assets, &materials))),
		item(222, "Blue Sphere", scene_assets.add(blue_sphere_scene(&mut mesh_assets, &materials))),
	])));
}

fn tree_scene(
	mesh_assets: &mut ResMut<Assets<Mesh>>,
	material_assets: &mut ResMut<Assets<StandardMaterial>>,
) -> Scene {
	let leaves = material_assets.add(Color::from(tailwind::GREEN_800));
	let wood = material_assets.add(Color::from(tailwind::AMBER_950));
	let mut world = World::new();
	world.spawn((
		Mesh3d(mesh_assets.add(Cylinder::new(0.1, 0.5))),
		MeshMaterial3d(wood.clone()),
		Transform::from_xyz(0., 0.25, 0.),
	));
	world.spawn((
		Mesh3d(mesh_assets.add(Cone::new(0.2, 0.5))),
		MeshMaterial3d(wood.clone()),
		Transform::from_translation(Vec3::ZERO),
	));
	for i in 0..3 {
		let i_f = i as f32;
		world.spawn((
			Mesh3d(mesh_assets.add(Cone::new(0.5 - i_f / 10., 0.5 - i_f / 20.))),
			MeshMaterial3d(leaves.clone()),
			Transform::from_xyz(0., 0.75 + i_f / 4., 0.),
		));
	}
	Scene::new(world)
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
