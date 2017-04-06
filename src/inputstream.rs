use csfml_system_sys::sfInputStream;
use std::io::{Read, Seek, SeekFrom};
use std::os::raw::{c_longlong, c_void};
use std::ptr;

unsafe extern "C" fn read<T: Read + Seek>(data: *mut c_void,
                                          size: c_longlong,
                                          user_data: *mut c_void)
                                          -> c_longlong {
    let stream: &mut T = &mut *(user_data as *mut T);
    if size == 0 {
        return 0;
    } else if size > 0 {
        let mut chunk = stream.take(size as u64);
        let mut buf = vec![];
        let status = chunk.read_to_end(&mut buf);
        if status.is_ok() {
            ptr::copy_nonoverlapping(buf.as_ptr(), data as *mut u8, size as usize);
            return status.unwrap() as i64;
        }
    }
    -1
}

unsafe extern "C" fn get_size<T: Read + Seek>(user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = &mut *(user_data as *mut T);
    let pos = stream.seek(SeekFrom::Current(0)).unwrap();
    let size = stream.seek(SeekFrom::End(0)).unwrap();
    let _ = stream.seek(SeekFrom::Start(pos));
    size as i64
}

unsafe extern "C" fn tell<T: Read + Seek>(user_data: *mut c_void) -> c_longlong {
    let stream: &mut T = &mut *(user_data as *mut T);
    stream.seek(SeekFrom::Current(0)).unwrap() as i64
}

unsafe extern "C" fn seek<T: Read + Seek>(position: c_longlong,
                                          user_data: *mut c_void)
                                          -> c_longlong {
    let stream: &mut T = &mut *(user_data as *mut T);
    match stream.seek(SeekFrom::Start(position as u64)) {
        Ok(n) => n as i64,
        Err(_) => -1,
    }
}

#[repr(C)]
pub struct InputStream(pub sfInputStream);

impl InputStream {
    pub fn new<T: Read + Seek>(stream: &mut T) -> Self {
        let user_data: *const T = stream;
        InputStream(sfInputStream {
                        userData: user_data as *mut c_void,
                        read: Some(read::<T>),
                        seek: Some(seek::<T>),
                        tell: Some(tell::<T>),
                        getSize: Some(get_size::<T>),
                    })
    }
}
