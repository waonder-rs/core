/// Topological layer.
/// 
/// Gives the general shape of the planet.
pub struct Topology {
	// ...
}

impl Topology {
	pub fn new() -> Topology {
		Topology {
			// foo.
		}
	}

	pub fn geometry(&self) -> crate::topology::Sphere {
		crate::topology::Sphere::new()
	}
}