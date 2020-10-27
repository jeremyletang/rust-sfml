use csfml_system_sys::{sfBool, sfFalse, sfTrue};

pub trait SfBoolExt {
    fn to_bool(self) -> bool;
    fn from_bool(src: bool) -> Self;
}

impl SfBoolExt for sfBool {
    fn to_bool(self) -> bool {
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
