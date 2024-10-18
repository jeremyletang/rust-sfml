use crate::ffi::{window as ffi, window::EventType};

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
/// # let mut window = Window::new_open((32, 32),
/// #                                   "test",
/// #                                   Style::CLOSE,
/// #                                   &Default::default()).unwrap();
/// # fn do_something_with_the_new_size(_x: u32, _y: u32) {}
/// while let Some(event) = window.poll_event() {
///     match event {
///         Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
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
        code: ffi::Key,
        /// The scancode of the pressed key
        scan: ffi::Scancode,
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
        code: ffi::Key,
        /// The scancode of the released key
        scan: ffi::Scancode,
        /// Is alt released too?
        alt: bool,
        /// Is ctrl released too?
        ctrl: bool,
        /// Is shift released too?
        shift: bool,
        /// Is system released too?
        system: bool,
    },
    #[doc(hidden)]
    /// Do not use. Needed for compatibility with SFML.
    MouseWheelMoved,
    /// The mouse wheel was scrolled
    MouseWheelScrolled {
        /// Which wheel (for mice with multiple ones).
        wheel: ffi::MouseWheel,
        /// Wheel offset (positive is up/left, negative is down/right).
        /// High-precision mice may use non-integral offsets.
        delta: f32,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32,
    },
    /// A mouse button was pressed
    MouseButtonPressed {
        /// Code of the button that has been pressed.
        button: ffi::MouseButton,
        /// X position of the mouse pointer, relative to the left of the owner window.
        x: i32,
        /// Y position of the mouse pointer, relative to the top of the owner window.
        y: i32,
    },
    /// A mouse button was released
    MouseButtonReleased {
        /// Code of the button that has been pressed.
        button: ffi::MouseButton,
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
        /// Index of the joystick (in range `0 .. `[`joystick::COUNT`]` - 1`)
        ///
        /// [`joystick::COUNT`]: crate::window::joystick::COUNT
        joystickid: u32,
        /// Index of the button that has been pressed (in range `0 .. `[`joystick::BUTTON_COUNT`]` - 1`)
        ///
        /// [`joystick::BUTTON_COUNT`]: crate::window::joystick::BUTTON_COUNT
        button: u32,
    },
    /// A joystick button was released
    JoystickButtonReleased {
        /// Index of the joystick (in range `0 .. `[`joystick::COUNT`]` - 1`)
        ///
        /// [`joystick::COUNT`]: crate::window::joystick::COUNT
        joystickid: u32,
        /// Index of the button that has been pressed (in range `0 .. `[`joystick::BUTTON_COUNT`]` - 1`)
        ///
        /// [`joystick::BUTTON_COUNT`]: crate::window::joystick::BUTTON_COUNT
        button: u32,
    },
    /// The joystick moved along an axis
    JoystickMoved {
        /// Index of the joystick (in range `0 .. `[`joystick::COUNT`]` - 1`)
        ///
        /// [`joystick::COUNT`]: crate::window::joystick::COUNT
        joystickid: u32,
        /// Axis on which the joystick moved.
        axis: crate::window::joystick::Axis,
        /// New position on the axis (in range [-100 .. 100])
        position: f32,
    },
    /// A joystick was connected
    JoystickConnected {
        /// Index of the joystick (in range `0 .. `[`joystick::COUNT`]` - 1`)
        ///
        /// [`joystick::COUNT`]: crate::window::joystick::COUNT
        joystickid: u32,
    },
    /// A joystick was disconnected
    JoystickDisconnected {
        /// Index of the joystick (in range `0 .. `[`joystick::COUNT`]` - 1`)
        ///
        /// [`joystick::COUNT`]: crate::window::joystick::COUNT
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
        type_: ffi::window::sfSensorType,
        /// Current value of the sensor on X axis.
        x: f32,
        /// Current value of the sensor on Y axis.
        y: f32,
        /// Current value of the sensor on Z axis.
        z: f32,
    },
}

