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

//! Module for the `Event` type.

use window::keyboard::Key;
use window::mouse::MouseButton;
use window::joystick::Axis;

/// Defines a system event and its parameters.
///
/// Events may be retreived using `poll_event()` and `wait_event()` on the
/// `Window` and `RenderWindow` types.
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Event {
    /// The window requested to be closed.
    Closed,
    /// The window was resized.
    Resized {
        /// The new width of the window.
        width: u32,
        /// The new height of the window.
        height: u32
    },
    /// The window lost the focus.
    LostFocus,
    /// The window gained the focus.
    GainedFocus,
    /// A character was entered.
    TextEntered {
        /// The character entered by the user.
        code: char
    },
    /// A key was pressed.
    KeyPressed {
        /// The pressed key.
        code: Key,
        /// Is the Alt key pressed?
        alt: bool,
        /// Is the Ctrl key pressed?
        ctrl: bool,
        /// Is the Shift key pressed?
        shift: bool,
        /// Is the System key pressed?
        system: bool
    },
    /// A key was released.
    KeyReleased {
        /// The released key.
        code: Key,
        /// Is the Alt key pressed?
        alt: bool,
        /// Is the Ctrl key pressed?
        ctrl: bool,
        /// Is the Shift key pressed?
        shift: bool,
        /// Is the System key pressed?
        system: bool
    },
    /// The mouse wheel was scrolled.
    MouseWheelMoved {
        /// Number of ticks the wheel has moved (positive is up, negative is down).
        delta: i32,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32
    },
    /// A mouse button was pressed.
    MouseButtonPressed {
        /// The mouse button that was pressed.
        button: MouseButton,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32
    },
    /// A mouse button was released.
    MouseButtonReleased {
		/// The mouse button that was released.
        button: MouseButton,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32
    },
    /// The mouse cursor moved.
    MouseMoved {
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32
    },
    /// The mouse cursor entered the area of the window.
    MouseEntered,
    /// The mouse cursor left the area of the window.
    MouseLeft,
    /// A joystick button was pressed.
    JoystickButtonPressed {
        /// Index of the joystick, in range [0, `joystick::COUNT`).
        joystick: u32,
        /// Index of the pressed button, in range [0, `joystick::BUTTON_COUNT`).
        button: u32
    },
    /// A joystick button was released.
    JoystickButtonReleased {
        /// Index of the joystick, in range [0, `joystick::COUNT`).
        joystick: u32,
        /// Index of the released button, in range [0, `joystick::BUTTON_COUNT`).
        button: u32
    },
    /// The joystick moved along an axis.
    JoystickMoved {
        /// Index of the joystick, in range [0, `joystick::COUNT`).
        joystick: u32,
        /// Axis on which the joystick moved.
        axis: Axis,
        /// New position on the axis, in range [-100, 100].
        position: f32
    },
    /// A joystick was connected.
    JoystickConnected {
        /// Index of the joystick, in range [0, `joystick::COUNT`).
        joystick: u32
    },
    /// A joystick was disconnected.
    JoystickDisconnected {
        /// Index of the joystick, in range [0, `joystick::COUNT`).
        joystick: u32
    },
	/*
	/// A touch event began.
	TouchBegan {
		finger: u32,
		x: i32,
		y: i32
	},
	/// A touch moved.
	TouchMoved {
		finger: u32,
		x: i32,
		y: i32
	},
	/// A touch event ended.
	TouchEnded {
		finger: u32,
		x: i32,
		y: i32
	},
	/// A sensor value changed.
	SensorChanged {
		sensor: SensorType,
		x: f32,
		y: f32,
		z: f32
	},*/
}

#[doc(hidden)]
#[allow(non_upper_case_globals, non_camel_case_types)]
pub mod raw {
    use super::Event;
    use ffi::SfBool;
	use libc::{c_int, c_uint};
	use std::mem::transmute;

    type sfKeyCode = c_int;
    type sfMouseButton = c_uint;
    type sfJoystickAxis = c_uint;

