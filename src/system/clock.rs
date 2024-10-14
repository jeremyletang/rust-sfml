use crate::{ffi::system as ffi, system::Time, IntoSfResult as _, SfBox, SfResult};
pub use ffi::sfClock as Clock;

impl Drop for Clock {
    fn drop(&mut self) {
        unsafe {
            ffi::sfClock_delete(self);
        }
    }
}

impl Clock {
    /// Creates a new Clock and starts it automatically.
    pub fn start() -> SfResult<SfBox<Self>> {
        unsafe { SfBox::new(ffi::sfClock_new()) }.into_sf_result()
    }

    /// Gets the elapsed time.
    ///
    /// This function returns the time elapsed since the last call to [`restart`]
    /// (or the construction of the instance if [`restart`] has not been called).
    ///
    /// [`restart`]: Clock::restart
    #[must_use]
    pub fn elapsed_time(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_getElapsedTime(self)) }
    }

    /// Restarts the clock.
    ///
    /// This function puts the time counter back to zero.
    /// It also returns the time elapsed since the clock was started.
    pub fn restart(&mut self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_restart(self)) }
    }
}
