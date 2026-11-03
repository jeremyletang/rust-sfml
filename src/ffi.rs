#![allow(non_camel_case_types)]

macro_rules! decl_opaque {
    ($($(#[$attr:meta])* $v:vis $name:ident;)+) => {
        $(
            $(#[$attr])*
            #[repr(C)]
            #[allow(missing_copy_implementations)]
            $v struct $name {
                _opaque: [u8; 0],
            }
            impl ::std::fmt::Debug for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, concat!(stringify!($name), "(<opaque> @ {:p})"), self)
                }
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

use std::{ffi::c_void, os::raw::c_char};

#[cfg(feature = "window")]
use crate::system::Vector2;
#[cfg(any(feature = "audio", feature = "window"))]
use {
    crate::system::Vector3,
    std::os::raw::{c_int, c_uint},
};

#[cfg(any(feature = "audio", feature = "window"))]
pub type sfVector3f = Vector3<f32>;
#[cfg(feature = "window")]
pub type sfVector2i = Vector2<c_int>;
#[cfg(feature = "window")]
pub type sfVector2u = Vector2<c_uint>;
#[cfg(feature = "graphics")]
pub type sfVector2f = Vector2<f32>;
