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
* Defines a system event and its parameters
*
* Event holds all the informations about a system event that just happened.
*
*/

use window::keyboard::Key;
use window::mouse::MouseButton;
use window::joystick::Axis;

/// Definition of all the event types
#[deriving(Clone, Eq, Ord)]
pub enum Event {
    /// The window requested to be closed
    Closed,
    /// The window was resized
    Resized { width : int, height : int },
    /// The window lost the focus
    LostFocus,
    /// The window gained the focus
    GainedFocus,
    /// A character was entered
    TextEntered { code : char },
    /// A key was pressed
    KeyPressed { code : Key, alt : bool, ctrl : bool, shift : bool, system : bool },
    /// A key was released
    KeyReleased { code : Key, alt : bool, ctrl : bool, shift : bool, system : bool },
    /// The mouse wheel was scrolled
    MouseWheelMoved { delta : int, x : int, y : int },
    /// A mouse button was pressed
    MouseButtonPressed { button : MouseButton, x : int, y : int },
    /// A mouse button was released
    MouseButtonReleased { button : MouseButton, x : int, y : int },
    /// The mouse cursor moved
    MouseMoved { x : int, y : int },
    /// The mouse cursor entered the area of the window
    MouseEntered,
    /// The mouse cursor left the area of the window
    MouseLeft,
    /// A joystick button was pressed
    JoystickButtonPressed { joystickid : int, button : int },
    /// A joystick button was released
    JoystickButtonReleased { joystickid : int, button : int },
    /// The joystick moved along an axis
    JoystickMoved { joystickid : uint, axis : Axis, position : f32 },
    /// A joystick was connected
    JoystickConnected { joystickid : uint },
    /// A joystick was disconnected
    JoystickDisconnected { joystickid : uint },
    /// No Event
    NoEvent
}

