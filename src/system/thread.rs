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

use std::libc::{c_void};

use std::io;
use std::cast;
use std::borrow;

pub mod csfml {
    
    use std::libc::{c_void};
    use system::thread;

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

pub trait threadObject {
    pub fn execute(&self);
}

struct test;

impl test {
    
    pub fn new() -> test {
        test
    }
    

}

impl threadObject for test {
    pub fn execute(&self) -> () {
        io::println("Hello world");
    }
}

extern fn threadable_function<T : threadObject>(userData : &T) -> () {
    userData.execute();
}

pub struct Thread {
    priv thread : *csfml::sfThread
}
impl Thread {
    pub fn new<T : threadObject>(params : &T) -> Thread {
        let t = unsafe {csfml::sfThread_create(threadable_function, cast::transmute::<*T, *c_void>(params))};
        Thread {thread : t}
    }

    pub fn launch(&self) -> () {
        unsafe {
            csfml::sfThread_launch(self.thread)
        }
    }
}