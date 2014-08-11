//! rsfml example use opengl

extern crate rsfml;
extern crate gl;
extern crate native;

use rsfml::window::{Window, ContextSettings, Close, event, VideoMode};

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let setting = ContextSettings {
        depth_bits: 0,
        stencil_bits: 0,
        antialiasing_level: 0,
        major_version: 3,
        minor_version: 2
    };
    let mut window = match Window::new(VideoMode::new_init(800, 600, 32),
                                       "SFML GL Example",
                                       Close,
                                       &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Window.")
    };

    gl::load_with(|cstr| rsfml::gl::get_proc_address(cstr));

    'main: loop {
        for e in window.events() {
            match e {
                event::Closed => { window.close(); break 'main },
                _ => {}
            }
        }

        gl::ClearColor(0.9, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        window.display();
    }
}
