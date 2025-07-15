use std::collections::HashMap;
use bevy::prelude::*;
use hexx::Hex;
use noise::{Perlin, NoiseFn};
use rand::prelude::*;

use crate::util::hex::{axial_to_xz, offset_to_axial};
use crate::game::surface::Surface;

#[derive(Debug, Default, Clone, Copy)]
pub struct GridCell {
	pub height: u16,
	pub surface: Surface,
	pub item_id: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WorldGenSettings {
	/// 1-50
	pub peak_height: f64,
	/// 1-50
	pub peak_width: f64,
	/// 1-50
	pub slope_height: f64, // TODO: Add more parameters.
}
impl Default for WorldGenSettings {
	fn default() -> Self {Self {
		peak_height: 10., peak_width: 30., slope_height: 40.
	}}
}

#[derive(Resource, Debug, Clone)]
pub struct Grid {
	pub cells: HashMap<Hex, GridCell>,
	pub width: u16,
	pub length: u16,
	pub settings: WorldGenSettings
}
impl Grid {
	pub const WATER_HEIGHT: f64 = -3.;
	
	/// Generates a new grid with set width and length.
	/// It is recommended to use an odd number for width to avoid sharp corners.
	pub fn new(width: u16, length: u16, settings: WorldGenSettings) -> Self {
		let mut cells: HashMap<Hex, GridCell> = HashMap::new();
		let mut rng = rand::rng();
		let perlin = Perlin::new(rng.random());
		let max_z = length as f64 * f64::sqrt(3.); // TODO: Use fancy new std::f32::consts::SQRT_3 when available. https://github.com/rust-lang/rust/issues/103883
		for col in 0..width as i32 {
			for row in 0..length as i32 + (col % 2) { // Adds one extra row every other column (avoids sharp corners.
				let pos_axial = offset_to_axial(col, row);
				
				let [x, z] = axial_to_xz(&pos_axial); // NOTE: xz are pixel coordinates, not hexagonal.
				let height = perlin.get([x as f64 / settings.peak_width, z as f64 / settings.peak_width])
				* settings.peak_height + (z as f64 / max_z) * settings.slope_height;

				cells.insert(pos_axial, GridCell {
					height: height as u16,
					surface: if height < Grid::WATER_HEIGHT {Surface::Water} else {Surface::Normal},
					..Default::default()
				});
				
			}
		}
		Grid {
			cells: cells,
			width: width,
			length: length,
			settings: settings,
		}
	}
}
