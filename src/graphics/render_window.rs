use crate::graphics::csfml_graphics_sys as ffi;
use crate::graphics::{
    CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType, RectangleShape,
    RenderStates, RenderTarget, Sprite, Text, Vertex, VertexArray, View,
};
use crate::sf_bool_ext::SfBoolExt;
use crate::system::{SfString, Vector2f, Vector2i, Vector2u};
use crate::window::{ContextSettings, Event, Handle, Style, VideoMode};
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
    render_window: *mut ffi::sfRenderWindow,
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
    pub fn new<V: Into<VideoMode>, S: Into<SfString>>(
        mode: V,
        title: S,
        style: Style,
        settings: &ContextSettings,
    ) -> RenderWindow {
        let utf32 = title.into();
        let sf_render_win: *mut ffi::sfRenderWindow = unsafe {
            ffi::sfRenderWindow_createUnicode(
                mode.into().raw(),
                utf32.as_ptr(),
                style.bits(),
                &settings.raw(),
            )
        };
        assert!(!sf_render_win.is_null(), "Failed to create RenderWindow");
        RenderWindow {
            render_window: sf_render_win,
        }
    }

    /// Create a render window from an existing platform-specific window handle
    ///
    /// This function creates a render window based on an existing platform
    /// specific window handle which has been allocated outside of SFML. This is
    /// only intended to be used in cases where you need to integrate SFML with
    /// some other windowing library.
    ///
    /// This function is unsafe because it is the caller's responsibility to
    /// ensure that it is called with a valid window handle.
    ///
    /// # Arguments
    /// * handle - The handle to the platform-specific window handle to use for
    ///            the window.
    /// * settings - Additional settings for the underlying OpenGL context
    pub unsafe fn from_handle(handle: Handle, settings: &ContextSettings) -> RenderWindow {
        let sf_render_win: *mut ffi::sfRenderWindow =
            ffi::sfRenderWindow_createFromHandle(handle, &settings.raw());
        assert!(!sf_render_win.is_null(), "Failed to create Window");
        RenderWindow {
            render_window: sf_render_win,
        }
    }

    /// Change a render window's icon
    /// pixels must be an array of width x height pixels in 32-bits RGBA format.
    ///
    /// # Arguments
    /// * width - Icon's width, in pixels
    /// * height - Icon's height, in pixels
    /// * pixels - Vector of pixels
    pub fn set_icon(&mut self, width: u32, height: u32, pixels: &[u8]) {
        unsafe { ffi::sfRenderWindow_setIcon(self.render_window, width, height, pixels.as_ptr()) }
    }

    /// Pop the event on top of event queue, if any, and return it
    ///
    /// This function is not blocking: if there's no pending event then
    /// it will return false and leave \a event unmodified.
    /// Note that more than one event may be present in the event queue,
    /// thus you should always call this function in a loop
    /// to make sure that you process every pending event.
    ///
    /// Return Some(event) if an event was returned, or None if the event queue was empty
    pub fn poll_event(&mut self) -> Option<Event> {
        let mut event = unsafe { ::std::mem::uninitialized() };
        let have_event =
            unsafe { ffi::sfRenderWindow_pollEvent(self.render_window, &mut event) }.to_bool();
        if have_event {
            unsafe { Event::from_raw(&event) }
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
        let mut event = unsafe { ::std::mem::uninitialized() };
        let have_event =
            unsafe { ffi::sfRenderWindow_waitEvent(self.render_window, &mut event) }.to_bool();
        if have_event {
            unsafe { Event::from_raw(&event) }
        } else {
            None
        }
    }

    /// Close a render window and destroy all the attached resources
    ///
    /// After calling this method, the Window object remains
    /// valid.
    /// All other functions such as poll_event or display
    /// will still work (i.e. you don't have to test is_open
    /// every time), and will have no effect on closed windows.
    pub fn close(&mut self) {
        unsafe {
            ffi::sfRenderWindow_close(self.render_window);
        }
    }

    /// Tell whether or not a window is opened
    ///
    /// This function returns whether or not the window exists.
    /// Note that a hidden window (set_visible(false)) will return
    /// true.
    ///
    pub fn is_open(&self) -> bool {
        unsafe { ffi::sfRenderWindow_isOpen(self.render_window) }.to_bool()
    }

    /// Display on screen what has been rendered to the window so far
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    ///
    pub fn display(&mut self) {
        unsafe { ffi::sfRenderWindow_display(self.render_window) }
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
        unsafe { ffi::sfRenderWindow_setFramerateLimit(self.render_window, limit) }
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
    pub fn settings(&self) -> ContextSettings {
        unsafe { ContextSettings::from_raw(ffi::sfRenderWindow_getSettings(self.render_window)) }
    }

    /// Change the title of a window
    ///
    /// # Arguments
    /// * title - New title
    ///
    pub fn set_title<S: Into<SfString>>(&mut self, title: S) {
        let utf32 = title.into();
        unsafe {
            ffi::sfRenderWindow_setUnicodeTitle(self.render_window, utf32.as_ptr());
        }
    }

    /// Show or hide a window
    ///
    /// # Arguments
    /// * visible - true to show the window, false to hide it
    ///
    pub fn set_visible(&mut self, visible: bool) {
        unsafe {
            ffi::sfRenderWindow_setVisible(self.render_window, sfBool::from_bool(visible));
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
                self.render_window,
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
                self.render_window,
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
                self.render_window,
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
            ffi::sfRenderWindow_setKeyRepeatEnabled(self.render_window, sfBool::from_bool(enabled));
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
        unsafe { ffi::sfRenderWindow_setActive(self.render_window, sfBool::from_bool(enabled)) }
            .to_bool()
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
        unsafe { ffi::sfRenderWindow_setJoystickThreshold(self.render_window, threshold) }
    }

    /// Get the position of a window
    ///
    /// Return the position in pixels
    ///
    pub fn position(&self) -> Vector2i {
        unsafe { Vector2i::from_raw(ffi::sfRenderWindow_getPosition(self.render_window)) }
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
        unsafe { ffi::sfRenderWindow_setPosition(self.render_window, position.raw()) }
    }

    /// Change the size of the rendering region of a window
    ///
    /// # Arguments
    /// * size - New size, in pixels
    ///
    pub fn set_size<S: Into<Vector2u>>(&mut self, size: S) {
        unsafe { ffi::sfRenderWindow_setSize(self.render_window, size.into().raw()) }
    }

    /// Returns the current position of the mouse relative to the window.
    pub fn mouse_position(&self) -> Vector2i {
        unsafe { Vector2i::from_raw(ffi::sfMouse_getPositionRenderWindow(self.render_window)) }
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
        unsafe { ffi::sfMouse_setPositionRenderWindow(position.raw(), self.render_window) }
    }

    /// Returns the current position of a touch in window coordinates.
    pub fn touch_position(&self, finger: u32) -> Vector2i {
        unsafe {
            Vector2i::from_raw(ffi::sfTouch_getPositionRenderWindow(
                finger,
                self.render_window,
            ))
        }
    }

    /// Check whether the window has the input focus.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or most mouse events.
    pub fn has_focus(&self) -> bool {
        unsafe { ffi::sfRenderWindow_hasFocus(self.render_window).to_bool() }
    }

    /// Request the current window to be made the active foreground window.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or mouse events. If a window requests focus, it only hints to the
    /// operating system, that it would like to be focused. The operating system is free to
    /// deny the request. This is not to be confused with [`RenderWindow::set_active`].
    pub fn request_focus(&self) {
        unsafe { ffi::sfRenderWindow_requestFocus(self.render_window) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfRenderWindow {
        self.render_window
    }
}

impl RenderTarget for RenderWindow {
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_pushGLStates(self.render_window) }
    }
    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_popGLStates(self.render_window) }
    }
    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_resetGLStates(self.render_window) }
    }
    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderWindow_setView(self.render_window, view.raw()) }
    }
    fn view(&self) -> &View {
        unsafe { &*(ffi::sfRenderWindow_getView(self.render_window) as *const View) }
    }
    fn default_view(&self) -> &View {
        unsafe { &*(ffi::sfRenderWindow_getDefaultView(self.render_window) as *const View) }
    }
    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfRenderWindow_mapPixelToCoords(
                self.render_window,
                point.raw(),
                view.raw(),
            ))
        }
    }
    fn map_pixel_to_coords_current_view(&self, point: Vector2i) -> Vector2f {
        let view = unsafe { ffi::sfRenderWindow_getView(self.render_window) };
        unsafe {
            Vector2f::from_raw(ffi::sfRenderWindow_mapPixelToCoords(
                self.render_window,
                point.raw(),
                view,
            ))
        }
    }
    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        unsafe {
            Vector2i::from_raw(ffi::sfRenderWindow_mapCoordsToPixel(
                self.render_window,
                point.raw(),
                view.raw(),
            ))
        }
    }
    fn map_coords_to_pixel_current_view(&self, point: Vector2f) -> Vector2i {
        let curr_view = unsafe { ffi::sfRenderWindow_getView(self.render_window) };
        unsafe {
            Vector2i::from_raw(ffi::sfRenderWindow_mapCoordsToPixel(
                self.render_window,
                point.raw(),
                curr_view,
            ))
        }
    }
    fn viewport(&self, view: &View) -> IntRect {
        unsafe {
            IntRect::from_raw(ffi::sfRenderWindow_getViewport(
                self.render_window,
                view.raw(),
            ))
        }
    }
    fn size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfRenderWindow_getSize(self.render_window)) }
    }
    fn draw(&mut self, object: &Drawable) {
        object.draw(self, RenderStates::default());
    }
    fn draw_with_renderstates(&mut self, object: &Drawable, render_states: RenderStates) {
        object.draw(self, render_states);
    }
    fn draw_text(&self, text: &Text, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window, text.raw(), &render_states.raw())
        }
    }
    fn draw_shape(&self, shape: &CustomShape, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawShape(self.render_window, shape.raw(), &render_states.raw())
        }
    }
    fn draw_sprite(&self, sprite: &Sprite, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window, sprite.raw(), &render_states.raw())
        }
    }
    fn draw_circle_shape(&self, circle_shape: &CircleShape, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(
                self.render_window,
                circle_shape.raw(),
                &render_states.raw(),
            )
        }
    }
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(
                self.render_window,
                rectangle_shape.raw(),
                &render_states.raw(),
            )
        }
    }
    fn draw_convex_shape(&self, convex_shape: &ConvexShape, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(
                self.render_window,
                convex_shape.raw(),
                &render_states.raw(),
            )
        }
    }
    fn draw_vertex_array(&self, vertex_array: &VertexArray, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawVertexArray(
                self.render_window,
                vertex_array.raw(),
                &render_states.raw(),
            )
        }
    }
    fn draw_primitives(&self, vertices: &[Vertex], ty: PrimitiveType, rs: RenderStates) {
        let len = vertices.len();
        unsafe {
            ffi::sfRenderWindow_drawPrimitives(
                self.render_window,
                vertices.as_ptr() as *const _,
                len,
                ty.raw(),
                &rs.raw(),
            );
        }
    }
    fn clear(&mut self, color: Color) {
        unsafe { ffi::sfRenderWindow_clear(self.render_window, color.raw()) }
    }
}

impl Drop for RenderWindow {
    fn drop(&mut self) {
        unsafe {
            ffi::sfRenderWindow_destroy(self.render_window);
        }
    }
}
