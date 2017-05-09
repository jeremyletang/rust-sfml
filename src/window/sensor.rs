//! Access to the real-time state of the sensors.
//!
//! `sensor` provides an interface to the state of the various sensors that a device provides.
//!
//! This module allows users to query the sensors values at any time and directly, without having
//! to deal with a window and its events. Compared to the `SensorChanged` event, `sensor` can
//! retrieve the state of a sensor at any time (you don't need to store and update its current
//! value on your side).
//!
//! Depending on the OS and hardware of the device (phone, tablet, ...), some sensor types may
//! not be available. You should always check the availability of a sensor before trying to
//! read it, with the `sensor::is_available` function.
//!
//! You may wonder why some sensor types look so similar, for example `Accelerometer` and
//! `Gravity` / `UserAcceleration`. The first one is the raw measurement of the acceleration,
//! and takes into account both the earth gravity and the user movement. The others are more
//! precise: they provide these components separately, which is usually more useful.
//! In fact they are not direct sensors, they are computed internally based on the raw acceleration
//! and other sensors. This is exactly the same for `Gyroscope` vs `Orientation`.
//!
//! Because sensors consume a non-negligible amount of current, they are all disabled by default.
//! You must call `sensor::set_enabled` for each sensor in which you are interested.
//!
//! # Usage example
//!
//! ```
//! use sfml::window::sensor;
//!
//! if sensor::is_available(sensor::Type::Gravity) {
//!     // enable the gravity sensor
//!     sensor::set_enabled(sensor::Type::Gravity, true);
//!     // get the current value of gravity
//!     let _gravity = sensor::value(sensor::Type::Gravity);
//! }
//! ```

use csfml_window_sys::*;
use sf_bool_ext::SfBoolExt;
use system::Vector3f;

/// Get the current sensor value.
pub fn value(sensor: Type) -> Vector3f {
    unsafe { Vector3f::from_raw(sfSensor_getValue(sensor.raw())) }
}

/// Check if a sensor is available on the underlying platform.
pub fn is_available(sensor: Type) -> bool {
    unsafe { sfSensor_isAvailable(sensor.raw()).to_bool() }
}

/// Enable or disable a sensor.
///
/// All sensors are disabled by default, to avoid consuming too much battery power.
/// Once a sensor is enabled, it starts sending events of the corresponding type.
///
/// This function does nothing if the sensor is unavailable.
pub fn set_enabled(sensor: Type, enabled: bool) {
    unsafe { sfSensor_setEnabled(sensor.raw(), SfBoolExt::from_bool(enabled)) }
}

/// Sensor type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Type {
    /// Measures the raw acceleration (m/s^2)
    Accelerometer = 0,
    /// Measures the raw rotation rates (degrees/s)
    Gyroscope = 1,
    /// Measures the ambient magnetic field (micro-teslas)
    Magnetometer = 2,
    /// Measures the direction and intensity of gravity, independent of device acceleration (m/s^2)
    Gravity = 3,
    /// Measures the direction and intensity of device acceleration,
    /// independent of the gravity (m/s^2)
    UserAcceleration = 4,
    /// Measures the absolute 3D orientation (degrees)
    Orientation = 5,
    /// The total number of sensor types.
    Count = 6,
}

impl Type {
    pub unsafe fn from_raw(raw: sfSensorType) -> Self {
        ::std::mem::transmute(raw)
    }
    pub fn raw(&self) -> sfSensorType {
        unsafe { ::std::mem::transmute(*self) }
    }
}
