use crate::inputstream::InputStream;
use std::{
    ffi::c_void,
    fmt,
    io::{Read, Seek},
    ptr,
    mem,
};

pub struct OwnPtr {
    raw: *mut c_void,
    drop: unsafe fn(*mut c_void),
}

#[repr(C)]
struct OwnStreamPtr<T> {
    istream: InputStream,
    data: T,
}

impl OwnPtr {
    pub fn new() -> Self {
        Self {
            raw: ptr::null_mut(),
            drop,
        }
    }

    pub fn from_stream<T: Read + Seek>(stream: T) -> Self {
        let osp = OwnStreamPtr {
            istream: InputStream::from_raw(ptr::null_mut::<T>()),
            data: stream,
        };
        let raw = Box::into_raw(Box::new(osp)) as *mut c_void;
        let istream = unsafe {
            let align = mem::align_of::<InputStream>();
            raw.add(raw.align_offset(align)) as *mut InputStream
        };
        let data = unsafe {
            let mut istream = istream as *mut c_void;
            let (size, align) = (mem::size_of::<InputStream>(), mem::align_of::<T>());
            istream = istream.add(size);
            istream.add(istream.align_offset(align))
        };
        unsafe { (*istream).0.userData = data };
        let drop = |raw| unsafe {
            let _ = Box::from_raw(raw as *mut OwnStreamPtr<T>);
        };
        Self { raw, drop }
    }

    pub fn from_memory(mem: impl Into<Box<[u8]>>) -> (Self, usize) {
        let boxed = mem.into();
        let len = boxed.len();
        let raw = Box::into_raw(boxed) as *mut c_void;
        let drop = |raw| unsafe {
            let _ = Box::from_raw(raw as *mut u8);
        };
        (Self { raw, drop }, len)
    }

    pub fn raw(&self) -> *mut c_void {
        self.raw
    }
}

impl fmt::Debug for OwnPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.raw)
    }
}

impl Drop for OwnPtr {
    fn drop(&mut self) {
        let &mut Self { raw, drop } = self;
        unsafe { drop(raw) }
    }
}
