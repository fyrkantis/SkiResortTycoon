use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use bevy::{prelude::*, color::palettes::tailwind};

#[derive(Debug, Clone, Copy)]
pub struct Item {
	pub id: usize,
	pub name: &'static str,
//	pub scene: Scene,
}
/// Constructs new item.
pub const fn item(id: usize, name: &'static str/*, scene: Scene*/) -> Item {Item {id: id, name: name/*, scene: scene*/}}
impl PartialEq for Item {fn eq(&self, other: &Self) -> bool {self.id == other.id}}
impl Eq for Item {}
impl Ord for Item {fn cmp(&self, other: &Self) -> Ordering {self.id.cmp(&other.id)}}
impl PartialOrd for Item {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Hash for Item {fn hash<H: Hasher>(&self, state: &mut H) {self.id.hash(state)}}

#[derive(Debug, Resource, Clone)]
pub struct ItemList(pub Vec<Item>);

pub fn load_items_system(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.insert_resource(ItemList(vec![
		item(111, "Red Box"/*, (
			Mesh3d(meshes.add(Cuboid::new(2., 2., 2.))),
			Transform::from_xyz(0., 1., 0.),
			MeshMaterial3d(materials.add(Color::from(tailwind::RED_600))),
		)*/),
		item(222, "Blue Sphere"/*, (
			Mesh3d(meshes.add(Sphere::new(2.))),
			Transform::from_xyz(0., 1., 0.),
			MeshMaterial3d(materials.add(Color::from(tailwind::BLUE_600))),
		)*/),
	]));
}

