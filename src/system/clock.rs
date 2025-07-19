use crate::{IntoSfResult as _, SfResult, cpp::FBox, ffi::system as ffi, system::Time};

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
    /// let mut clock = Clock::new().unwrap();
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
    ///
    /// # Usage example
    /// ```
    /// # use sfml::system::Clock;
    /// let mut clock = Clock::new().unwrap();
    /// ```
    #[must_use = "You should use the result of `Clock::new()` to avoid losing the created clock."]
    pub fn new() -> SfResult<FBox<Self>> {
        unsafe { FBox::new(ffi::sfClock_new()) }.into_sf_result()
    }

    /// Gets the elapsed time.
    ///
    /// This function returns the time elapsed since the last call to [`restart`]
    /// (or the construction of the instance if [`restart`] has not been called).
    ///
    /// [`restart`]: Clock::restart
    ///
    /// # Usage example
    /// ```
    /// # use sfml::system::Clock;
    /// let mut clock = Clock::new().unwrap();
    /// clock.start();
    /// # use std::{thread, time::Duration};
    /// # thread::sleep(Duration::from_nanos(1));
    /// assert!(clock.elapsed_time() != Default::default());
    /// ```
    #[must_use]
    pub fn elapsed_time(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_getElapsedTime(self)) }
    }

    /// Check whether the clock is running
    ///
    /// returns true if the clock is running, false otherwise
    ///
    /// # Usage example
    /// ```
    /// # use sfml::system::Clock;
    /// let mut clock = Clock::new().unwrap();
    /// clock.start();
    /// assert!(clock.is_running());
    /// ```
    #[must_use]
    pub fn is_running(&self) -> bool {
        unsafe { ffi::sfClock_isRunning(self) }
    }

    /// Start the clock
    ///
    /// see [`stop`]: `Clock::stop`
    ///
    /// # Usage example
    /// ```
    /// # use sfml::system::Clock;
    /// let mut clock = Clock::new().unwrap();
    /// clock.stop();
    /// assert_eq!(clock.is_running(), false);
    /// clock.start();
    /// assert_eq!(clock.is_running(), true);
    /// ```
    pub fn start(&mut self) {
        unsafe { ffi::sfClock_start(self) }
    }

    /// Stop the clock
    ///
    /// see [`start`]: `Clock::stop`
    ///
    /// # Usage example
    /// ```
    /// # use sfml::system::Clock;
    /// let mut clock = Clock::new().unwrap();
    /// clock.start();
    /// clock.stop();
    /// assert_eq!(clock.is_running(), false);
    /// clock.start();
    /// assert_eq!(clock.is_running(), true);
    /// ```
    pub fn stop(&mut self) {
        unsafe { ffi::sfClock_stop(self) }
    }

    /// Restarts the clock.
    ///
    /// This function puts the time counter back to zero.
    /// It also returns the time elapsed since the clock was started.
    ///
    /// # Usage example
    /// ```
    /// # use sfml::system::Clock;
    /// let mut clock = Clock::new().unwrap();
    /// clock.start();
    /// # use std::{thread, time::Duration};
    /// # thread::sleep(Duration::from_nanos(1));
    /// assert!(clock.restart() != Default::default());
    /// assert_eq!(clock.is_running(), true);
    /// ```
    pub fn restart(&mut self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_restart(self)) }
    }

    /// Resets the clock.
    ///
    /// This function puts the time counter back to zero.
    /// It also returns the time elapsed since the clock was started.
    ///
    /// # Usage example
    /// ```
    /// # use sfml::system::Clock;
    /// let mut clock = Clock::new().unwrap();
    /// clock.start();
    /// # use std::{thread, time::Duration};
    /// # thread::sleep(Duration::from_nanos(1));
    /// assert!(clock.reset() != Default::default());
    /// assert_eq!(clock.is_running(), false);
    /// ```
    pub fn reset(&mut self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_reset(self)) }
    }
}
