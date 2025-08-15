mod structure;
mod lift;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct ObjectInstanceId(u32);

/// A placed object.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ObjectInstance {
	Structure(structure::StructureInstance),
	Lift(lift::LiftInstance),
}
impl ObjectInstance {
	pub fn object_type() {
		
	}
	pub const fn name(&self)/* -> &str */ {
		/*match self {
			Self::Structure(instance) => instance.name,
			Self::Lift(instance) => instance.name
		}*/
	}
}