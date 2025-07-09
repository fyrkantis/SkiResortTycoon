use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use bevy::{prelude::Srgba, color::palettes::tailwind};

#[derive(Debug, Clone, Copy)]
pub struct Item {
	pub id: usize,
	pub name: &'static str,
	pub color: Srgba,
}
/// Constructs new item.
pub const fn item(id: usize, name: &'static str, color: Srgba) -> Item {Item {id: id, name: name, color: color}}
impl PartialEq for Item {fn eq(&self, other: &Self) -> bool {self.id == other.id}}
impl Eq for Item {}
impl Ord for Item {fn cmp(&self, other: &Self) -> Ordering {self.id.cmp(&other.id)}}
impl PartialOrd for Item {fn partial_cmp(&self, other: &Self) -> Option<Ordering> {Some(self.cmp(other))}}
impl Hash for Item {fn hash<H: Hasher>(&self, state: &mut H) {self.id.hash(state)}}

pub const ITEMS: [Item; 2] = [
	item(111, "Brown", tailwind::AMBER_900),
	item(222, "Gray", tailwind::NEUTRAL_600),
];
