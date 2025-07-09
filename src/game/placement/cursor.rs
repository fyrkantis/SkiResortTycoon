use bevy::prelude::*;
use hexx::Hex;

use crate::game::placement::grid::GridCell;

#[derive(Resource, Default, Debug, Clone)]
pub struct Cursor {
	pub hover_cell: Option<(Hex, GridCell)>
}

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(Cursor::default());
	}
}
