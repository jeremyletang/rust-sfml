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

#![allow(non_upper_case_globals)]

//! Defines a system event and its parameters
//!
//! Event holds all the informations about a system event that just happened.

use window::keyboard::Key;
use window::mouse::MouseButton;
use window::joystick::Axis;

/// Definition of all the event types
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum Event {
    /// The window requested to be closed
    Closed,
    /// The window was resized
    Resized {
       /// The new width of the window
        pub width: u32,
       /// The new height of the window
        pub height: u32
    },
    /// The window lost the focus
    LostFocus,
    /// The window gained the focus
    GainedFocus,
    /// A character was entered
    TextEntered {
       /// The character entered by the user
        pub code: char
    },
    /// A key was pressed
    KeyPressed {
       /// The pressed key
        pub code: Key,
       /// Is alt pressed too?
        pub alt: bool,
       /// Is ctrl pressed too?
        pub ctrl: bool,
       /// Is shift pressed too?
        pub shift: bool,
       /// Is system pressed too?
        pub system: bool
    },
    /// A key was released
    KeyReleased {
       /// The released key
        pub code: Key,
       /// Is alt released too?
        pub alt: bool,
       /// Is ctrl released too?
        pub ctrl: bool,
       /// Is shift released too?
        pub shift: bool,
       /// Is system released too?
        pub system: bool
    },
    /// The mouse wheel was scrolled
    MouseWheelMoved {
       /// Number of ticks the wheel has moved (positive is up, negative is down)
        pub delta: i32,
       /// X position of the mouse pointer, relative to the left of the owner window.
        pub x: i32,
       /// Y position of the mouse pointer, relative to the top of the owner window.
        pub y: i32
    },
    /// A mouse button was pressed
    MouseButtonPressed {
       /// Code of the button that has been pressed.
        pub button: MouseButton,
       /// X position of the mouse pointer, relative to the left of the owner window.
        pub x: i32,
       /// Y position of the mouse pointer, relative to the top of the owner window.
        pub y: i32
    },
    /// A mouse button was released
    MouseButtonReleased {
       /// Code of the button that has been pressed.
        pub button: MouseButton,
       /// X position of the mouse pointer, relative to the left of the owner window.
        pub x: i32,
       /// Y position of the mouse pointer, relative to the top of the owner window.
        pub y: i32
    },
    /// The mouse cursor moved
    MouseMoved {
       /// X position of the mouse pointer, relative to the left of the owner window.
        pub x: i32,
       /// Y position of the mouse pointer, relative to the top of the owner window.
        pub y: i32
    },
    /// The mouse cursor entered the area of the window
    MouseEntered,
    /// The mouse cursor left the area of the window
    MouseLeft,
    /// A joystick button was pressed
    JoystickButtonPressed {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        pub joystickid: u32,
       /// Index of the button that has been pressed (in range [0 .. Joystick::button_count - 1])
        pub button: u32
    },
    /// A joystick button was released
    JoystickButtonReleased {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        pub joystickid: u32,
       /// Index of the button that has been pressed (in range [0 .. Joystick::button_count - 1])
        pub button: u32
    },
    /// The joystick moved along an axis
    JoystickMoved {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        pub joystickid: u32,
       /// Axis on which the joystick moved.
        pub axis: Axis,
       /// New position on the axis (in range [-100 .. 100])
        pub position: f32
    },
    /// A joystick was connected
    JoystickConnected {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        pub joystickid: u32
    },
    /// A joystick was disconnected
    JoystickDisconnected {
       /// Index of the joystick (in range [0 .. joystick::Count - 1])
        pub joystickid: u32
    },
    /// No Event
    NoEvent
}

#[doc(hidden)]
pub mod raw {

    use ffi::sfml_types::SfBool;

    pub type sfKeyCode = ::libc::c_int;

    pub type sfMouseButton = ::libc::c_uint;
    pub type sfJoystickAxis = ::libc::c_uint;

