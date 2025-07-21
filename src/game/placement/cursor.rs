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

#[derive(Resource, Default, Debug, Clone)]
pub struct Cursor {
	pub hover_cell: Option<Hex>,
	/// Instance ID of a hovered item.
	pub hover_item: Option<u32>,
	pub tool: Tool,
	pub selected_surface: Option<Surface>,
	pub selected_object_type: Option<ObjectType>,
}
