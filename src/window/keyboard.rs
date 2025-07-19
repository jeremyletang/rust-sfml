pub use crate::ffi::window::Key;
use crate::{ffi::window as ffi, system::SfStr, window::thread_safety};

use super::Scancode;

impl Key {
    /// Return whether this key is currently pressed.
    ///
    /// Queries the real-time state of the keyboard, even if keys have been
    /// pressed or released while no window was focused and no events were
    /// triggered.
    #[must_use]
    pub fn is_key_pressed(self) -> bool {
        thread_safety::set_window_thread();

        unsafe { ffi::sfKeyboard_isKeyPressed(self) }
    }
}

impl From<Scancode> for Key {
    fn from(scancode: Scancode) -> Self {
        unsafe { ffi::sfKeyboard_localize(scancode) }
    }
}

impl Scancode {
    /// Check if a key is pressed
    ///
    /// Queries the real-time state of the keyboard, even if keys have been
    /// pressed or released while no window was focused and no events were
    /// triggered.
    #[must_use]
    pub fn is_scancode_pressed(self) -> bool {
        thread_safety::set_window_thread();
        unsafe { ffi::sfKeyboard_isScancodePressed(self) }
    }

    /// Provide a string representation for a given scancode
    ///
    /// The returned string is a short, non-technical description of
    /// the key represented with the given scancode. Most effectively
    /// used in user interfaces, as the description for the key takes
    /// the users keyboard layout into consideration.
    ///
    /// # Warning
    /// The result is OS-dependent: for example, sfScanLSystem
    /// is "Left Meta" on Linux, "Left Windows" on Windows and
    /// "Left Command" on macOS.
    ///
    /// The current keyboard layout set by the operating system is used to
    /// interpret the scancode: for example, sfKeySemicolon is
    /// mapped to ";" for layout and to "é" for others.
    ///
    /// The returned const char* owns the string and must be freed to
    /// avoid memory leaks.
    ///
    /// # Arguments
    /// code - Scancode to describe
    #[must_use]
    pub fn description(self) -> String {
        unsafe {
            let sf_string = ffi::sfKeyboard_getDescription(self);
            let data = ffi::system::sfString_getData(sf_string);
            let string = SfStr::from_ptr_str(data).to_rust_string();
            ffi::system::sfString_delete(sf_string);
            string
        }
    }
}

impl From<Key> for Scancode {
    fn from(key: Key) -> Self {
        unsafe { ffi::sfKeyboard_delocalize(key) }
    }
}

/// Show or hide the virtual keyboard.
///
/// Warning: the virtual keyboard is not supported on all systems. It will typically be
/// implemented on mobile OSes (Android, iOS) but not on desktop OSes (Windows, Linux, ...).
///
/// If the virtual keyboard is not available, this function does nothing.
pub fn set_virtual_keyboard_visible(visible: bool) {
    unsafe { ffi::sfKeyboard_setVirtualKeyboardVisible(visible) }
}
