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

use libc::{c_char, c_void};

#[allow(non_snake_case_functions)]
pub fn __CFStringMakeConstantString(c: *const c_char) -> *const c_void {
    unsafe { cf::__CFStringMakeConstantString(c) }
}

#[allow(non_snake_case_functions)]
pub fn CFBundleGetBundleWithIdentifier(ptr: *const c_void) -> *const c_void {
    unsafe { cf::CFBundleGetBundleWithIdentifier(ptr) }
}

#[allow(non_snake_case_functions)]
pub fn CFBundleGetFunctionPointerForName(p1: *const c_void, p2: *const c_void) -> *const c_void {
    unsafe { cf::CFBundleGetFunctionPointerForName(p1, p2) }
}

#[allow(non_snake_case_functions)]
pub fn CFRelease(cfstr: *const c_void) {
    unsafe { cf::CFRelease(cfstr) }
}

mod cf {

    use libc::{c_char, c_void};

    #[link(name = "CoreFoundation", kind = "framework")]
    #[link(name = "Cocoa", kind = "framework")]
    extern "C" {
        pub fn __CFStringMakeConstantString(c: *const c_char) -> *const c_void;
        pub fn CFBundleGetBundleWithIdentifier(ptr: *const c_void) -> *const c_void;
        pub fn CFBundleGetFunctionPointerForName(p1: *const c_void, p2: *const c_void) -> *const c_void;
        pub fn CFRelease(cfstr: *const c_void);
    }
}