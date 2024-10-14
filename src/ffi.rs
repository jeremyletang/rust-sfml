#![allow(non_camel_case_types)]

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

#[cfg(feature = "audio")]
pub(crate) mod audio;
#[cfg(feature = "graphics")]
pub(crate) mod graphics;
pub(crate) mod system;
#[cfg(any(feature = "window", feature = "graphics"))]
pub(crate) mod window;

use {
    crate::system::{Vector2, Vector3},
    std::{
        ffi::c_void,
        fmt::Display,
        os::raw::{c_char, c_int, c_uint},
        str::Utf8Error,
    },
};

pub type sfVector3f = Vector3<f32>;
pub type sfVector2i = Vector2<c_int>;
pub type sfVector2u = Vector2<c_uint>;
pub type sfVector2f = Vector2<f32>;
