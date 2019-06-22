pub mod location;
pub use location::Location;

pub mod planet;
pub use planet::Planet;

/**
 * A ground layout.
 */
trait Layout {
    type Location : Location;
}
