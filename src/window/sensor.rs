//! Access to the real-time state of the sensors.
//!
//! `sensor` provides an interface to the state of the various sensors that a device provides.
//!
//! This module allows users to query the sensors values at any time and directly, without having
//! to deal with a window and its events. Compared to the [`SensorChanged`] event, `sensor` can
//! retrieve the state of a sensor at any time (you don't need to store and update its current
//! value on your side).
//!
//! [`SensorChanged`]: crate::window::Event::SensorChanged
//!
//! Depending on the OS and hardware of the device (phone, tablet, ...), some sensor types may
//! not be available. You should always check the availability of a sensor before trying to
//! read it, with the [`is_available`] function.
//!
//! You may wonder why some sensor types look so similar, for example [`ACCELEROMETER`] and
//! [`GRAVITY`] / [`USER_ACCELERATION`].
//! The first one is the raw measurement of the acceleration,
//! and takes into account both the earth gravity and the user movement. The others are more
//! precise: they provide these components separately, which is usually more useful.
//! In fact they are not direct sensors, they are computed internally based on the raw acceleration
//! and other sensors. This is exactly the same for [`GYROSCOPE`] vs [`ORIENTATION`].
//!
//! Because sensors consume a non-negligible amount of current, they are all disabled by default.
//! You must call [`set_enabled`] for each sensor in which you are interested.
//!
//! # Usage example
//!
//! ```
//! use sfml::window::sensor;
//!
//! if sensor::is_available(sensor::Type::GRAVITY) {
//!     // enable the gravity sensor
//!     sensor::set_enabled(sensor::Type::GRAVITY, true);
//!     // get the current value of gravity
//!     let _gravity = sensor::value(sensor::Type::GRAVITY);
//! }
//! ```
//!
//! [`is_available`]: is_available
//! [`set_enabled`]: set_enabled
//! [`ACCELEROMETER`]: Type::ACCELEROMETER
//! [`GRAVITY`]: Type::GRAVITY
//! [`USER_ACCELERATION`]: Type::USER_ACCELERATION
//! [`GYROSCOPE`]: Type::GYROSCOPE
//! [`ORIENTATION`]: Type::ORIENTATION
//!

use crate::{ffi::window::*, system::Vector3f};

/// Get the current sensor value.
#[must_use]
pub fn value(sensor: Type) -> Vector3f {
    unsafe { Vector3f::from_raw(sfSensor_getValue(sensor.0)) }
}

/// Check if a sensor is available on the underlying platform.
#[must_use]
pub fn is_available(sensor: Type) -> bool {
    unsafe { sfSensor_isAvailable(sensor.0) }
}

/// Enable or disable a sensor.
///
/// All sensors are disabled by default, to avoid consuming too much battery power.
/// Once a sensor is enabled, it starts sending events of the corresponding type.
///
/// This function does nothing if the sensor is unavailable.
pub fn set_enabled(sensor: Type, enabled: bool) {
    unsafe { sfSensor_setEnabled(sensor.0, enabled) }
}

/// Sensor type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Type(pub(super) sfSensorType);

impl Type {
    /// Measures the raw acceleration (`m/s^2`)
    pub const ACCELEROMETER: Self = Self(sfSensorType::Accelerometer);
    /// Measures the raw rotation rates (degrees/s)
    pub const GYROSCOPE: Self = Self(sfSensorType::Gyroscope);
    /// Measures the ambient magnetic field (micro-teslas)
    pub const MAGNETOMETER: Self = Self(sfSensorType::Magnetometer);
    /// Measures the direction and intensity of gravity,
    /// independent of device acceleration (`m/s^2`)
    pub const GRAVITY: Self = Self(sfSensorType::Gravity);
    /// Measures the direction and intensity of device acceleration,
    /// independent of the gravity (`m/s^2`)
    pub const USER_ACCELERATION: Self = Self(sfSensorType::UserAcceleration);
    /// Measures the absolute 3D orientation (degrees)
    pub const ORIENTATION: Self = Self(sfSensorType::Orientation);
    /// The total number of sensor types.
    pub const COUNT: sfSensorType = sfSensorType::Count;
}
