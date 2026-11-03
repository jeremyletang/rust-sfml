use crate::system::Time;

/// Make the current thread sleep for a given duration.
///
/// # Examples
///
/// ```
/// # use sfml::system::{sleep, Time};
/// sleep(Time::ZERO);
/// sleep(Time::milliseconds(1));
/// ```
pub fn sleep(time: Time) {
    unsafe { crate::ffi::system::sfSleep(time.raw()) }
}