    type sfEventType = c_uint;
    const sfEvtClosed: sfEventType = 0;
    const sfEvtResized: sfEventType = 1;
    const sfEvtLostFocus: sfEventType = 2;
    const sfEvtGainedFocus: sfEventType = 3;
    const sfEvtTextEntered: sfEventType = 4;
    const sfEvtKeyPressed: sfEventType = 5;
    const sfEvtKeyReleased: sfEventType = 6;
    const sfEvtMouseWheelMoved: sfEventType = 7;
    const sfEvtMouseButtonPressed: sfEventType = 8;
    const sfEvtMouseButtonReleased: sfEventType = 9;
    const sfEvtMouseMoved: sfEventType = 10;
    const sfEvtMouseEntered: sfEventType = 11;
    const sfEvtMouseLeft: sfEventType = 12;
    const sfEvtJoystickButtonPressed: sfEventType = 13;
    const sfEvtJoystickButtonReleased: sfEventType = 14;
    const sfEvtJoystickMoved: sfEventType = 15;
    const sfEvtJoystickConnected: sfEventType = 16;
    const sfEvtJoystickDisconnected: sfEventType = 17;

    #[repr(C)]
    struct sfKeyEvent {
        _type: sfEventType,
        code: sfKeyCode,
        alt: SfBool,
        control: SfBool,
        shift: SfBool,
        system: SfBool,
    }

    #[repr(C)]
    struct sfTextEvent {
        _type: sfEventType,
        unicode: c_uint,
    }

    #[repr(C)]
    struct sfMouseMoveEvent {
        _type: sfEventType,
        x: c_int,
        y: c_int,
    }

    #[repr(C)]
    struct sfMouseButtonEvent {
        _type: sfEventType,
        button: sfMouseButton,
        x: c_int,
        y: c_int,
    }

    #[repr(C)]
    struct sfMouseWheelEvent {
        _type: sfEventType,
        delta: c_int,
        x: c_int,
        y: c_int,
    }

    #[repr(C)]
    struct sfJoystickMoveEvent {
        _type: sfEventType,
        joystickid: c_uint,
        axis: sfJoystickAxis,
        position: ::libc::c_float,
    }

    #[repr(C)]
    struct sfJoystickButtonEvent {
        _type: sfEventType,
        joystickid: c_uint,
        button: c_uint,
    }

    #[repr(C)]
    struct sfJoystickConnectEvent {
        _type: sfEventType,
        joystickid: c_uint,
    }

    #[repr(C)]
    struct sfSizeEvent {
        _type: sfEventType,
        width: c_uint,
        height: c_uint,
    }
	
	/*#[repr(C)]
	#[derive(Clone, Copy)]
	struct sfTouchEvent {
		_type: sfEventType,
		finger: c_uint,
		x: c_int,
		y: c_int,
	}
	
	#[repr(C)]
	#[derive(Clone, Copy)]
	struct sfSensorEvent {
		_type: sfEventType,
		sensor: sfSensorType,
		x: ::libc::c_float,
		y: ::libc::c_float,
		z: ::libc::c_float,
	}*/

	// Make sure this is always big enough for any event.
    #[repr(C)]
    pub struct sfEvent {
        data: [u32; 6],
    }

	impl sfEvent {
		#[inline(always)]
		pub fn new() -> sfEvent {
			sfEvent { data: [0; 6] }
		}

