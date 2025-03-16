use std::ops::Deref;

use crate::ffi::system::{sfBuffer_destroy, sfBuffer_getData, sfBuffer_getSize};

decl_opaque! {
    pub Buffer;
}

impl Deref for Buffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        let data = unsafe { sfBuffer_getData(self) };
        let size = unsafe { sfBuffer_getSize(self) };
        if data.is_null() || size == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(data, size) }
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { sfBuffer_destroy(self) }
    }
}
