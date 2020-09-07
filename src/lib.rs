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

#[cfg(feature = "window")]
#[macro_use]
extern crate bitflags;
extern crate csfml_system_sys;
#[cfg(feature = "window")]
extern crate csfml_window_sys;

#[cfg(any(feature = "graphics", feature = "audio"))]
mod inputstream;
mod sf_bool_ext;

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "graphics")]
pub mod graphics;
pub mod system;
#[cfg(feature = "window")]
pub mod window;
