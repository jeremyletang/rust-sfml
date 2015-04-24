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


//! Window manipulation
//!
//! Provides OpenGL-based windows,
//! and abstractions for events and input handling.

use libc::{c_uint, c_float};
use std::vec::Vec;
use std::ffi::CString;

use traits::Wrappable;
use window::{event, VideoMode, ContextSettings, WindowStyle};
use system::vector2::{Vector2i, Vector2u};

use ffi::sfml_types::SfBool;
use ffi::window::window as ffi;

///
/// Window manipulation
///
/// Provides OpenGL-based windows,
/// and abstractions for events and input handling.
///
pub struct Window {
    window: *mut ffi::sfWindow,
}

/// An iterator over all the events in the events queue (internally call poll_event)
pub struct Events {
    window: *mut ffi::sfWindow,
}

impl Window {
    /// Construct a new window
    ///
    /// This function creates the window with the size and pixel
    /// depth defined in mode. An optional style can be passed to
    /// customize the look and behaviour of the window (borders,
    /// title bar, resizable, closable, ...). If style contains
    /// sfFullscreen, then mode must be a valid video mode.
    ///
    /// The fourth parameter is a pointer to a structure specifying
    /// advanced OpenGL context settings such as antialiasing,
    /// depth-buffer bits, etc.
    ///
    /// # Arguments
    /// * mode - Video mode to use (defines the width, height and depth of the rendering area of the window)
    /// * title - Title of the window
    /// * style - Window style
    /// * settings - Additional settings for the underlying OpenGL context
    ///
    /// Return Some(Window) or None
    pub fn new(mode: VideoMode,
               title: &str,
               style: WindowStyle,
               settings: &ContextSettings) -> Option<Window> {

        let c_str = CString::new(title.as_bytes()).unwrap().as_ptr();
        let sf_win: *mut ffi::sfWindow = unsafe {
            ffi::sfWindow_create(mode.unwrap(), c_str, style as u32, settings)
        };
        if sf_win.is_null() {
            None
        } else {
            Some (Window {
                    window: sf_win,
                })
        }
    }

    /// Construct a new window (with a UTF-32 title)
    ///
    /// This function creates the window with the size and pixel
    /// depth defined in mode. An optional style can be passed to
    /// customize the look and behaviour of the window (borders,
    /// title bar, resizable, closable, ...). If style contains
    /// sfFullscreen, then mode must be a valid video mode.
    ///
    /// The fourth parameter is a pointer to a structure specifying
    /// advanced OpenGL context settings such as antialiasing,
    /// depth-buffer bits, etc.
    ///
    /// # Arguments
    /// * mode - Video mode to use (defines the width, height and depth of the rendering area of the window)
    /// * title - Title of the window (UTF-32)
    /// * style - Window style
    /// * settings - Additional settings for the underlying OpenGL context
    ///
    /// Return Some(Window) or None
    pub fn new_with_unicode(mode: VideoMode,
                            title: Vec<u32>,
                            style: WindowStyle,
                            settings: &ContextSettings) -> Option<Window> {

        let sf_win =
            unsafe { ffi::sfWindow_createUnicode(mode.unwrap(),
                                                 title.as_ptr(),
                                                 style as u32, settings) };
        if sf_win.is_null() {
            None
        } else {
            Some (Window {
                    window: sf_win,
                })
        }
    }

    /// Return an iterator over all the event currently in the events queue.
    pub fn events(&self) -> Events {
        Events {
            window: self.window.clone(),
        }
    }

    ///  Pop the event on top of event queue, if any, and return it
    ///
    /// This function is not blocking: if there's no pending event then
    /// it will return false and leave \a event unmodified.
    /// Note that more than one event may be present in the event queue,
    /// thus you should always call this function in a loop
    /// to make sure that you process every pending event.
    ///
    /// Return the event if an event was returned, or NoEvent if the event queue was empty
    pub fn poll_event(&mut self) -> event::Event {
        let mut event = event::raw::sfEvent { data: [032; 6] };
        let have_event = unsafe {
            ffi::sfWindow_pollEvent(self.window, &mut event).to_bool()
        };
        if !have_event {
            event::NoEvent
        } else {
            event::raw::get_wrapped_event(&mut event)
        }
    }

