use csfml_system_sys::*;
use graphics::{CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType,
               RectangleShape, RenderStates, RenderTarget, Sprite, Text, Vertex, VertexArray,
               View, ViewRef};
use graphics::csfml_graphics_sys as ffi;
use sf_bool_ext::SfBoolExt;
use std::marker::PhantomData;
use system::{Vector2f, Vector2i, Vector2u};
use system::raw_conv::{FromRaw, Raw};
use window::{ContextSettings, Event, Style, VideoMode};

/// Window that can serve as a target for 2D drawing.
///
/// `RenderWindow` is the main type of the graphics module.
/// It defines an OS window that can be painted using the other classes
/// of the graphics module.
#[derive(Debug)]
pub struct RenderWindow {
    render_window: *mut ffi::sfRenderWindow,
}

/// An iterator over all the events in the events queue (internally call `poll_event`)
#[derive(Debug)]
pub struct Events<'a> {
    render_window: *mut ffi::sfRenderWindow,
    winref: PhantomData<&'a mut RenderWindow>,
}

impl RenderWindow {
    /// Construct a new render window
    ///
    /// This function creates the render window with the size and pixel
    /// depth defined in mode. An optional style can be passed to
    /// customize the look and behaviour of the window (borders,
    /// title bar, resizable, closable, ...). If style contains
    /// `FULLSCREEN`, then mode must be a valid video mode.
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
    pub fn new<V: Into<VideoMode>>(mode: V,
                                   title: &str,
                                   style: Style,
                                   settings: &ContextSettings)
                                   -> RenderWindow {
        let utf32 = ::unicode_conv::str_to_csfml(title);
        let sf_render_win: *mut ffi::sfRenderWindow =
            unsafe {
                ffi::sfRenderWindow_createUnicode(mode.into().raw(),
                                                  utf32.as_ptr() as _,
                                                  style.bits(),
                                                  &settings.raw())
            };
        assert!(!sf_render_win.is_null(), "Failed to create RenderWindow");
        RenderWindow { render_window: sf_render_win }
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

    /// Return an iterator over all the event currently in the events queue.
    pub fn events(&self) -> Events {
        Events {
            render_window: self.render_window,
            winref: PhantomData,
        }
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
        let mut event = unsafe { ::std::mem::zeroed() };
        let have_event = unsafe { ffi::sfRenderWindow_pollEvent(self.render_window, &mut event) }
            .to_bool();
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
        let mut event = unsafe { ::std::mem::zeroed() };
        let have_event = unsafe { ffi::sfRenderWindow_waitEvent(self.render_window, &mut event) }
            .to_bool();
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
    /// each call to `RenderWindow::display` to ensure that the current frame
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
    /// passed to the `RenderWindow::create` function,
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
    pub fn set_title(&mut self, title: &str) {
        let utf32 = ::unicode_conv::str_to_csfml(title);
        unsafe {
            ffi::sfRenderWindow_setUnicodeTitle(self.render_window, utf32.as_ptr() as _);
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
            ffi::sfRenderWindow_setMouseCursorVisible(self.render_window,
                                                      sfBool::from_bool(visible));
        }
    }

    /// Grab or release the mouse cursor.
    ///
    /// If set, grabs the mouse cursor inside this window's client area so it may no longer be
    /// moved outside its bounds. Note that grabbing is only active while the window has focus.
    pub fn set_mouse_cursor_grabbed(&mut self, grabbed: bool) {
        unsafe {
            ffi::sfRenderWindow_setMouseCursorGrabbed(self.render_window,
                                                      sfBool::from_bool(grabbed))
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
            ffi::sfRenderWindow_setVerticalSyncEnabled(self.render_window,
                                                       sfBool::from_bool(enabled));
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
    /// no JoyMoved event will be generated.
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
    pub fn set_position(&mut self, position: &Vector2i) {
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
    pub fn set_mouse_position(&mut self, position: &Vector2i) {
        unsafe { ffi::sfMouse_setPositionRenderWindow(position.raw(), self.render_window) }
    }

    /// Returns the current position of a touch in window coordinates.
    pub fn touch_position(&self, finger: u32) -> Vector2i {
        unsafe {
            FromRaw::from_raw(ffi::sfTouch_getPositionRenderWindow(finger, self.render_window))
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
    /// deny the request. This is not to be confused with `set_active()`.
    pub fn request_focus(&self) {
        unsafe { ffi::sfRenderWindow_requestFocus(self.render_window) }
    }
}

impl Raw for RenderWindow {
    type Raw = *const ffi::sfRenderWindow;
    fn raw(&self) -> Self::Raw {
        self.render_window
    }
}

impl RenderTarget for RenderWindow {
    /// Save the current OpenGL render states and matrices
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering. Combined with popGLStates,
    /// it ensures that:
    /// SFML's internal states are not messed up by your OpenGL code
    /// and that your OpenGL states are not modified by a call to a SFML function
    ///
    /// Note that this function is quite expensive: it saves all the
    /// possible OpenGL states and matrices, even the ones you
    /// don't care about. Therefore it should be used wisely.
    /// It is provided for convenience, but the best results will
    /// be achieved if you handle OpenGL states yourself (because
    /// you know which states have really changed, and need to be
    /// saved and restored). Take a look at the resetGLStates
    /// function if you do so.
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_pushGLStates(self.render_window) }
    }

    /// Restore the previously saved OpenGL render states and matrices
    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_popGLStates(self.render_window) }
    }

    /// Reset the internal OpenGL states so that the target is ready for drawing
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering, if you choose not to use
    /// push_gl_states/pop_gl_states. It makes sure that all OpenGL
    /// states needed by SFML are set, so that subsequent draw()
    /// calls will work as expected.
    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_resetGLStates(self.render_window) }
    }


    /// Change the current active view of a render window
    ///
    /// # Arguments
    /// * view - The new view
    ///
    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderWindow_setView(self.render_window, view.raw()) }
    }

    /// Get the current active view of a render window
    ///
    /// Return the current active view
    ///
    fn view(&self) -> &ViewRef {
        unsafe { &*(ffi::sfRenderWindow_getView(self.render_window) as *const ViewRef) }
    }

    /// Get the default view of a render window
    ///
    /// Return the default view of the render window
    ///
    fn default_view(&self) -> &ViewRef {
        unsafe { &*(ffi::sfRenderWindow_getDefaultView(self.render_window) as *const ViewRef) }
    }

    /// Convert a point from window coordinates to world coordinates
    ///
    /// This function finds the 2D position that matches the
    /// given pixel of the render-window. In other words, it does
    /// the inverse of what the graphics card does, to find the
    /// initial position of a rendered pixel.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render window, this assertion is not true anymore, ie. a point
    /// located at (10, 50) in your render-window may map to the point
    /// (150, 75) in your 2D world -- if the view is translated by (140, 25).
    ///
    /// This function is typically used to find which point (or object) is
    /// located below the mouse cursor.
    ///
    /// This version uses a custom view for calculations, see the
    /// [map_pixel_to_coords_current_view](#method.map_pixel_to_coords_current_view)
    /// function if you want to use the current view of the
    /// render-window.
    ///
    /// # Arguments
    /// * point - Pixel to convert
    /// * view - The view to use for converting the point
    ///
    /// Return the converted point, in "world" units
    fn map_pixel_to_coords(&self, point: &Vector2i, view: &View) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfRenderWindow_mapPixelToCoords(self.render_window,
                                                                    point.raw(),
                                                                    view.raw()))
        }
    }

    /// Convert a point from window coordinates to world coordinates
    ///
    /// This function finds the 2D position that matches the
    /// given pixel of the render-window. In other words, it does
    /// the inverse of what the graphics card does, to find the
    /// initial position of a rendered pixel.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render window, this assertion is not true anymore, ie. a point
    /// located at (10, 50) in your render-window may map to the point
    /// (150, 75) in your 2D world -- if the view is translated by (140, 25).
    ///
    /// This function is typically used to find which point (or object) is
    /// located below the mouse cursor.
    ///
    /// This version uses the current view for calculations, see the
    /// [map_pixel_to_coords](#method.map_pixel_to_coords) function if you want to
    /// use a custom view.
    ///
    /// # Arguments
    /// * point - Pixel to convert
    ///
    /// Return the converted point, in "world" units
    fn map_pixel_to_coords_current_view(&self, point: &Vector2i) -> Vector2f {
        let view = unsafe { ffi::sfRenderWindow_getView(self.render_window) };
        unsafe {
            Vector2f::from_raw(ffi::sfRenderWindow_mapPixelToCoords(self.render_window,
                                                                    point.raw(),
                                                                    view))
        }
    }

    /// Convert a point from world coordinates to window coordinates
    ///
    /// This function finds the pixel of the render-window that matches
    /// the given 2D point. In other words, it goes through the same process
    /// as the graphics card, to compute the final position of a rendered point.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render window, this assertion is not true anymore, ie. a point
    /// located at (150, 75) in your 2D world may map to the pixel
    /// (10, 50) of your render-window -- if the view is translated by (140, 25).
    ///
    /// This version uses a custom view for calculations, see
    /// [map_coords_to_pixel_current_view](#method.map_coords_to_pixel_current_view)
    /// if you want to use the current view of the render-window.
    ///
    /// # Arguments
    /// * point - Point to convert
    /// * view - The view to use for converting the point
    fn map_coords_to_pixel(&self, point: &Vector2f, view: &View) -> Vector2i {
        unsafe {
            Vector2i::from_raw(ffi::sfRenderWindow_mapCoordsToPixel(self.render_window,
                                                                    point.raw(),
                                                                    view.raw()))
        }
    }

    /// Convert a point from world coordinates to window coordinates
    ///
    /// This function finds the pixel of the render-window that matches
    /// the given 2D point. In other words, it goes through the same process
    /// as the graphics card, to compute the final position of a rendered point.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render window, this assertion is not true anymore, ie. a point
    /// located at (150, 75) in your 2D world may map to the pixel
    /// (10, 50) of your render-window -- if the view is translated by (140, 25).
    ///
    /// This version uses the current view for calculations, see
    /// [map_coords_to_pixel](#method.map_coords_to_pixel) if you want to use a custom view.
    ///
    /// # Arguments
    /// * point - Point to convert
    fn map_coords_to_pixel_current_view(&self, point: &Vector2f) -> Vector2i {
        let curr_view = unsafe { ffi::sfRenderWindow_getView(self.render_window) };
        unsafe {
            Vector2i::from_raw(ffi::sfRenderWindow_mapCoordsToPixel(self.render_window,
                                                                    point.raw(),
                                                                    curr_view))
        }
    }

    /// Get the viewport of a view applied to this target
    ///
    /// # Arguments
    /// * view - Target view
    ///
    /// Return the viewport rectangle, expressed in pixels in the current target
    fn viewport(&self, view: &View) -> IntRect {
        unsafe {
            IntRect::from_raw(ffi::sfRenderWindow_getViewport(self.render_window, view.raw()))
        }
    }

    /// Get the size of the rendering region of a window
    ///
    /// The size doesn't include the titlebar and borders of the window.
    ///
    /// Return the size in pixels
    fn size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfRenderWindow_getSize(self.render_window)) }
    }

    /// Draw a drawable object to the render target
    ///
    /// # Arguments
    /// * object - Object to draw
    fn draw(&mut self, object: &Drawable) {
        object.draw(self, RenderStates::default());
    }

    /// Draw a drawable object to the render-target with a RenderStates
    ///
    /// # Arguments
    /// * object - Object to draw
    /// * renderStates - The renderStates to associate to the object
    fn draw_with_renderstates(&mut self, object: &Drawable, render_states: RenderStates) {
        object.draw(self, render_states);
    }

    /// Draw a Text with a RenderStates
    fn draw_text(&self, text: &Text, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window, text.raw(), &render_states.raw())
        }
    }

    /// Draw a Shape with a RenderStates
    fn draw_shape(&self, shape: &CustomShape, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawShape(self.render_window, shape.raw(), &render_states.raw())
        }
    }

    /// Draw a sprite with a RenderStates
    fn draw_sprite(&self, sprite: &Sprite, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window, sprite.raw(), &render_states.raw())
        }
    }

    /// Draw a CircleShape with a RenderStates
    fn draw_circle_shape(&self, circle_shape: &CircleShape, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window,
                                                circle_shape.raw(),
                                                &render_states.raw())
        }
    }

    /// Draw a RectangleShape with a RenderStates
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window,
                                                   rectangle_shape.raw(),
                                                   &render_states.raw())
        }
    }

    /// Draw a ConvexShape with a RenderStates
    fn draw_convex_shape(&self, convex_shape: &ConvexShape, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window,
                                                convex_shape.raw(),
                                                &render_states.raw())
        }
    }

    /// Draw a VertexArray with a RenderStates
    fn draw_vertex_array(&self, vertex_array: &VertexArray, render_states: RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawVertexArray(self.render_window,
                                                vertex_array.raw(),
                                                &render_states.raw())
        }
    }

    /// draw primitives
    fn draw_primitives(&self, vertices: &[Vertex], ty: PrimitiveType, rs: RenderStates) {

        let len = vertices.len();
        unsafe {
            ffi::sfRenderWindow_drawPrimitives(self.render_window,
                                               vertices.as_ptr() as *const _,
                                               len,
                                               ty.raw(),
                                               &rs.raw());
        }
    }

    /// Clear window with the given color
    fn clear(&mut self, color: &Color) {
        unsafe { ffi::sfRenderWindow_clear(self.render_window, color.raw()) }
    }
}

impl<'a> Iterator for Events<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        let mut event = unsafe { ::std::mem::zeroed() };
        if unsafe { ffi::sfRenderWindow_pollEvent(self.render_window, &mut event) }.to_bool() {
            unsafe { Event::from_raw(&event) }
        } else {
            None
        }
    }
}

impl Drop for RenderWindow {
    fn drop(&mut self) {
        unsafe {
            ffi::sfRenderWindow_destroy(self.render_window);
        }
    }
}
