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

use libc::{c_float, c_uint};

use window::{ContextSettings, VideoMode, Event, WindowStyle};
use window::raw_event::sfEvent;
use system::{Vector2f, Vector2i, Vector2u};
use graphics::{Color, CircleShape, RectangleShape, Text, Sprite,
               RenderStates, View, Image, IntRect, RenderTarget,
               Vertex, PrimitiveType, BaseShape};

use ffi::{SfBool, Foreign, Ref};
use ffi::graphics as ffi;

/// Window that can serve as a target for 2D drawing.
///
/// `RenderWindow` is the main class of the Graphics module, and defines an OS
/// window that can be painted to using the other types of the graphics module.
/// It derives from `Window` and inherits its features: events, window
/// management, OpenGL rendering, etc. It also adds more features related to 2D
/// drawing with the graphics module (see `RenderTarget` for details).
pub struct RenderWindow(Foreign<ffi::sfRenderWindow>);

impl RenderWindow {
    /// Construct a new render window.
    ///
    /// This function creates the render window with the width, height, and
    /// pixel depth defined in `mode`. An optional style can be passed to
    /// customize the look and behaviour of the window (borders,
    /// title bar, resizable, closable, ...). If style contains
    /// `FULLSCREEN`, then mode must be a valid video mode.
    ///
	/// The `settings` parameter is a structure specifying advanced OpenGL
	/// context settings such as antialiasing, depth-buffer bits, etc. If you
	/// aren't doing your own OpenGL rendering and don't care about these
	/// settings, pass `ContextSettings::default()`.
    ///
    /// Returns Some(RenderWindow) or None on failure.
    pub fn new(mode: VideoMode, title: &str, style: WindowStyle, settings: ContextSettings) -> Option<RenderWindow> {
		let vec = ::ffi::to_utf32(title);
        unsafe {
            Foreign::new(ffi::sfRenderWindow_createUnicode(mode, vec.as_ptr(), style.bits(), &settings))
        }.map(RenderWindow)
    }

