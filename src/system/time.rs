/*!
* Represents a time value.
*
* Time encapsulates a time value in a flexible way. 
*
*/

pub use core::libc::{c_long, c_float, c_int};

#[doc(hidden)]
pub mod csfml {
    
    pub use core::libc::{c_long, c_float, c_int};

    pub struct sfTime {
        microseconds : c_long
    }
    
    pub extern "C" {
        fn sfTime_asSeconds(time : sfTime) -> c_float;
        fn sfTime_asMilliseconds(time : sfTime) -> c_int;
        fn sfTime_asMicroseconds(time : sfTime) -> c_long;
        fn sfSeconds(amount : c_float) -> sfTime;
        fn sfMilliseconds(amount : c_int) -> sfTime;
        fn sfMicroseconds(amount : c_long) -> sfTime;
    }
}

#[doc(hiddden)]
pub struct Time {
    priv time : csfml::sfTime
}

impl Time {
    
    /**
    * Construct a time value from a number of seconds
    */
    pub fn new_with_seconds(seconds : f32) -> Time {
        Time {time : unsafe {csfml::sfSeconds(seconds as c_float)}}
    }

    /**
    * Construct a time value from a number of milliseconds
    */
    pub fn new_with_milliseconds(milliseconds : i32) -> Time {
        Time {time : unsafe {csfml::sfMilliseconds(milliseconds as c_int)}}
    }

    /**
    * Construct a time value from a number of microseconds
    */
    pub fn new_with_microseconds(microseconds : i64) -> Time {
        Time {time : unsafe {csfml::sfMicroseconds(microseconds as c_long)}}
    }

    /**
    * Return a time value as a number of seconds
    */
    pub fn as_seconds(&self) -> f32 {
        unsafe {
            csfml::sfTime_asSeconds(self.time)
        }
    }

    /**
    * Return a time value as a number of milliseconds
    */
    pub fn as_milliseconds(&self) -> i32 {
        unsafe {
            csfml::sfTime_asMilliseconds(self.time)
        }
    }
    
    /**
    * Return a time value as a number of microseconds
    */
    pub fn as_microseconds(&self) -> i64 {
        unsafe {
            csfml::sfTime_asMicroseconds(self.time)
        }        
    }

    #[doc(hidden)]
    pub fn wrap(time : csfml::sfTime) -> Time {
        Time {time : time}
    }
    #[doc(hidden)]
    pub fn unwrap(&self) -> csfml::sfTime {
        self.time
    }
}