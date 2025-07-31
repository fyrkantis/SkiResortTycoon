use bevy::prelude::*;
use hexx::Hex;

use crate::game::{surface::Surface, object::ObjectType};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Tool {
	#[default]
	None,
	/// Instance ID of selected object.
	Select(u32),
	Place,
	Surface,
	Terrain,
	Remove,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum HoverObjects {
	#[default]
	None,
	Single(u32),
	/// Multiple objects are hovered, but only one is focused.
	///
	/// Fist argument is a vector containing all hovered objects, second is the index of the focused object.
	Many(Vec<u32>, usize),
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Cursor {
	pub hover_cell: Option<Hex>,
	/// Instance IDs of all objects that are currently under the mouse.
	///
	/// Use hover_item() to get the focused object instance ID.
	pub hover_objects: HoverObjects,
	pub tool: Tool,
	pub selected_surface: Option<Surface>,
	pub selected_object_type: Option<ObjectType>,
}
impl Cursor {
	/// Instance ID of currently hovered and focused object (but not necessarily selected).
	pub fn hover_object(&self) -> Option<u32> {
		match &self.hover_objects {
			HoverObjects::None => None,
			HoverObjects::Single(object_id) => Some(*object_id),
			HoverObjects::Many(object_ids, object_id_index) => match object_ids.clone().get(*object_id_index) {
				Some(object_id) => Some(*object_id),
				None => {warn!("Multiple objects instances are hovered, but the focused hover object index is out of range."); None},
			},
		}
	}
}
