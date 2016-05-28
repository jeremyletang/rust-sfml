use csfml_system_sys::{sfBool, sfTrue, sfFalse};

pub trait SfBoolExt {
    fn to_bool(self) -> bool;
    fn from_bool(src: bool) -> Self;
}

impl SfBoolExt for sfBool {
    #![allow(non_upper_case_globals)]
    fn to_bool(self) -> bool {
        match self {
            sfFalse => false,
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
