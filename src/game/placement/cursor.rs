use bevy::prelude::*;
use hexx::Hex;

use crate::game::{surface::Surface, object::ObjectType};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Tool {
	#[default]
	None,
	Select(Hex, Option<u16>),
	Place,
	Surface,
	Terrain,
	Remove,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Cursor {
	pub hover_cell: Option<Hex>,
	pub tool: Tool,
	pub selected_surface: Option<Surface>,
	pub selected_object_type: Option<ObjectType>,
}
