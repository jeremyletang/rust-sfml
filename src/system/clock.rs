use csfml_system_sys as ffi;
use system::Time;

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
/// let mut clock = Clock::start();
/// // ...
/// let time1 = clock.elapsed_time();
/// // ...
/// let time2 = clock.restart();
/// ```
///
/// The [`Time`] value returned by the clock can then be converted to
/// a number of seconds, milliseconds or even microseconds.
#[derive(Debug)]
pub struct Clock(*mut ffi::sfClock);

impl Clock {
    /// Creates a new Clock and starts it automatically.
    pub fn start() -> Clock {
        Clock(unsafe { ffi::sfClock_create() })
    }

    /// Gets the elapsed time.
    ///
    /// This function returns the time elapsed since the last call to [`restart`]
    /// (or the construction of the instance if [`restart`] has not been called).
    ///
    /// [`restart`]: Clock::restart
    pub fn elapsed_time(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_getElapsedTime(self.0)) }
    }

    /// Restarts the clock.
    ///
    /// This function puts the time counter back to zero.
    /// It also returns the time elapsed since the clock was started.
    pub fn restart(&mut self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_restart(self.0)) }
    }
}

impl Clone for Clock {
    fn clone(&self) -> Self {
        Clock(unsafe { ffi::sfClock_copy(self.0) })
    }
}

impl Drop for Clock {
    fn drop(&mut self) {
        unsafe { ffi::sfClock_destroy(self.0) }
    }
}

impl Default for Clock {
    /// Equivalent to `Clock::start()`.
    fn default() -> Self {
        Clock::start()
    }
}
