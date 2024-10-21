use crate::{ffi::system as ffi, system::Time, IntoSfResult as _, SfBox, SfResult};

decl_opaque! {
    /// Utility type that measures the elapsed time.
    ///
    /// Its provides the most precise time that the underlying OS can
    /// achieve (generally microseconds or nanoseconds).
    /// It also ensures monotonicity, which means that the returned time can never go backward,
    /// even if the system time is changed.
    ///
    /// # Usage example
    /// ```
    /// # use sfml::system::Clock;
    /// let mut clock = Clock::start().unwrap();
    /// // ...
    /// let time1 = clock.elapsed_time();
    /// // ...
    /// let time2 = clock.restart();
    /// ```
    ///
    /// The [`Time`](crate::system::Time) value returned by the clock can then be converted to
    /// a number of seconds, milliseconds or even microseconds.
    pub Clock;
}

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
