//! # rust-sfml
//!
//! Rust bindings for [SFML](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.
//!
//! Prerequisites
//! =============
//!
//! SFML 2.4 and CSFML 2.4 must be installed on your computer. You can download them here:
//!
//! - SFML 2.4: http://www.sfml-dev.org/download.php
//! - CSFML 2.4: http://www.sfml-dev.org/download/csfml/
//!
//! __Rust-sfml__ works on Linux, windows and OSX.
//!
//! # Short example
//!
//! Here is a short example, draw a circle shape and display it.
//!
//! ```no_run
//! extern crate sfml;
//!
//! use sfml::system::Vector2f;
//! use sfml::window::{ContextSettings, VideoMode, Event, style};
//! use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable};
//!
//! fn main() {
//!     // Create the window of the application
//!     let mut window = RenderWindow::new(VideoMode::new(800, 600, 32),
//!                                        "SFML Example",
//!                                        style::CLOSE,
//!                                        &ContextSettings::default());
//!
//!     // Create a CircleShape
//!     let mut circle = CircleShape::new();
//!     circle.set_radius(30.);
//!     circle.set_fill_color(&Color::red());
//!     circle.set_position(&Vector2f::new(100., 100.));
//!
//!     loop {
//!         // Handle events
//!         for event in window.events() {
//!             if let Event::Closed = event {
//!                 return;
//!             }
//!         }
//!
//!         // Clear the window
//!         window.clear(&Color::rgb(0, 200, 200));
//!         // Draw the shape
//!         window.draw(&circle);
//!         // Display things on screen
//!         window.display();
//!     }
//! }
//!
//! ```
//!
//! # License
//!
//! This software is a binding of the SFML library created by Laurent Gomila, which
//! is provided under the Zlib/png license.
//!
//! This software is provided under the same license than the SFML, the Zlib/png
//! license.
//!

#![warn(missing_docs)]

#[cfg(feature="window")]
#[macro_use]
extern crate bitflags;
extern crate csfml_system_sys;
#[cfg(feature="window")]
extern crate csfml_window_sys;
#[cfg(feature="graphics")]
extern crate csfml_graphics_sys;
#[cfg(feature="audio")]
extern crate csfml_audio_sys;
#[cfg(feature="network")]
extern crate csfml_network_sys;

#[cfg(any(feature="graphics", feature="audio"))]
mod inputstream;
mod ext {
    #[cfg(feature="window")]
    pub mod event;
    pub mod sf_bool_ext;
}
#[cfg(feature="window")]
mod unicode_conv;

pub mod system;
#[cfg(feature="window")]
pub mod window;
#[cfg(feature="audio")]
pub mod audio;
#[cfg(feature="graphics")]
pub mod graphics;
#[cfg(feature="network")]
pub mod network;
