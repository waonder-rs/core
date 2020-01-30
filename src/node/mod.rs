//! Procedural generation nodes.
//!
//! Each node represent a section of the universe.
//! Deeper nodes represent smaller sections but with greater detail.
//!
//! Here is the hierachical order of nodes by depth:
//! - [`Planet`]

use render::Vector3D;

pub mod planet;
pub use planet::Planet;

pub trait Node {
    /// Set the focus point of the node.
    fn focus(&mut self, pos: Vector3D<f32>);
}
