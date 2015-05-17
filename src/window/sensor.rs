use system::Vector3f;

use ffi::SfBool;
use ffi::window as ffi;

/// The available sensor types.
///
/// Depending on the OS and hardware of the device (phone, tablet, ...), some
/// sensor types may not be available. You should always check the availability
/// of a sensor before trying to read it, with the `is_available()` function.
///
/// You may wonder why some sensor types look so similar, for example
/// Accelerometer and Gravity / UserAcceleration. The first one is the raw
/// measurement of the acceleration, and takes in account both the earth gravity
/// and the user movement. The others are more precise: they provide these
/// components separately, which is usually more useful. In fact they are not
/// direct sensors, they are computed internally based on the raw acceleration
/// and other sensors. This is exactly the same for Gyroscope vs Orientation.
///
/// Because sensors consume a non-negligible amount of current, they are all
/// disabled by default. You must call `set_enabled()` for each sensor in which
/// you are interested.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sensor {
	/// Measures the raw acceleration (m/s<sup>2</sup>).
	Accelerometer,
	/// Measures the raw rotation rates (degrees/s).
	Gyroscope,
	/// Measures the ambient magnetic field (micro-teslas).
	Magnetometer,
	/// Measures the direction and intensity of gravity, independent of device
	/// acceleration (m/s<sup>2</sup>).
	Gravity,
	/// Measures the direction and intensity of device acceleration, independent
	/// of the gravity (m/s<sup>2</sup>).
	UserAcceleration,
	/// Measures the absolute 3D orientation (degrees).
	Orientation,
}

impl Sensor {
	/// Check if a sensor is available on the underlying platform.
	pub fn is_available(self) -> bool {
		unsafe { ffi::sfSensor_isAvailable(self) }.to_bool()
	}

	/// Enable or disable the sensor.
	///
	/// All sensors are disabled by default, to avoid consuming too much
	/// battery power. Once a sensor is enabled, it starts sending events of
	/// the corresponding type.
	///
	/// This function does nothing if the sensor is unavailable.
	pub fn set_enabled(self, enabled: bool) {
		unsafe { ffi::sfSensor_setEnabled(self, SfBool::from_bool(enabled)) }
	}

	/// Get the current sensor value.
	///
	/// Queries the real-time state of the sensor, even if it has updated
	/// while no window was focused and no events were triggered.
	pub fn get_value(self) -> Vector3f {
		unsafe { ffi::sfSensor_getValue(self) }
	}
}
