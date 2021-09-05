use crate::{
    ffi::{sfBool, sfFalse, sfTrue},
    ResourceLoadError,
};

pub trait SfBoolExt {
    fn into_bool(self) -> bool;
    fn from_bool(src: bool) -> Self;
    fn into_load_result(self) -> Result<(), ResourceLoadError>;
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
    fn into_load_result(self) -> Result<(), ResourceLoadError> {
        if self == sfTrue {
            Ok(())
        } else {
            Err(ResourceLoadError)
        }
    }
}
