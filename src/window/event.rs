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

//! Defines a system event and its parameters
//!
//! Event holds all the informations about a system event that just happened.

pub use self::Event::{Closed, Resized, LostFocus, GainedFocus, TextEntered,
                     KeyPressed, KeyReleased, MouseWheelMoved,
                     MouseButtonPressed, MouseButtonReleased, MouseMoved,
                     MouseEntered, MouseLeft, JoystickButtonPressed,
                     JoystickButtonReleased, JoystickMoved, JoystickConnected,
                     JoystickDisconnected, NoEvent};

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
        height: u32
    },
    /// The window lost the focus
    LostFocus,
    /// The window gained the focus
    GainedFocus,
    /// A character was entered
    TextEntered {
       /// The character entered by the user
        code: char
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
        system: bool
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
        system: bool
    },
    /// The mouse wheel was scrolled
    MouseWheelMoved {
       /// Number of ticks the wheel has moved (positive is up, negative is down)
        delta: i32,
       /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
       /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32
    },
    /// A mouse button was pressed
    MouseButtonPressed {
       /// Code of the button that has been pressed.
        button: MouseButton,
       /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
       /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32
    },
    /// A mouse button was released
    MouseButtonReleased {
       /// Code of the button that has been pressed.
        button: MouseButton,
       /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
       /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32
    },
    /// The mouse cursor moved
    MouseMoved {
       /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
       /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32
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
        button: u32
    },
    /// A joystick button was released
    JoystickButtonReleased {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32,
       /// Index of the button that has been pressed (in range [0 .. Joystick::button_count - 1])
        button: u32
    },
    /// The joystick moved along an axis
    JoystickMoved {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32,
       /// Axis on which the joystick moved.
        axis: Axis,
       /// New position on the axis (in range [-100 .. 100])
        position: f32
    },
    /// A joystick was connected
    JoystickConnected {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32
    },
    /// A joystick was disconnected
    JoystickDisconnected {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        joystickid: u32
    },
    /// No Event
    NoEvent
}

#[doc(hidden)]
#[allow(non_upper_case_globals, non_camel_case_types)]
pub mod raw {
    use csfml_window_sys::*;

    trait sfEventExt {
        fn type_(&mut self) -> *mut sfEventType;
        fn size(&mut self) -> super::Event;
        fn key(&mut self, type_: sfEventType) -> super::Event;
        fn text(&mut self) -> super::Event;
        fn mouse_move(&mut self) -> super::Event;
        fn mouse_button(&mut self, type_: sfEventType) -> super::Event;
        fn mouse_wheel(&mut self) -> super::Event;
        fn joystick_move(&mut self) -> super::Event;
        fn joystick_button(&mut self, type_: sfEventType) -> super::Event;
        fn joystick_connect(&mut self, type_: sfEventType) -> super::Event;
    }

    impl sfEventExt for sfEvent {
        fn type_(&mut self) -> *mut sfEventType {
            unsafe { ::std::mem::transmute(self) }
        }

        fn size(&mut self) -> super::Event {
            let e: *mut sfSizeEvent = unsafe { ::std::mem::transmute(self) };
            unsafe { super::Resized { width: (*e).width, height: (*e).height } }
        }

        fn key(&mut self, type_: sfEventType) -> super::Event {
            let e: *mut sfKeyEvent = unsafe { ::std::mem::transmute(self) };
            let code = unsafe { ::std::mem::transmute((*e).code as i64) };
            let alt = unsafe { (*e).alt.to_bool() };
            let ctrl = unsafe { (*e).control.to_bool() };
            let shift = unsafe { (*e).shift.to_bool() };
            let system = unsafe { (*e).system.to_bool() };
            match type_ {
                sfEvtKeyPressed => {
                    super::KeyPressed {
                        code: code,
                        alt: alt,
                        ctrl: ctrl,
                        shift: shift,
                        system: system
                    }
                },
                sfEvtKeyReleased => {
                    super::KeyReleased {
                        code: code,
                        alt: alt,
                        ctrl: ctrl,
                        shift: shift,
                        system: system
                    }
                },
                _ => unreachable!()
            }
        }

