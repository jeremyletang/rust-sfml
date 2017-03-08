// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

use std::marker::PhantomData;

use system::raw_conv::{Raw, FromRaw};
use window::{Event, VideoMode, ContextSettings, Style};
use system::{Vector2i, Vector2u};
use csfml_system_sys::sfBool;
use system::SfBoolExt;

use csfml_window_sys as ffi;
use ext;

/// Window that serves as a target for OpenGL rendering.
///
/// `Window` is the main class of the Window module.
///
/// It defines an OS window that is able to receive an OpenGL rendering.
///
/// The `Window` type provides a simple interface for manipulating the window:
/// move, resize, show/hide, control mouse cursor, etc.
/// It also provides event handling through its `poll_event()` and `wait_event()` functions.
///
/// Note that OpenGL experts can pass their own parameters
/// (antialiasing level, bits for the depth and stencil buffers, etc.) to the OpenGL context
/// attached to the window, with the `ContextSettings` structure which is passed as an
/// optional argument when creating the window.
///
/// # Usage example
///
/// ```no_run
/// use sfml::window::{Window, VideoMode, Event, style};
/// // Create a new window
/// let mut window = Window::new(VideoMode::new(800, 600, 32),
///                              "SFML window",
///                              style::CLOSE,
///                              &Default::default()).unwrap();
/// // Limit the framerate to 60 frames per second (this step is optional)
/// window.set_framerate_limit(60);
///
/// // The main loop - ends as soon as the window is closed
/// while window.is_open() {
///     // Event processing
///     while let Some(event) = window.poll_event() {
///         // Request closing for the window
///         if event == Event::Closed {
///             window.close();
///         }
///     }
///
///     // Activate the window for OpenGL rendering
///     window.set_active(true);
///
///     // OpenGL drawing commands go here...
///
///     // End the current frame and display its contents on screen
///     window.display();
/// }
/// ```
pub struct Window {
    window: *mut ffi::sfWindow,
}