		// returns None on invalid or unknown-typed event
		pub fn wrap(self) -> Option<Event> {
			unsafe {
				let _type = *transmute::<&sfEvent, &sfEventType>(&self);
				Some(match _type {
					sfEvtClosed => Event::Closed,
					sfEvtLostFocus => Event::LostFocus,
					sfEvtGainedFocus => Event::GainedFocus,
					sfEvtMouseEntered => Event::MouseEntered,
					sfEvtMouseLeft => Event::MouseLeft,
					sfEvtResized => self.size(),
					sfEvtTextEntered => return self.text(),
					sfEvtKeyPressed => self.key(_type),
					sfEvtKeyReleased => self.key(_type),
					sfEvtMouseWheelMoved => self.mouse_wheel(),
					sfEvtMouseButtonPressed => self.mouse_button(_type),
					sfEvtMouseButtonReleased => self.mouse_button(_type),
					sfEvtMouseMoved => self.mouse_move(),
					sfEvtJoystickButtonPressed => self.joystick_button(_type),
					sfEvtJoystickButtonReleased => self.joystick_button(_type),
					sfEvtJoystickMoved => self.joystick_move(),
					sfEvtJoystickConnected => self.joystick_connect(_type),
					sfEvtJoystickDisconnected => self.joystick_connect(_type),
					_ => return None
				})
			}
		}

        unsafe fn size(&self) -> Event {
            let e: &sfSizeEvent = transmute(self);
			Event::Resized { width: e.width, height: e.height }
        }

        unsafe fn key(&self, _type: sfEventType) -> Event {
			let e: &sfKeyEvent = transmute(self);
			let code = transmute(e.code as i64);
			let alt = e.alt.to_bool();
			let ctrl = e.control.to_bool();
			let shift = e.shift.to_bool();
			let system = e.system.to_bool();
            match _type {
                sfEvtKeyPressed => Event::KeyPressed {
					code: code,
					alt: alt,
					ctrl: ctrl,
					shift: shift,
					system: system
				},
                sfEvtKeyReleased => Event::KeyReleased {
					code: code,
					alt: alt,
					ctrl: ctrl,
					shift: shift,
					system: system
				},
                _ => unreachable!()
            }
        }

        unsafe fn text(&self) -> Option<Event> {
            let e: &sfTextEvent = transmute(self);
			::std::char::from_u32(e.unicode).map(|ch| Event::TextEntered { code: ch })
        }

        unsafe fn mouse_move(&self) -> Event {
            let e: &sfMouseMoveEvent = transmute(self);
			Event::MouseMoved { x: e.x, y: e.y }
        }

        unsafe fn mouse_button(&self, _type: sfEventType) -> Event {
            let e: &sfMouseButtonEvent = transmute(self);
			let button = transmute(e.button as u8);
			let (x, y) = (e.x, e.y);
            match _type {
                sfEvtMouseButtonReleased => Event::MouseButtonReleased { button: button, x: x, y: y },
                sfEvtMouseButtonPressed => Event::MouseButtonPressed { button: button, x: x, y: y },
                _ => unreachable!()
            }
        }

        unsafe fn mouse_wheel(&self) -> Event {
            let e: &sfMouseWheelEvent = transmute(self);
            Event::MouseWheelMoved { delta: e.delta, x: e.x, y: e.y }
        }

        unsafe fn joystick_move(&self) -> Event {
            let e: &sfJoystickMoveEvent = transmute(self);
            Event::JoystickMoved {
                joystick: e.joystickid,
                axis: transmute(e.axis as u8),
                position: e.position
            }
        }

        unsafe fn joystick_button(&self, _type: sfEventType) -> Event {
            let e: &sfJoystickButtonEvent = transmute(self);
			let (joy, btn) = (e.joystickid, e.button);
            match _type {
                sfEvtJoystickButtonPressed => Event::JoystickButtonPressed { joystick: joy, button: btn },
                sfEvtJoystickButtonReleased => Event::JoystickButtonReleased { joystick: joy, button: btn },
                _ => unreachable!()
            }
        }

        unsafe fn joystick_connect(&self, _type: sfEventType) -> Event {
            let e: &sfJoystickConnectEvent = transmute(self);
            let joy = e.joystickid;
            match _type {
                sfEvtJoystickConnected => Event::JoystickConnected { joystick: joy },
                sfEvtJoystickDisconnected => Event::JoystickDisconnected { joystick: joy },
                _ => unreachable!()
            }
        }
    }
}
