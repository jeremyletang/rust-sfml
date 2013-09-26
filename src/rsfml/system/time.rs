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

use traits::wrappable::Wrappable;

#[doc(hidden)]
pub mod ffi {
    
    pub use std::libc::{c_long, c_float, c_int};

    pub struct sfTime {
        microseconds : c_long
    }
    
    extern "C" {
        pub fn sfTime_asSeconds(time : sfTime) -> c_float;
        pub fn sfTime_asMilliseconds(time : sfTime) -> c_int;
        pub fn sfTime_asMicroseconds(time : sfTime) -> c_long;
        pub fn sfSeconds(amount : c_float) -> sfTime;
        pub fn sfMilliseconds(amount : c_int) -> sfTime;
        pub fn sfMicroseconds(amount : c_long) -> sfTime;
    }
}

pub struct Time {
    priv time : ffi::sfTime
}

impl Time {
    
    /**
    * Construct a time value from a number of seconds
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn with_seconds(seconds : f32) -> Time {
        Time {
            time : unsafe { ffi::sfSeconds(seconds as c_float) }
        }
    }

    /**
    * Construct a time value from a number of milliseconds
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn with_milliseconds(milliseconds : i32) -> Time {
        Time {
            time : unsafe { ffi::sfMilliseconds(milliseconds as c_int) }
        }
    }

    /**
    * Construct a time value from a number of microseconds
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn with_microseconds(microseconds : i64) -> Time {
        Time {
            time : unsafe { ffi::sfMicroseconds(microseconds as c_long) }
        }
    }

    /**
    * Return a time value as a number of seconds
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn as_seconds(&self) -> f32 {
        unsafe {
            ffi::sfTime_asSeconds(self.time)
        }
    }

    /**
    * Return a time value as a number of milliseconds
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn as_milliseconds(&self) -> i32 {
        unsafe {
            ffi::sfTime_asMilliseconds(self.time)
        }
    }
    
    /**
    * Return a time value as a number of microseconds
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn as_microseconds(&self) -> i64 {
        unsafe {
            ffi::sfTime_asMicroseconds(self.time)
        }        
    }
}

impl Eq for Time {
    fn eq(&self, other : &Time) -> bool {
        self.as_microseconds() == other.as_microseconds()
    }
    fn ne(&self, other : &Time) -> bool {
        self.as_microseconds() != other.as_microseconds() 
    }
}

impl Ord for Time {
    fn lt(&self, other: &Time) -> bool {
        self.as_microseconds() < other.as_microseconds()
    }
    fn le(&self, other: &Time) -> bool {
        self.as_microseconds() <= other.as_microseconds()
    }
    fn gt(&self, other: &Time) -> bool {
        self.as_microseconds() > other.as_microseconds()
    }
    fn ge(&self, other: &Time) -> bool {
        self.as_microseconds() >= other.as_microseconds()
    }
}

impl Add<Time, Time> for Time {
    fn add(&self, other : &Time) -> Time {
         Time::with_microseconds(self.as_microseconds() + other.as_microseconds())
    }
}

impl Sub<Time, Time> for Time {
    fn sub(&self, other : &Time) -> Time {
         Time::with_microseconds(self.as_microseconds() - other.as_microseconds())
    }
}

impl Mul<Time, Time> for Time {
    fn mul(&self, other : &Time) -> Time {
         Time::with_microseconds(self.as_microseconds() * other.as_microseconds())
    }
}

impl Div<Time, Time> for Time {
    fn div(&self, other : &Time) -> Time {
         Time::with_microseconds(self.as_microseconds() / other.as_microseconds())
    }
}

#[doc(hidden)]
impl Wrappable<ffi::sfTime> for Time {
    fn wrap(time : ffi::sfTime) -> Time {
        Time {
            time : time
        }
    }
 
    fn unwrap(&self) -> ffi::sfTime {
        self.time
    }
}
