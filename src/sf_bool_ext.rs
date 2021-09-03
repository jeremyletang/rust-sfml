use crate::ffi::{sfBool, sfFalse, sfTrue};

pub trait SfBoolExt {
    fn into_bool(self) -> bool;
    fn from_bool(src: bool) -> Self;
}

impl SfBoolExt for sfBool {
    fn into_bool(self) -> bool {
        self != sfFalse
    }
    fn from_bool(src: bool) -> Self {
        if src {
            sfTrue
        } else {
            sfFalse
        }
    }
}