impl Event {
    pub(crate) unsafe fn from_raw(event: &ffi::Event) -> Option<Self> {
        use crate::window::Event::*;

        let evt = match event.type_ {
            EventType::Closed => Closed,
            EventType::Resized => {
                let e = unsafe { event.union.size };

                Resized {
                    width: e.width,
                    height: e.height,
                }
            }
            EventType::LostFocus => LostFocus,
            EventType::GainedFocus => GainedFocus,
            EventType::TextEntered => TextEntered {
                unicode: std::char::from_u32(unsafe { event.union.text.unicode })
                    .expect("Invalid unicode encountered on TextEntered event"),
            },
            EventType::KeyPressed => KeyPressed {
                code: unsafe { event.union.key.code },
                scan: unsafe { event.union.key.scan },
                alt: unsafe { event.union.key.alt },
                ctrl: unsafe { event.union.key.control },
                shift: unsafe { event.union.key.shift },
                system: unsafe { event.union.key.system },
            },
            EventType::KeyReleased => KeyReleased {
                code: unsafe { event.union.key.code },
                scan: unsafe { event.union.key.scan },
                alt: unsafe { event.union.key.alt },
                ctrl: unsafe { event.union.key.control },
                shift: unsafe { event.union.key.shift },
                system: unsafe { event.union.key.system },
            },
            EventType::MouseWheelScrolled => MouseWheelScrolled {
                wheel: unsafe { event.union.mouse_wheel_scroll.wheel },
                delta: unsafe { event.union.mouse_wheel_scroll.delta },
                x: unsafe { event.union.mouse_wheel_scroll.x },
                y: unsafe { event.union.mouse_wheel_scroll.y },
            },
            EventType::MouseButtonPressed => MouseButtonPressed {
                button: unsafe { event.union.mouse_button.button },
                x: unsafe { event.union.mouse_button.x },
                y: unsafe { event.union.mouse_button.y },
            },
            EventType::MouseButtonReleased => MouseButtonReleased {
                button: unsafe { event.union.mouse_button.button },
                x: unsafe { event.union.mouse_button.x },
                y: unsafe { event.union.mouse_button.y },
            },
            EventType::MouseMoved => MouseMoved {
                x: unsafe { event.union.mouse_move.x },
                y: unsafe { event.union.mouse_move.y },
            },
            EventType::MouseEntered => MouseEntered,
            EventType::MouseLeft => MouseLeft,
            EventType::JoystickButtonPressed => JoystickButtonPressed {
                joystickid: unsafe { event.union.joystick_button.joystick_id },
                button: unsafe { event.union.joystick_button.button },
            },
            EventType::JoystickButtonReleased => JoystickButtonReleased {
                joystickid: unsafe { event.union.joystick_button.joystick_id },
                button: unsafe { event.union.joystick_button.button },
            },
            EventType::JoystickMoved => JoystickMoved {
                joystickid: unsafe { event.union.joystick_move.joystick_id },
                axis: unsafe { event.union.joystick_move.axis },
                position: unsafe { event.union.joystick_move.position },
            },
            EventType::JoystickConnected => JoystickConnected {
                joystickid: unsafe { event.union.joystick_connect.joystick_id },
            },
            EventType::JoystickDisconnected => JoystickDisconnected {
                joystickid: unsafe { event.union.joystick_connect.joystick_id },
            },
            EventType::TouchBegan => TouchBegan {
                finger: unsafe { event.union.touch.finger },
                x: unsafe { event.union.touch.x },
                y: unsafe { event.union.touch.y },
            },
            EventType::TouchMoved => TouchMoved {
                finger: unsafe { event.union.touch.finger },
                x: unsafe { event.union.touch.x },
                y: unsafe { event.union.touch.y },
            },
            EventType::TouchEnded => TouchEnded {
                finger: unsafe { event.union.touch.finger },
                x: unsafe { event.union.touch.x },
                y: unsafe { event.union.touch.y },
            },
            EventType::SensorChanged => SensorChanged {
                type_: unsafe { event.union.sensor.type_ },
                x: unsafe { event.union.sensor.x },
                y: unsafe { event.union.sensor.y },
                z: unsafe { event.union.sensor.z },
            },
            EventType::MouseWheelMoved => Event::MouseWheelMoved,
        };
        Some(evt)
    }
}
