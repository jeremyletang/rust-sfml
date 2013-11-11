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

/*!
# rust-sfml

__SFML__ bindings for Rust

This is a Rust binding for __SFML__, the Simple and Fast Multimedia Library, developped by Laurent Gomila.

__SFML__ website  : www.sfml-dev.org

# Installation

You must install on your computer the SFML2 and CSFML2 libraries who are used for the binding.

SFML2 : http://www.sfml-dev.org/download/sfml/2.0/

CSFML2 : http://www.sfml-dev.org/download/csfml/

Then clone the repo and build the library with the following command.

__rust-sfml__ is build with the rustpkg tool :

```Shell
> rustpkg build rsfml
```

Examples are build too with rustpkg :

```Shell
> rustpkg build examples/pong
```

__rust-sfml__ works on Linux, windows and OSX.

# OSX Specific

On OSX window must be launched in the main thread. You should override the rust runtime start function.

```Rust
#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}
```

# Short example

Here is a short example, draw a circle shape and display it.

```Rust
extern mod rsfml;

use rsfml::system::Vector2f;
use rsfml::window::{ContextSettings, VideoMode, Event};
use rsfml::graphics::{RenderWindow, sfClose, CircleShape, Color};

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main () -> () {
     // Create the window of the application
    let setting = ContextSettings::default();
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), ~"SFML Example", sfClose, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };

    // Create a CircleShape
    let mut circle = match CircleShape::new() {
        Some(circle)    => circle,
        None()          => fail!("Error, cannot create ball")
    };
    circle.set_radius(30.);
    circle.set_fill_color(~Color::red());
    circle.set_position(~Vector2f::new(100., 100.));


    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
                event::NoEvent => break,
                _ => {}
            }
        }

        // Clear the window
        window.clear(~Color::new_RGB(0, 200, 200));
        // Draw the shape
           window.draw(&circle);
        // Display things on screen
        window.display()
    }
}
```

# License

This software is a binding of the SFML library created by Laurent Gomila, which is provide under the Zlib/png license.

This software is provide under the same license than the SFML, the Zlib/png license.

# Modules

Here is a list of all modules :
*/

#[feature(globs, managed_boxes, struct_variant)];

#[link(name = "rsfml",
       vers = "0.2",
       package_id = "github.com/JeremyLetang/rust-sfml",
       author = "letang.jeremy@gmail.com",
       uuid = "4F3334F2-A32B-4460-A63A-9B56C98D1D78",
       url = "http://https://github.com/JeremyLetang/rust-sfml")];

#[desc = "Rust binding for sfml"];
#[license = "Zlib/png"];
#[crate_type = "lib"];

extern mod extra;

pub mod traits;
pub mod system;
pub mod window;
pub mod audio;
pub mod graphics;
pub mod network;
#[doc(hidden)]

#[feature(globs)]
mod sfml_types;