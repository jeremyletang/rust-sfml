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

//! The point in the scene from where all sounds are heard.
//!
//! The audio listener defines the global properties of the audio environment,
//! namely the position, direction, up vector, and global volume.
//!
//! If `View` is the eyes of the user, then `listener` is their ears (by the
//! way, they are often linked together – same position, orientation, etc.).

use system::Vector3f;

use ffi::audio as ffi;

/// Change the global volume of all the sounds and music.
///
/// The volume is a number in the range [0, 100], and is combined with the
/// individual volume of each sound / music.
/// The default value for the volume is 100 (maximum).
pub fn set_global_volume(volume: f32) {
    unsafe {
        ffi::sfListener_setGlobalVolume(volume as f32)
    }
}

/// Get the current value of the global volume, in the range [0, 100].
pub fn get_global_volume() -> f32 {
    unsafe {
        ffi::sfListener_getGlobalVolume() as f32
    }
}

/// Set the position of the listener in the scene.
///
/// The default position of the listener is (0, 0, 0).
pub fn set_position(position: &Vector3f) {
    unsafe {
        ffi::sfListener_setPosition(*position)
    }
}

/// Set the position of the listener in the scene.
///
/// The default position of the listener is (0, 0, 0).
pub fn set_position3f(x: f32, y: f32, z: f32) {
    unsafe {
        ffi::sfListener_setPosition(Vector3f::new(x, y, z))
    }
}

/// Get the current position of the listener in the scene.
pub fn get_position() -> Vector3f {
    unsafe {
        ffi::sfListener_getPosition()
    }
}

/// Set the forward vector of the listener in the scene.
///
/// The direction (also called "at vector") is the vector pointing forward from
/// the listener's perspective. Together with the up vector, it defines the 3D
/// orientation of the listener in the scene. The direction vector doesn't have
/// to be normalized. The default listener's direction is (0, 0, -1).
pub fn set_direction(direction: &Vector3f) -> () {
    unsafe {
        ffi::sfListener_setDirection(*direction)
    }
}

/// Set the forward vector of the listener in the scene.
///
/// The direction (also called "at vector") is the vector pointing forward from
/// the listener's perspective. Together with the up vector, it defines the 3D
/// orientation of the listener in the scene. The direction vector doesn't have
/// to be normalized. The default listener's direction is (0, 0, -1).
pub fn set_direction3f(x: f32, y: f32, z: f32) -> () {
    unsafe {
        ffi::sfListener_setDirection(Vector3f::new(x, y, z))
    }
}

/// Get the current forward vector of the listener in the scene.
pub fn get_direction() -> Vector3f {
    unsafe {
        ffi::sfListener_getDirection()
    }
}

/// Set the upward vector of the listener in the scene.
///
/// The up vector is the vector that points upward from the listener's
/// perspective. Together with the direction, it defines the 3D orientation of
/// the listener in the scene. The up vector doesn't have to be normalized. The
/// default listener's up vector is (0, 1, 0). It is usually not necessary to
/// change it, especially in 2D scenarios.
pub fn set_up_vector(vec: &Vector3f) {
	unsafe { ffi::sfListener_setUpVector(*vec) }
}

/// Set the upward vector of the listener in the scene.
///
/// The up vector is the vector that points upward from the listener's
/// perspective. Together with the direction, it defines the 3D orientation of
/// the listener in the scene. The up vector doesn't have to be normalized. The
/// default listener's up vector is (0, 1, 0). It is usually not necessary to
/// change it, especially in 2D scenarios.
pub fn set_up_vector3f(x: f32, y: f32, z: f32) {
	unsafe { ffi::sfListener_setUpVector(Vector3f::new(x, y, z)) }
}

/// Get the current upward vector of the listener in the scene.
pub fn get_up_vector() -> Vector3f {
	unsafe { ffi::sfListener_getUpVector() }
}
