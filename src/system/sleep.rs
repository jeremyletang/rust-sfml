/*!
* Make thread sleeping
*/

use system::time::*;

#[doc(hidden)]
pub mod csfml {
    
    use system::time::*;

    pub extern "C" {
        fn sfSleep(duration : csfml::sfTime) -> ();
    }
}

/**
* Make the current thread sleep for a given duration
*/
pub fn sleep(time :Time) -> () {
    unsafe {
        csfml::sfSleep(time.unwrap_time())
    }
}