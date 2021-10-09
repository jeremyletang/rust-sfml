use crate::ffi::system::sfString;
pub use crate::ffi::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SizeEvent {
    pub(crate) width: c_uint,
    pub(crate) height: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct KeyEvent {
    pub(crate) code: Key,
    pub(crate) alt: bool,
    pub(crate) control: bool,
    pub(crate) shift: bool,
    pub(crate) system: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TextEvent {
    pub(crate) unicode: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MouseMoveEvent {
    pub(crate) x: c_int,
    pub(crate) y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MouseButtonEvent {
    pub(crate) button: MouseButton,
    pub(crate) x: c_int,
    pub(crate) y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MouseWheelEvent {
    pub(crate) delta: c_int,
    pub(crate) x: c_int,
    pub(crate) y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MouseWheelScrollEvent {
    pub(crate) wheel: MouseWheel,
    pub(crate) delta: f32,
    pub(crate) x: c_int,
    pub(crate) y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct JoystickConnectEvent {
    pub(crate) joystick_id: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct JoystickMoveEvent {
    pub(crate) joystick_id: c_uint,
    pub(crate) axis: JoystickAxis,
    pub(crate) position: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct JoystickButtonEvent {
    pub(crate) joystick_id: c_uint,
    pub(crate) button: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TouchEvent {
    pub(crate) finger: c_uint,
    pub(crate) x: c_int,
    pub(crate) y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SensorEvent {
    pub(crate) type_: SensorType,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SensorType {
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Gravity,
    UserAcceleration,
    Orientation,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)] // constructed on C++ side by SFML
pub(crate) enum EventType {
    Closed,
    Resized,
    LostFocus,
    GainedFocus,
    TextEntered,
    KeyPressed,
    KeyReleased,
    MouseWheelMoved,
    MouseWheelScrolled,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseEntered,
    MouseLeft,
    JoystickButtonPressed,
    JoystickButtonReleased,
    JoystickMoved,
    JoystickConnected,
    JoystickDisconnected,
    TouchBegan,
    TouchMoved,
    TouchEnded,
    SensorChanged,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct Event {
    pub(crate) type_: EventType,
    pub(crate) union: EventUnion,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union EventUnion {
    pub(crate) size: SizeEvent,
    pub(crate) key: KeyEvent,
    pub(crate) text: TextEvent,
    pub(crate) mouse_move: MouseMoveEvent,
    pub(crate) mouse_button: MouseButtonEvent,
    pub(crate) mouse_wheel: MouseWheelEvent,
    pub(crate) mouse_wheel_scroll: MouseWheelScrollEvent,
    pub(crate) joystick_move: JoystickMoveEvent,
    pub(crate) joystick_button: JoystickButtonEvent,
    pub(crate) joystick_connect: JoystickConnectEvent,
    pub(crate) touch: TouchEvent,
    pub(crate) sensor: SensorEvent,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JoystickAxis {
    X,
    Y,
    Z,
    R,
    U,
    V,
    PovX,
    PovY,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum MouseWheel {
    VerticalWheel,
    HorizontalWheel,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    XButton1,
    XButton2,
}

/// Key codes known to SFML.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Key {
    Unknown = -1,
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Escape,
    LControl,
    LShift,
    LAlt,
    LSystem,
    RControl,
    RShift,
    RAlt,
    RSystem,
    Menu,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Period,
    Quote,
    Slash,
    Backslash,
    Tilde,
    Equal,
    Hyphen,
    Space,
    Enter,
    Backspace,
    Tab,
    PageUp,
    PageDown,
    End,
    Home,
    Insert,
    Delete,
    Add,
    Subtract,
    Multiply,
    Divide,
    Left,
    Right,
    Up,
    Down,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    Pause,
}

#[repr(C)]
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct JoystickIdentification {
    _opaque: [u8; 0],
}

impl Dispose for JoystickIdentification {
    unsafe fn dispose(&mut self) {
        sfJoystickIdentification_destroy(self);
    }
}

impl<'a> IntoIterator for &'a sfVideoModeVector {
    type IntoIter = sfVideoModeVectorIter<'a>;
    type Item = &'a sfVideoMode;
    fn into_iter(self) -> Self::IntoIter {
        sfVideoModeVectorIter {
            vec: self,
            len: unsafe { sfVideoModeVector_getLength(self) },
            cursor: 0,
        }
    }
}

#[derive(Debug)]
pub struct sfVideoModeVectorIter<'a> {
    vec: &'a sfVideoModeVector,
    len: usize,
    cursor: usize,
}

impl<'a> Iterator for sfVideoModeVectorIter<'a> {
    type Item = &'a sfVideoMode;
    fn next(&mut self) -> Option<&'a sfVideoMode> {
        if self.cursor >= self.len {
            return None;
        }
        unsafe {
            let item = sfVideoModeVector_index(self.vec, self.cursor);
            self.cursor += 1;
            Some(&*item)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct sfVideoModeVector {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn sfJoystick_getIdentification(joystick: c_uint) -> *mut JoystickIdentification;
    pub fn sfJoystickIdentification_destroy(ident: *mut JoystickIdentification);
    pub fn sfJoystickIdentification_getProductId(ident: *const JoystickIdentification) -> c_uint;
    pub fn sfJoystickIdentification_getVendorId(ident: *const JoystickIdentification) -> c_uint;
    pub fn sfJoystickIdentification_getName(
        ident: *const JoystickIdentification,
    ) -> *const sfString;
    pub fn sfVideoMode_getFullscreenModes() -> *const sfVideoModeVector;

    pub(crate) fn sfWindow_pollEvent(window: *mut sfWindow, event: *mut Event) -> sfBool;
    pub(crate) fn sfWindow_waitEvent(window: *mut sfWindow, event: *mut Event) -> sfBool;

    pub(crate) fn sfKeyboard_isKeyPressed(key: Key) -> sfBool;
    pub(crate) fn sfKeyboard_setVirtualKeyboardVisible(visible: sfBool);
    pub(crate) fn sfMouse_isButtonPressed(button: MouseButton) -> sfBool;
    pub fn sfVideoModeVector_getLength(vec: *const sfVideoModeVector) -> usize;
    pub fn sfVideoModeVector_index(
        vec: *const sfVideoModeVector,
        index: usize,
    ) -> *const sfVideoMode;
}
