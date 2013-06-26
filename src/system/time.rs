/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
* Represents a time value.
*
* Time encapsulates a time value in a flexible way. 
*
*/

pub use std::libc::{c_long, c_float, c_int};

#[doc(hidden)]
pub mod csfml {
    
    pub use std::libc::{c_long, c_float, c_int};

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
    pub fn with_seconds(seconds : f32) -> Time {
        Time {time : unsafe {csfml::sfSeconds(seconds as c_float)}}
    }

    /**
    * Construct a time value from a number of milliseconds
    */
    pub fn with_milliseconds(milliseconds : i32) -> Time {
        Time {time : unsafe {csfml::sfMilliseconds(milliseconds as c_int)}}
    }

    /**
    * Construct a time value from a number of microseconds
    */
    pub fn with_microseconds(microseconds : i64) -> Time {
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