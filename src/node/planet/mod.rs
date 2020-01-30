use std::sync::{Arc, Weak};
use render::Vector3D;
use render::Context;
use engine::Transformation;

use crate::{layer, Node};

mod geometry;
pub use geometry::Geometry;

pub struct Descriptor {
	/// Radius of the planet, in m.
	pub radius: f32
}

pub struct Planet<C: Context> {
	/// Parent node.
	parent: Weak<dyn engine::Node<C>>,

	/// Global infos about the planet.
	d: Descriptor,

	/// Low detail geometry of the planet.
	geometry: Geometry<C>,

    /// Topology layer.
    topology: layer::planet::Topology<C>,
}

impl<C: Context> Planet<C> {
	pub fn new(context: &C, parent: &Arc<dyn engine::Node<C>>, d: Descriptor) -> Planet<C> {
		let geometry = Geometry::new(context, d.radius);
		let topology = layer::planet::Topology::new(context, &d);

		Planet {
			parent: Arc::downgrade(parent),
			d,
			geometry,
			topology
		}
	}

	pub fn descriptor(&self) -> &Descriptor {
		&self.d
	}

	pub fn altitude(&self, pos: Vector3D<f32>) -> f32 {
		panic!("TODO")
	}
}

impl<C: Context> Node for Planet<C> {
	fn focus(&mut self, pos: Vector3D<f32>) {
		let d = self.altitude(pos);

		if d > 0.0 {
			let d_magnitude = (d/1000.0).log(10.0);
		} else {
			// nothing to do.
		}
	}
}

impl<C: Context> engine::Node<C> for Planet<C> {
	fn parent(&self) -> Weak<dyn engine::Node<C>> {
		self.parent.clone()
	}

	fn transformation(&self) -> Option<&Transformation> {
		None
	}

	fn render(&self, render: &mut engine::Renderer<C>) {
		render.geometry(self.topology.program(), &self.geometry);
	}
}
