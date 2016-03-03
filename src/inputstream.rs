/*
* Rust-SFML - Copyright (c) 2015 Bogdan Cuza.
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

use libc::{c_void, c_longlong};
use std::io::{Read, Seek, SeekFrom};
use std::ptr;
use csfml_system_sys::sfInputStream;

extern fn read<T: Read + Seek>(data: *mut c_void, size: c_longlong, user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data as *mut T) };
    if size == (0 as i64) {
         return 0 as i64;
    } else if size > 0 {
        let mut chunk = stream.take(size as u64);
        let mut buf = vec!();
        let status = chunk.read_to_end(&mut buf);
        if status.is_ok() {
            unsafe { ptr::copy_nonoverlapping(buf.as_ptr(), data as *mut u8, size as usize) };
            return status.unwrap() as i64;
        }
    }
    -1
}

extern fn get_size<T: Read + Seek>(user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data as *mut T) };
    let pos = stream.seek(SeekFrom::Current(0)).unwrap();
    let size = stream.seek(SeekFrom::End(0)).unwrap();
    let _ = stream.seek(SeekFrom::Start(pos));
    size as i64
}

extern fn tell<T: Read + Seek>(user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data as *mut T) };
    stream.seek(SeekFrom::Current(0)).unwrap() as i64
}

extern fn seek<T: Read + Seek>(position: c_longlong, user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = unsafe { &mut *(user_data as *mut T) };
    match stream.seek(SeekFrom::Start(position as u64)) {
        Ok(n) => n as i64,
        Err(_) => -1 as i64
    }
}

#[repr(C)]
pub struct InputStream(pub sfInputStream);

impl InputStream {
    pub fn new<T: Read + Seek>(stream: &mut T) -> Self {
        InputStream(sfInputStream{
            userData: stream as *const _ as *mut c_void,
            read: read::<T>,
            seek: seek::<T>,
            tell: tell::<T>,
            getSize: get_size::<T>
        })
    }
}
