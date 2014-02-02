/*!
* Example from SFML : Shape
*/

#[crate_id = "shape"];
#[desc = "Shape example for rsfml"];
#[crate_type = "bin"];

extern mod native;
extern mod rsfml;

use rsfml::graphics::{RenderWindow, Color, Shape};
use rsfml::window::{VideoMode, ContextSettings, event, keyboard, Close};
use rsfml::traits::shape_impl::ShapeImpl;
use rsfml::system::Vector2f;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}


pub struct CustomShape;

impl ShapeImpl for CustomShape {
	fn get_point_count(&self) -> u32 {
		3
	}

    fn get_point(&self, point : u32) -> Vector2f {
    	match point {
    		0 => Vector2f {x: 10., y: 10.},
    		1 => Vector2f {x: 100., y: 100.},
    		2 => Vector2f {x: 200., y: 300.},
    		_ => fail!("error")
    	}
    }
}

fn main () -> () {
    // Create the window of the application
    let setting : ContextSettings = ContextSettings::default();
    let mut window : RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "SFML Shape Example", Close, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);


    let mut shape = Shape::new(~CustomShape).expect("Error, cannot create a Shape");
   	shape.set_fill_color(&Color::red());
    shape.set_outline_color(&Color::green());
    shape.set_outline_thickness(3.);
    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed 				=> window.close(),
                event::KeyPressed{code, ..} => match code {
                    keyboard::Escape	=> {window.close(); break},
                    _                   => {}
                },
                event::NoEvent 				=> break,
                _ 							=> {}
            }
        }
        // Clear the window
        window.clear(&Color::black());
        window.draw(&shape);
        // Display things on screen
        window.display()

    }
}

