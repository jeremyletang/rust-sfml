rust-sfml [![Build Status](https://travis-ci.org/JeremyLetang/rust-sfml.png?branch=master)](https://travis-ci.org/JeremyLetang/rust-sfml)
=========


SFML bindings for Rust

This is a Rust binding for SFML, the Simple and Fast Multimedia Library, developped by Laurent Gomila.

SFML website  : www.sfml-dev.org

Installation
============

You must install on your computer the SFML2.1 and CSFML2.1 libraries who are used for the binding.

SFML2.1 : http://www.sfml-dev.org/download/sfml/2.1/

CSFML2.1 : http://www.sfml-dev.org/download/csfml/

Then clone the repo and build the library with the following command.

Rust-sfml is now build with the rustpkg tool :

```Shell
> rustpkg build rsfml
```

Examples are build too with rustpkg :

```Shell
> rustpkg build examples/pong
```

Rust-sfml works on Linux, windows and OSX.

OSX Specific
============

On OSX window must be launched in the main thread. You should override the rust runtime start function.

```Rust
#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}
```

Short example
=============

Here is a short example, draw a circle shape and display it.

```Rust
extern mod native;
extern mod rsfml;

use rsfml::system::Vector2f;
use rsfml::window::{ContextSettings, VideoMode, event, Close};
use rsfml::graphics::{RenderWindow, CircleShape, Color};

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main () -> () {
     // Create the window of the application
    let setting = ContextSettings::default();
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "SFML Example", Close, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };

    // Create a CircleShape
    let mut circle = match CircleShape::new() {
        Some(circle)    => circle,
        None()          => fail!("Error, cannot create ball")
    };
    circle.set_radius(30.);
    circle.set_fill_color(&Color::red());
    circle.set_position(&Vector2f::new(100., 100.));


    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
                event::NoEvent => break,
                _ => {}
            }
        }

        // Clear the window
        window.clear(&Color::new_RGB(0, 200, 200));
        // Draw the shape
           window.draw(&circle);
        // Display things on screen
        window.display()
    }
}
```


License
=======

This software is a binding of the SFML library created by Laurent Gomila, which is provided under the Zlib/png license.

This software is provided under the same license than the SFML, the Zlib/png license.

