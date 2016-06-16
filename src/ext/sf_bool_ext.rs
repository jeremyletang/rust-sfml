use csfml_system_sys::{self, sfBool, sfTrue, sfFalse};

pub trait SfBoolExt {
    fn to_bool(self) -> bool;
    fn from_bool(src: bool) -> Self;
}

impl SfBoolExt for sfBool {
    fn to_bool(self) -> bool {
        match self {
            csfml_system_sys::sfFalse => false,
            _ => true,
        }
    }
    fn from_bool(src: bool) -> Self {
        if src {
            sfTrue
        } else {
            sfFalse
        }
    }
}
