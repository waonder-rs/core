pub mod layer;

pub struct Planet {
	/// General topology of the planet.
	topology: layer::Topology
}

impl Planet {
	pub fn new() -> Planet {
		Planet {
			topology: layer::Topology::new()
		}
	}
}