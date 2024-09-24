//! Rust bindings for [SFML](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.
//!
//! Requirements
//! =============
//!
//! - Linux, Windows, or OS X
//! - Rust 1.66 or later
//! - [SFML 2.6](http://www.sfml-dev.org/download.php)
//! - A C++ compiler for building CSFML
//!
//! Environment variables
//! =============
//! If you get errors about SFML headers not being found, or linker errors, that probably means
//! SFML is not installed in a global location.
//! In that case, you can set two environment variables to help rust-sfml find the required files:
//! - `SFML_INCLUDE_DIR`. Set this to the `include` folder of your SFML location.
//! - `SFML_LIBS_DIR`. Set this to the `lib` folder of your SFML location.
//!
//! Linux users may also have to set this environment variable to help your binary find the
//! shared object files.
//! - `LD_LIBRARY_PATH`. Set this to the `lib` folder of your SFML location.
//!
//! # !! Thread safety warning !!
//!
//! rust-sfml strives to be memory-safe, as a Rust library should be, but currently there is no
//! clear plan on how to solve thread-safety issues. You should be fine as long as you only use
//! SFML on the main thread, but as soon as you try to call into SFML from another thread, you
//! are on your own.
//!
//! # License
//!
//! This software is a binding of the SFML library created by Laurent Gomila,
//! which is provided under the Zlib/png license.
//!
//! This software is provided under the same license as SFML, the Zlib/png license.
//!

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
    clippy::unwrap_used
)]

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
pub use sf_box::{SfBox, SfResource};
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

impl<T: SfResource> IntoSfResult<SfBox<T>> for Option<SfBox<T>> {
    fn into_sf_result(self) -> SfResult<SfBox<T>> {
        self.ok_or(SfError::CallFailed)
    }
}
