mod algebra;

pub mod spherical;
mod vector2d;
mod vector3d;

pub use algebra::*;
pub use vector2d::*;
pub use vector3d::*;

/**
 * A 3d location on the layout.
 * 2d location + height.
 */
pub trait Location {
    // ...
}
