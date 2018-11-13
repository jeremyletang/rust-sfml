//! Base module of SFML, defining various utilities.
//!
//! It provides vector types, timing types.
//!

pub use self::clock::Clock;
pub use self::sf_box::SfBox;
pub(crate) use self::sf_box::{Dispose, RawDefault};
pub use self::sleep::sleep;
pub use self::time::Time;
pub use self::vector2::{Vector2, Vector2f, Vector2i, Vector2u};
pub use self::vector3::{Vector3, Vector3f, Vector3i};
use csfml_system_sys::{sfBool, sfFalse, sfTrue};

/// Boolean type used by CSFML.
///
/// This is required in some places instead of `bool` for FFI reasons.
///
/// # Example
/// ```ignore
/// use sfml::window::ContextSettings;
/// use sfml::system;
/// let mut context_settings = ContextSettings::default();
/// context_settings.srgb_capable = system::TRUE;
/// ```
pub type Bool = sfBool;

/// Boolean `false` value used by CSFML.
pub const FALSE: Bool = sfFalse;
/// Boolean `true` value used by CSFML.
pub const TRUE: Bool = sfTrue;

mod clock;
mod sf_box;
mod sleep;
mod time;
mod vector2;
mod vector3;
