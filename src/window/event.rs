use csfml_window_sys as ffi;
use window::joystick::Axis;
use window::keyboard::Key;
use window::mouse::{Button, Wheel};

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
/// # use sfml::window::{Event, style, Key, Window};
/// # let mut window = Window::new((32, 32),
/// #                              "test",
/// #                              style::CLOSE,
/// #                              &Default::default());
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

impl Event {
    pub(crate) unsafe fn from_raw(event: &ffi::sfEvent) -> Option<Self> {
        use csfml_window_sys::sfEventType::*;
        use window::Event::*;
        use sf_bool_ext::SfBoolExt;

        let type_ = *event.type_.as_ref();

        let evt = match type_ {
            sfEvtClosed => Closed,
            sfEvtResized => {
                let e = *event.size.as_ref();

                Resized {
                    width: e.width,
                    height: e.height,
                }
            }
            sfEvtLostFocus => LostFocus,
            sfEvtGainedFocus => GainedFocus,
            sfEvtTextEntered => TextEntered {
                unicode: ::std::char::from_u32((*event.text.as_ref()).unicode)
                    .expect("Invalid unicode encountered on TextEntered event"),
            },
            sfEvtKeyPressed => {
                let e = event.key.as_ref();

                KeyPressed {
                    code: ::std::mem::transmute(e.code),
                    alt: e.alt.to_bool(),
                    ctrl: e.control.to_bool(),
                    shift: e.shift.to_bool(),
                    system: e.system.to_bool(),
                }
            }
            sfEvtKeyReleased => {
                let e = event.key.as_ref();

                KeyReleased {
                    code: ::std::mem::transmute(e.code),
                    alt: e.alt.to_bool(),
                    ctrl: e.control.to_bool(),
                    shift: e.shift.to_bool(),
                    system: e.system.to_bool(),
                }
            }
            sfEvtMouseWheelScrolled => {
                let e = event.mouseWheelScroll.as_ref();
                MouseWheelScrolled {
                    wheel: Wheel::from_raw(e.wheel),
                    delta: e.delta,
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtMouseButtonPressed => {
                let e = event.mouseButton.as_ref();

                MouseButtonPressed {
                    button: Button::from_raw(e.button),
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtMouseButtonReleased => {
                let e = event.mouseButton.as_ref();

                MouseButtonReleased {
                    button: Button::from_raw(e.button),
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtMouseMoved => {
                let e = event.mouseMove.as_ref();
                MouseMoved { x: e.x, y: e.y }
            }
            sfEvtMouseEntered => MouseEntered,
            sfEvtMouseLeft => MouseLeft,
            sfEvtJoystickButtonPressed => {
                let e = event.joystickButton.as_ref();

                JoystickButtonPressed {
                    joystickid: (*e).joystickId,
                    button: (*e).button,
                }
            }
            sfEvtJoystickButtonReleased => {
                let e = event.joystickButton.as_ref();

                JoystickButtonReleased {
                    joystickid: (*e).joystickId,
                    button: (*e).button,
                }
            }
            sfEvtJoystickMoved => {
                let e = event.joystickMove.as_ref();

                JoystickMoved {
                    joystickid: e.joystickId,
                    axis: Axis::from_raw(e.axis),
                    position: e.position,
                }
            }
            sfEvtJoystickConnected => JoystickConnected {
                joystickid: (*event.joystickConnect.as_ref()).joystickId,
            },
            sfEvtJoystickDisconnected => JoystickDisconnected {
                joystickid: (*event.joystickConnect.as_ref()).joystickId,
            },
            sfEvtTouchBegan => {
                let e = event.touch.as_ref();

                TouchBegan {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtTouchMoved => {
                let e = event.touch.as_ref();

                TouchMoved {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtTouchEnded => {
                let e = event.touch.as_ref();

                TouchEnded {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            sfEvtSensorChanged => {
                let e = event.sensor.as_ref();

                SensorChanged {
                    type_: ::window::sensor::Type::from_raw(e.sensorType),
                    x: e.x,
                    y: e.y,
                    z: e.z,
                }
            }

            // Ignore deprecated events
            sfEvtMouseWheelMoved => return None,
            sfEvtCount => unreachable!(),
        };
        Some(evt)
    }
}
