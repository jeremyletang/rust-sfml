rust-sfml [![Build Status](https://api.travis-ci.org/jeremyletang/rust-sfml.png?branch=master)](https://travis-ci.org/jeremyletang/rust-sfml)
=========


SFML bindings for Rust

This is a Rust binding for SFML, the Simple and Fast Multimedia Library, developed by Laurent Gomila.

SFML website : www.sfml-dev.org

Installation
============

You must install the SFML2.1 and CSFML2.1 libraries on your computer which are used for the binding.

SFML2.1: http://www.sfml-dev.org/download/sfml/2.1/

CSFML2.1: http://www.sfml-dev.org/download/csfml/

Then clone the repo and build the library with the following command.

You can use Cargo to build rust-sfml:
```Shell
> cargo build
```

Examples are located under the `examples` directory.
You can run an example with `cargo run --example example_name`

Rust-sfml works on Linux, Windows and OSX.

Short example
=============

Here is a short example, draw a circle shape and display it.

```Rust
extern crate rsfml;

use rsfml::system::Vector2f;
use rsfml::window::{ContextSettings, VideoMode, event, Close};
use rsfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color};

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "SFML Example",
                                       Close,
                                       &ContextSettings::default())
                         .expect("Cannot create a new Render Window.");

    // Create a CircleShape
    let mut circle = CircleShape::new().expect("Error, cannot create ball.");
    circle.set_radius(30.);
    circle.set_fill_color(&Color::red());
    circle.set_position(&Vector2f::new(100., 100.));

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
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

