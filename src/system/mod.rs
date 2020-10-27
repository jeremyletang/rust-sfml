//! Base module of SFML, defining various utilities.
//!
//! It provides vector types, timing types.
//!

pub(crate) use self::sf_box::{Dispose, RawDefault};
#[cfg(feature = "window")]
pub use self::string::{SfStr, SfStrConv};
pub use self::{
    clock::Clock,
    sf_box::{SfBox, SfResource},
    sleep::sleep,
    time::Time,
    vector2::{Vector2, Vector2f, Vector2i, Vector2u},
    vector3::{Vector3, Vector3f, Vector3i},
};

mod clock;
mod sf_box;
mod sleep;
#[cfg(feature = "window")]
mod string;
mod time;
mod vector2;
mod vector3;
