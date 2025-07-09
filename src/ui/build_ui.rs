use bevy::prelude::*;
use bevy_egui::*;

use crate::game::item::ItemList;
use crate::game::placement::cursor::Cursor;

pub fn ui_system(
	mut contexts: EguiContexts,
	mut cursor: ResMut<Cursor>,
	items: Res<ItemList>,
) {
	egui::Window::new("Build")
	.collapsible(false)
	.resizable(false)
	.show(contexts.ctx_mut(), |ui| {
		ui.label("Items");
		ui.horizontal(|ui| {
			for item in items.0.iter() {
				let mut frame = egui::Frame::new()
				.fill(ui.visuals().widgets.open.bg_fill)
				.stroke(ui.visuals().widgets.open.bg_stroke)
				.corner_radius(ui.visuals().widgets.open.corner_radius)
				.begin(ui);
				{
					frame.content_ui.vertical(|ui| {
						ui.add(
							egui::Image::new(egui::include_image!("../../assets/settingsFaders.svg"))
							.fit_to_exact_size(egui::Vec2::new(100., 100.))
						);
						ui.label(item.name);
					});
					let space = frame.allocate_space(ui);
					let response = ui.allocate_rect(space.interact_rect, egui::Sense::click());
					if response.clicked() {
						println!("Clicked {:?}", item);
						cursor.selected_item = Some(*item);
					}
					if cursor.selected_item == Some(*item) {
						frame.frame.fill = ui.visuals().widgets.active.bg_fill;
						frame.frame.stroke = ui.visuals().widgets.active.bg_stroke;
						frame.frame.corner_radius = ui.visuals().widgets.active.corner_radius;
					} else if response.hovered() {
						frame.frame.fill = ui.visuals().widgets.hovered.bg_fill;
						frame.frame.stroke = ui.visuals().widgets.hovered.bg_stroke;
						frame.frame.corner_radius = ui.visuals().widgets.hovered.corner_radius;
					}
				}
				frame.paint(ui);
			}
		});
		
	});
}
