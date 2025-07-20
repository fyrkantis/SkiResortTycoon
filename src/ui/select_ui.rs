use bevy::prelude::*;
use bevy_egui::*;

use crate::game::{
	object::structure::StructureTypes,
	placement::{
		cursor::{Cursor, Tool},
		grid::Grid,
	},
};

pub fn setup(
	mut contexts: EguiContexts,
	cursor: Res<Cursor>,
	structures: Res<StructureTypes>,
	grid: Res<Grid>,
) {
	match cursor.tool {
		Tool::Select(pos, structure_id) => {
			let structure = match structure_id {Some(structure_id) => structures.0.get(&structure_id), None => None};
			egui::Window::new("Selected")
			.collapsible(false)
			.show(contexts.ctx_mut(), |ui| {
				ui.label(match structure {Some(structure) => structure.name, None => "Empty cell"});
			});
		},
		_ => (),
	}
}
