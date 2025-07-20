use bevy::prelude::*;
use bevy_egui::*;

use crate::game::{object::ObjectType, object::structure::StructureTypes};
use crate::game::placement::cursor::{Cursor, Tool};

pub fn setup(
	mut contexts: EguiContexts,
	mut cursor: ResMut<Cursor>,
	structures: Res<StructureTypes>,
) {
	egui::Window::new("Build")
	.collapsible(false)
	.resizable(false)
	.show(contexts.ctx_mut(), |ui| {
		ui.label("Tools");
		if ui.button("Place Structure").clicked() {cursor.tool = Tool::Place;}
		if ui.button("Remove Structure").clicked() {cursor.tool = Tool::Remove;}
		if ui.button("Add/remove piste").clicked() {cursor.tool = Tool::Surface;}
		if ui.button("Raise/lower terrain").clicked() {cursor.tool = Tool::Terrain;}
		if ui.button("None").clicked() {cursor.tool = Tool::None;}

		if matches!(cursor.tool, Tool::Place) {
			ui.label("Structures");
			ui.horizontal(|ui| {
				for (_, structure) in structures.0.iter() {
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
							ui.label(structure.name);
						});
						let space = frame.allocate_space(ui);
						let response = ui.allocate_rect(space.interact_rect, egui::Sense::click());
						if response.clicked() {
							cursor.selected_object_type = Some(ObjectType::Structure(structure.id));
						}
						if cursor.selected_object_type == Some(ObjectType::Structure(structure.id)) {
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
		}
	});
}
