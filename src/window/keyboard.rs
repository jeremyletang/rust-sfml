use crate::{ffi, sf_bool_ext::SfBoolExt, window::thread_safety};

/// Key codes known to SFML.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Hash)]
pub struct Key(pub(super) ffi::sfKeyCode);

impl Key {
    /// Unhandled key
    pub const UNKNOWN: Self = Self(ffi::sfKeyCode_sfKeyUnknown);
    /// The A key
    pub const A: Self = Self(ffi::sfKeyCode_sfKeyA);
    /// The B key
    pub const B: Self = Self(ffi::sfKeyCode_sfKeyB);
    /// The C key
    pub const C: Self = Self(ffi::sfKeyCode_sfKeyC);
    /// The D key
    pub const D: Self = Self(ffi::sfKeyCode_sfKeyD);
    /// The E key
    pub const E: Self = Self(ffi::sfKeyCode_sfKeyE);
    /// The F key
    pub const F: Self = Self(ffi::sfKeyCode_sfKeyF);
    /// The G key
    pub const G: Self = Self(ffi::sfKeyCode_sfKeyG);
    /// The H key
    pub const H: Self = Self(ffi::sfKeyCode_sfKeyH);
    /// The I key
    pub const I: Self = Self(ffi::sfKeyCode_sfKeyI);
    /// The J key
    pub const J: Self = Self(ffi::sfKeyCode_sfKeyJ);
    /// The K key
    pub const K: Self = Self(ffi::sfKeyCode_sfKeyK);
    /// The L key
    pub const L: Self = Self(ffi::sfKeyCode_sfKeyL);
    /// The M key
    pub const M: Self = Self(ffi::sfKeyCode_sfKeyM);
    /// The N key
    pub const N: Self = Self(ffi::sfKeyCode_sfKeyN);
    /// The O key
    pub const O: Self = Self(ffi::sfKeyCode_sfKeyO);
    /// The P key
    pub const P: Self = Self(ffi::sfKeyCode_sfKeyP);
    /// The Q key
    pub const Q: Self = Self(ffi::sfKeyCode_sfKeyQ);
    /// The R key
    pub const R: Self = Self(ffi::sfKeyCode_sfKeyR);
    /// The S key
    pub const S: Self = Self(ffi::sfKeyCode_sfKeyS);
    /// The T key
    pub const T: Self = Self(ffi::sfKeyCode_sfKeyT);
    /// The U key
    pub const U: Self = Self(ffi::sfKeyCode_sfKeyU);
    /// The V key
    pub const V: Self = Self(ffi::sfKeyCode_sfKeyV);
    /// The W key
    pub const W: Self = Self(ffi::sfKeyCode_sfKeyW);
    /// The X key
    pub const X: Self = Self(ffi::sfKeyCode_sfKeyX);
    /// The Y key
    pub const Y: Self = Self(ffi::sfKeyCode_sfKeyY);
    /// The Z key
    pub const Z: Self = Self(ffi::sfKeyCode_sfKeyZ);
    /// The 0 key
    pub const NUM0: Self = Self(ffi::sfKeyCode_sfKeyNum0);
    /// The 1 key
    pub const NUM1: Self = Self(ffi::sfKeyCode_sfKeyNum1);
    /// The 2 key
    pub const NUM2: Self = Self(ffi::sfKeyCode_sfKeyNum2);
    /// The 3 key
    pub const NUM3: Self = Self(ffi::sfKeyCode_sfKeyNum3);
    /// The 4 key
    pub const NUM4: Self = Self(ffi::sfKeyCode_sfKeyNum4);
    /// The 5 key
    pub const NUM5: Self = Self(ffi::sfKeyCode_sfKeyNum5);
    /// The 6 key
    pub const NUM6: Self = Self(ffi::sfKeyCode_sfKeyNum6);
    /// The 7 key
    pub const NUM7: Self = Self(ffi::sfKeyCode_sfKeyNum7);
    /// The 8 key
    pub const NUM8: Self = Self(ffi::sfKeyCode_sfKeyNum8);
    /// The 9 key
    pub const NUM9: Self = Self(ffi::sfKeyCode_sfKeyNum9);
    /// The Escape key
    pub const ESCAPE: Self = Self(ffi::sfKeyCode_sfKeyEscape);
    /// The left Control key
    pub const LCONTROL: Self = Self(ffi::sfKeyCode_sfKeyLControl);
    /// The left Shift key
    pub const LSHIFT: Self = Self(ffi::sfKeyCode_sfKeyLShift);
    /// The left Alt key
    pub const LALT: Self = Self(ffi::sfKeyCode_sfKeyLAlt);
    /// The left OS specific key: window (Windows and Linux), apple (Mac OS X), ...
    pub const LSYSTEM: Self = Self(ffi::sfKeyCode_sfKeyLSystem);
    /// The right Control key
    pub const RCONTROL: Self = Self(ffi::sfKeyCode_sfKeyRControl);
    /// The right Shift key
    pub const RSHIFT: Self = Self(ffi::sfKeyCode_sfKeyRShift);
    /// The right Alt key
    pub const RALT: Self = Self(ffi::sfKeyCode_sfKeyRAlt);
    /// The right OS specific key: window (Windows and Linux), apple (Mac OS X), ...
    pub const RSYSTEM: Self = Self(ffi::sfKeyCode_sfKeyRSystem);
    /// The Menu key
    pub const MENU: Self = Self(ffi::sfKeyCode_sfKeyMenu);
    /// The [ key
    pub const LBRACKET: Self = Self(ffi::sfKeyCode_sfKeyLBracket);
    /// The ] key
    pub const RBRACKET: Self = Self(ffi::sfKeyCode_sfKeyRBracket);
    /// The ; key
    pub const SEMICOLON: Self = Self(ffi::sfKeyCode_sfKeySemicolon);
    /// The , key
    pub const COMMA: Self = Self(ffi::sfKeyCode_sfKeyComma);
    /// The . key
    pub const PERIOD: Self = Self(ffi::sfKeyCode_sfKeyPeriod);
    /// The ' key
    pub const QUOTE: Self = Self(ffi::sfKeyCode_sfKeyQuote);
    /// The / key
    pub const SLASH: Self = Self(ffi::sfKeyCode_sfKeySlash);
    /// The \ key
    pub const BACKSLASH: Self = Self(ffi::sfKeyCode_sfKeyBackslash);
    /// The ~ key
    pub const TILDE: Self = Self(ffi::sfKeyCode_sfKeyTilde);
    /// The = key
    pub const EQUAL: Self = Self(ffi::sfKeyCode_sfKeyEqual);
    /// The - key (hyphen)
    pub const HYPHEN: Self = Self(ffi::sfKeyCode_sfKeyHyphen);
    /// The Space key
    pub const SPACE: Self = Self(ffi::sfKeyCode_sfKeySpace);
    /// The Enter/Return keys
    pub const ENTER: Self = Self(ffi::sfKeyCode_sfKeyEnter);
    /// The Backspace key
    pub const BACKSPACE: Self = Self(ffi::sfKeyCode_sfKeyBackspace);
    /// The Tabulation key
    pub const TAB: Self = Self(ffi::sfKeyCode_sfKeyTab);
    /// The Page up key
    pub const PAGEUP: Self = Self(ffi::sfKeyCode_sfKeyPageUp);
    /// The Page down key
    pub const PAGEDOWN: Self = Self(ffi::sfKeyCode_sfKeyPageDown);
    /// The End key
    pub const END: Self = Self(ffi::sfKeyCode_sfKeyEnd);
    /// The Home key
    pub const HOME: Self = Self(ffi::sfKeyCode_sfKeyHome);
    /// The Insert key
    pub const INSERT: Self = Self(ffi::sfKeyCode_sfKeyInsert);
    /// The Delete key
    pub const DELETE: Self = Self(ffi::sfKeyCode_sfKeyDelete);
    /// The + key
    pub const ADD: Self = Self(ffi::sfKeyCode_sfKeyAdd);
    /// The - key (minus, usually from numpad)
    pub const SUBTRACT: Self = Self(ffi::sfKeyCode_sfKeySubtract);
    /// The * key
    pub const MULTIPLY: Self = Self(ffi::sfKeyCode_sfKeyMultiply);
    /// The / key
    pub const DIVIDE: Self = Self(ffi::sfKeyCode_sfKeyDivide);
    /// Left arrow
    pub const LEFT: Self = Self(ffi::sfKeyCode_sfKeyLeft);
    /// Right arrow
    pub const RIGHT: Self = Self(ffi::sfKeyCode_sfKeyRight);
    /// Up arrow
    pub const UP: Self = Self(ffi::sfKeyCode_sfKeyUp);
    /// Down arrow
    pub const DOWN: Self = Self(ffi::sfKeyCode_sfKeyDown);
    /// The numpad 0 key
    pub const NUMPAD0: Self = Self(ffi::sfKeyCode_sfKeyNumpad0);
    /// The numpad 1 key
    pub const NUMPAD1: Self = Self(ffi::sfKeyCode_sfKeyNumpad1);
    /// The numpad 2 key
    pub const NUMPAD2: Self = Self(ffi::sfKeyCode_sfKeyNumpad2);
    /// The numpad 3 key
    pub const NUMPAD3: Self = Self(ffi::sfKeyCode_sfKeyNumpad3);
    /// The numpad 4 key
    pub const NUMPAD4: Self = Self(ffi::sfKeyCode_sfKeyNumpad4);
    /// The numpad 5 key
    pub const NUMPAD5: Self = Self(ffi::sfKeyCode_sfKeyNumpad5);
    /// The numpad 6 key
    pub const NUMPAD6: Self = Self(ffi::sfKeyCode_sfKeyNumpad6);
    /// The numpad 7 key
    pub const NUMPAD7: Self = Self(ffi::sfKeyCode_sfKeyNumpad7);
    /// The numpad 8 key
    pub const NUMPAD8: Self = Self(ffi::sfKeyCode_sfKeyNumpad8);
    /// The numpad 9 key
    pub const NUMPAD9: Self = Self(ffi::sfKeyCode_sfKeyNumpad9);
    /// The F1 key
    pub const F1: Self = Self(ffi::sfKeyCode_sfKeyF1);
    /// The F2 key
    pub const F2: Self = Self(ffi::sfKeyCode_sfKeyF2);
    /// The F3 key
    pub const F3: Self = Self(ffi::sfKeyCode_sfKeyF3);
    /// The F4 key
    pub const F4: Self = Self(ffi::sfKeyCode_sfKeyF4);
    /// The F5 key
    pub const F5: Self = Self(ffi::sfKeyCode_sfKeyF5);
    /// The F6 key
    pub const F6: Self = Self(ffi::sfKeyCode_sfKeyF6);
    /// The F7 key
    pub const F7: Self = Self(ffi::sfKeyCode_sfKeyF7);
    /// The F8 key
    pub const F8: Self = Self(ffi::sfKeyCode_sfKeyF8);
    /// The F9 key
    pub const F9: Self = Self(ffi::sfKeyCode_sfKeyF9);
    /// The F10 key
    pub const F10: Self = Self(ffi::sfKeyCode_sfKeyF10);
    /// The F11 key
    pub const F11: Self = Self(ffi::sfKeyCode_sfKeyF11);
    /// The F12 key
    pub const F12: Self = Self(ffi::sfKeyCode_sfKeyF12);
    /// The F13 key
    pub const F13: Self = Self(ffi::sfKeyCode_sfKeyF13);
    /// The F14 key
    pub const F14: Self = Self(ffi::sfKeyCode_sfKeyF14);
    /// The F15 key
    pub const F15: Self = Self(ffi::sfKeyCode_sfKeyF15);
    /// The Pause key
    pub const PAUSE: Self = Self(ffi::sfKeyCode_sfKeyPause);
    /// The total number of keyboard keys
    pub const COUNT: Self = Self(ffi::sfKeyCode_sfKeyCount);
}

impl Key {
    /// Return whether this key is currently pressed.
    ///
    /// Queries the real-time state of the keyboard, even if keys have been
    /// pressed or released while no window was focused and no events were
    /// triggered.
    #[must_use]
    pub fn is_pressed(self) -> bool {
        thread_safety::set_window_thread();

        unsafe { ffi::sfKeyboard_isKeyPressed(self.0) }.into_bool()
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
