/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! Audio listener
//!
//! The audio listener is the point in the scene from where all the sounds are heard.

use system::vector3::Vector3f;

use ffi::audio as ffi;

/// Change the global volume of all the sounds and musics
///
/// The volume is a number between 0 and 100, it is combined with
/// the individual volume of each sound / music.
/// The default value for the volume is 100 (maximum).
///
/// # Arguments
/// * volume - The new global volume, in the range [0, 100]
pub fn set_global_volume(volume: f32) -> () {
    unsafe {
        ffi::sfListener_setGlobalVolume(volume as f32)
    }
}

/// Get the current value of the global volume
///
/// Return the current global volume, in the range [0, 100]
pub fn get_global_volume() -> f32 {
    unsafe {
        ffi::sfListener_getGlobalVolume() as f32
    }
}

/// Set the position of the listener in the scene
///
/// The default listener's position is (0, 0, 0).
///
/// # Arguments
/// * * position - the New position of the listener
pub fn set_position(position: &Vector3f) -> () {
    unsafe {
        ffi::sfListener_setPosition(*position)
    }
}

/// Set the position of the listener in the scene
///
/// The default listener's position is (0, 0, 0).
///
/// # Arguments
/// * x - the New position of the listener in x
/// * y - the New position of the listener in y
/// * z - the New position of the listener in z
pub fn set_position3f(x: f32, y: f32, z: f32) -> () {
    unsafe {
        ffi::sfListener_setPosition(Vector3f::new(x, y, z))
    }
}

/// Get the current position of the listener in the scene
///
/// Return the listener's position
pub fn get_position() -> Vector3f {
    unsafe {
        ffi::sfListener_getPosition()
    }
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
pub fn set_direction(direction: &Vector3f) -> () {
    unsafe {
        ffi::sfListener_setDirection(*direction)
    }
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
pub fn set_direction3f(x: f32, y: f32, z: f32) -> () {
    unsafe {
        ffi::sfListener_setDirection(Vector3f::new(x, y, z))
    }
}

/// Get the current orientation of the listener in the scene
///
/// Return the listener's direction
pub fn get_direction() -> Vector3f {
    unsafe {
        ffi::sfListener_getDirection()
    }
}