    /// Wait for an event and return it
    ///
    /// This function is blocking: if there's no pending event then
    /// it will wait until an event is received.
    /// After this function returns (and no error occured),
    /// the event object is always valid and filled properly.
    /// This function is typically used when you have a thread that
    /// is dedicated to events handling: you want to make this thread
    /// sleep as long as no new event is received.
    ///
    /// Return the event or NoEvent if an error has occured
    pub fn wait_event(&mut self) -> event::Event {
        let mut event = event::raw::sfEvent { data: [032; 6] };
        let have_event = unsafe {
            ffi::sfWindow_waitEvent(self.window, &mut event).to_bool()
        };
        if !have_event {
            event::NoEvent
        } else {
            event::raw::get_wrapped_event(&mut event)
        }
    }

    /// Change the title of a window (with a UTF-32 string)
    ///
    /// # Arguments
    /// * title - New title
    pub fn set_unicode_title(&mut self, title: Vec<u32>) -> () {
        unsafe {
            ffi::sfWindow_setUnicodeTitle(self.window, title.as_ptr())
        }
    }

    /// Change a window's icon
    /// pixels must be an array of width x height pixels in 32-bits RGBA format.
    ///
    /// # Arguments
    /// * width - Icon's width, in pixels
    /// * height - Icon's height, in pixels
    /// * pixels - Vector of pixels
    pub fn set_icon(&mut self, width: u32, height: u32, pixels: Vec<u8>) -> () {
        unsafe {
            ffi::sfWindow_setIcon(self.window, width as c_uint, height as c_uint, pixels.as_ptr())
        }
    }

    /// Close a window and destroy all the attached resources
    ///
    /// After calling this method, the Window object remains
    /// valid.
    /// All other functions such as poll_event or display
    /// will still work (i.e. you don't have to test is_open
    /// every time), and will have no effect on closed windows.
    pub fn close(&mut self) -> () {
        unsafe {
            ffi::sfWindow_close(self.window);
        }
    }

    /// Tell whether or not a window is opened
    ///
    /// This function returns whether or not the window exists.
    /// Note that a hidden window (set_visible(false)) will return
    /// true.
    pub fn is_open(&self) -> bool {
        unsafe { ffi::sfWindow_isOpen(self.window) }.to_bool()
    }

    /// Get the settings of the OpenGL context of a window
    ///
    /// Note that these settings may be different from what was
    /// passed to the sfWindow_create function,
    /// if one or more settings were not supported. In this case,
    /// SFML chose the closest match.
    ///
    /// Return a structure containing the OpenGL context settings
    pub fn get_settings(&self) -> ContextSettings {
        unsafe {ffi::sfWindow_getSettings(self.window)}
    }

    /// Change the title of a window
    ///
    /// # Arguments
    /// * title - New title
    pub fn set_title(&mut self, title: &str) -> () {
        let c_str = CString::new(title.as_bytes()).unwrap().as_ptr();
        unsafe {
            ffi::sfWindow_setTitle(self.window, c_str)
        }
    }

    /// Show or hide a window
    ///
    /// # Arguments
    /// * visible - true to show the window, false to hide it
    pub fn set_visible(&mut self, visible: bool) -> () {
        unsafe {
            ffi::sfWindow_setVisible(self.window, SfBool::from_bool(visible))
        }
    }

    /// Show or hide the mouse cursor
    ///
    /// # Arguments
    /// * visible - true to  false to hide
    pub fn set_mouse_cursor_visible(&mut self, visible: bool) -> () {
        unsafe {
            ffi::sfWindow_setMouseCursorVisible(self.window, SfBool::from_bool(visible))
        }
    }

    /// Enable or disable vertical synchronization
    ///
    /// Activating vertical synchronization will limit the number
    /// of frames displayed to the refresh rate of the monitor.
    /// This can avoid some visual artifacts, and limit the framerate
    /// to a good value (but not constant across different computers).
    ///
    /// # Arguments
    /// * enabled - true to enable v-sync, false to deactivate
    pub fn set_vertical_sync_enabled(&mut self, enabled: bool) -> () {
        unsafe {
            ffi::sfWindow_setVerticalSyncEnabled(self.window, SfBool::from_bool(enabled))
        }
    }

