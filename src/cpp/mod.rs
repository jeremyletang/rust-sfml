//! Types for interfacing with C++

mod fbox;
mod string;
mod vector;

#[cfg(feature = "graphics")]
pub(crate) use fbox::RawDefault;
pub(crate) use vector::CppVectorItem;
#[cfg(any(feature = "audio", feature = "window"))]
pub(crate) use vector::CppVectorSliceable;
pub use {fbox::FBox, string::CppString, vector::CppVector};
pub(crate) type CppStringVector = CppVector<CppString>;
