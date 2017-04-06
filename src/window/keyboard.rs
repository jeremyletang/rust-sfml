use csfml_window_sys as ffi;
use sf_bool_ext::SfBoolExt;

/// Key codes known to SFML.
#[repr(i32)]
#[allow(missing_docs)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Hash)]
pub enum Key {
    /// An unhandled key.
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
    /// The left OS-specific key: Window, Apple, so on.
    LSystem,
    RControl,
    RShift,
    RAlt,
    /// The right OS-specific key: Window, Apple, so on.
    RSystem,
    Menu,
    LBracket,
    RBracket,
    SemiColon,
    Comma,
    Period,
    Quote,
    Slash,
    BackSlash,
    Tilde,
    Equal,
    Dash,
    Space,
    Return,
    BackSpace,
    Tab,
    PageUp,
    PageDown,
    End,
    Home,
    Insert,
    Delete,
    /// The numpad addition key.
    Add,
    /// The numpad subtraction key.
    Subtract,
    /// The numpad multiplication key.
    Multiply,
    /// The numpad division key.
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
    /// The maximum available key code (not a real key).
    Count,
}

impl Key {
    /// Return whether this key is currently pressed.
    ///
    /// Queries the real-time state of the keyboard, even if keys have been
    /// pressed or released while no window was focused and no events were
    /// triggered.
    pub fn is_pressed(self) -> bool {
        unsafe { ffi::sfKeyboard_isKeyPressed(::std::mem::transmute(self)) }.to_bool()
    }
}

/// Show or hide the virtual keyboard.
///
/// Warning: the virtual keyboard is not supported on all systems. It will typically be
/// implemented on mobile OSes (Android, iOS) but not on desktop OSes (Windows, Linux, ...).
///
/// If the virtual keyboard is not available, this function does nothing.
pub fn set_virtual_keyboard_visible(visible: bool) {
    unsafe { ffi::sfKeyboard_setVirtualKeyboardVisible(SfBoolExt::from_bool(visible)) }
}
