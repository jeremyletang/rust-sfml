use crate::ffi::system::sfString;
pub use crate::ffi::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use self::system::{sfVideoModeVector_getLength, sfVideoModeVector_index};

use super::graphics::sfRenderWindow;

decl_opaque! {
    sfCursor;
    sfContext;
    sfWindow;
    JoystickIdentification;
    sfVideoModeVector;
}

type sfJoystickIdentification = JoystickIdentification;

/// Enumeration of the native system cursor types.
///
/// Refer to the following table to determine which cursor is available on which platform.
///
/// | Type                     | Linux | Mac OS X | Windows |
/// |--------------------------|-------|----------|---------|
/// | `Arrow`                  | yes   | yes      | yes     |
/// | `ArrowWait`              | no    | no       | yes     |
/// | `Wait`                   | yes   | no       | yes     |
/// | `Text`                   | yes   | yes      | yes     |
/// | `Hand`                   | yes   | yes      | yes     |
/// | `SizeHorizontal`         | yes   | yes      | yes     |
/// | `SizeVertical`           | yes   | yes      | yes     |
/// | `SizeTopLeftBottomRight` | no    | yes*     | yes     |
/// | `SizeBottomLeftTopRight` | no    | yes*     | yes     |
/// | `SizeAll`                | yes   | no       | yes     |
/// | `Cross`                  | yes   | yes      | yes     |
/// | `Help`                   | yes   | yes*     | yes     |
/// | `NotAllowed`             | yes   | yes      | yes     |
///
/// * These cursor types are undocumented so may not be available on all versions,
/// but have been tested on 10.13
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub enum sfCursorType {
    Arrow,
    ArrowWait,
    Wait,
    Text,
    Hand,
    SizeHorizontal,
    SizeVertical,
    SizeTopLeftBottomRight,
    SizeBottomLeftTopRight,
    SizeAll,
    Cross,
    Help,
    NotAllowed,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct sfVideoMode {
    /// Video mode width, in pixels
    pub width: c_uint,
    /// Video mode height, in pixels
    pub height: c_uint,
    /// Video mode pixel depth, in bits per pixels
    pub bits_per_pixel: c_uint,
}

/// Structure defining the settings of the OpenGL context attached to a window.
///
/// `ContextSettings` allows to define several advanced settings of the OpenGL context attached
/// to a window.
///
/// All these settings with the exception of the compatibility flag and anti-aliasing level
/// have no impact on the regular SFML rendering (graphics module), so you may need to use
/// this structure only if you're using SFML as a windowing system for custom OpenGL rendering.
///
/// The `depth_bits` and `stencil_bits` members define the number of bits per pixel requested for
/// the (respectively) depth and stencil buffers.
///
/// `antialiasing_level` represents the requested number of multisampling levels for anti-aliasing.
///
/// `major_version` and `minor_version` define the version of the OpenGL context that you want.
/// Only versions greater or equal to 3.0 are relevant; versions lesser than 3.0 are all handled
/// the same way (i.e. you can use any version < 3.0 if you don't want an OpenGL 3 context).
///
/// When requesting a context with a version greater or equal to 3.2, you have the option of
/// specifying whether the context should follow the core or compatibility profile of
/// all newer (>= 3.2) OpenGL specifications. For versions 3.0 and 3.1 there is only the
/// core profile. By default a compatibility context is created. You only need to specify the
/// core flag if you want a core profile context to use with your own OpenGL rendering.
/// Warning: The graphics module will not function if you request a core profile context.
/// Make sure the attributes are set to Default if you want to use the graphics module.
///
/// Setting the debug attribute flag will request a context with additional debugging features
/// enabled. Depending on the system, this might be required for advanced OpenGL debugging.
/// OpenGL debugging is disabled by default.
///
/// Special Note for OS X: Apple only supports choosing between either a
/// legacy context (OpenGL 2.1) or a core context (OpenGL version depends on the
/// operating system version but is at least 3.2). Compatibility contexts are not supported.
/// Further information is available on the OpenGL Capabilities Tables page.
/// OS X also currently does not support debug contexts.
///
/// Please note that these values are only a hint. No failure will be reported if one or more
/// of these values are not supported by the system; instead, SFML will try to find the closest
/// valid match. You can then retrieve the settings that the window actually used to create
/// its context, with [`Window::settings`].
///
/// [`Window::settings`]: crate::window::Window::settings
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct sfContextSettings {
    /// Bits of the depth buffer.
    pub depth_bits: c_uint,
    /// Bits of the stencil buffer.
    pub stencil_bits: c_uint,
    /// Level of antialiasing.
    pub antialiasing_level: c_uint,
    /// Major number of the context version to create.
    pub major_version: c_uint,
    /// Minor number of the context version to create.
    pub minor_version: c_uint,
    /// The attribute flags to create the context with.
    pub attribute_flags: u32,
    /// Whether the context framebuffer is sRGB capable.
    pub srgb_capable: bool,
}

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
    pub(crate) type_: crate::ffi::window::sfSensorType,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
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
pub struct Event {
    pub(crate) type_: EventType,
    pub(crate) union: EventUnion,
}

type sfEvent = Event;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseWheel {
    VerticalWheel,
    HorizontalWheel,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    XButton1,
    XButton2,
}

type sfMouseButton = MouseButton;

/// Key codes known to SFML.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

type sfKeyboardKey = Key;

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

// Window handle is HWND (HWND__*) on Windows
#[cfg(target_os = "windows")]
pub type sfWindowHandle = *mut c_void;
// Window handle is Window (unsigned long) on Unix - X11
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub type sfWindowHandle = std::os::raw::c_ulong;
// Window handle is NSWindow (void*) on Mac OS X - Cocoa
#[cfg(target_os = "macos")]
pub type sfWindowHandle = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum sfJoystickAxis {
    ///< The X axis
    sfJoystickX,
    ///< The Y axis
    sfJoystickY,
    ///< The Z axis
    sfJoystickZ,
    ///< The R axis
    sfJoystickR,
    ///< The U axis
    sfJoystickU,
    ///< The V axis
    sfJoystickV,
    ///< The X axis of the point-of-view hat
    sfJoystickPovX,
    ///< The Y axis of the point-of-view hat
    sfJoystickPovY,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum sfSensorType {
    ///< Measures the raw acceleration (m/s^2)
    Accelerometer,
    ///< Measures the raw rotation rates (degrees/s)
    Gyroscope,
    ///< Measures the ambient magnetic field (micro-teslas)
    Magnetometer,
    ///< Measures the direction and intensity of gravity, independent of device acceleration (m/s^2)
    Gravity,
    ///< Measures the direction and intensity of device acceleration, independent of the gravity (m/s^2)
    UserAcceleration,
    ///< Measures the absolute 3D orientation (degrees)
    Orientation,

    ///< Keep last -- the total number of sensor types
    Count,
}

include!("window_bindgen.rs");
