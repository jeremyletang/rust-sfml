#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    trivial_numeric_casts,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_results,
    trivial_casts,
    unsafe_op_in_unsafe_fn,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::mut_mut,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::unwrap_used,
    clippy::unreadable_literal,
    clippy::ptr_as_ptr,
    clippy::cast_lossless
)]

extern crate link_cplusplus;

#[macro_use]
mod ffi;

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "graphics")]
pub mod graphics;
mod sf_box;
pub mod system;
#[cfg(feature = "window")]
pub mod window;
pub use sf_box::SfBox;
use std::{
    error::Error,
    ffi::{CString, NulError},
    fmt::Display,
};

/// An SFML operation has failed
#[derive(Clone, Copy, Debug)]
pub enum SfError {
    /// An string argument passed had interior nul bytes
    NulInStr,
    /// Call to SFML function returned an error
    CallFailed,
}

impl Display for SfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SfError::NulInStr => write!(f, "Interior nul byte in string passed to SFML"),
            SfError::CallFailed => write!(f, "Call to SFML function returned an error"),
        }
    }
}

impl Error for SfError {}

/// Result of a fallible SFML operation
pub type SfResult<T> = Result<T, SfError>;

trait IntoSfResult<T> {
    fn into_sf_result(self) -> SfResult<T>;
}

impl IntoSfResult<()> for bool {
    fn into_sf_result(self) -> SfResult<()> {
        if self {
            Ok(())
        } else {
            Err(SfError::CallFailed)
        }
    }
}

impl IntoSfResult<CString> for Result<CString, NulError> {
    fn into_sf_result(self) -> SfResult<CString> {
        self.map_err(|_| SfError::NulInStr)
    }
}

impl<T> IntoSfResult<SfBox<T>> for Option<SfBox<T>> {
    fn into_sf_result(self) -> SfResult<SfBox<T>> {
        self.ok_or(SfError::CallFailed)
    }
}
