use bevy::{prelude::*, color::palettes::tailwind};

use crate::game::material::Materials;

pub fn tree_scene(
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

pub fn red_box_scene(
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &Materials,
) -> Scene {
	let mut world = World::new();
	world.spawn((
		Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
		MeshMaterial3d(materials.red.clone()),
		Transform::from_xyz(0., 0.5, 0.),
		Pickable {
			is_hoverable: true,
			should_block_lower: false,
		},
	));
	Scene::new(world)
}

pub fn blue_sphere_scene(
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &Materials,
) -> Scene {
	let mut world = World::new();
	world.spawn((
		Mesh3d(meshes.add(Sphere::new(0.5))),
		MeshMaterial3d(materials.blue.clone()),
		Transform::from_xyz(0., 0.25, 0.),
		Pickable {
			is_hoverable: true,
			should_block_lower: false,
		},
	));
	Scene::new(world)
}
