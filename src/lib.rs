//! Rust bindings for [SFML](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.
//!
//! Prerequisites
//! =============
//!
//! - Rust 1.42 or later
//!
//! - SFML 2.5 and CSFML 2.5 must be installed on your computer. You can download them here:
//!
//!     - SFML 2.5: <http://www.sfml-dev.org/download.php>
//!     - CSFML 2.5: <http://www.sfml-dev.org/download/csfml/>
//!
//! - Supported platforms:
//!     - Linux
//!     - Windows
//!     - Mac OS X
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
#[cfg(any(feature = "window", feature = "audio"))]
mod sf_box;
pub mod system;
#[cfg(feature = "window")]
pub mod window;
use std::{error::Error, fmt::Display};

#[cfg(any(feature = "window", feature = "audio"))]
pub use sf_box::{SfBox, SfResource};

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
