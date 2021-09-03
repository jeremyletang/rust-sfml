use crate::{
    ffi,
    window::{
        joystick::Axis,
        keyboard::Key,
        mouse::{Button, Wheel},
    },
};

/// Defines a system event and its parameters.
///
/// `Event` holds all the informations about a system event that just happened.
///
/// Events are retrieved using the
/// [`Window::poll_event`] or [`Window::wait_event`] functions.
///
/// An `Event` instance contains the type of the event
/// (mouse moved, key pressed, window closed, ...) as well as the details about this
/// particular event.
///
/// # Usage example
///
#[cfg_attr(feature = "ci-headless", doc = "```no_run")]
#[cfg_attr(not(feature = "ci-headless"), doc = "```")]
/// # use sfml::window::{Event, Style, Key, Window};
/// # let mut window = Window::new((32, 32),
/// #                              "test",
/// #                              Style::CLOSE,
/// #                              &Default::default());
/// # fn do_something_with_the_new_size(_x: u32, _y: u32) {}
/// while let Some(event) = window.poll_event() {
///     match event {
///         Event::Closed | Event::KeyPressed { code: Key::ESCAPE, .. } => window.close(),
///         Event::Resized { width, height } => do_something_with_the_new_size(width, height),
///         _ => { /* Do nothing */ }
///     }
/// }
/// ```
///
/// [`Window::poll_event`]: crate::window::Window::poll_event
/// [`Window::wait_event`]: crate::window::Window::wait_event
///
#[derive(Clone, PartialEq, Debug, Copy)]
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
        type_: crate::window::sensor::Type,
        /// Current value of the sensor on X axis.
        x: f32,
        /// Current value of the sensor on Y axis.
        y: f32,
        /// Current value of the sensor on Z axis.
        z: f32,
    },
}

impl Event {
    pub(crate) unsafe fn from_raw(event: &ffi::sfEvent) -> Option<Self> {
        use crate::{sf_bool_ext::SfBoolExt, window::Event::*};

        let type_ = event.type_;

        let evt = match type_ {
            ffi::sfEvtClosed => Closed,
            ffi::sfEvtResized => {
                let e = event.size;

                Resized {
                    width: e.width,
                    height: e.height,
                }
            }
            ffi::sfEvtLostFocus => LostFocus,
            ffi::sfEvtGainedFocus => GainedFocus,
            ffi::sfEvtTextEntered => TextEntered {
                unicode: std::char::from_u32(event.text.unicode)
                    .expect("Invalid unicode encountered on TextEntered event"),
            },
            ffi::sfEvtKeyPressed => {
                let e = event.key;

                KeyPressed {
                    code: Key(e.code),
                    alt: e.alt.into_bool(),
                    ctrl: e.control.into_bool(),
                    shift: e.shift.into_bool(),
                    system: e.system.into_bool(),
                }
            }
            ffi::sfEvtKeyReleased => {
                let e = event.key;

                KeyReleased {
                    code: Key(e.code),
                    alt: e.alt.into_bool(),
                    ctrl: e.control.into_bool(),
                    shift: e.shift.into_bool(),
                    system: e.system.into_bool(),
                }
            }
            ffi::sfEvtMouseWheelScrolled => {
                let e = event.mouseWheelScroll;
                MouseWheelScrolled {
                    wheel: Wheel::from_raw(e.wheel),
                    delta: e.delta,
                    x: e.x,
                    y: e.y,
                }
            }
            ffi::sfEvtMouseButtonPressed => {
                let e = event.mouseButton;

                MouseButtonPressed {
                    button: Button(e.button),
                    x: e.x,
                    y: e.y,
                }
            }
            ffi::sfEvtMouseButtonReleased => {
                let e = event.mouseButton;

                MouseButtonReleased {
                    button: Button(e.button),
                    x: e.x,
                    y: e.y,
                }
            }
            ffi::sfEvtMouseMoved => {
                let e = event.mouseMove;
                MouseMoved { x: e.x, y: e.y }
            }
            ffi::sfEvtMouseEntered => MouseEntered,
            ffi::sfEvtMouseLeft => MouseLeft,
            ffi::sfEvtJoystickButtonPressed => {
                let e = event.joystickButton;

                JoystickButtonPressed {
                    joystickid: e.joystickId,
                    button: e.button,
                }
            }
            ffi::sfEvtJoystickButtonReleased => {
                let e = event.joystickButton;

                JoystickButtonReleased {
                    joystickid: e.joystickId,
                    button: e.button,
                }
            }
            ffi::sfEvtJoystickMoved => {
                let e = event.joystickMove;

                JoystickMoved {
                    joystickid: e.joystickId,
                    axis: Axis(e.axis),
                    position: e.position,
                }
            }
            ffi::sfEvtJoystickConnected => JoystickConnected {
                joystickid: event.joystickConnect.joystickId,
            },
            ffi::sfEvtJoystickDisconnected => JoystickDisconnected {
                joystickid: event.joystickConnect.joystickId,
            },
            ffi::sfEvtTouchBegan => {
                let e = event.touch;

                TouchBegan {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            ffi::sfEvtTouchMoved => {
                let e = event.touch;

                TouchMoved {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            ffi::sfEvtTouchEnded => {
                let e = event.touch;

                TouchEnded {
                    finger: e.finger,
                    x: e.x,
                    y: e.y,
                }
            }
            ffi::sfEvtSensorChanged => {
                let e = event.sensor;

                SensorChanged {
                    type_: crate::window::sensor::Type(e.sensorType),
                    x: e.x,
                    y: e.y,
                    z: e.z,
                }
            }

            // Ignore deprecated events
            _ => panic!("Unhandled event type ({})", type_),
        };
        Some(evt)
    }
}
