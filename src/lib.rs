/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! # Rust-SFML: __SFML__ bindings for Rust
//!
//! *This library is an in-progress fork of the
//! [original Rust-SFML](https://github.com/jeremyletang/rust-sfml).*
//!
//! This is a Rust binding for [SFML](http://www.sfml-dev.org/), the Simple and
//! Fast Multimedia Library, developed by Laurent Gomila.
//!
//! The binding requires [CSFML 2.2](http://www.sfml-dev.org/download/csfml/) to
//! be already installed somewhere the compiler will find it. Specifically,
//! Rust-SFML requires the window, graphics, and audio components available.
//!
//! The library can be built with Cargo, using the usual `cargo build`.
//! Examples are located under the `examples` directory.
//! You can run an example with `cargo run --example <example_name>`.
//!
//! Rust-SFML supports Linux, Windows, and OSX.
//!
//! # Example
//!
//! This short example opens a window, creates a circle shape, and displays the
//! circle shape to the window until it is closed.
//!
//! ```no_run
//! extern crate sfml;
//!
//! use sfml::system::Vector2f;
//! use sfml::window::{ContextSettings, VideoMode, Event, window_style};
//! use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color};
//! use sfml::graphics::{Shape, Transformable};
//!
//! fn main() {
//!     // Create the window of the application
//!     let mut window = RenderWindow::new(
//!         VideoMode::new(800, 600),
//!         "SFML Example",
//!         window_style::CLOSE,
//!         ContextSettings::default()
//!     ).expect("Failed to create RenderWindow.");
//!
//!     // Create a CircleShape
//!     let mut circle = CircleShape::new().expect("Failed to create CircleShape.");
//!     circle.set_radius(30.);
//!     circle.set_fill_color(Color::red());
//!     circle.set_position2f(100., 100.);
//! 
//!     // Loop until the window is closed
//!     while window.is_open() {
//!         // Handle events
//!         while let Some(event) = window.poll_event() {
//!             match event {
//!                 Event::Closed => window.close(),
//!                 _ => {/* do nothing */}
//!             }
//!         }
//!
//!         // Clear the window
//!         window.clear(Color::new_rgb(0, 200, 200));
//!         // Draw the shape
//!         window.draw(&circle);
//!         // Display things on screen
//!         window.display()
//!     }
//! }
//! ```
//!
//! # License
//!
//! This software is a binding of the SFML library created by Laurent Gomila,
//! which is provided under the Zlib/png license. This binding is itself
//! provided under the same license, the Zlib/png license. See the
//! `LICENSE.txt` file for details.

#![doc(html_logo_url = "http://rust-sfml.org/logo_rsfml.png")]
#![warn(missing_docs)]

extern crate libc;
#[macro_use]
extern crate bitflags;

pub mod system;
pub mod window;
pub mod audio;
pub mod graphics;
mod ffi;
