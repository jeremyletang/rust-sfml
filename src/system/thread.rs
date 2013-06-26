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

pub use std::libc::{c_void};

pub mod csfml {
    
    pub use std::libc::{c_void};
    
    pub struct sfThread {
        This : *c_void
    }

    pub extern "C" {
        fn sfThread_create(function : *u8, userData : *c_void) -> *sfThread;
        fn sfThread_destroy(thread : *sfThread) -> ();
        fn sfThread_launch(thread : *sfThread) -> ();
        fn sfThread_wait(thread : *sfThread) -> ();
        fn sfThread_terminate(thread : *sfThread) -> ();
    }
}

extern fn threadable_function(userData : *c_void) -> () {
   // let data : Thread = unsafe {cast::transmute(userData as *Thread) };
   // let func : @fn(x : *c_void) =  data.func;
   // func(data.param);
}

pub struct ThreadableStructData<T> {
    func : @fn(x : T) -> (),
    param : T
}

pub struct Thread {
    priv thread : *csfml::sfThread,
    func : @fn(x : *c_void),
    param : *c_void
}

impl Thread {
    pub fn new(func : @fn(x : *c_void), params : *c_void) -> Thread {
        let mut t : Thread = Thread{thread : unsafe {ptr::null()}, func : func, param : params};
        t.thread = unsafe {csfml::sfThread_create(threadable_function, ptr::null())};
        return t;
    }
}
