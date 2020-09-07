use csfml_system_sys::{self, sfBool, sfFalse, sfTrue};

pub trait SfBoolExt {
    fn to_bool(self) -> bool;
    fn from_bool(src: bool) -> Self;
}

impl SfBoolExt for sfBool {
    fn to_bool(self) -> bool {
        !matches!(self, csfml_system_sys::sfFalse)
    }
    fn from_bool(src: bool) -> Self {
        if src {
            sfTrue
        } else {
            sfFalse
        }
    }
}
