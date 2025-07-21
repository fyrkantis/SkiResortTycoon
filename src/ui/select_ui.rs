use bevy::prelude::*;
use bevy_egui::*;

use crate::game::{
	object::{
		ObjectType,
		ObjectInstance,
		structure::StructureTypes,
	},
	placement::{
		cursor::{Cursor, Tool},
		grid::Grid,
	},
};

pub fn setup(
	mut contexts: EguiContexts,
	cursor: Res<Cursor>,
	structure_types: Res<StructureTypes>,
	grid: Res<Grid>,
) {
	match cursor.tool {
		Tool::Select(instance_id) => {
			let object_instance = match grid.objects.get(&instance_id) {Some(instance) => instance, None => {error_once!("Selected object instance ID {} could not be found in grid.", instance_id); return}};
			egui::Window::new("Selected")
			.collapsible(false)
			.show(contexts.ctx_mut(), |ui| {
				match object_instance {
					ObjectInstance::Structure(structure) => {
						ui.label("Structure");
						match structure_types.0.get(&structure.structure_id) {
							Some(structure_type) => {
								ui.label(format!("Name: {}", structure_type.name));
							},
							None => {
								error_once!("Selected structure type ID {} could not be found in assets.", structure.structure_id);
								ui.label("Error: Unknown structure type.");
							}
						}
						ui.label(format!("At {:?}", structure.pos));
					},
					ObjectInstance::Lift(lift) => {
						ui.label("Lift");
						ui.label(format!("Nodes: {:?}", lift.nodes));
					}
				}
			});
		},
		_ => (),
	}
}
