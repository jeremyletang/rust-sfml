pub use crate::ffi::*;

decl_opaque! {
    pub(crate) sfInputStreamHelper;
}

use crate::{
    cpp::{CppString as sfStdString, CppStringVector as sfStdStringVector},
    system::{Clock as sfClock, SfString as sfString},
};

pub type sfTime = i64;

type sfInputStreamHelperReadCb =
    Option<unsafe extern "C" fn(data: *mut c_void, size: i64, userData: *mut c_void) -> i64>;
type sfInputStreamHelperSeekCb =
    Option<unsafe extern "C" fn(pos: i64, user_data: *mut c_void) -> i64>;
type sfInputStreamHelperTellCb = Option<unsafe extern "C" fn(userData: *mut c_void) -> i64>;
type sfInputStreamHelperGetSizeCb = Option<unsafe extern "C" fn(user_data: *mut c_void) -> i64>;
pub type sfBuffer = crate::system::buffer::Buffer;

include!("system_bindgen.rs");
