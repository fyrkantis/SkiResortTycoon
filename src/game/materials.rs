use bevy::{prelude::*, color::palettes::tailwind};

#[derive(Resource, Debug, Clone)]
pub struct Materials {
	pub snow: Handle<StandardMaterial>,
	pub piste: Handle<StandardMaterial>,
	pub water: Handle<StandardMaterial>,
	pub dirt: Handle<StandardMaterial>,
	pub rock: Handle<StandardMaterial>,

	pub red: Handle<StandardMaterial>,
	pub blue: Handle<StandardMaterial>,
}

pub fn setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.insert_resource(Materials {
		snow: materials.add(Color::from(tailwind::SLATE_300)),
		piste: materials.add(Color::WHITE),
		water: materials.add(StandardMaterial {
			base_color: Color::from(tailwind::SKY_800),
			reflectance: 1.,
			..Default::default()
		}),
		dirt: materials.add(StandardMaterial {
			base_color: Color::from(tailwind::YELLOW_950),
			reflectance: 0.,
			..Default::default()
		}),
		rock: materials.add(StandardMaterial {
			base_color: Color::from(tailwind::SLATE_800),
			reflectance: 0.,
			metallic: 0.1,
			..Default::default()
		}),

		red: materials.add(Color::srgb(1., 0., 0.)),
		blue: materials.add(Color::srgb(0., 0., 1.)),
	})
}
