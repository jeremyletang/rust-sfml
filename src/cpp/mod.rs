//! Types for interfacing with C++

mod fbox;
mod string;

pub(crate) use fbox::RawDefault;
pub use {
    fbox::FBox,
    string::{CppString, CppStringVector},
};
