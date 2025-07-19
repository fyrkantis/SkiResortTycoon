use bevy::prelude::*;
use bevy_egui::*;

use crate::game::{
	item::Items,
	placement::{
		cursor::{Cursor, Tool},
		grid::Grid,
	},
};

pub fn setup(
	mut contexts: EguiContexts,
	cursor: Res<Cursor>,
	items: Res<Items>,
	grid: Res<Grid>,
) {
	match cursor.tool {
		Tool::Select(pos, item_id) => {
			let item = match item_id {Some(item_id) => items.0.get(&item_id), None => None};
			egui::Window::new("Selected")
			.collapsible(false)
			.show(contexts.ctx_mut(), |ui| {
				ui.label(match item {Some(item) => item.name, None => "Empty cell"});
			});
		},
		_ => (),
	}
}
