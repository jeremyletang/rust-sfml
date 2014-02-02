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
    
    use std::libc::{c_void, c_uint, c_float, c_char};    

    use window::ContextSettings;
    use ffi::window::video_mode::sfVideoMode;    
    use system::vector2::{Vector2i, Vector2u};
    use sfml_types::SfBool;

    pub struct sfWindow {
        This : *c_void
    }
    
    pub struct sfEvent {
        typeEvent : c_uint,
        p1 :        c_uint,
        p2 :        c_uint,
        p3 :        c_float,
        p4 :        c_uint,
        p5 :        c_uint
    }

    pub enum sfEventType {
        sfEvtClosed, // 0
        sfEvtResized, // 1
        sfEvtLostFocus, // 2
        sfEvtGainedFocus, // 3
        sfEvtTextEntered, // 4
        sfEvtKeyPressed, // 5
        sfEvtKeyReleased, // 6
        sfEvtMouseWheelMoved, // 7
        sfEvtMouseButtonPressed, // 8
        sfEvtMouseButtonReleased, // 9
        sfEvtMouseMoved, // 10
        sfEvtMouseEntered, // 11
        sfEvtMouseLeft, // 12
        sfEvtJoystickButtonPressed, // 13
        sfEvtJoystickButtonReleased, // 14
        sfEvtJoystickMoved, // 15
        sfEvtJoystickConnected, // 16
        sfEvtJoystickDisconnected // 17
    }

    extern "C" {
        pub fn sfWindow_create(mode : sfVideoMode, title : *c_char, style : c_uint, settings : *ContextSettings) -> *sfWindow;
        pub fn sfWindow_createUnicode(mode : sfVideoMode, title : *u32, style : c_uint, setting : *ContextSettings) -> *sfWindow;
        //fn sfWindow_createFromHandle(handle : sfWindowHandle, settings : *sfContextSettings) -> *sfWindow;
        pub fn sfWindow_close(window : *sfWindow) -> ();
        pub fn sfWindow_destroy(window : *sfWindow) -> ();
        pub fn sfWindow_isOpen(window : *sfWindow) -> SfBool;
        pub fn sfWindow_getSettings(window : *sfWindow) -> ContextSettings;
        pub fn sfWindow_setTitle(window : *sfWindow, title : *c_char) -> ();
        pub fn sfWindow_setUnicodeTitle(window : *sfWindow, title : *u32) -> ();
        pub fn sfWindow_setIcon(window : *sfWindow, width : c_uint, height : c_uint, pixel : *u8) -> (); 
        pub fn sfWindow_setVisible(window : *sfWindow, visible : SfBool) -> ();
        pub fn sfWindow_setMouseCursorVisible(window : *sfWindow, visible : SfBool) -> ();
        pub fn sfWindow_setVerticalSyncEnabled(window : *sfWindow, enabled : SfBool) -> ();
        pub fn sfWindow_setKeyRepeatEnabled(window : *sfWindow, enabled : SfBool) -> ();
        pub fn sfWindow_setActive(window : *sfWindow, active : SfBool) -> SfBool;
        pub fn sfWindow_display(window : *sfWindow) -> ();
        pub fn sfWindow_setFramerateLimit(window : *sfWindow, limit : c_uint) -> ();
        pub fn sfWindow_setJoystickThreshold(window : *sfWindow, threshold : c_float) -> ();
        pub fn sfWindow_getPosition(window : *sfWindow) -> Vector2i;
        pub fn sfWindow_setPosition(window : *sfWindow, position : Vector2i) -> ();
        pub fn sfWindow_getSize(window : *sfWindow) -> Vector2u;
        pub fn sfWindow_setSize(window : *sfWindow, size : Vector2u) -> ();
        pub fn sfWindow_pollEvent(window : *sfWindow, event : *sfEvent) -> SfBool;
        pub fn sfWindow_waitEvent(window : *sfWindow, event : *sfEvent) -> SfBool;
        pub fn sfMouse_getPosition(relativeTo : *sfWindow) -> Vector2i;
        pub fn sfMouse_setPosition(position : Vector2i, relativeTo : *sfWindow) -> ();
        //fn sfWindow_getSystemHandle(window : *sfWindow) -> sfWindowHandle;
    }
}

pub mod context {

    use std::libc::c_void;
    use sfml_types::SfBool;

    pub struct sfContext {
        This: *c_void
    }

    extern "C" {
        pub fn sfContext_create() -> *sfContext;
        pub fn sfContext_destroy(context : *sfContext) -> ();
        pub fn sfContext_setActive(context : *sfContext, active : SfBool) -> ();
    }
}

pub mod joystick {
    
    use std::libc::{c_float, c_uint};
    use sfml_types::SfBool;

    extern "C" {
        pub fn sfJoystick_isConnected(joystick : c_uint) -> SfBool;
        pub fn sfJoystick_getButtonCount(joystick : c_uint) -> c_uint;
        pub fn sfJoystick_hasAxis(joystick : c_uint, axis : c_uint) -> SfBool;
        pub fn sfJoystick_isButtonPressed(joystick : c_uint, button : c_uint) -> SfBool;
        pub fn sfJoystick_getAxisPosition(joystick : c_uint, axis : c_uint) -> c_float;
        pub fn sfJoystick_update() -> ();
    }
}

pub mod keyboard {
    
    pub use std::libc::c_int;
    use sfml_types::SfBool;

    extern "C" {
        pub fn sfKeyboard_isKeyPressed(key : c_int) -> SfBool;
    }
}

pub mod mouse {
    
    use std::libc::c_uint;
    use sfml_types::SfBool;

    extern "C" {
        pub fn sfMouse_isButtonPressed(button : c_uint) -> SfBool;
        
    }
}

pub mod video_mode {
    
    use std::libc::{c_uint, size_t};
    use sfml_types::SfBool;

    pub struct sfVideoMode {
        width:          c_uint,
        height:         c_uint,
        bits_per_pixel: c_uint 
    }

    impl Clone for sfVideoMode {
        fn clone(&self) -> sfVideoMode {
           sfVideoMode {
               width : self.width,
               height : self.height,
               bits_per_pixel : self.bits_per_pixel
            }   
        }
    }
    
    extern "C" {
        pub fn sfVideoMode_getDesktopMode() -> sfVideoMode;
        pub fn sfVideoMode_getFullscreenModes(Count : *size_t) -> *sfVideoMode;
        pub fn sfVideoMode_isValid(mode : sfVideoMode) -> SfBool;
    }
}
