/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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

use window::keyboard::*;
use window::mouse::*;
use window::joystick::*;

/// Definition of all the event types
pub enum Event {
    Closed,
    Resized {width : int, height : int},
    LostFocus,
    GainedFocus,
    TextEntered {code : char},
    KeyPressed {code : Key, alt : bool, ctrl : bool, shift : bool, system : bool},
    KeyReleased {code : Key, alt : bool, ctrl : bool, shift : bool, system : bool},
    MouseWheelMoved {delta : int, x : int, y : int},
    MouseButtonPressed {button : MouseButton, x : int, y : int},
    MouseButtonReleased {button : MouseButton, x : int, y : int},
    MouseMoved {x : int, y : int},
    MouseEntered,
    MouseLeft,
    JoystickButtonPressed {joystickid : int, button : int},
    JoystickButtonReleased {joystickid : int, button : int},
    JoystickMoved {joystickid : uint, axis : Axis, position : float},
    JoystickConnected {joystickid : uint},
    JoystickDisconnected{joystickid : uint},
    NoEvent
}


