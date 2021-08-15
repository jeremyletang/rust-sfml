use std::ptr::NonNull;

use crate::{
    graphics::{
        CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType,
        RectangleShape, RenderStates, RenderTarget, Sprite, Text, Vertex, VertexBuffer, View,
    },
    sf_bool_ext::SfBoolExt,
    system::{SfStrConv, Vector2f, Vector2i, Vector2u},
    window::{thread_safety, ContextSettings, Cursor, Event, Handle, Style, VideoMode},
};
use csfml_graphics_sys as ffi;
use csfml_system_sys::*;

/// [`Window`] that can serve as a target for 2D drawing.
///
/// `RenderWindow` is the main type of the graphics module.
/// It defines an OS window that can be painted using the other classes
/// of the graphics module.
///
/// [`Window`]: crate::window::Window
#[derive(Debug)]
pub struct RenderWindow {
    render_window: NonNull<ffi::sfRenderWindow>,
}

impl RenderWindow {
    /// Construct a new render window
    ///
    /// This function creates the render window with the size and pixel
    /// depth defined in mode. An optional style can be passed to
    /// customize the look and behaviour of the window (borders,
    /// title bar, resizable, closable, ...). If style contains
    /// [`Style::FULLSCREEN`], then mode must be a valid video mode.
    ///
    /// The fourth parameter is a pointer to a structure specifying
    /// advanced OpenGL context settings such as antialiasing,
    /// depth-buffer bits, etc.
    ///
    /// # Arguments
    /// * mode - Video mode to use (defines the width, height and depth of the
    ///                             rendering area of the render window)
    /// * title - Title of the render window
    /// * style - Window style
    /// * settings - Additional settings for the underlying OpenGL context
    pub fn new<V: Into<VideoMode>, S: SfStrConv>(
        mode: V,
        title: S,
        style: Style,
        settings: &ContextSettings,
    ) -> RenderWindow {
        thread_safety::set_window_thread();

        title.with_as_sfstr(|sfstr| {
            let sf_render_win: *mut ffi::sfRenderWindow = unsafe {
                ffi::sfRenderWindow_createUnicode(
                    mode.into().raw(),
                    sfstr.as_ptr(),
                    style.bits(),
                    &settings.0,
                )
            };
            RenderWindow {
                render_window: NonNull::new(sf_render_win).expect("Failed to create RenderWindow"),
            }
        })
    }

    /// Create a render window from an existing platform-specific window handle
    ///
    /// This function creates a render window based on an existing platform
    /// specific window handle which has been allocated outside of SFML. This is
    /// only intended to be used in cases where you need to integrate SFML with
    /// some other windowing library.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that it is called with a valid window handle.
    ///
    /// # Arguments
    /// * handle - The handle to the platform-specific window handle to use for
    ///            the window.
    /// * settings - Additional settings for the underlying OpenGL context
    #[must_use]
    pub unsafe fn from_handle(handle: Handle, settings: &ContextSettings) -> RenderWindow {
        thread_safety::set_window_thread();

        let sf_render_win: *mut ffi::sfRenderWindow =
            ffi::sfRenderWindow_createFromHandle(handle, &settings.0);
        RenderWindow {
            render_window: NonNull::new(sf_render_win).expect("Failed to create Window"),
        }
    }

    /// Get the OS-specific handle of the window.
    ///
    /// The type of the returned handle is Handle, which is a typedef to the handle type defined by the OS.
    /// You shouldn't need to use this function, unless you have very specific stuff to implement that SFML
    /// doesn't support, or implement a temporary workaround until a bug is fixed.
    #[must_use]
    pub fn system_handle(&self) -> Handle {
        unsafe { ffi::sfRenderWindow_getSystemHandle(self.render_window.as_ptr()) }
    }

    /// Change a render window's icon
    /// pixels must be an array of width x height pixels in 32-bits RGBA format.
    ///
    /// # Arguments
    /// * width - Icon's width, in pixels
    /// * height - Icon's height, in pixels
    /// * pixels - Vector of pixels
    pub fn set_icon(&mut self, width: u32, height: u32, pixels: &[u8]) {
        unsafe {
            ffi::sfRenderWindow_setIcon(self.render_window.as_ptr(), width, height, pixels.as_ptr())
        }
    }

    /// Pop the event on top of event queue, if any, and return it
    ///
    /// This function is not blocking: if there's no pending event then
    /// it will return `None`.
    /// Note that more than one event may be present in the event queue,
    /// thus you should always call this function in a loop
    /// to make sure that you process every pending event.
    ///
    /// Returns `Some(event)` if an event was returned, or `None` if the event queue was empty
    pub fn poll_event(&mut self) -> Option<Event> {
        let mut event = std::mem::MaybeUninit::uninit();
        let have_event = unsafe {
            ffi::sfRenderWindow_pollEvent(self.render_window.as_ptr(), event.as_mut_ptr())
        }
        .into_bool();
        if have_event {
            unsafe { Event::from_raw(&event.assume_init()) }
        } else {
            None
        }
    }

