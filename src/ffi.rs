/* automatically generated by rust-bindgen 0.59.1 */
#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    trivial_casts,
    deref_nullptr
)]

macro_rules! decl_opaque {
    ($($(#[$attr:meta])* $name:ident;)+) => {
        $(
            $(#[$attr])*
            #[repr(C)]
            #[derive(Debug)]
            #[allow(missing_copy_implementations)]
            pub struct $name {
                _opaque: [u8; 0],
            }
        )+
    };
}

pub(crate) mod audio;
pub(crate) mod graphics;
pub(crate) mod system;
pub(crate) mod window;

use std::{
    ffi::c_void,
    fmt::Display,
    os::raw::{c_char, c_int, c_uint},
    str::Utf8Error,
};

use widestring::U32Str;

use crate::sf_box::Dispose;

// Bindgen autogenerated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
