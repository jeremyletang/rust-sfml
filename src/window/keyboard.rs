pub use crate::ffi::window::Key;
use crate::{ffi::window as ffi, sf_bool_ext::SfBoolExt, window::thread_safety};

impl Key {
    /// Return whether this key is currently pressed.
    ///
    /// Queries the real-time state of the keyboard, even if keys have been
    /// pressed or released while no window was focused and no events were
    /// triggered.
    #[must_use]
    pub fn is_pressed(self) -> bool {
        thread_safety::set_window_thread();

        unsafe { ffi::sfKeyboard_isKeyPressed(self) }.into_bool()
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