    /// Wait for an event and return it
    ///
    /// This function is blocking: if there's no pending event then
    /// it will wait until an event is received.
    ///
    /// This function is typically used when you have a thread that
    /// is dedicated to events handling: you want to make this thread
    /// sleep as long as no new event is received.
    ///
    /// Returns `Some(event)` or `None` if an error has occured
    pub fn wait_event(&mut self) -> Option<Event> {
        let mut event = std::mem::MaybeUninit::uninit();
        let have_event = unsafe {
            ffi::sfRenderWindow_waitEvent(self.render_window.as_ptr(), event.as_mut_ptr())
        }
        .into_bool();
        if have_event {
            unsafe { Event::from_raw(&event.assume_init()) }
        } else {
            None
        }
    }

    /// Close a render window and destroy all the attached resources
    ///
    /// After calling this method, the Window object remains
    /// valid.
    /// All other functions such as `poll_event` or display
    /// will still work (i.e. you don't have to test `is_open`
    /// every time), and will have no effect on closed windows.
    pub fn close(&mut self) {
        unsafe {
            ffi::sfRenderWindow_close(self.render_window.as_ptr());
        }
    }

    /// Tell whether or not a window is opened
    ///
    /// This function returns whether or not the window exists.
    /// Note that a hidden window `(set_visible(false))` will return
    /// true.
    ///
    #[must_use]
    pub fn is_open(&self) -> bool {
        unsafe { ffi::sfRenderWindow_isOpen(self.render_window.as_ptr()) }.into_bool()
    }

    /// Display on screen what has been rendered to the window so far
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    ///
    pub fn display(&mut self) {
        unsafe { ffi::sfRenderWindow_display(self.render_window.as_ptr()) }
    }

    /// Limit the framerate to a maximum fixed frequency
    ///
    /// If a limit is set, the window will use a small delay after
    /// each call to [`RenderWindow::display`] to ensure that the current frame
    /// lasted long enough to match the framerate limit.
    ///
    /// # Arguments
    /// * limit - Framerate limit, in frames per seconds (use 0 to disable limit)
    ///
    pub fn set_framerate_limit(&mut self, limit: u32) {
        unsafe { ffi::sfRenderWindow_setFramerateLimit(self.render_window.as_ptr(), limit) }
    }

    /// Get the settings of the OpenGL context of a window
    ///
    /// Note that these settings may be different from what was
    /// passed to the [`RenderWindow::new`] function,
    /// if one or more settings were not supported. In this case,
    /// SFML chose the closest match.
    ///
    /// Return a structure containing the OpenGL context settings
    ///
    #[must_use]
    pub fn settings(&self) -> ContextSettings {
        unsafe { ContextSettings(ffi::sfRenderWindow_getSettings(self.render_window.as_ptr())) }
    }

    /// Change the title of a window
    ///
    /// # Arguments
    /// * title - New title
    ///
    pub fn set_title<S: SfStrConv>(&mut self, title: S) {
        title.with_as_sfstr(|sfstr| unsafe {
            ffi::sfRenderWindow_setUnicodeTitle(self.render_window.as_ptr(), sfstr.as_ptr());
        })
    }

    /// Show or hide a window
    ///
    /// # Arguments
    /// * visible - true to show the window, false to hide it
    ///
    pub fn set_visible(&mut self, visible: bool) {
        unsafe {
            ffi::sfRenderWindow_setVisible(self.render_window.as_ptr(), sfBool::from_bool(visible));
        }
    }

    /// Show or hide the mouse cursor
    ///
    /// # Arguments
    /// * visible - true to  false to hide
    ///
    pub fn set_mouse_cursor_visible(&mut self, visible: bool) {
        unsafe {
            ffi::sfRenderWindow_setMouseCursorVisible(
                self.render_window.as_ptr(),
                sfBool::from_bool(visible),
            );
        }
    }

