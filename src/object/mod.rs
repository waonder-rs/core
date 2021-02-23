pub mod planet;

pub use planet::Planet;

pub enum Object {
	Planet(Planet)
}