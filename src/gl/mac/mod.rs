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

mod ffi;

/// Load an opengl function pointer
pub fn get_proc_address(proc_name: &str) ->  *const ::libc::c_void {
    // get mac os opengl bundle
    let id = "com.apple.opengl".with_c_str(|c_str| ffi::__CFStringMakeConstantString(c_str));
    let bundle = ffi::CFBundleGetBundleWithIdentifier(id);
    if bundle.is_null() { return ::std::ptr::null(); } // bundle not found

    // find the symbole in the bundle
    let mut symbol: *const ::libc::c_void;
    let cf_proc_name = proc_name.with_c_str(|c_str| ffi::__CFStringMakeConstantString(c_str));
    symbol = ffi::CFBundleGetFunctionPointerForName(bundle, cf_proc_name);

    // release CFStringRef
    ffi::CFRelease(cf_proc_name);

    // the symbol or null if not found
    symbol
}