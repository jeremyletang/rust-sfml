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

//! The audio listener is the point in the scene from where all the sounds are heard.
//!
//! The audio listener defines the global properties of the audio environment,
//! it defines where and how sounds and musics are heard.
//!
//! If `View` is the eyes of the user, then `listener` is his ears (by the way, they are often
//! linked together – same position, orientation, etc.).
//!
//! `listener` is a simple interface, which allows to setup the listener in the 3D audio environment
//! (position, direction and up vector), and to adjust the global volume.
//!
//! # Usage example
//!
//! ```
//! use sfml::audio::listener;
//!
//! // Move the listener to the position (1, 0, -5)
//! listener::set_position3f(1., 0., -5.);
//!
//! // Make it face the right axis (1, 0, 0)
//! listener::set_direction3f(1., 0., 0.);
//!
//! // Reduce the global volume
//! listener::set_global_volume(50.);
//! ```

use system::Vector3f;
use csfml_audio_sys as ffi;
use raw_conv::{Raw, FromRaw};
use csfml_system_sys::sfVector3f;

/// Change the global volume of all the sounds and musics
///
/// The volume is a number between 0 and 100, it is combined with
/// the individual volume of each sound / music.
/// The default value for the volume is 100 (maximum).
///
/// # Arguments
/// * volume - The new global volume, in the range [0, 100]
pub fn set_global_volume(volume: f32) {
    unsafe { ffi::sfListener_setGlobalVolume(volume as f32) }
}

/// Get the current value of the global volume
///
/// Return the current global volume, in the range [0, 100]
pub fn get_global_volume() -> f32 {
    unsafe { ffi::sfListener_getGlobalVolume() as f32 }
}

/// Set the position of the listener in the scene
///
/// The default listener's position is (0, 0, 0).
///
/// # Arguments
/// * * position - the New position of the listener
pub fn set_position(position: &Vector3f) {
    unsafe { ffi::sfListener_setPosition(position.raw()) }
}

/// Set the position of the listener in the scene
///
/// The default listener's position is (0, 0, 0).
///
/// # Arguments
/// * x - the New position of the listener in x
/// * y - the New position of the listener in y
/// * z - the New position of the listener in z
pub fn set_position3f(x: f32, y: f32, z: f32) {
    unsafe { ffi::sfListener_setPosition(sfVector3f { x: x, y: y, z: z }) }
}

/// Get the current position of the listener in the scene
///
/// Return the listener's position
pub fn get_position() -> Vector3f {
    unsafe { Vector3f::from_raw(ffi::sfListener_getPosition()) }
}

/// Set the orientation of the listener in the scene
///
/// The orientation defines the 3D axes of the listener
/// (left, up, front) in the scene. The orientation vector
/// doesn't have to be normalized.
/// The default listener's orientation is (0, 0, -1).
///
/// # Arguments
/// * direction - New listener's orientation
pub fn set_direction(direction: &Vector3f) {
    unsafe { ffi::sfListener_setDirection(direction.raw()) }
}

/// Set the orientation of the listener in the scene
///
/// The orientation defines the 3D axes of the listener
/// (left, up, front) in the scene. The orientation vector
/// doesn't have to be normalized.
/// The default listener's orientation is (0, 0, -1).
///
/// # Arguments
/// * x - X coordinate of the listener's orientation
/// * y - Y coordinate of the listener's orientation
/// * z - Z coordinate of the listener's orientation
pub fn set_direction3f(x: f32, y: f32, z: f32) {
    unsafe { ffi::sfListener_setDirection(sfVector3f { x: x, y: y, z: z }) }
}

/// Get the current orientation of the listener in the scene
///
/// Return the listener's direction
pub fn get_direction() -> Vector3f {
    unsafe { Vector3f::from_raw(ffi::sfListener_getDirection()) }
}

/// Set the upward vector of the listener in the scene.
///
/// The up vector is the vector that points upward from the listener's perspective.
/// Together with the direction, it defines the 3D orientation of the listener in the scene.
/// The up vector doesn't have to be normalized. The default listener's up vector is (0, 1, 0).
/// It is usually not necessary to change it, especially in 2D scenarios.
pub fn set_up_vector(value: &Vector3f) {
    unsafe { ffi::sfListener_setUpVector(value.raw()) }
}

/// Get the current upward vector of the listener in the scene. (not normalized)
pub fn up_vector() -> Vector3f {
    unsafe { Vector3f::from_raw(ffi::sfListener_getUpVector()) }
}