    pub type sfEventType = ::libc::c_uint;
    pub const sfEvtClosed: ::libc::c_uint = 0;
    pub const sfEvtResized: ::libc::c_uint = 1;
    pub const sfEvtLostFocus: ::libc::c_uint = 2;
    pub const sfEvtGainedFocus: ::libc::c_uint = 3;
    pub const sfEvtTextEntered: ::libc::c_uint = 4;
    pub const sfEvtKeyPressed: ::libc::c_uint = 5;
    pub const sfEvtKeyReleased: ::libc::c_uint = 6;
    pub const sfEvtMouseWheelMoved: ::libc::c_uint = 7;
    pub const sfEvtMouseButtonPressed: ::libc::c_uint = 8;
    pub const sfEvtMouseButtonReleased: ::libc::c_uint = 9;
    pub const sfEvtMouseMoved: ::libc::c_uint = 10;
    pub const sfEvtMouseEntered: ::libc::c_uint = 11;
    pub const sfEvtMouseLeft: ::libc::c_uint = 12;
    pub const sfEvtJoystickButtonPressed: ::libc::c_uint = 13;
    pub const sfEvtJoystickButtonReleased: ::libc::c_uint = 14;
    pub const sfEvtJoystickMoved: ::libc::c_uint = 15;
    pub const sfEvtJoystickConnected: ::libc::c_uint = 16;
    pub const sfEvtJoystickDisconnected: ::libc::c_uint = 17;

    #[repr(C)]
    pub struct sfKeyEvent {
        pub _type: sfEventType,
        pub code: sfKeyCode,
        pub alt: SfBool,
        pub control: SfBool,
        pub shift: SfBool,
        pub system: SfBool,
    }

    #[repr(C)]
    pub struct sfTextEvent {
        pub _type: sfEventType,
        pub unicode: ::libc::c_uint,
    }

    #[repr(C)]
    pub struct sfMouseMoveEvent {
        pub _type: sfEventType,
        pub x: ::libc::c_int,
        pub y: ::libc::c_int,
    }

    #[repr(C)]
    pub struct sfMouseButtonEvent {
        pub _type: sfEventType,
        pub button: sfMouseButton,
        pub x: ::libc::c_int,
        pub y: ::libc::c_int,
    }

    #[repr(C)]
    pub struct sfMouseWheelEvent {
        pub _type: sfEventType,
        pub delta: ::libc::c_int,
        pub x: ::libc::c_int,
        pub y: ::libc::c_int,
    }

    #[repr(C)]
    pub struct sfJoystickMoveEvent {
        pub _type: sfEventType,
        pub joystickid: ::libc::c_uint,
        pub axis: sfJoystickAxis,
        pub position: ::libc::c_float,
    }

    #[repr(C)]
    pub struct sfJoystickButtonEvent {
        pub _type: sfEventType,
        pub joystickid: ::libc::c_uint,
        pub button: ::libc::c_uint,
    }

    #[repr(C)]
    pub struct sfJoystickConnectEvent {
        pub _type: sfEventType,
        pub joystickid: ::libc::c_uint,
    }

    #[repr(C)]
    pub struct sfSizeEvent {
        pub _type: sfEventType,
        pub width: ::libc::c_uint,
        pub height: ::libc::c_uint,
    }

    #[repr(C)]
    pub struct sfEvent {
        pub data: [u32, ..6u],
    }

    impl sfEvent {
        pub fn _type(&mut self) -> *mut sfEventType {
            unsafe { ::std::mem::transmute(self) }
        }

        pub fn size(&mut self) -> super::Event {
            let e: *mut sfSizeEvent = unsafe { ::std::mem::transmute(self) };
            unsafe { super::Resized { width: (*e).width, height: (*e).height } }
        }

