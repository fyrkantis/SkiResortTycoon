use std::ops::{Add, Sub};
use hexx::Hex;

// TODO: Use fancy new std::f32::consts::SQRT_3 when available. https://github.com/rust-lang/rust/issues/103883
const SQRT_3: f32 = 1.732050807568877293527446341505872367;

/// Specifies a corner of a flat-top orientation hexagon, ordered as follows:
/// ```text
///   /F-A\
///  E     B
///   \D-C/
/// ```
/// Default corner is `A`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub enum Rotation {
	#[default]
	A,
	B,
	C,
	D,
	E,
	F,
}
impl Rotation {
	pub const ALL: [Self; 6] = [Self::A, Self::B, Self::C, Self::D, Self::E, Self::F];
	pub const EVEN: [Self; 3] = [Self::A, Self::C, Self::E];
	pub const ODD: [Self; 3] = [Self::B, Self::D, Self::F];

	pub const fn from_usize(value: usize) -> Self {Self::ALL[value % 6]}

	pub const fn is_even(self) -> bool {
		//match self {Self::A => true, Self::B => false, Self::C => true, Self::D => false, Self::E => true, Self::F => false} // TODO: Remove if not needed.
		self as usize % 2 == 0
	}
	
	/// 2D coordinates of the specified corner.
	pub const fn corner_xz(&self) -> [f32; 2] {
		const SQRT_3_DIV_2: f32 = SQRT_3 / 2.;
		match self {
			Self::A => [0.5, -SQRT_3_DIV_2],
			Self::B => [1., 0.],
			Self::C => [0.5, SQRT_3_DIV_2],
			Self::D => [-0.5, SQRT_3_DIV_2],
			Self::E => [-1., 0.],
			Self::F => [-0.5, -SQRT_3_DIV_2],
		}
	}
	pub const fn corner_next(self) -> Self {Self::from_usize(self as usize + 1)}
	pub const fn corner_last(self) -> Self {Self::from_usize(self as usize - 1)} // TODO: Test this.

	/// Axial coordinates of the edge 
	pub const fn edge_last(&self) -> Hex {
		match self {
			Self::A => Hex::new(0, -1),
			Self::B => Hex::new(1, -1),
			Self::C => Hex::new(1, 0),
			Self::D => Hex::new(0, 1),
			Self::E => Hex::new(-1, 1),
			Self::F => Hex::new(-1, 0),
		}
	}
	pub const fn edge_next(self) -> Hex {self.corner_next().edge_last()}
}
impl From<usize> for Rotation {
	fn from(value: usize) -> Self {Self::from_usize(value)}
}
impl Add<usize> for Rotation {
	type Output = Self;
	fn add(self, other: usize) -> Self {Self::from(self as usize + other)}
}
impl Sub<usize> for Rotation {
	type Output = Self;
	fn sub(self, other: usize) -> Self {Self::from(self as usize - other)}
}