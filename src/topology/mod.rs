//! Topologies
//!
//! A topology describe the global geometrical form of an object.
//! It can be refined on demand, producing smaller and smaller *regions*.

pub mod utils;
pub mod sphere;

pub use sphere::Sphere;
