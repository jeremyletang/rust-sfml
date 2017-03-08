// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

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
