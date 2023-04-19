//! Rust bindings for [SFML](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.
//!
//! Requirements
//! =============
//!
//! - Linux, Windows, or OS X
//! - Rust 1.66 or later
//! - [SFML 2.5](http://www.sfml-dev.org/download.php)
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
//! This software is a binding of the SFML library created by Laurent Gomila, which
//! is provided under the Zlib/png license.
//!
//! This software is provided under the same license than the SFML, the Zlib/png
//! license.
//!

#![warn(
    missing_docs,
    trivial_numeric_casts,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_results,
    trivial_casts,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::mut_mut,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
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
pub use sf_box::{SfBox, SfResource};
use std::{error::Error, fmt::Display};

/// Error when failing to load an SFML resource.
#[derive(Clone, Copy, Debug)]
pub struct ResourceLoadError;

impl Display for ResourceLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to load SFML resource")
    }
}

impl Error for ResourceLoadError {}

/// Result for loading an SFML resource
pub type LoadResult<T> = Result<T, ResourceLoadError>;

trait IntoLoadResult {
    fn into_load_result(self) -> LoadResult<()>;
}

impl IntoLoadResult for bool {
    fn into_load_result(self) -> LoadResult<()> {
        if self {
            Ok(())
        } else {
            Err(ResourceLoadError)
        }
    }
}
