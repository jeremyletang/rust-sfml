// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! # rust-sfml
//!
//! Rust bindings for [SFML](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.
//!
//! Prerequisites
//! =============
//!
//! SFML 2.3 and CSFML 2.3 must be installed on your computer. You can download them here:
//!
//! - SFML 2.3: http://www.sfml-dev.org/download.php
//! - CSFML 2.3: http://www.sfml-dev.org/download/csfml/
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
//! use sfml::window::{ContextSettings, VideoMode, Event, window_style};
//! use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable};
//!
//! fn main() {
//!     // Create the window of the application
//!     let mut window = RenderWindow::new(VideoMode::new(800, 600, 32),
//!                                        "SFML Example",
//!                                        window_style::CLOSE,
//!                                        &ContextSettings::default())
//!                          .unwrap();
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
//!         window.clear(&Color::new_rgb(0, 200, 200));
//!         // Draw the shape
//!         window.draw(&circle);
//!         // Display things on screen
//!         window.display()
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

extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate csfml_system_sys;
extern crate csfml_window_sys;
extern crate csfml_graphics_sys;
extern crate csfml_audio_sys;
extern crate csfml_network_sys;

mod inputstream;
mod raw_conv;
mod ext {
    pub mod event;
    pub mod sf_bool_ext;
}

pub mod system;
pub mod window;
pub mod audio;
pub mod graphics;
pub mod network;
