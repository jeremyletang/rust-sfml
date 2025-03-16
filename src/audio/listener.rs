//! The audio listener is the point in the scene from where all the sounds are heard.
//!
//! The audio listener defines the global properties of the audio environment,
//! it defines where and how sounds and musics are heard.
//!
//! If [`View`] is the eyes of the user, then `listener` is their ears (by the way, they are often
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
//! listener::set_position((1., 0., -5.));
//!
//! // Make it face the right axis (1, 0, 0)
//! listener::set_direction((1., 0., 0.));
//!
//! // Reduce the global volume
//! listener::set_global_volume(50.);
//! ```
//!
//! [`View`]: crate::graphics::View
//!

use std::f32::consts::PI;

use crate::{
    ffi,
    system::{Angle, Vector3f},
};

/// Structure defining the properties of a directional cone
///
/// Sounds will play at gain 1 when they are positioned
/// within the inner angle of the cone. Sounds will play
/// at `outerGain` when they are positioned outside the
/// outer angle of the cone. The gain declines linearly
/// from 1 to `outerGain` as the sound moves from the inner
/// angle to the outer angle.
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cone {
    /// Inner angle, in degrees
    pub inner_angle: Angle,
    /// Outer angle, in degrees
    pub outer_angle: Angle,
    /// Outer gain
    pub outer_gain: f32,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            inner_angle: Angle::degrees(2. * PI),
            outer_angle: Angle::degrees(2. * PI),
            outer_gain: 1.,
        }
    }
}

/// Change the global volume of all the sounds and musics
///
/// The volume is a number between 0 and 100, it is combined with
/// the individual volume of each sound / music.
/// The default value for the volume is 100 (maximum).
///
/// # Arguments
/// * volume - The new global volume, in the range [0, 100]
pub fn set_global_volume(volume: f32) {
    unsafe { ffi::audio::sfListener_setGlobalVolume(volume) }
}

/// Get the current value of the global volume
///
/// Return the current global volume, in the range [0, 100]
#[must_use]
pub fn global_volume() -> f32 {
    unsafe { ffi::audio::sfListener_getGlobalVolume() }
}

/// Set the position of the listener in the scene
///
/// The default listener's position is (0, 0, 0).
///
/// # Arguments
///
/// * position - the New position of the listener
pub fn set_position<P: Into<Vector3f>>(position: P) {
    unsafe { ffi::audio::sfListener_setPosition(position.into()) }
}

/// Get the current position of the listener in the scene
///
/// Return the listener's position
#[must_use]
pub fn position() -> Vector3f {
    unsafe { ffi::audio::sfListener_getPosition() }
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
pub fn set_direction<D: Into<Vector3f>>(direction: D) {
    unsafe { ffi::audio::sfListener_setDirection(direction.into()) }
}

/// Get the current orientation of the listener in the scene
///
/// Return the listener's direction
#[must_use]
pub fn direction() -> Vector3f {
    unsafe { ffi::audio::sfListener_getDirection() }
}

/// Set the upward vector of the listener in the scene.
///
/// The up vector is the vector that points upward from the listener's perspective.
/// Together with the direction, it defines the 3D orientation of the listener in the scene.
/// The up vector doesn't have to be normalized. The default listener's up vector is (0, 1, 0).
/// It is usually not necessary to change it, especially in 2D scenarios.
pub fn set_up_vector(value: Vector3f) {
    unsafe { ffi::audio::sfListener_setUpVector(value) }
}

/// Get the current upward vector of the listener in the scene. (not normalized)
#[must_use]
pub fn up_vector() -> Vector3f {
    unsafe { ffi::audio::sfListener_getUpVector() }
}

/// Get the current forward vector of the listener in the scene
#[must_use]
pub fn velocity() -> Vector3f {
    unsafe { ffi::audio::sfListener_getVelocity() }
}

/// Set the velocity of the listener in the scene
///
/// The default listener's velocity is (0, 0, -1).
pub fn set_velocity(velocity: Vector3f) {
    unsafe { ffi::audio::sfListener_setVelocity(velocity) }
}

/// Set the cone properties of the listener in the audio scene
///
/// The cone defines how directional attenuation is applied.
/// The default cone of a sound is (2 * PI, 2 * PI, 1).
pub fn set_cone(cone: Cone) {
    unsafe { ffi::audio::sfListener_setCone(cone.into()) }
}

/// Get the cone properties of the listener in the audio scene
#[must_use]
pub fn cone() -> Cone {
    unsafe { ffi::audio::sfListener_getCone().into() }
}
