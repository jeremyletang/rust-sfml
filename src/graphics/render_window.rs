use crate::{
    ffi::graphics as ffi,
    graphics::{
        CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType, RcSprite,
        RcText, RectangleShape, RenderStates, RenderTarget, Sprite, Text, Vertex, VertexBuffer,
        View,
    },
    sf_box::Dispose,
    system::{SfStrConv, Vector2f, Vector2i, Vector2u},
    window::{thread_safety, ContextSettings, Cursor, Event, Handle, Style, VideoMode},
    IntoSfResult, SfBox, SfError, SfResult,
};

decl_opaque! {
    /// [`Window`] that can serve as a target for 2D drawing.
    ///
    /// `RenderWindow` is the main type of the graphics module.
    /// It defines an OS window that can be painted using the other classes
    /// of the graphics module.
    ///
    /// [`Window`]: crate::window::Window
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// use sfml::window::{Event, Style};
    /// use sfml::graphics::{RenderWindow, RenderTarget, Color};
    /// // Create a new window
    /// let mut window = RenderWindow::new((800, 600),
    ///                              "SFML window",
    ///                              Style::CLOSE,
    ///                              &Default::default()).unwrap();
    /// // Limit the framerate to 60 frames per second (this step is optional)
    /// window.set_framerate_limit(60);
    ///
    /// // The main loop - ends as soon as the window is closed
    /// while window.is_open() {
    ///     // Event processing
    ///     while let Some(event) = window.poll_event() {
    ///         match event {
    ///             Event::Closed => window.close(),
    ///             _ => {}
    ///         }
    ///     }
    ///
    ///     window.clear(Color::BLACK);
    ///     // SFML drawing commands go here...
    ///
    ///     // End the current frame and display its contents on screen
    ///     window.display();
    /// }
    ///
    /// ```
    RenderWindow;
}

/// Creation
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
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// use sfml::window::Style;
    /// use sfml::graphics::{RenderWindow};
    /// // Create a new window
    /// let mut window = RenderWindow::new((800, 600),
    ///                              "SFML window",
    ///                              Style::CLOSE,
    ///                              &Default::default());
    /// ```
    pub fn new<V: Into<VideoMode>, S: SfStrConv>(
        mode: V,
        title: S,
        style: Style,
        settings: &ContextSettings,
    ) -> SfResult<SfBox<Self>> {
        thread_safety::set_window_thread();

        title.with_as_sfstr(|sfstr| {
            let ptr = unsafe {
                ffi::sfRenderWindow_createUnicode_new(
                    mode.into().raw(),
                    sfstr.as_ptr(),
                    style.bits(),
                    settings,
                )
            };
            SfBox::new(ptr).ok_or(SfError::CallFailed)
        })
    }
    /// Recreate with new settings. See [`Self::new`] for more information.
    pub fn recreate<V: Into<VideoMode>, S: SfStrConv>(
        &mut self,
        mode: V,
        title: S,
        style: Style,
        settings: &ContextSettings,
    ) {
        thread_safety::set_window_thread();

        title.with_as_sfstr(|sfstr| unsafe {
            ffi::sfRenderWindow_createUnicode(
                self,
                mode.into().raw(),
                sfstr.as_ptr(),
                style.bits(),
                settings,
            );
        });
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
    pub unsafe fn from_handle(handle: Handle, settings: &ContextSettings) -> SfResult<SfBox<Self>> {
        thread_safety::set_window_thread();
        let ptr = unsafe { ffi::sfRenderWindow_createFromHandle(handle, settings) };
        SfBox::new(ptr).ok_or(SfError::CallFailed)
    }
}

