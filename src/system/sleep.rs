use crate::system::Time;

/// Make the current thread sleep for a given duration.
pub fn sleep(time: Time) {
    unsafe { ::csfml_system_sys::sfSleep(time.raw()) }
}
