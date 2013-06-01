/*!
* Mouse events.
*
* Give access to the real-time state of the mouse.
*
*/

use core::libc::{c_uint};
use window::window::*;
use system::vector2;

/// Mouse buttons
pub enum MouseButton {
    MouseLeft,
    MouseRight,
    MouseMiddle,
    MouseXButton1,
    MouseXButton2
}

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_uint};
    use rsfml::sfTypes::{sfBool};
    use window::window::*;
    use system::vector2;

    pub extern "C" {
        fn sfMouse_isButtonPressed(button : c_uint) -> sfBool;
        fn sfMouse_getPosition(relativeTo : *csfml::sfWindow) -> vector2::csfml::sfVector2i;
        fn sfMouse_setPosition(position : vector2::csfml::sfVector2i, relativeTo : *csfml::sfWindow) -> ();
    }
}

/**
* Check if a mouse button is pressed
*/
pub fn mouse_is_button_pressed(button : MouseButton) -> bool {
    unsafe {
        match csfml::sfMouse_isButtonPressed(button as c_uint) {
            0   => false,
            _   => true
        }
    }
}

/**
*  Get the current position of the mouse
*
* This function returns the current position of the mouse cursor relative to the given window.
*
*/
pub fn mouse_get_position(relativeTo : &Window) -> vector2::Vector2i {
    unsafe {
        vector2::wrap_vector2i(csfml::sfMouse_getPosition(relativeTo.get_sfWindow()))
    }
}

/**
* Set the current position of the mouse
*
* This function sets the current position of the mouse cursor relative to the given window.
*
*/
pub fn mouse_set_position(position : &vector2::Vector2i, relativeTo : &Window) -> () {
    unsafe {
        io::println(fmt!("Pos %d, %d", position.x, position.y));
        csfml::sfMouse_setPosition(vector2::unwrap_vector2i(position), relativeTo.get_sfWindow())
    }
}