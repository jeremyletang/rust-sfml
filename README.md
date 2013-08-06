rust-sfml
=========

SFML bindings for Rust

This is a Rust binding for SFML, the Simple and Fast Multimedia Library, developped by Laurent Gomila.

SFML website  : www.sfml-dev.org

Installation
============

You must install on your computer the SFML2 and CSFML2 libraries who are used for the binding.

SFML2 : http://www.sfml-dev.org/download/sfml/2.0/

CSFML2 : http://www.sfml-dev.org/download/csfml/

Then clone the repo and build the library with the following command.

```Shell
> rustc rsfml.rc
>
```
Finally build your program with the rust-sfml library. For exemple, if the library is in the same path than your program.

```Shell
> rustc main.rs -L .
>
```

Normaly rust-sfml works on Linux and windows and OSX.

OSX Specific
============

On OSX window must be launched in the main thread. You should override the rust runtime start function.

```Rust
#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}
```

Short example
=============

Here is a short example, draw a circle shape and display it.

```Rust
fn main () -> () {
     // Create the window of the application
    let setting = ContextSettings{
        depthBits: 10,
        stencilBits: 10, 
        antialiasingLevel: 1, 
        majorVersion: 0, 
        minorVersion: 1
    };
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32), ~"SFML Pong", sfClose, &setting) {
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
        window.clear(~Color::new_from_RGB(0, 200, 200));
        // Draw the shape
		window.draw(&circle);
        // Display things on screen
        window.display()
    }
}
```


License
=======

This software is a binding of the SFML library created by Laurent Gomila, which is provide under the Zlib/png license.

This software is provide under the same license than the SFML, the Zlib/png license.