	fn raw(&self) -> &ffi::sfRenderWindow { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfRenderWindow { self.0.as_mut() }
    #[doc(hidden)]
    pub fn unwrap(&self) -> &ffi::sfRenderWindow { self.raw() }

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
            ffi::sfRenderWindow_setIcon(self.raw_mut(),
                                        width as c_uint,
                                        height as c_uint,
                                        pixels.as_ptr())
        }
    }

    /// Change the window's icon using an image.
	pub fn set_icon_image(&mut self, image: &Image) {
		let size = image.get_size();
		self.set_icon(size.x, size.y, image.get_pixels());
	}

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
				ffi::sfRenderWindow_pollEvent(self.raw_mut(), &mut event).to_bool()
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
            ffi::sfRenderWindow_waitEvent(self.raw_mut(), &mut event).to_bool()
        };
		if have_event {
			event.wrap()
		} else {
			None
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
        unsafe { ffi::sfRenderWindow_close(self.raw_mut()); }
    }

	/// Request the current window to be made the active foreground window.
	///
	/// At any given time, only one window may have the input focus to receive
	/// input events such as keystrokes or mouse events. If a window requests
	/// focus, it only hints to the operating system, that it would like to be
	/// focused. The operating system is free to deny the request. This is not
	/// to be confused with `set_active()`.
	pub fn request_focus(&mut self) {
		unsafe { ffi::sfRenderWindow_requestFocus(self.raw_mut()) }
	}

	/// Check whether the window has the input focus.
	///
	/// At any given time, only one window may have the input focus to receive
	/// input events such as keystrokes or most mouse events.
	pub fn has_focus(&self) -> bool {
		unsafe { ffi::sfRenderWindow_hasFocus(self.raw()) }.to_bool()
	}

    /// Tell whether or not the window is open.
    ///
    /// This function returns whether or not the window exists.
    /// Note that a hidden window (`set_visible(false)`) will return
    /// true.
    pub fn is_open(&self) -> bool {
        unsafe { ffi::sfRenderWindow_isOpen(self.raw()) }.to_bool()
    }

    /// Display on screen what has been rendered to the window so far.
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    pub fn display(&mut self) {
        unsafe {
            ffi::sfRenderWindow_display(self.raw_mut())
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
            ffi::sfRenderWindow_setFramerateLimit(self.raw_mut(), limit as c_uint)
        }
    }

    /// Get the settings of the OpenGL context of the window.
    ///
    /// Note that these settings may be different from what was
    /// passed to `Window::new`, if one or more settings were not supported.
	/// In this case, SFML chose the closest match.
    pub fn get_settings(&self) -> ContextSettings {
        unsafe {
            ffi::sfRenderWindow_getSettings(self.raw())
        }
    }

    /// Change the title of the window.
    pub fn set_title(&mut self, title: &str) {
		let vec = ::ffi::to_utf32(title);
        unsafe {
			ffi::sfRenderWindow_setUnicodeTitle(self.raw_mut(), vec.as_ptr());
        }
    }

	/// Show or hide the window.
	///
	/// The window is shown by default.
    pub fn set_visible(&mut self, visible: bool) {
        unsafe {
            ffi::sfRenderWindow_setVisible(self.raw_mut(), SfBool::from_bool(visible));
        }
    }

    /// Show or hide the mouse cursor.
	///
	/// The mouse cursor is visible by default.
    pub fn set_mouse_cursor_visible(&mut self, visible: bool) {
        unsafe {
            ffi::sfRenderWindow_setMouseCursorVisible(self.raw_mut(), SfBool::from_bool(visible));
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
    pub fn set_vertical_sync_enabled(&mut self, enabled: bool) {
        unsafe {
            ffi::sfRenderWindow_setVerticalSyncEnabled(self.raw_mut(), SfBool::from_bool(enabled));
        }
    }

    /// Enable or disable automatic key-repeat.
    ///
    /// If key repeat is enabled, you will receive repeated
    /// KeyPress events while keeping a key pressed. If it is disabled,
    /// you will only get a single event when the key is pressed.
    ///
    /// Key repeat is enabled by default.
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) {
        unsafe {
            ffi::sfRenderWindow_setKeyRepeatEnabled(self.raw_mut(), SfBool::from_bool(enabled));
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
    pub fn set_active(&mut self, enabled: bool) -> bool {
        unsafe {
            ffi::sfRenderWindow_setActive(self.raw_mut(), SfBool::from_bool(enabled))
        }.to_bool()
    }

    /// Change the joystick threshold.
    ///
    /// The joystick threshold is the value in the range [0, 100] below which
    /// no JoystickMoved events will be generated.
	///
	/// The threshold value is 0.1 by default.
    pub fn set_joystick_threshold(&mut self, threshold: f32) {
        unsafe {
            ffi::sfRenderWindow_setJoystickThreshold(self.raw_mut(), threshold as c_float)
        }
    }

    /// Get the position of the window on-screen, in pixels.
    pub fn get_position(&self) -> Vector2i {
        unsafe {
            ffi::sfRenderWindow_getPosition(self.raw())
        }
    }

    /// Change the position of the window on-screen, in pixels.
    ///
    /// This function only works for top-level windows
    /// (i.e. it will be ignored for windows created from
    /// the handle of a child window/control).
    pub fn set_position(&mut self, position: Vector2i) {
        unsafe {
            ffi::sfRenderWindow_setPosition(self.raw_mut(), position)
        }
    }

    /// Change the size of the rendering region of the window, in pixels.
    pub fn set_size(&mut self, size: Vector2u) {
        unsafe {
            ffi::sfRenderWindow_setSize(self.raw_mut(), size)
        }
    }

    /// Change the size of the rendering region of the window, in pixels.
	#[inline]
    pub fn set_size2u(&mut self, x: u32, y: u32) {
		self.set_size(Vector2u::new(x, y))
    }

    /// Get the current position of the mouse, relative to this window.
    pub fn get_mouse_position(&self) -> Vector2i {
        unsafe {
            ffi::sfMouse_getPositionRenderWindow(self.raw())
        }
    }

	/// Set the current position of the mouse, relative to this window.
    pub fn set_mouse_position(&mut self, position: Vector2i) {
        unsafe {
            ffi::sfMouse_setPositionRenderWindow(position, self.raw_mut())
        }
    }

    /// Copy the current contents of the window to an image.
    ///
    /// This is a slow operation, whose main purpose is to make
    /// screenshots of the application. If you want to update an
    /// image with the contents of the window and then use it for
    /// drawing, you should rather use a [Texture](struct.Texture.html) and its
    /// [`update(Window)`](struct.Texture.html#method.update_from_window)
	/// function. You can also draw things directly to a texture with the
    /// `RenderTexture` type.
    ///
    /// Returns a new image containing the captured contents.
    pub fn capture(&self) -> Option<Image> {
        unsafe {
			Image::wrap(ffi::sfRenderWindow_capture(self.raw()))
		}
    }
}

impl RenderTarget for RenderWindow {
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_pushGLStates(self.raw_mut()) }
    }

    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_popGLStates(self.raw_mut()) }
    }

    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_resetGLStates(self.raw_mut()) }
    }

    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderWindow_setView(self.raw_mut(), view.unwrap()) }
    }

    fn get_view(&self) -> Ref<View> {
        unsafe {
            Ref::new(ffi::sfRenderWindow_getView(self.raw())).expect("Failed to wrap view")
        }
    }

    fn get_default_view(&self) -> Ref<View> {
        unsafe {
            Ref::new(ffi::sfRenderWindow_getDefaultView(self.raw())).expect("Failed to wrap view")
        }
    }

    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        unsafe {
            ffi::sfRenderWindow_mapPixelToCoords(self.raw(), point, view.unwrap())
        }
    }

    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        unsafe {
            ffi::sfRenderWindow_mapCoordsToPixel(self.raw(), point, view.unwrap())
        }
    }

    fn get_viewport(&self, view: &View) -> IntRect {
        unsafe { ffi::sfRenderWindow_getViewport(self.raw(), view.unwrap()) }
    }

    fn get_size(&self) -> Vector2u {
        unsafe { ffi::sfRenderWindow_getSize(self.raw()) }
    }

    fn draw_text_rs(&mut self, text: &Text, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawText(self.raw_mut(), text.unwrap(), &rs.unwrap())
        }
    }

    fn draw_shape_rs(&mut self, shape: &BaseShape, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawShape(self.raw_mut(), shape.unwrap(), &rs.unwrap())
        }
    }

    fn draw_sprite_rs(&mut self, sprite: &Sprite, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.raw_mut(), sprite.unwrap(), &rs.unwrap())
        }
    }

    fn draw_circle_shape_rs(&mut self, circle: &CircleShape, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.raw_mut(), circle.unwrap(), &rs.unwrap())
        }
    }

    fn draw_rectangle_shape_rs(&mut self, rect: &RectangleShape, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.raw_mut(), rect.unwrap(), &rs.unwrap())
        }
    }

    fn draw_primitives_rs(&mut self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawPrimitives(self.raw_mut(),
                                               vertices.as_ptr(),
                                               vertices.len() as u32,
                                               ty,
                                               &rs.unwrap());
        }
    }

    fn clear(&mut self, color: Color) {
        unsafe { ffi::sfRenderWindow_clear(self.raw_mut(), color) }
    }
}
