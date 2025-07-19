use bevy::{prelude::*, color::palettes::tailwind};

#[derive(Resource, Debug, Default, Clone)]
pub struct Materials {
	pub snow: Handle<StandardMaterial>,
	pub piste: Handle<StandardMaterial>,
	pub water: Handle<StandardMaterial>,
	pub dirt: Handle<StandardMaterial>,
	pub rock: Handle<StandardMaterial>,

	pub red: Handle<StandardMaterial>,
	pub blue: Handle<StandardMaterial>,

	pub error: Handle<StandardMaterial>,
}

pub fn load_assets(
	mut material_assets: ResMut<Assets<StandardMaterial>>,
	mut materials: ResMut<Materials>,
) {
	*materials = Materials {
		snow: material_assets.add(Color::from(tailwind::SLATE_300)),
		piste: material_assets.add(Color::WHITE),
		water: material_assets.add(StandardMaterial {
			base_color: Color::from(tailwind::SKY_800),
			reflectance: 1.,
			..Default::default()
		}),
		dirt: material_assets.add(StandardMaterial {
			base_color: Color::from(tailwind::YELLOW_950),
			reflectance: 0.,
			..Default::default()
		}),
		rock: material_assets.add(StandardMaterial {
			base_color: Color::from(tailwind::SLATE_800),
			reflectance: 0.,
			metallic: 0.1,
			..Default::default()
		}),

		red: material_assets.add(Color::srgb(1., 0., 0.)),
		blue: material_assets.add(Color::srgb(0., 0., 1.)),

		error: material_assets.add(Color::srgb(1., 0., 0.)),
	};
}
