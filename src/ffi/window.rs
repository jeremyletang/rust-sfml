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

pub mod window {
    use libc::{c_uint, c_float, c_char};

    use window::ContextSettings;
    use system::vector2::{Vector2i, Vector2u};

    use ffi::window::video_mode::sfVideoMode;
    use ffi::sfml_types::SfBool;

    #[repr(C)]
    pub struct sfWindow;

    extern "C" {
        pub fn sfWindow_create(mode: sfVideoMode, title: *const c_char, style: c_uint, settings: *const ContextSettings) -> *mut sfWindow;
        pub fn sfWindow_createUnicode(mode: sfVideoMode, title: *const u32, style: c_uint, setting: *const ContextSettings) -> *mut sfWindow;
        //fn sfWindow_createFromHandle(handle: sfWindowHandle, settings: *mut sfContextSettings) -> *mut sfWindow;
        pub fn sfWindow_close(window: *mut sfWindow);
        pub fn sfWindow_destroy(window: *mut sfWindow);
        pub fn sfWindow_isOpen(window: *mut sfWindow) -> SfBool;
        pub fn sfWindow_getSettings(window: *mut sfWindow) -> ContextSettings;
        pub fn sfWindow_setTitle(window: *mut sfWindow, title: *const c_char);
        pub fn sfWindow_setUnicodeTitle(window: *mut sfWindow, title: *const u32);
        pub fn sfWindow_setIcon(window: *mut sfWindow, width: c_uint, height: c_uint, pixel: *const u8);
        pub fn sfWindow_setVisible(window: *mut sfWindow, visible: SfBool);
        pub fn sfWindow_setMouseCursorVisible(window: *mut sfWindow, visible: SfBool);
        pub fn sfWindow_setVerticalSyncEnabled(window: *mut sfWindow, enabled: SfBool);
        pub fn sfWindow_setKeyRepeatEnabled(window: *mut sfWindow, enabled: SfBool);
        pub fn sfWindow_setActive(window: *mut sfWindow, active: SfBool) -> SfBool;
        pub fn sfWindow_display(window: *mut sfWindow);
        pub fn sfWindow_setFramerateLimit(window: *mut sfWindow, limit: c_uint);
        pub fn sfWindow_setJoystickThreshold(window: *mut sfWindow, threshold: c_float);
        pub fn sfWindow_getPosition(window: *mut sfWindow) -> Vector2i;
        pub fn sfWindow_setPosition(window: *mut sfWindow, position: Vector2i);
        pub fn sfWindow_getSize(window: *mut sfWindow) -> Vector2u;
        pub fn sfWindow_setSize(window: *mut sfWindow, size: Vector2u);
        pub fn sfWindow_pollEvent(window: *mut sfWindow, event: *mut ::window::event::raw::sfEvent) -> SfBool;
        pub fn sfWindow_waitEvent(window: *mut sfWindow, event: *mut ::window::event::raw::sfEvent) -> SfBool;
        pub fn sfMouse_getPosition(relativeTo: *mut sfWindow) -> Vector2i;
        pub fn sfMouse_setPosition(position: Vector2i, relativeTo: *mut sfWindow);
        //fn sfWindow_getSystemHandle(window: *mut sfWindow) -> sfWindowHandle;
    }
}

pub mod context {
    use ffi::sfml_types::SfBool;

    #[repr(C)]
    pub struct sfContext;

    extern "C" {
        pub fn sfContext_create() -> *mut sfContext;
        pub fn sfContext_destroy(context: *mut sfContext);
        pub fn sfContext_setActive(context: *mut sfContext, active: SfBool);
    }
}

pub mod joystick {
    use libc::{c_float, c_uint};

    use ffi::sfml_types::SfBool;

    extern "C" {
        pub fn sfJoystick_isConnected(joystick: c_uint) -> SfBool;
        pub fn sfJoystick_getButtonCount(joystick: c_uint) -> c_uint;
        pub fn sfJoystick_hasAxis(joystick: c_uint, axis: c_uint) -> SfBool;
        pub fn sfJoystick_isButtonPressed(joystick: c_uint, button: c_uint) -> SfBool;
        pub fn sfJoystick_getAxisPosition(joystick: c_uint, axis: c_uint) -> c_float;
        pub fn sfJoystick_update();
    }
}

pub mod keyboard {
    pub use libc::c_int;

    use ffi::sfml_types::SfBool;

    extern "C" {
        pub fn sfKeyboard_isKeyPressed(key: c_int) -> SfBool;
    }
}

pub mod mouse {

    use libc::c_uint;

    use ffi::sfml_types::SfBool;

    extern "C" {
        pub fn sfMouse_isButtonPressed(button: c_uint) -> SfBool;
    }
}

pub mod video_mode {
    use libc::{c_uint, size_t};

    use ffi::sfml_types::SfBool;

    #[repr(C)]
    pub struct sfVideoMode {
        pub width:          c_uint,
        pub height:         c_uint,
        pub bits_per_pixel: c_uint
    }

    impl Clone for sfVideoMode {
        fn clone(&self) -> sfVideoMode {
           sfVideoMode {
               width: self.width,
               height: self.height,
               bits_per_pixel: self.bits_per_pixel
            }
        }
    }

    extern "C" {
        pub fn sfVideoMode_getDesktopMode() -> sfVideoMode;
        pub fn sfVideoMode_getFullscreenModes(Count: *mut size_t) -> *mut sfVideoMode;
        pub fn sfVideoMode_isValid(mode: sfVideoMode) -> SfBool;
    }
}
