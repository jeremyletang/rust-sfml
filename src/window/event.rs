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

use window::keyboard::Key;
use window::mouse::{Button, Wheel};
use window::joystick::Axis;

/// Defines a system event and its parameters.
///
/// `Event` holds all the informations about a system event that just happened.
///
/// Events are retrieved using the
/// `Window::pollEvent`, `Window::waitEvent`, or `Window::events` functions.
///
/// An `Event` instance contains the type of the event
/// (mouse moved, key pressed, window closed, ...) as well as the details about this
/// particular event.
///
/// # Usage example
///
/// ```
/// # use sfml::graphics::RenderWindow;
/// # use sfml::window::{Event, VideoMode, style, Key};
/// # let mut window = RenderWindow::new(VideoMode::new(32, 32, 32),
/// #                                    "test",
/// #                                    style::CLOSE,
/// #                                    &Default::default()).unwrap();
/// # fn do_something_with_the_new_size(_x: u32, _y: u32) {}
/// while let Some(event) = window.poll_event() {
///     match event {
///         Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
///         Event::Resized { width, height } => do_something_with_the_new_size(width, height),
///         _ => { /* Do nothing */ }
///     }
/// }
/// ```
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
        unicode: char,
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
    MouseWheelScrolled {
        /// Which wheel (for mice with multiple ones).
        wheel: Wheel,
        /// Wheel offset (positive is up/left, negative is down/right).
        /// High-precision mice may use non-integral offsets.
        delta: f32,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// X position of the mouse pointer, relative to the left of the owner window.
        y: i32,
    },
    /// A mouse button was pressed
    MouseButtonPressed {
        /// Code of the button that has been pressed.
        button: Button,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32,
    },
    /// A mouse button was released
    MouseButtonReleased {
        /// Code of the button that has been pressed.
        button: Button,
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
    /// A touch event began
    TouchBegan {
        /// Index of the finger in case of multi-touch events.
        finger: u32,
        /// X position of the touch, relative to the left of the owner window.
        x: i32,
        /// Y position of the touch, relative to the top of the owner window.
        y: i32,
    },
    /// A touch moved
    TouchMoved {
        /// Index of the finger in case of multi-touch events.
        finger: u32,
        /// X position of the touch, relative to the left of the owner window.
        x: i32,
        /// Y position of the touch, relative to the top of the owner window.
        y: i32,
    },
    /// A touch event ended
    TouchEnded {
        /// Index of the finger in case of multi-touch events.
        finger: u32,
        /// X position of the touch, relative to the left of the owner window.
        x: i32,
        /// Y position of the touch, relative to the top of the owner window.
        y: i32,
    },
    /// A sensor value changed
    SensorChanged {
        /// Type of the sensor.
        type_: ::window::sensor::Type,
        /// Current value of the sensor on X axis.
        x: f32,
        /// 	Current value of the sensor on Y axis.
        y: f32,
        /// Current value of the sensor on Z axis.
        z: f32,
    },
}