        pub fn key(&mut self, _type: sfEventType) -> super::Event {
            let e: *mut sfKeyEvent = unsafe { ::std::mem::transmute(self) };
            let code = unsafe { ::std::mem::transmute((*e).code as i64) };
            let alt = unsafe { (*e).alt.to_bool() };
            let ctrl = unsafe { (*e).control.to_bool() };
            let shift = unsafe { (*e).shift.to_bool() };
            let system = unsafe { (*e).system.to_bool() };
            match _type {
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

        pub fn text(&mut self) -> super::Event {
            let e: *mut sfTextEvent = unsafe { ::std::mem::transmute(self) };
            unsafe { super::TextEntered { code: ((*e).unicode as u8) as char } }
        }

        pub fn mouse_move(&mut self) -> super::Event {
            let e: *mut sfMouseMoveEvent = unsafe { ::std::mem::transmute(self) };
            unsafe { super::MouseMoved {x: (*e).x, y: (*e).y } }
        }

        pub fn mouse_button(&mut self, _type: sfEventType) -> super::Event {
            let e: *mut sfMouseButtonEvent = unsafe { ::std::mem::transmute(self) };
            let button = unsafe { ::std::mem::transmute((*e).button as u8) };
            let x = unsafe { (*e).x };
            let y = unsafe { (*e).y };

            match _type {
                sfEvtMouseButtonReleased => super::MouseButtonReleased { button: button, x: x, y: y },
                sfEvtMouseButtonPressed => super::MouseButtonPressed { button: button, x: x, y: y },
                _ => unreachable!()
            }
        }

        pub fn mouse_wheel(&mut self) -> super::Event {
            let e: *mut sfMouseWheelEvent = unsafe { ::std::mem::transmute(self) };
            unsafe { super::MouseWheelMoved { delta: (*e).delta, x: (*e).x, y: (*e).y } }
        }

        pub fn joystick_move(&mut self) -> super::Event {
            let e: *mut sfJoystickMoveEvent = unsafe { ::std::mem::transmute(self) };
            super::JoystickMoved {
                joystickid: unsafe { (*e).joystickid },
                axis: unsafe { ::std::mem::transmute((*e).axis as u8) },
                position: unsafe { (*e).position }
            }
        }

        pub fn joystick_button(&mut self, _type: sfEventType) -> super::Event {
            let e: *mut sfJoystickButtonEvent = unsafe { ::std::mem::transmute(self) };
            let jid = unsafe { (*e).joystickid };
            let btn = unsafe { (*e).button };

            match _type {
                sfEvtJoystickButtonPressed =>
                    super::JoystickButtonPressed { joystickid: jid, button: btn },
                sfEvtJoystickButtonReleased =>
                    super::JoystickButtonReleased { joystickid: jid, button: btn },
                _ => unreachable!()
            }
        }

        pub fn joystick_connect(&mut self, _type: sfEventType) -> super::Event {
            let e: *mut sfJoystickConnectEvent = unsafe { ::std::mem::transmute(self) };
            let jid = unsafe { (*e).joystickid };

            match _type {
                sfEvtJoystickConnected => super::JoystickConnected { joystickid: jid },
                sfEvtJoystickDisconnected => super::JoystickDisconnected { joystickid: jid},
                _ => unreachable!()
            }
        }
    }

    pub fn get_wrapped_event(event: &mut sfEvent) -> super::Event {
        let _type = unsafe { *event._type() };

        match _type {
            sfEvtClosed => super::Closed,
            sfEvtResized => event.size(),
            sfEvtLostFocus => super::LostFocus,
            sfEvtGainedFocus => super::GainedFocus,
            sfEvtTextEntered => event.text(),
            sfEvtKeyPressed => event.key(_type),
            sfEvtKeyReleased => event.key(_type),
            sfEvtMouseWheelMoved => event.mouse_wheel(),
            sfEvtMouseButtonPressed => event.mouse_button(_type),
            sfEvtMouseButtonReleased => event.mouse_button(_type),
            sfEvtMouseMoved => event.mouse_move(),
            sfEvtMouseEntered => super::MouseLeft,
            sfEvtMouseLeft => super::MouseEntered,
            sfEvtJoystickButtonPressed => event.joystick_button(_type),
            sfEvtJoystickButtonReleased => event.joystick_button(_type),
            sfEvtJoystickMoved => event.joystick_move(),
            sfEvtJoystickConnected => event.joystick_connect(_type),
            sfEvtJoystickDisconnected => event.joystick_connect(_type),
            _ => super::NoEvent
        }
    }
}
