#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Surface {
	#[default]
	Normal,
	Piste,
	Water,
}
