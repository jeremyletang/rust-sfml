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

use window::{Event, VideoMode, ContextSettings, WindowStyle};
use window::raw_event::sfEvent;
use system::{Vector2i, Vector2u};

use libc::{c_uint, c_float};
use ffi::{SfBool, Foreign};
use ffi::window as ffi;

/// Window that serves as a target for OpenGL rendering.
///
/// `Window` provides a simple interface for manipulating the window: move,
/// resize, show/hide, control mouse cursor, etc. It also provides event
/// handling through its `poll_event()` and `wait_event()` functions.
pub struct Window(Foreign<ffi::sfWindow>);

impl Window {
    /// Construct a new window.
    ///
    /// This function creates the window with the size and pixel
    /// depth defined in `mode`. An optional style can be passed to
    /// customize the look and behaviour of the window (borders,
    /// title bar, resizable, closable, ...). If `style` contains
    /// `window_style::FULLSCREEN`, then `mode` must be a valid video mode.
    ///
	/// The fourth parameter is an optional structure specifying advanced
	/// OpenGL context settings such as antialiasing, depth-buffer bits, etc.
    ///
    /// # Arguments
    /// * mode - Video mode to use (defines the width, height, and depth of the
	///   rendering area of the window)
    /// * title - Title of the window
    /// * style - Window style
    /// * settings - Additional settings for the underlying OpenGL context
    ///
    /// Returns Some(Window) or None.
    pub fn new(mode: VideoMode,
               title: &str,
               style: WindowStyle,
               settings: &ContextSettings) -> Option<Window> {
		let vec = ::ffi::to_utf32(title);
        unsafe {
            Foreign::new(ffi::sfWindow_createUnicode(mode, vec.as_ptr(), style.bits(), settings))
        }.map(Window)
    }

	fn raw(&self) -> &ffi::sfWindow { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfWindow { self.0.as_mut() }
    #[doc(hidden)]
    pub unsafe fn unwrap(&self) -> &ffi::sfWindow { self.raw() }

    /// Pop the event on top of event queue, if any, and return it.
    ///
    /// This function is not blocking: if there's no pending event then
    /// it will return None.
    /// Note that more than one event may be present in the event queue,
    /// thus you should always call this function in a loop
    /// to make sure that you process every pending event.
	///
	/// ```ignore
	/// while let Some(event) = window.poll_event() {
	///     // process event...
	/// }
	/// ```
    pub fn poll_event(&mut self) -> Option<Event> {
		loop {
			let mut event = sfEvent::new();
			let have_event = unsafe {
				ffi::sfWindow_pollEvent(self.raw_mut(), &mut event).to_bool()
			};
			if have_event {
				// If this returns None, there was actually an event, but it
				// failed to unwrap. For now, throw it away, but maybe in the
				// future report this better.
				if let Some(event) = event.wrap() {
					return Some(event)
				}
			} else {
				return None
			}
		}
    }

    /// Wait for an event and return it.
    ///
    /// This function is blocking: if there's no pending event then
    /// it will wait until an event is received.
    /// After this function returns (and no error occured),
    /// the event object is always valid and filled properly.
    /// This function is typically used when you have a thread that
    /// is dedicated to events handling: you want to make this thread
    /// sleep as long as no new event is received.
    ///
    /// Returns Some(event), or None if an error has occured.
    pub fn wait_event(&mut self) -> Option<Event> {
		let mut event = sfEvent::new();
        let have_event = unsafe {
            ffi::sfWindow_waitEvent(self.raw_mut(), &mut event).to_bool()
        };
		if have_event {
			event.wrap()
		} else {
			None
		}
    }

    /// Change the window's icon.
	///
	/// The width and height must be explicitly specified. `pixels` must be an
	/// array of `width` x `height` pixels in 32-bit RGBA format.
    pub fn set_icon(&mut self, width: u32, height: u32, pixels: &[u8]) {
		if pixels.len() != width as usize * height as usize * 4 {
			// TODO: emit an error in a more sane way
			panic!("set_icon was passed ({}, {}), but got {} instead of {} bytes", width, height, pixels.len(), width * height * 4);
		}
        unsafe {
            ffi::sfWindow_setIcon(self.raw_mut(), width as c_uint, height as c_uint, pixels.as_ptr())
        }
    }

    /// Close the window and destroy all the attached resources.
    ///
    /// After calling this method, the Window object remains
    /// valid.
    /// All other functions such as `poll_event()` or `display()`
    /// will still work (i.e. you don't have to test `is_open()`
    /// every time), and will have no effect on closed windows.
    pub fn close(&mut self) {
        unsafe {
            ffi::sfWindow_close(self.raw_mut());
        }
    }

    /// Tell whether or not the window is open.
    ///
    /// This function returns whether or not the window exists.
    /// Note that a hidden window (`set_visible(false)`) will return
    /// true.
    pub fn is_open(&self) -> bool {
        unsafe { ffi::sfWindow_isOpen(self.raw()) }.to_bool()
    }

    /// Get the settings of the OpenGL context of the window.
    ///
    /// Note that these settings may be different from what was
    /// passed to `Window::new`, if one or more settings were not supported.
	/// In this case, SFML chose the closest match.
    pub fn get_settings(&self) -> ContextSettings {
        unsafe { ffi::sfWindow_getSettings(self.raw()) }
    }

    /// Change the title of the window.
    pub fn set_title(&mut self, title: &str) -> () {
		let vec = ::ffi::to_utf32(title);
        unsafe {
            ffi::sfWindow_setUnicodeTitle(self.raw_mut(), vec.as_ptr())
        }
    }

	/// Show or hide the window.
	///
	/// The window is shown by default.
    pub fn set_visible(&mut self, visible: bool) -> () {
        unsafe {
            ffi::sfWindow_setVisible(self.raw_mut(), SfBool::from_bool(visible))
        }
    }

    /// Show or hide the mouse cursor.
	///
	/// The mouse cursor is visible by default.
    pub fn set_mouse_cursor_visible(&mut self, visible: bool) -> () {
        unsafe {
            ffi::sfWindow_setMouseCursorVisible(self.raw_mut(), SfBool::from_bool(visible))
        }
    }

    /// Enable or disable vertical synchronization.
    ///
    /// Activating vertical synchronization will limit the number
    /// of frames displayed to the refresh rate of the monitor.
    /// This can avoid some visual artifacts, and limit the framerate
    /// to a good value (but not constant across different computers).
    ///
	/// Vertical synchronization is disabled by default.
    pub fn set_vertical_sync_enabled(&mut self, enabled: bool) -> () {
        unsafe {
            ffi::sfWindow_setVerticalSyncEnabled(self.raw_mut(), SfBool::from_bool(enabled))
        }
    }

    /// Enable or disable automatic key-repeat.
    ///
    /// If key repeat is enabled, you will receive repeated
    /// KeyPress events while keeping a key pressed. If it is disabled,
    /// you will only get a single event when the key is pressed.
    ///
    /// Key repeat is enabled by default.
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) -> () {
        unsafe {
            ffi::sfWindow_setKeyRepeatEnabled(self.raw_mut(), SfBool::from_bool(enabled))
        }
    }

