/*!
* Example from SFML : Mix borrow / rc CircleShape
*/

#[crate_id = "shape"]
#[desc = "Shape example for rsfml"]
#[crate_type = "bin"]

extern mod native;
extern mod rsfml;

use rsfml::graphics::{RenderWindow, Color, RcCircleShape, CircleShape};
use rsfml::window::{VideoMode, ContextSettings, event, keyboard, Close};
use rsfml::system::Vector2f;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}


fn main () -> () {
    // Create the window of the application
    let setting : ContextSettings = ContextSettings::default();
    let mut window : RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "Use Rc / Borrow CircleShape", Close, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);


    let mut borrow: CircleShape = CircleShape::new().expect("Error, cannot create a borrow CircleShape");
   	borrow.set_fill_color(&Color::red());
    borrow.set_outline_color(&Color::green());
    borrow.set_outline_thickness(3.);
    borrow.set_position(&Vector2f {x: 100., y: 100.});
    borrow.set_radius(50.);


    let mut rc: RcCircleShape = RcCircleShape::new().expect("Error, cannot create an rc CircleShape");
    rc.set_fill_color(&Color::blue());
    rc.set_outline_color(&Color::green());
    rc.set_outline_thickness(3.);
    rc.set_position(&Vector2f {x: 200., y: 200.});
    rc.set_radius(20.);

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
        window.draw(&borrow);
        window.draw(&rc);
        // Display things on screen
        window.display()

    }
}