        fn text(&mut self) -> super::Event {
            let e: *mut sfTextEvent = unsafe { ::std::mem::transmute(self) };
            unsafe { super::TextEntered { code: ((*e).unicode as u8) as char } }
        }

        fn mouse_move(&mut self) -> super::Event {
            let e: *mut sfMouseMoveEvent = unsafe { ::std::mem::transmute(self) };
            unsafe { super::MouseMoved {x: (*e).x, y: (*e).y } }
        }

        fn mouse_button(&mut self, type_: sfEventType) -> super::Event {
            let e: *mut sfMouseButtonEvent = unsafe { ::std::mem::transmute(self) };
            let button = unsafe { ::std::mem::transmute((*e).button as u8) };
            let x = unsafe { (*e).x };
            let y = unsafe { (*e).y };

            match type_ {
                sfEvtMouseButtonReleased => super::MouseButtonReleased { button: button, x: x, y: y },
                sfEvtMouseButtonPressed => super::MouseButtonPressed { button: button, x: x, y: y },
                _ => unreachable!()
            }
        }

        fn mouse_wheel(&mut self) -> super::Event {
            let e: *mut sfMouseWheelEvent = unsafe { ::std::mem::transmute(self) };
            unsafe { super::MouseWheelMoved { delta: (*e).delta, x: (*e).x, y: (*e).y } }
        }

        fn joystick_move(&mut self) -> super::Event {
            let e: *mut sfJoystickMoveEvent = unsafe { ::std::mem::transmute(self) };
            super::JoystickMoved {
                joystickid: unsafe { (*e).joystickid },
                axis: unsafe { ::std::mem::transmute((*e).axis as u8) },
                position: unsafe { (*e).position }
            }
        }

        fn joystick_button(&mut self, type_: sfEventType) -> super::Event {
            let e: *mut sfJoystickButtonEvent = unsafe { ::std::mem::transmute(self) };
            let jid = unsafe { (*e).joystickid };
            let btn = unsafe { (*e).button };

            match type_ {
                sfEvtJoystickButtonPressed =>
                    super::JoystickButtonPressed { joystickid: jid, button: btn },
                sfEvtJoystickButtonReleased =>
                    super::JoystickButtonReleased { joystickid: jid, button: btn },
                _ => unreachable!()
            }
        }

        fn joystick_connect(&mut self, type_: sfEventType) -> super::Event {
            let e: *mut sfJoystickConnectEvent = unsafe { ::std::mem::transmute(self) };
            let jid = unsafe { (*e).joystickid };

            match type_ {
                sfEvtJoystickConnected => super::JoystickConnected { joystickid: jid },
                sfEvtJoystickDisconnected => super::JoystickDisconnected { joystickid: jid},
                _ => unreachable!()
            }
        }
    }

    pub fn get_wrapped_event(event: &mut ::csfml_window_sys::sfEvent) -> super::Event {
        let type_ = unsafe { *event.type_() };

        match type_ {
            sfEvtClosed => super::Closed,
            sfEvtResized => event.size(),
            sfEvtLostFocus => super::LostFocus,
            sfEvtGainedFocus => super::GainedFocus,
            sfEvtTextEntered => event.text(),
            sfEvtKeyPressed => event.key(type_),
            sfEvtKeyReleased => event.key(type_),
            sfEvtMouseWheelMoved => event.mouse_wheel(),
            sfEvtMouseButtonPressed => event.mouse_button(type_),
            sfEvtMouseButtonReleased => event.mouse_button(type_),
            sfEvtMouseMoved => event.mouse_move(),
            sfEvtMouseEntered => super::MouseEntered,
            sfEvtMouseLeft => super::MouseLeft,
            sfEvtJoystickButtonPressed => event.joystick_button(type_),
            sfEvtJoystickButtonReleased => event.joystick_button(type_),
            sfEvtJoystickMoved => event.joystick_move(),
            sfEvtJoystickConnected => event.joystick_connect(type_),
            sfEvtJoystickDisconnected => event.joystick_connect(type_),
            _ => super::NoEvent
        }
    }
}