/// Event handling
impl RenderWindow {
    /// Pop the event on top of event queue, if any, and return it
    ///
    /// This function is not blocking: if there's no pending event then
    /// it will return `None`.
    /// Note that more than one event may be present in the event queue,
    /// thus you should always call this function in a loop
    /// to make sure that you process every pending event.
    ///
    /// Returns `Some(event)` if an event was returned, or `None` if the event queue was empty
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// while window.is_open() {
    ///     // Event processing
    ///     while let Some(event) = window.poll_event() {
    ///         match event {
    ///             Event::Closed => window.close(),
    ///             _ => {},
    ///         }
    ///     }
    /// }
    /// ```
    pub fn poll_event(&mut self) -> Option<Event> {
        let mut event = std::mem::MaybeUninit::uninit();
        let have_event = unsafe { ffi::sfRenderWindow_pollEvent(self, event.as_mut_ptr()) };
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
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// // The main loop - ends as soon as the window is closed
    /// while window.is_open() {
    ///     // Event processing
    ///     match window.wait_event() { // Stops program from continuing until new event occurs
    ///         Some(Event::Closed) => window.close(),
    ///         _ => {},
    ///     }
    /// }
    /// ```
    pub fn wait_event(&mut self) -> Option<Event> {
        let mut event = std::mem::MaybeUninit::uninit();
        let have_event = unsafe { ffi::sfRenderWindow_waitEvent(self, event.as_mut_ptr()) };
        if have_event {
            unsafe { Event::from_raw(&event.assume_init()) }
        } else {
            None
        }
    }
}

/// Rendering. See also [`RenderTarget`], which `RenderWindow` implements.
impl RenderWindow {
    /// Display on screen what has been rendered to the window so far
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::{ RenderWindow, RenderTarget, Color };
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// while window.is_open() {
    ///     window.clear(Color::BLACK);
    ///     // Draw something
    ///
    ///     window.display();
    /// }
    /// ```
    pub fn display(&mut self) {
        unsafe { ffi::sfRenderWindow_display(self) }
    }

    /// Limit the framerate to a maximum fixed frequency
    ///
    /// If a limit is set, the window will use a small delay after
    /// each call to [`RenderWindow::display`] to ensure that the current frame
    /// lasted long enough to match the framerate limit.
    ///
    /// # Arguments
    /// * limit - Framerate limit, in frames per seconds (use 0 to disable limit)
    pub fn set_framerate_limit(&mut self, limit: u32) {
        unsafe { ffi::sfRenderWindow_setFramerateLimit(self, limit) }
    }

    /// Get the settings of the OpenGL context of a window
    ///
    /// Note that these settings may be different from what was
    /// passed to the [`RenderWindow::new`] function,
    /// if one or more settings were not supported. In this case,
    /// SFML chose the closest match.
    ///
    /// Return a structure containing the OpenGL context settings
    #[must_use]
    pub fn settings(&self) -> &ContextSettings {
        unsafe { &*ffi::sfRenderWindow_getSettings(self) }
    }