    /// Grab or release the mouse cursor.
    ///
    /// If set, grabs the mouse cursor inside this window's client area so it may no longer be
    /// moved outside its bounds. Note that grabbing is only active while the window has focus.
    pub fn set_mouse_cursor_grabbed(&mut self, grabbed: bool) {
        unsafe {
            ffi::sfRenderWindow_setMouseCursorGrabbed(
                self.render_window.as_ptr(),
                sfBool::from_bool(grabbed),
            )
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
    ///
    pub fn set_vertical_sync_enabled(&mut self, enabled: bool) {
        unsafe {
            ffi::sfRenderWindow_setVerticalSyncEnabled(
                self.render_window.as_ptr(),
                sfBool::from_bool(enabled),
            );
        }
    }

    /// Enable or disable automatic key-repeat
    ///
    /// If key repeat is enabled, you will receive repeated
    /// [`crate::window::Event::KeyPressed`] events while keeping a key pressed.
    /// If it is disabled, you will only get a single event when the key is pressed.
    ///
    /// Key repeat is enabled by default.
    ///
    /// # Arguments
    /// * enabled - true to enable, false to disable
    ///
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) {
        unsafe {
            ffi::sfRenderWindow_setKeyRepeatEnabled(
                self.render_window.as_ptr(),
                sfBool::from_bool(enabled),
            );
        }
    }

    /// Activate or deactivate a render window as the current target for OpenGL rendering
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
    ///
    pub fn set_active(&mut self, enabled: bool) -> bool {
        unsafe {
            ffi::sfRenderWindow_setActive(self.render_window.as_ptr(), sfBool::from_bool(enabled))
        }
        .into_bool()
    }

    /// Change the joystick threshold
    ///
    /// The joystick threshold is the value below which
    /// no [`crate::window::Event::JoystickMoved`] event will be generated.
    ///
    /// # Arguments
    /// * threshold - New threshold, in the range [0, 100]
    ///
    pub fn set_joystick_threshold(&mut self, threshold: f32) {
        unsafe { ffi::sfRenderWindow_setJoystickThreshold(self.render_window.as_ptr(), threshold) }
    }

    /// Get the position of a window
    ///
    /// Return the position in pixels
    ///
    #[must_use]
    pub fn position(&self) -> Vector2i {
        unsafe { Vector2i::from_raw(ffi::sfRenderWindow_getPosition(self.render_window.as_ptr())) }
    }

    /// Change the position of a window on screen
    ///
    /// This function only works for top-level windows
    /// (i.e. it will be ignored for windows created from
    /// the handle of a child window/control).
    ///
    /// # Arguments
    /// * position - New position of the window, in pixels
    ///
    pub fn set_position(&mut self, position: Vector2i) {
        unsafe { ffi::sfRenderWindow_setPosition(self.render_window.as_ptr(), position.raw()) }
    }

    /// Change the size of the rendering region of a window
    ///
    /// # Arguments
    /// * size - New size, in pixels
    ///
    pub fn set_size<S: Into<Vector2u>>(&mut self, size: S) {
        unsafe { ffi::sfRenderWindow_setSize(self.render_window.as_ptr(), size.into().raw()) }
    }

    /// Returns the current position of the mouse relative to the window.
    #[must_use]
    pub fn mouse_position(&self) -> Vector2i {
        unsafe {
            Vector2i::from_raw(ffi::sfMouse_getPositionRenderWindow(
                self.render_window.as_ptr(),
            ))
        }
    }

    /// Set the current position of the mouse relatively to a render window
    ///
    /// This function sets the current position of the mouse cursor relative
    /// to the given render window
    ///
    /// # Arguments
    /// * `position` - the positon to set
    ///
    pub fn set_mouse_position(&mut self, position: Vector2i) {
        unsafe { ffi::sfMouse_setPositionRenderWindow(position.raw(), self.render_window.as_ptr()) }
    }

    /// Set the displayed cursor to a native system cursor.
    ///
    /// Upon window creation, the arrow cursor is used by default.
    /// The cursor can not be destroyed while in use by the window.
    pub fn set_mouse_cursor(&mut self, cursor: &Cursor) {
        unsafe { ffi::sfRenderWindow_setMouseCursor(self.render_window.as_ptr(), cursor.raw()) }
    }

    /// Returns the current position of a touch in window coordinates.
    #[must_use]
    pub fn touch_position(&self, finger: u32) -> Vector2i {
        unsafe {
            Vector2i::from_raw(ffi::sfTouch_getPositionRenderWindow(
                finger,
                self.render_window.as_ptr(),
            ))
        }
    }

    /// Check whether the window has the input focus.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or most mouse events.
    #[must_use]
    pub fn has_focus(&self) -> bool {
        unsafe { ffi::sfRenderWindow_hasFocus(self.render_window.as_ptr()).into_bool() }
    }

    /// Request the current window to be made the active foreground window.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or mouse events. If a window requests focus, it only hints to the
    /// operating system, that it would like to be focused. The operating system is free to
    /// deny the request. This is not to be confused with [`RenderWindow::set_active`].
    pub fn request_focus(&self) {
        unsafe { ffi::sfRenderWindow_requestFocus(self.render_window.as_ptr()) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfRenderWindow {
        self.render_window.as_ptr()
    }
}

impl RenderTarget for RenderWindow {
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_pushGLStates(self.render_window.as_ptr()) }
    }
    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_popGLStates(self.render_window.as_ptr()) }
    }
    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_resetGLStates(self.render_window.as_ptr()) }
    }
    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderWindow_setView(self.render_window.as_ptr(), view.raw()) }
    }
    fn view(&self) -> &View {
        unsafe { &*(ffi::sfRenderWindow_getView(self.render_window.as_ptr()) as *const View) }
    }
    fn default_view(&self) -> &View {
        unsafe {
            &*(ffi::sfRenderWindow_getDefaultView(self.render_window.as_ptr()) as *const View)
        }
    }
    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfRenderWindow_mapPixelToCoords(
                self.render_window.as_ptr(),
                point.raw(),
                view.raw(),
            ))
        }
    }
    fn map_pixel_to_coords_current_view(&self, point: Vector2i) -> Vector2f {
        let view = unsafe { ffi::sfRenderWindow_getView(self.render_window.as_ptr()) };
        unsafe {
            Vector2f::from_raw(ffi::sfRenderWindow_mapPixelToCoords(
                self.render_window.as_ptr(),
                point.raw(),
                view,
            ))
        }
    }
    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        unsafe {
            Vector2i::from_raw(ffi::sfRenderWindow_mapCoordsToPixel(
                self.render_window.as_ptr(),
                point.raw(),
                view.raw(),
            ))
        }
    }
    fn map_coords_to_pixel_current_view(&self, point: Vector2f) -> Vector2i {
        let curr_view = unsafe { ffi::sfRenderWindow_getView(self.render_window.as_ptr()) };
        unsafe {
            Vector2i::from_raw(ffi::sfRenderWindow_mapCoordsToPixel(
                self.render_window.as_ptr(),
                point.raw(),
                curr_view,
            ))
        }
    }
    fn viewport(&self, view: &View) -> IntRect {
        unsafe {
            IntRect::from_raw(ffi::sfRenderWindow_getViewport(
                self.render_window.as_ptr(),
                view.raw(),
            ))
        }
    }
    fn size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfRenderWindow_getSize(self.render_window.as_ptr())) }
    }
    fn draw(&mut self, object: &dyn Drawable) {
        object.draw(self, &RenderStates::DEFAULT);
    }
    fn draw_with_renderstates(&mut self, object: &dyn Drawable, render_states: &RenderStates) {
        object.draw(self, render_states);
    }
    fn draw_text(&self, text: &Text, render_states: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawText(
                self.render_window.as_ptr(),
                text.raw(),
                render_states.raw_ref(),
            )
        }
    }
    fn draw_shape(&self, shape: &CustomShape, render_states: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawShape(
                self.render_window.as_ptr(),
                shape.raw(),
                render_states.raw_ref(),
            )
        }
    }
    fn draw_sprite(&self, sprite: &Sprite, render_states: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawSprite(
                self.render_window.as_ptr(),
                sprite.raw(),
                render_states.raw_ref(),
            )
        }
    }
    fn draw_circle_shape(&self, circle_shape: &CircleShape, render_states: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(
                self.render_window.as_ptr(),
                circle_shape.raw(),
                render_states.raw_ref(),
            )
        }
    }
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape, render_states: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(
                self.render_window.as_ptr(),
                rectangle_shape.raw(),
                render_states.raw_ref(),
            )
        }
    }
    fn draw_convex_shape(&self, convex_shape: &ConvexShape, render_states: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(
                self.render_window.as_ptr(),
                convex_shape.raw(),
                render_states.raw_ref(),
            )
        }
    }
    fn draw_vertex_buffer(&self, vertex_buffer: &VertexBuffer, render_states: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawVertexBuffer(
                self.render_window.as_ptr(),
                vertex_buffer.raw(),
                render_states.raw_ref(),
            )
        }
    }
    fn draw_primitives(&self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates) {
        let len = vertices.len();
        unsafe {
            ffi::sfRenderWindow_drawPrimitives(
                self.render_window.as_ptr(),
                vertices.as_ptr() as *const _,
                len,
                ty.0,
                rs.raw_ref(),
            );
        }
    }
    fn clear(&mut self, color: Color) {
        unsafe { ffi::sfRenderWindow_clear(self.render_window.as_ptr(), color.0) }
    }
}

impl Drop for RenderWindow {
    fn drop(&mut self) {
        unsafe {
            ffi::sfRenderWindow_destroy(self.render_window.as_ptr());
        }
    }
}
