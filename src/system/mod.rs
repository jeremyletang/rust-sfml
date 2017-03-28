//! Base module of SFML, defining various utilities.
//!
//! It provides vector types, timing types.
//!

pub use self::vector2::{Vector2, Vector2u, Vector2i, Vector2f};
pub use self::vector3::{Vector3, Vector3i, Vector3f};
pub use self::sleep::sleep;
pub use self::time::{Time, ZERO as TIME_ZERO};
pub use self::clock::Clock;
pub use self::sf_bool::{SfBool, TRUE as SF_TRUE, FALSE as SF_FALSE};

mod time;
mod clock;
mod sleep;
mod vector2;
mod vector3;
mod sf_bool;
pub mod raw_conv;