    /// Tell if the render texture will use sRGB encoding when drawing it
    ///
    /// Returns true if sRGB encoding is enabled, false if sRGB encoding is disabled
    #[must_use]
    pub fn is_srgb(&self) -> bool {
        unsafe { ffi::sfRenderWindow_isSrgb(self) }
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
        unsafe {
            ffi::sfRenderWindow_setVerticalSyncEnabled(self, enabled);
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
    pub fn set_active(&mut self, enabled: bool) -> SfResult<()> {
        unsafe { ffi::sfRenderWindow_setActive(self, enabled) }.into_sf_result()
    }
}

/// Input
impl RenderWindow {
    /// Show or hide the mouse cursor
    ///
    /// # Arguments
    /// * visible - true to  false to hide
    pub fn set_mouse_cursor_visible(&mut self, visible: bool) {
        unsafe {
            ffi::sfRenderWindow_setMouseCursorVisible(self, visible);
        }
    }

    /// Grab or release the mouse cursor.
    ///
    /// If set, grabs the mouse cursor inside this window's client area so it may no longer be
    /// moved outside its bounds. Note that grabbing is only active while the window has focus.
    pub fn set_mouse_cursor_grabbed(&mut self, grabbed: bool) {
        unsafe { ffi::sfRenderWindow_setMouseCursorGrabbed(self, grabbed) }
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
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) {
        unsafe {
            ffi::sfRenderWindow_setKeyRepeatEnabled(self, enabled);
        }
    }
    /// Change the joystick threshold
    ///
    /// The joystick threshold is the value below which
    /// no [`crate::window::Event::JoystickMoved`] event will be generated.
    ///
    /// # Arguments
    /// * threshold - New threshold, in the range [0, 100]
    pub fn set_joystick_threshold(&mut self, threshold: f32) {
        unsafe { ffi::sfRenderWindow_setJoystickThreshold(self, threshold) }
    }
    /// Returns the current position of the mouse relative to the window.
    #[must_use]
    pub fn mouse_position(&self) -> Vector2i {
        unsafe { crate::ffi::graphics::sfMouse_getPositionRenderWindow(self) }
    }

    /// Set the current position of the mouse relatively to a render window
    ///
    /// This function sets the current position of the mouse cursor relative
    /// to the given render window
    ///
    /// # Arguments
    /// * `position` - the positon to set
    pub fn set_mouse_position(&mut self, position: Vector2i) {
        unsafe { crate::ffi::graphics::sfMouse_setPositionRenderWindow(position, self) }
    }
    /// Returns the current position of a touch in window coordinates.
    #[must_use]
    pub fn touch_position(&self, finger: u32) -> Vector2i {
        unsafe { crate::ffi::graphics::sfTouch_getPositionRenderWindow(finger, self) }
    }
}

/// Window operations
impl RenderWindow {
    /// Change a render window's icon
    /// pixels must be an array of width x height pixels in 32-bits RGBA format.
    ///
    /// # Arguments
    /// * width - Icon's width, in pixels
    /// * height - Icon's height, in pixels
    /// * pixels - Vector of pixels
    ///
    /// # Safety
    ///
    /// `pixels` not being at least `width * height * 4` will likely cause undefined behavior.
    ///
    /// Platform-specific behavior is also unclear (limits on max size, etc).
    ///
    /// # Usage example
    ///
    /// ```no_run
    /// # use sfml::window::Style;
    /// # use sfml::graphics::{RenderWindow};
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// while window.is_open() {
    /// // Creates a bright red window icon
    /// let (width, height) = (1, 1);
    /// let pixels: [u8; 4] = [255, 0, 0, 255];
    /// unsafe { window.set_icon(width, height, &pixels); }
    ///     window.display();
    /// }
    /// ```
    pub unsafe fn set_icon(&mut self, width: u32, height: u32, pixels: &[u8]) {
        unsafe { ffi::sfRenderWindow_setIcon(self, width, height, pixels.as_ptr()) }
    }

    /// Close a render window and destroy all the attached resources
    ///
    /// After calling this method, the Window object remains
    /// valid.
    /// All other functions such as `poll_event` or display
    /// will still work (i.e. you don't have to test `is_open`
    /// every time), and will have no effect on closed windows.
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// // The main loop - ends as soon as the window is closed
    /// while window.is_open() {
    ///     // Event processing
    ///     while let Some(event) = window.poll_event() {
    ///         match event {
    ///             Event::Closed => window.close(),
    ///             _ => {}
    ///         }
    ///     }
    /// }
    /// // Once window is closed, we can do other things.
    /// ```
    pub fn close(&mut self) {
        unsafe {
            ffi::sfRenderWindow_close(self);
        }
    }

    /// Tell whether or not a window is opened
    ///
    /// This function returns whether or not the window exists.
    /// Note that a hidden window `(set_visible(false))` will return
    /// true.
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// use sfml::window::{Event, Style};
    /// use sfml::graphics::RenderWindow;
    /// // Create a new window
    /// let mut window = RenderWindow::new((800, 600),
    ///                              "SFML window",
    ///                              Style::CLOSE,
    ///                              &Default::default()).unwrap();
    ///
    /// while window.is_open() {
    ///     // Do something
    /// }
    /// ```
    #[must_use]
    pub fn is_open(&self) -> bool {
        unsafe { ffi::sfRenderWindow_isOpen(self) }
    }

    /// Change the title of a window
    ///
    /// # Arguments
    /// * title - New title
    pub fn set_title<S: SfStrConv>(&mut self, title: S) {
        title.with_as_sfstr(|sfstr| unsafe {
            ffi::sfRenderWindow_setUnicodeTitle(self, sfstr.as_ptr());
        })
    }

    /// Show or hide a window.
    ///
    /// # Arguments
    /// * visible - true to show the window, false to hide it
    pub fn set_visible(&mut self, visible: bool) {
        unsafe {
            ffi::sfRenderWindow_setVisible(self, visible);
        }
    }

    /// Get the position of a window
    ///
    /// Return the position in pixels
    #[must_use]
    pub fn position(&self) -> Vector2i {
        unsafe { ffi::sfRenderWindow_getPosition(self) }
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
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # use sfml::system::Vector2;
    /// # // Create a new window with SFML window as name
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// window.set_position(Vector2::new(100, 400));
    /// use std::{thread, time::Duration};
    /// // You need to wait for the OS the set the window's position before checking
    /// thread::sleep(Duration::from_millis(250));
    /// assert_eq!(window.position(), Vector2::new(100, 400));
    /// ```
    pub fn set_position(&mut self, position: Vector2i) {
        unsafe { ffi::sfRenderWindow_setPosition(self, position) }
    }

    /// Change the size of the rendering region of a window
    ///
    /// # Arguments
    /// * size - New size, in pixels
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::{ RenderWindow, RenderTarget };
    /// # use sfml::system::Vector2;
    /// # // Create a new window with SFML window as name
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// window.set_size(Vector2::new(100, 400));
    /// use std::{thread, time::Duration};
    /// // You need to wait for the OS the set the window's size before checking
    /// thread::sleep(Duration::from_millis(250));
    /// assert_eq!(window.size(), Vector2::new(100, 400));
    /// ```
    pub fn set_size<S: Into<Vector2u>>(&mut self, size: S) {
        unsafe { ffi::sfRenderWindow_setSize(self, size.into()) }
    }

    /// Set the displayed cursor to a native system cursor.
    ///
    /// Upon window creation, the arrow cursor is used by default.
    ///
    /// # Safety
    ///
    /// The cursor can not be destroyed while in use by the window.
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window with SFML window as name
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// # use sfml::window::{ Cursor, CursorType };
    /// let cursor = Cursor::from_system(CursorType::Arrow);
    /// if let Ok(arrow_cursor) = &cursor {
    ///     unsafe { window.set_mouse_cursor(arrow_cursor); }
    /// }
    /// // You need to ensure the SFML window closes before the cursor's end of life.
    /// // Doing it the other way around will cause undefined behavior.
    /// window.close();
    /// drop(cursor);
    /// ```
    pub unsafe fn set_mouse_cursor(&mut self, cursor: &Cursor) {
        unsafe { ffi::sfRenderWindow_setMouseCursor(self, cursor) }
    }

    /// Check whether the window has the input focus.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or most mouse events.
    #[must_use]
    pub fn has_focus(&self) -> bool {
        unsafe { ffi::sfRenderWindow_hasFocus(self) }
    }

    /// Request the current window to be made the active foreground window.
    ///
    /// At any given time, only one window may have the input focus to receive input events
    /// such as keystrokes or mouse events. If a window requests focus, it only hints to the
    /// operating system, that it would like to be focused. The operating system is free to
    /// deny the request. This is not to be confused with [`RenderWindow::set_active`].
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::window::{Event, Style};
    /// # use sfml::graphics::RenderWindow;
    /// # // Create a new window with SFML window as name
    /// # let mut window = RenderWindow::new((800, 600),
    /// #                              "SFML window",
    /// #                              Style::CLOSE,
    /// #                              &Default::default()).unwrap();
    /// window.request_focus();
    /// use std::{thread, time::Duration};
    /// // You need to wait for the OS the set the window's visibility before checking
    /// thread::sleep(Duration::from_millis(250));
    /// assert_eq!(window.has_focus(), true);
    /// ```
    pub fn request_focus(&mut self) {
        unsafe { ffi::sfRenderWindow_requestFocus(self) }
    }
}

/// System integration
impl RenderWindow {
    /// Get the OS-specific handle of the window.
    ///
    /// The type of the returned handle is Handle, which is a typedef to the handle type defined by the OS.
    /// You shouldn't need to use this function, unless you have very specific stuff to implement that SFML
    /// doesn't support, or implement a temporary workaround until a bug is fixed.
    #[must_use]
    pub fn system_handle(&self) -> Handle {
        unsafe { ffi::sfRenderWindow_getSystemHandle(self) }
    }
}

impl RenderTarget for RenderWindow {
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_pushGLStates(self) }
    }
    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_popGLStates(self) }
    }
    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderWindow_resetGLStates(self) }
    }
    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderWindow_setView(self, view) }
    }
    fn view(&self) -> &View {
        unsafe { &*(ffi::sfRenderWindow_getView(self)) }
    }
    fn default_view(&self) -> &View {
        unsafe { &*(ffi::sfRenderWindow_getDefaultView(self)) }
    }
    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        unsafe { ffi::sfRenderWindow_mapPixelToCoords_View(self, point, view) }
    }
    fn map_pixel_to_coords_current_view(&self, point: Vector2i) -> Vector2f {
        unsafe { ffi::sfRenderWindow_mapPixelToCoords(self, point) }
    }
    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        unsafe { ffi::sfRenderWindow_mapCoordsToPixel_View(self, point, view) }
    }
    fn map_coords_to_pixel_current_view(&self, point: Vector2f) -> Vector2i {
        unsafe { ffi::sfRenderWindow_mapCoordsToPixel(self, point) }
    }
    fn viewport(&self, view: &View) -> IntRect {
        unsafe { ffi::sfRenderWindow_getViewport(self, view) }
    }
    fn size(&self) -> Vector2u {
        unsafe { ffi::sfRenderWindow_getSize(self) }
    }
    fn draw(&mut self, object: &dyn Drawable) {
        object.draw(self, &RenderStates::DEFAULT);
    }
    fn draw_with_renderstates(&mut self, object: &dyn Drawable, render_states: &RenderStates) {
        object.draw(self, render_states);
    }
    fn draw_text(&mut self, text: &Text, render_states: &RenderStates) {
        unsafe { ffi::sfRenderWindow_drawText(self, text.raw(), render_states) }
    }
    fn draw_rc_text(&mut self, text: &RcText, render_states: &RenderStates) {
        unsafe { ffi::sfRenderWindow_drawText(self, text.raw(), render_states) }
    }
    fn draw_shape(&mut self, shape: &CustomShape, render_states: &RenderStates) {
        unsafe { ffi::sfRenderWindow_drawShape(self, shape.raw(), render_states) }
    }
    fn draw_sprite(&mut self, sprite: &Sprite, render_states: &RenderStates) {
        unsafe { ffi::sfRenderWindow_drawSprite(self, sprite.raw(), render_states) }
    }
    fn draw_rc_sprite(&mut self, sprite: &RcSprite, render_states: &RenderStates) {
        unsafe { ffi::sfRenderWindow_drawSprite(self, sprite.raw(), render_states) }
    }
    fn draw_circle_shape(&mut self, circle_shape: &CircleShape, render_states: &RenderStates) {
        unsafe { ffi::sfRenderWindow_drawCircleShape(self, circle_shape.raw(), render_states) }
    }
    fn draw_rectangle_shape(
        &mut self,
        rectangle_shape: &RectangleShape,
        render_states: &RenderStates,
    ) {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self, rectangle_shape.raw(), render_states)
        }
    }
    fn draw_convex_shape(&mut self, convex_shape: &ConvexShape, render_states: &RenderStates) {
        unsafe { ffi::sfRenderWindow_drawConvexShape(self, convex_shape.raw(), render_states) }
    }
    fn draw_vertex_buffer(&mut self, vertex_buffer: &VertexBuffer, render_states: &RenderStates) {
        unsafe { ffi::sfRenderWindow_drawVertexBuffer(self, vertex_buffer.raw(), render_states) }
    }
    fn draw_primitives(&mut self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderWindow_drawPrimitives(
                self,
                vertices.as_ptr().cast(),
                vertices.len(),
                ty.0,
                rs,
            );
        }
    }
    fn clear(&mut self, color: Color) {
        unsafe { ffi::sfRenderWindow_clear(self, color) }
    }
}

impl Dispose for RenderWindow {
    unsafe fn dispose(&mut self) {
        unsafe {
            ffi::sfRenderWindow_destroy(self);
        }
    }
}
