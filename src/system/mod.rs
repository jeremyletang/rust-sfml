//! Base module of SFML, defining various utilities.
//!
//! It provides vector types, timing types.
//!

pub use self::clock::Clock;
pub use self::sf_bool::{FALSE, SfBool, TRUE};
pub use self::sleep::sleep;
pub use self::time::Time;
pub use self::vector2::{Vector2, Vector2f, Vector2i, Vector2u};
pub use self::vector3::{Vector3, Vector3f, Vector3i};

mod time;
mod clock;
mod sleep;
mod vector2;
mod vector3;
mod sf_bool;
