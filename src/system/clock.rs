/*!
* Measuring elapsed time
*
* Utility class that measures the elapsed time
*
*/

use system::time;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_void};
    use system::time::*;

    pub struct sfClock {
        This : *c_void
    }

    pub extern "C" {
        fn sfClock_create() -> *sfClock;
        fn sfClock_destroy(clock : *sfClock) -> ();
        fn sfClock_getElapsedTime(clock : *sfClock) -> csfml::sfTime;
        fn sfClock_restart(clock : *sfClock) -> csfml::sfTime;
    }
}


#[doc(hidden)]
pub struct Clock {
    priv clock : *csfml::sfClock
}

pub impl Clock {
    
    /**
    * Create a new Clock and start it.
    */
    pub fn new() -> Clock {
        Clock {clock : unsafe {csfml::sfClock_create()}}
    }

    /**
    * Get the time elapsed in a clock
    */
    pub fn get_elapsed_time(&self) -> time::Time {
        unsafe {
            time::Time::wrap(csfml::sfClock_getElapsedTime(self.clock))
        }
    }

    /**
    * Restart a Clock.
    */
    pub fn restart(&self) -> time::Time {
        unsafe {
            time::Time::wrap(csfml::sfClock_restart(self.clock))
        }
    }
}

impl Drop for Clock {
    
    /**
    * Create a new Clock and start it.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfClock_destroy(self.clock)
        }
    }
}