    /// Enable or disable automatic key-repeat
    ///
    /// If key repeat is enabled, you will receive repeated
    /// KeyPress events while keeping a key pressed. If it is disabled,
    /// you will only get a single event when the key is pressed.
    ///
    /// Key repeat is enabled by default.
    ///
    /// # Arguments
    /// * enabled - true to enable, false to disable
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) -> () {
        unsafe {
            ffi::sfWindow_setKeyRepeatEnabled(self.window, SfBool::from_bool(enabled))
        }
    }

    /// Activate or deactivate a window as the current target for OpenGL rendering
    ///
    /// A window is active only on the current thread, if you want to
    /// make it active on another thread you have to deactivate it
    /// on the previous thread first if it was active.
    /// Only one window can be active on a thread at a time, thus
    /// the window previously active (if any) automatically gets deactivated.
    ///
    /// # Arguments
    /// * active - true to activate, false to deactivate
    ///
    /// Return true if operation was successful, false otherwise
    pub fn set_active(&mut self, enabled: bool) -> bool {
        unsafe { ffi::sfWindow_setActive(self.window, SfBool::from_bool(enabled)) }.to_bool()
    }

    /// Display on screen what has been rendered to the window so far
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    pub fn display(&mut self) -> () {
        unsafe {
            ffi::sfWindow_display(self.window)
        }
    }

    /// Limit the framerate to a maximum fixed frequency
    ///
    /// If a limit is set, the window will use a small delay after
    /// each call to sfWindow_display to ensure that the current frame
    /// lasted long enough to match the framerate limit.
    ///
    /// # Arguments
    /// * limit - Framerate limit, in frames per seconds (use 0 to disable limit)
    pub fn set_framerate_limit(&mut self, limit: u32) -> () {
        unsafe {
            ffi::sfWindow_setFramerateLimit(self.window, limit as c_uint)
        }
    }

    /// Change the joystick threshold
    ///
    /// The joystick threshold is the value below which
    /// no JoyMoved event will be generated.
    ///
    /// # Arguments
    /// * threshold - New threshold, in the range [0, 100]
    pub fn set_joystick_threshold(&mut self, threshold: f32) -> () {
        unsafe {
            ffi::sfWindow_setJoystickThreshold(self.window, threshold as c_float)
        }
    }

    /// Get the position of a window
    ///
    /// Return the position in pixels
    pub fn get_position(&self) -> Vector2i {
        unsafe {
            ffi::sfWindow_getPosition(self.window)
        }
    }

    /// Change the position of a window on screen
    ///
    /// This function only works for top-level windows
    /// (i.e. it will be ignored for windows created from
    /// the handle of a child window/control).
    ///
    /// # Arguments
    /// * position - New position of the window, in pixels
    pub fn set_position(&mut self, position: &Vector2i) -> () {
        unsafe {
            ffi::sfWindow_setPosition(self.window, *position)
        }
    }

    /// Get the size of the rendering region of a window
    ///
    /// The size doesn't include the titlebar and borders of the window.
    ///
    /// Return the size in pixels
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfWindow_getSize(self.window)
        }
    }

    /// Change the size of the rendering region of a window
    ///
    /// # Arguments
    /// * size - New size, in pixels
    pub fn set_size(&mut self, size: &Vector2u) -> () {
        unsafe {
            ffi::sfWindow_setSize(self.window, *size)
        }
    }

    ///  Get the current position of the mouse
    ///
    /// This function returns the current position of the mouse cursor relative to the given window.
    ///
    /// # Arguments
    /// * relativeTo - Reference Window
    ///
    /// Return the position of the mouse cursor, relative to the given window
    pub fn get_mouse_position(&self) -> Vector2i {
        unsafe {
            ffi::sfMouse_getPosition(self.window)
        }
    }

    /// Set the current position of the mouse
    ///
    /// This function sets the current position of the mouse cursor relative to the given window.
    ///
    /// # Arguments
    /// * position - New position of the mouse
    /// * relativeTo - Reference Window
    ///
    pub fn set_mouse_position(&mut self, position: &Vector2i) -> () {
        unsafe {
            ffi::sfMouse_setPosition(*position, self.window)
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *mut ffi::sfWindow {
        self.window
    }
}

impl Iterator for Events {
    type Item = event::Event;

    fn next(&mut self) -> Option<event::Event> {
        let mut event = event::raw::sfEvent { data: [032; 6] };
        match unsafe { ffi::sfWindow_pollEvent(self.window, &mut event) }.to_bool() {
            false     => None,
            true      => Some(event::raw::get_wrapped_event(&mut event))
        }
    }
}

impl Drop for Window {
    /// Destructor for class Window. Destroy all the ressource.
    fn drop(&mut self) {
        unsafe {
            ffi::sfWindow_destroy(self.window);
        }
    }
}
