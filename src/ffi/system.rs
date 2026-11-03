pub(crate) use crate::ffi::*;

decl_opaque! {
    pub(crate) sfInputStreamHelper;
    pub(crate) sfString;
    pub(crate) sfVersion;
}

impl Drop for sfString {
    fn drop(&mut self) {
        unsafe { sfString_delete(self) }
    }
}

use crate::{
    cpp::{CppString as sfStdString, CppStringVector as sfStdStringVector},
    system::Clock as sfClock,
};

pub type sfTime = i64;

type sfInputStreamHelperReadCb =
    Option<unsafe extern "C" fn(data: *mut c_void, size: i64, user_data: *mut c_void) -> i64>;
type sfInputStreamHelperSeekCb =
    Option<unsafe extern "C" fn(pos: i64, user_data: *mut c_void) -> i64>;
type sfInputStreamHelperTellCb = Option<unsafe extern "C" fn(user_data: *mut c_void) -> i64>;
type sfInputStreamHelperGetSizeCb = Option<unsafe extern "C" fn(user_data: *mut c_void) -> i64>;
#[cfg_attr(not(feature = "graphics"), expect(dead_code))]
pub type sfBuffer = crate::system::buffer::Buffer;

// The generated block declares the whole system C API; only part of it is
// used depending on the enabled features.
#[allow(dead_code)]
mod bindings {
    use super::*;
    include!("system_bindgen.rs");
}
pub(crate) use bindings::*;
