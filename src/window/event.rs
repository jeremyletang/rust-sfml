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


