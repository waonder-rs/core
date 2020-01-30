use render::Context;
use engine::Program;

/// Planet topology.
pub struct Topology<C: Context> {
	program: Program<C>
}

impl<C: Context> Topology<C> {
	pub fn new(context: &C, d: &crate::node::planet::Descriptor) -> Topology<C> {
		panic!("TODO");
	}

	pub fn program(&self) -> &Program<C> {
		&self.program
	}
}
