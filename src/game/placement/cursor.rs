use bevy::prelude::*;
use hexx::Hex;

use crate::game::placement::grid::GridCell;
use crate::game::surface::Surface;
use crate::game::item::Item;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Tool {
	Place,
	Remove,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct Cursor {
	pub hover_cell: Option<(Hex, GridCell)>,
	pub tool: Option<Tool>,
	pub selected_surface: Option<Surface>,
	pub selected_item: Option<Item>,
}
