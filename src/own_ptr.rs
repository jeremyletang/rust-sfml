use std::{
    ffi::c_void,
    fmt,
    io::{Read, Seek},
    ptr,
};

pub struct OwnPtr {
    raw: *mut c_void,
    drop: unsafe fn(*mut c_void),
}

impl OwnPtr {
    pub fn new() -> Self {
        Self {
            raw: ptr::null_mut(),
            drop,
        }
    }

    pub fn from_stream<T: Read + Seek>(stream: T) -> Self {
        let raw = Box::into_raw(Box::new(stream)) as *mut c_void;
        let drop = |raw| unsafe {
            let _ = Box::from_raw(raw as *mut T);
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
