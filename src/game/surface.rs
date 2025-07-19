use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;

use crate::game::material::Materials;
use crate::util::hex::cell_slope;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Surface {
	#[default]
	Normal,
	Piste,
	Water,
}

pub const SNOW_MAX_SLOPE: u16 = 3;
pub const DIRT_MAX_SLOPE: u16 = 4;

pub fn cell_material<'a>(materials: &'a Materials, heights: &HashMap<Hex, u16>, pos: &Hex, surface: Surface) -> &'a Handle<StandardMaterial> {
	match surface {
		Surface::Piste => &materials.piste,
		Surface::Water => &materials.water,
		Surface::Normal => {
			let slope = cell_slope(heights, pos);
			if slope > DIRT_MAX_SLOPE {return &materials.rock}
			else if slope > SNOW_MAX_SLOPE {return &materials.dirt}
			else {return &materials.snow}
		}
	}
}
