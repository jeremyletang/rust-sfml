//! Base module of SFML, defining various utilities.
//!
//! It provides vector types, timing types.
//!

pub use self::{
    angle::Angle,
    clock::Clock,
    input_stream::InputStream,
    sleep::sleep,
    string::{SfStr, SfStrConv, SfString},
    time::Time,
    vector2::{Vector2, Vector2f, Vector2i, Vector2u},
    vector3::{Vector3, Vector3f, Vector3i},
};

mod angle;
pub(crate) mod buffer;
mod clock;
mod input_stream;
mod sleep;
mod string;
mod time;
mod vector2;
mod vector3;
