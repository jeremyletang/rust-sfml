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

//! Defines a system event and its parameters
//!
//! Event holds all the informations about a system event that just happened.

use window::keyboard::Key;
use window::mouse::MouseButton;
use window::joystick::Axis;

/// Definition of all the event types
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub enum Event {
    /// The window requested to be closed
    Closed,
    /// The window was resized
    Resized {
        /// The new width of the window
        width: u32,
        /// The new height of the window
        height: u32,
    },
    /// The window lost the focus
    LostFocus,
    /// The window gained the focus
    GainedFocus,
    /// A character was entered
    TextEntered {
        /// The character entered by the user
        code: char,
    },
    /// A key was pressed
    KeyPressed {
        /// The pressed key
        code: Key,
        /// Is alt pressed too?
        alt: bool,
        /// Is ctrl pressed too?
        ctrl: bool,
        /// Is shift pressed too?
        shift: bool,
        /// Is system pressed too?
        system: bool,
    },
    /// A key was released
    KeyReleased {
        /// The released key
        code: Key,
        /// Is alt released too?
        alt: bool,
        /// Is ctrl released too?
        ctrl: bool,
        /// Is shift released too?
        shift: bool,
        /// Is system released too?
        system: bool,
    },
    /// The mouse wheel was scrolled
    MouseWheelMoved {
        /// Number of ticks the wheel has moved (positive is up, negative is down)
        delta: i32,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32,
    },
    /// A mouse button was pressed
    MouseButtonPressed {
        /// Code of the button that has been pressed.
        button: MouseButton,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32,
    },
    /// A mouse button was released
    MouseButtonReleased {
        /// Code of the button that has been pressed.
        button: MouseButton,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32,
    },
    /// The mouse cursor moved
    MouseMoved {
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32,
    },
    /// The mouse cursor entered the area of the window
    MouseEntered,
    /// The mouse cursor left the area of the window
    MouseLeft,
    /// A joystick button was pressed
    JoystickButtonPressed {
        /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32,
        /// Index of the button that has been pressed (in range [0 .. Joystick::button_count - 1])
        button: u32,
    },
    /// A joystick button was released
    JoystickButtonReleased {
        /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32,
        /// Index of the button that has been pressed (in range [0 .. Joystick::button_count - 1])
        button: u32,
    },
    /// The joystick moved along an axis
    JoystickMoved {
        /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32,
        /// Axis on which the joystick moved.
        axis: Axis,
        /// New position on the axis (in range [-100 .. 100])
        position: f32,
    },
    /// A joystick was connected
    JoystickConnected {
        /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32,
    },
    /// A joystick was disconnected
    JoystickDisconnected {
        /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32,
    },
}