/// An iterator over all the events in the events queue (internally call `poll_event`)
pub struct Events<'a> {
    window: *mut ffi::sfWindow,
    winref: PhantomData<&'a mut Window>,
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
    /// * mode - Video mode to use (defines the width, height and depth of the
    ///                             rendering area of the window)
    /// * title - Title of the window
    /// * style - Window style
    /// * settings - Additional settings for the underlying OpenGL context
    ///
    /// Return Some(Window) or None
    pub fn new(mode: VideoMode,
               title: &str,
               style: Style,
               settings: &ContextSettings)
               -> Option<Window> {
        let utf32 = ::unicode_conv::str_to_csfml(title);
        let sf_win: *mut ffi::sfWindow = unsafe {
            ffi::sfWindow_createUnicode(mode.raw(),
                                        utf32.as_ptr() as _,
                                        style.bits(),
                                        &settings.raw())
        };
        if sf_win.is_null() {
            None
        } else {
            Some(Window { window: sf_win })
        }
    }

    /// Return an iterator over all the event currently in the events queue.
    pub fn events(&self) -> Events {
        Events {
            window: self.window,
            winref: PhantomData,
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
    /// Return Some(event) if an event was returned, or None if the event queue was empty
    pub fn poll_event(&mut self) -> Option<Event> {
        let mut event = unsafe { ::std::mem::zeroed() };
        let have_event = unsafe { ffi::sfWindow_pollEvent(self.window, &mut event).to_bool() };
        if have_event {
            ext::event::get_wrapped_event(&mut event)
        } else {
            None
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
    /// Return Some(event) or None if an error has occured
    pub fn wait_event(&mut self) -> Option<Event> {
        let mut event = unsafe { ::std::mem::zeroed() };
        let have_event = unsafe { ffi::sfWindow_waitEvent(self.window, &mut event).to_bool() };
        if have_event {
            ext::event::get_wrapped_event(&mut event)
        } else {
            None
        }
    }

    /// Change a window's icon
    /// pixels must be an array of width x height pixels in 32-bits RGBA format.
    ///
    /// # Arguments
    /// * width - Icon's width, in pixels
    /// * height - Icon's height, in pixels
    /// * pixels - Vector of pixels
    pub fn set_icon(&mut self, width: u32, height: u32, pixels: &[u8]) {
        unsafe { ffi::sfWindow_setIcon(self.window, width, height, pixels.as_ptr()) }
    }

    /// Close a window and destroy all the attached resources
    ///
    /// After calling this method, the Window object remains
    /// valid.
    /// All other functions such as `poll_event` or display
    /// will still work (i.e. you don't have to test is_open
    /// every time), and will have no effect on closed windows.
    pub fn close(&mut self) {
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
        unsafe { ContextSettings::from_raw(ffi::sfWindow_getSettings(self.window)) }
    }

    /// Change the title of a window
    ///
    /// # Arguments
    /// * title - New title
    pub fn set_title(&mut self, title: &str) {
        let utf32 = ::unicode_conv::str_to_csfml(title);
        unsafe { ffi::sfWindow_setUnicodeTitle(self.window, utf32.as_ptr() as _) }
    }

    /// Show or hide a window
    ///
    /// # Arguments
    /// * visible - true to show the window, false to hide it
    pub fn set_visible(&mut self, visible: bool) {
        unsafe { ffi::sfWindow_setVisible(self.window, sfBool::from_bool(visible)) }
    }

    /// Show or hide the mouse cursor
    ///
    /// # Arguments
    /// * visible - true to  false to hide
    pub fn set_mouse_cursor_visible(&mut self, visible: bool) {
        unsafe { ffi::sfWindow_setMouseCursorVisible(self.window, sfBool::from_bool(visible)) }
    }

    /// Grab or release the mouse cursor.
    ///
    /// If set, grabs the mouse cursor inside this window's client area so it may no longer be
    /// moved outside its bounds. Note that grabbing is only active while the window has focus.
    pub fn set_mouse_cursor_grabbed(&mut self, grabbed: bool) {
        unsafe { ffi::sfWindow_setMouseCursorGrabbed(self.window, sfBool::from_bool(grabbed)) }
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
    pub fn set_vertical_sync_enabled(&mut self, enabled: bool) {
        unsafe { ffi::sfWindow_setVerticalSyncEnabled(self.window, sfBool::from_bool(enabled)) }
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
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) {
        unsafe { ffi::sfWindow_setKeyRepeatEnabled(self.window, sfBool::from_bool(enabled)) }
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
        unsafe { ffi::sfWindow_setActive(self.window, sfBool::from_bool(enabled)) }.to_bool()
    }

    /// Display on screen what has been rendered to the window so far
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    pub fn display(&mut self) {
        unsafe { ffi::sfWindow_display(self.window) }
    }

    /// Limit the framerate to a maximum fixed frequency
    ///
    /// If a limit is set, the window will use a small delay after
    /// each call to sfWindow_display to ensure that the current frame
    /// lasted long enough to match the framerate limit.
    ///
    /// # Arguments
    /// * limit - Framerate limit, in frames per seconds (use 0 to disable limit)
    pub fn set_framerate_limit(&mut self, limit: u32) {
        unsafe { ffi::sfWindow_setFramerateLimit(self.window, limit) }
    }

    /// Change the joystick threshold
    ///
    /// The joystick threshold is the value below which
    /// no JoyMoved event will be generated.
    ///
    /// # Arguments
    /// * threshold - New threshold, in the range [0, 100]
    pub fn set_joystick_threshold(&mut self, threshold: f32) {
        unsafe { ffi::sfWindow_setJoystickThreshold(self.window, threshold) }
    }

    /// Get the position of a window
    ///
    /// Return the position in pixels
    pub fn get_position(&self) -> Vector2i {
        unsafe { Vector2i::from_raw(ffi::sfWindow_getPosition(self.window)) }
    }

    /// Change the position of a window on screen
    ///
    /// This function only works for top-level windows
    /// (i.e. it will be ignored for windows created from
    /// the handle of a child window/control).
    ///
    /// # Arguments
    /// * position - New position of the window, in pixels
    pub fn set_position(&mut self, position: &Vector2i) {
        unsafe { ffi::sfWindow_setPosition(self.window, position.raw()) }
    }

    /// Get the size of the rendering region of a window
    ///
    /// The size doesn't include the titlebar and borders of the window.
    ///
    /// Return the size in pixels
    pub fn get_size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfWindow_getSize(self.window)) }
    }

    /// Change the size of the rendering region of a window
    ///
    /// # Arguments
    /// * size - New size, in pixels
    pub fn set_size(&mut self, size: &Vector2u) {
        unsafe { ffi::sfWindow_setSize(self.window, size.raw()) }
    }

    /// Returns the current position of the mouse relative to the window.
    pub fn mouse_position(&self) -> Vector2i {
        unsafe { Vector2i::from_raw(ffi::sfMouse_getPosition(self.window)) }
    }

    /// Set the current position of the mouse
    ///
    /// This function sets the current position of the mouse cursor relative to the given window.
    ///
    /// # Arguments
    /// * position - New position of the mouse
    /// * relativeTo - Reference Window
    ///
    pub fn set_mouse_position(&mut self, position: &Vector2i) {
        unsafe { ffi::sfMouse_setPosition(position.raw(), self.window) }
    }

    /// Returns the current position of a touch in window coordinates.
    pub fn touch_position(&self, finger: u32) -> Vector2i {
        unsafe { FromRaw::from_raw(ffi::sfTouch_getPosition(finger, self.window)) }
    }

    /// Check whether the window has the input focus.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or most mouse events.
    pub fn has_focus(&self) -> bool {
        unsafe { ffi::sfWindow_hasFocus(self.window).to_bool() }
    }

    /// Request the current window to be made the active foreground window.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or mouse events. If a window requests focus, it only hints to the
    /// operating system, that it would like to be focused. The operating system is free to
    /// deny the request. This is not to be confused with `set_active()`.
    pub fn request_focus(&self) {
        unsafe { ffi::sfWindow_requestFocus(self.window) }
    }
}

impl Raw for Window {
    type Raw = *mut ffi::sfWindow;
    fn raw(&self) -> Self::Raw {
        self.window
    }
}

impl<'a> Iterator for Events<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        let mut event = unsafe { ::std::mem::zeroed() };
        if unsafe { ffi::sfWindow_pollEvent(self.window, &mut event) }.to_bool() {
            ext::event::get_wrapped_event(&mut event)
        } else {
            None
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            ffi::sfWindow_destroy(self.window);
        }
    }
}
