//! Types for interfacing with C++

mod fbox;
mod string;
mod vector;

pub use {fbox::FBox, string::CppString, vector::CppVector};
pub(crate) use {fbox::RawDefault, vector::CppVectorItem};
pub(crate) type CppStringVector = CppVector<CppString>;