    /// Activate or deactivate the window as the current target for OpenGL
	/// rendering.
    ///
    /// A window is active only on the current thread; if you want to
    /// make it active on another thread you have to deactivate it
    /// on the previous thread first.
    /// Only one window can be active on a thread at a time, so
    /// the window previously active (if any) automatically gets deactivated.
    ///
	/// Returns true if the operation was successful.
    pub fn set_active(&mut self, active: bool) -> bool {
        unsafe { ffi::sfWindow_setActive(self.raw_mut(), SfBool::from_bool(active)) }.to_bool()
    }

    /// Display on screen what has been rendered to the window so far.
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    pub fn display(&mut self) -> () {
        unsafe {
            ffi::sfWindow_display(self.raw_mut())
        }
    }

    /// Limit the framerate to a maximum fixed frequency.
    ///
    /// If a limit is set, the window will use a small delay after
    /// each call to `display()` to ensure that the current frame
    /// lasted long enough to match the framerate limit. SFML will try to match
	/// the given limit as long as it can, but since it internally uses a sleep
	/// whose precision relies on the OS, the results may be somewhat imprecise
	/// as well.
    ///
	/// Limit should be specified in frames per second, or may be `0` to disable
	/// the framerate limit.
    pub fn set_framerate_limit(&mut self, limit: u32) {
        unsafe {
            ffi::sfWindow_setFramerateLimit(self.raw_mut(), limit as c_uint)
        }
    }

    /// Change the joystick threshold.
    ///
    /// The joystick threshold is the value in the range [0, 100] below which
    /// no JoystickMoved events will be generated.
	///
	/// The threshold value is 0.1 by default.
    pub fn set_joystick_threshold(&mut self, threshold: f32) -> () {
        unsafe {
            ffi::sfWindow_setJoystickThreshold(self.raw_mut(), threshold as c_float)
        }
    }

    /// Get the position of the window on-screen, in pixels.
    pub fn get_position(&self) -> Vector2i {
        unsafe {
            ffi::sfWindow_getPosition(self.raw())
        }
    }

    /// Change the position of the window on-screen, in pixels.
    ///
    /// This function only works for top-level windows
    /// (i.e. it will be ignored for windows created from
    /// the handle of a child window/control).
    pub fn set_position(&mut self, position: &Vector2i) -> () {
        unsafe {
            ffi::sfWindow_setPosition(self.raw_mut(), *position)
        }
    }

    /// Get the size of the rendering region of the window, in pixels.
    ///
    /// The size doesn't include the titlebar and borders of the window.
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfWindow_getSize(self.raw())
        }
    }

    /// Change the size of the rendering region of the window, in pixels.
    pub fn set_size(&mut self, size: &Vector2u) -> () {
        unsafe {
            ffi::sfWindow_setSize(self.raw_mut(), *size)
        }
    }

    /// Get the current position of the mouse, relative to this window.
    pub fn get_mouse_position(&self) -> Vector2i {
        unsafe {
            ffi::sfMouse_getPosition(self.raw())
        }
    }

	/// Set the current position of the mouse, relative to this window.
    pub fn set_mouse_position(&mut self, position: &Vector2i) -> () {
        unsafe {
            ffi::sfMouse_setPosition(*position, self.raw_mut())
        }
    }
}
