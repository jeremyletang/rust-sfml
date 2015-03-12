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

#![allow(non_snake_case, missing_copy_implementations)]

//! Window that can serve as a target for 2D drawing.
//!
//! RenderWindow is the main class of the Graphics module.
//! It defines an OS window that can be painted using the other classes
//! of the graphics module.

use libc::{c_float, c_uint};
use std::ptr;
use std::vec::Vec;
use std::ffi::CString;

use traits::{Drawable, Wrappable};
use window::{ContextSettings, VideoMode, event, WindowStyle};
use system::vector2::{Vector2f, Vector2i, Vector2u};
use graphics::{Color, CircleShape, RectangleShape, Text, Sprite, VertexArray,
               RenderStates, View, Image, IntRect, RenderTarget,
               rc, Vertex, PrimitiveType, ConvexShape};

use ffi::sfml_types::{SfBool, SFTRUE, SFFALSE};
use ffi::graphics::render_window as ffi;

/// Window that can serve as a target for 2D drawing.
///
/// RenderWindow is the main class of the Graphics module.
/// It defines an OS window that can be painted using the other classes
/// of the graphics module.
pub struct RenderWindow {
    render_window: *mut ffi::sfRenderWindow,
    title_length: u32,
//    current_view: Rc<RefCell<View>>,
//    default_view: Rc<RefCell<View>>
}

/// An iterator over all the events in the events queue (internally call poll_event)
pub struct Events {
    render_window: *mut ffi::sfRenderWindow,
    event: event::raw::sfEvent,
}

impl RenderWindow {
    /// Construct a new render window
    ///
    /// This function creates the render window with the size and pixel
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
    /// * mode - Video mode to use (defines the width, height and depth of the rendering area of the render window)
    /// * title - Title of the render window
    /// * style - Window style
    /// * settings - Additional settings for the underlying OpenGL context
    ///
    /// Return Some(RenderWindow) or None
    pub fn new(mode: VideoMode,
               title: &str,
               style: WindowStyle,
               settings: &ContextSettings) -> Option<RenderWindow> {
        let c_str = CString::new(title.as_bytes()).unwrap().as_ptr();
        let sf_render_win: *mut ffi::sfRenderWindow = unsafe {
            ffi::sfRenderWindow_create(mode.unwrap(),
                                       c_str,
                                       style as u32,
                                       settings)
        };
        if sf_render_win.is_null() {
            None
        } else {
            Some (RenderWindow {
                      render_window: sf_render_win,
                      // event: sf_ev,
                      title_length: title.len() as u32
            })
        }
    }

    /// Construct a new render window (with a UTF-32 title)
    ///
    /// This function creates the render window with the size and pixel
    /// depth defined in mode. An optional style can be passed to
    /// customize the look and behaviour of the render window (borders,
    /// title bar, resizable, closable, ...). If style contains
    /// sfFullscreen, then mode must be a valid video mode.
    ///
    /// The fourth parameter is a pointer to a structure specifying
    /// advanced OpenGL context settings such as antialiasing,
    /// depth-buffer bits, etc.
    ///
    /// # Arguments
    /// * mode - Video mode to use (defines the width, height and depth of the rendering area of the render window)
    /// * title - Title of the render window (UTF-32)
    /// * style - Window style
    /// * settings - Additional settings for the underlying OpenGL context
    ///
    /// Return Some(RenderWindow) or None
    pub fn new_with_unicode(mode: VideoMode,
                            title: Vec<u32>,
                            style: WindowStyle,
                            settings: &ContextSettings) -> Option<RenderWindow> {

        let sf_render_win: *mut ffi::sfRenderWindow;
        unsafe {
            sf_render_win = ffi::sfRenderWindow_createUnicode(mode.unwrap(),
                                                              title.as_ptr() as *mut u32,
                                                              style as u32,
                                                              settings);
        }
        if sf_render_win.is_null() {
            None
        } else {
            Some (RenderWindow {
                    render_window: sf_render_win,
                    // event: sf_ev,
                    title_length: title.len() as u32
            })
        }
    }

    /// Change the title of a render window (with a UTF-32 string)
    ///
    /// # Arguments
    /// * title - New title
    pub fn set_unicode_title(&mut self, title: Vec<u32>) -> () {
        unsafe {
            self.title_length = title.len() as u32;
            ffi::sfRenderWindow_setUnicodeTitle(self.render_window,
                                                title.as_ptr())
        }
    }

    /// Change a render window's icon
    /// pixels must be an array of width x height pixels in 32-bits RGBA format.
    ///
    /// # Arguments
    /// * width - Icon's width, in pixels
    /// * height - Icon's height, in pixels
    /// * pixels - Vector of pixels
    pub fn set_icon(&mut self,
                    width: u32,
                    height: u32,
                    pixels: &[u8]) -> () {
        unsafe {
            ffi::sfRenderWindow_setIcon(self.render_window,
                                        width as c_uint,
                                        height as c_uint,
                                        pixels.as_ptr())
        }
    }

    /// Return an iterator over all the event currently in the events queue.
    pub fn events(&self) -> Events {
        Events {
            render_window: self.render_window.clone(),
            event: event::raw::sfEvent { data: [032; 6] }
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
    /// Return the event if an event was returned, or NoEvent if the event queue was empty
    pub fn poll_event(&mut self) -> event::Event {
        let mut event = event::raw::sfEvent { data: [032; 6] };
        let have_event: bool =  unsafe {
            match ffi::sfRenderWindow_pollEvent(self.render_window, &mut event) {
                SFFALSE     => false,
                SFTRUE      => true
            }
        };
        if have_event == false {
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
        let have_event: bool =  unsafe {
            match ffi::sfRenderWindow_waitEvent(self.render_window, &mut event) {
                SFFALSE     => false,
                SFTRUE      => true
            }
        };
        if have_event == false {
            event::NoEvent
        } else {
            event::raw::get_wrapped_event(&mut event)
        }
    }

    /// Close a render window and destroy all the attached resources
    ///
    /// After calling this method, the Window object remains
    /// valid.
    /// All other functions such as poll_event or display
    /// will still work (i.e. you don't have to test is_open
    /// every time), and will have no effect on closed windows.
    pub fn close(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_close(self.render_window);
        }
    }

    /// Tell whether or not a window is opened
    ///
    /// This function returns whether or not the window exists.
    /// Note that a hidden window (set_visible(false)) will return
    /// true.
    ////
    pub fn is_open(&self) -> bool {
        let tmp: SfBool;
        unsafe {
            tmp = ffi::sfRenderWindow_isOpen(self.render_window);
        }
        match tmp {
            SFFALSE => false,
            SFTRUE  => true
        }
    }

    /// Display on screen what has been rendered to the window so far
    ///
    /// This function is typically called after all OpenGL rendering
    /// has been done for the current frame, in order to show
    /// it on screen.
    ////
    pub fn display(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_display(self.render_window)
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
    ////
    pub fn set_framerate_limit(&mut self, limit: u32) -> () {
        unsafe {
            ffi::sfRenderWindow_setFramerateLimit(self.render_window,
                                                  limit as c_uint)
        }
    }

    /// Get the settings of the OpenGL context of a window
    ///
    /// Note that these settings may be different from what was
    /// passed to the sfWindow_create function,
    /// if one or more settings were not supported. In this case,
    /// SFML chose the closest match.
    ///
    /// Return a structure containing the OpenGL context settings
    ////
    pub fn get_settings(&self) -> ContextSettings {
        unsafe {
            ffi::sfRenderWindow_getSettings(self.render_window)
        }
    }

    /// Change the title of a window
    ///
    /// # Arguments
    /// * title - New title
    ////
    pub fn set_title(&mut self, title: &str) -> () {
        let c_str = CString::new(title.as_bytes()).unwrap().as_ptr();
        unsafe {
            ffi::sfRenderWindow_setTitle(self.render_window, c_str);
        }
        self.title_length = title.len() as u32;
    }

    /// Show or hide a window
    ///
    /// # Arguments
    /// * visible - true to show the window, false to hide it
    ////
    pub fn set_visible(&mut self, visible: bool) -> () {
        let tmp: SfBool = match visible {
            true    => SFTRUE,
            false   => SFFALSE
        };
        unsafe {
            ffi::sfRenderWindow_setVisible(self.render_window, tmp);
        }
    }

    /// Show or hide the mouse cursor
    ///
    /// # Arguments
    /// * visible - true to  false to hide
    ////
    pub fn set_mouse_cursor_visible(&mut self, visible: bool) -> () {
        let tmp: SfBool = match visible {
            true    => SFTRUE,
            false   => SFFALSE
        };
        unsafe {
            ffi::sfRenderWindow_setMouseCursorVisible(self.render_window, tmp);
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
    ////
    pub fn set_vertical_sync_enabled(&mut self, enabled: bool) -> () {
        let tmp: SfBool =
            match enabled {
            true    => SFTRUE,
            false   => SFFALSE
        };
        unsafe {
            ffi::sfRenderWindow_setVerticalSyncEnabled(self.render_window, tmp);
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
    ////
    pub fn set_key_repeat_enabled(&mut self, enabled: bool) -> () {
        let tmp: SfBool = match enabled {
            true    => SFTRUE,
            false   => SFFALSE
        };
        unsafe {
            ffi::sfRenderWindow_setKeyRepeatEnabled(self.render_window, tmp);
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
    ////
    pub fn set_active(&mut self, enabled: bool) -> bool {
        let tmp: SfBool = match enabled {
            true    => SFTRUE,
            false   => SFFALSE
        };
        let res: SfBool = unsafe {
            ffi::sfRenderWindow_setActive(self.render_window, tmp)
        };
        match res {
            SFTRUE      => true,
            SFFALSE     => false
        }
    }

    /// Change the joystick threshold
    ///
    /// The joystick threshold is the value below which
    /// no JoyMoved event will be generated.
    ///
    /// # Arguments
    /// * threshold - New threshold, in the range [0, 100]
    ////
    pub fn set_joystick_threshold(&mut self, threshold: f32) -> () {
        unsafe {
            ffi::sfRenderWindow_setJoystickThreshold(self.render_window,
                                                     threshold as c_float)
        }
    }

    /// Get the position of a window
    ///
    /// Return the position in pixels
    ////
    pub fn get_position(&self) -> Vector2i {
        unsafe {
            ffi::sfRenderWindow_getPosition(self.render_window)
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
    ////
    pub fn set_position(&mut self, position: &Vector2i) -> () {
        unsafe {
            ffi::sfRenderWindow_setPosition(self.render_window, *position)
        }
    }



    /// Change the size of the rendering region of a window
    ///
    /// # Arguments
    /// * size - New size, in pixels
    ////
    pub fn set_size(&mut self, size: &Vector2u) -> () {
        unsafe {
            ffi::sfRenderWindow_setSize(self.render_window, *size)
        }
    }

    /// Change the size of the rendering region of a window
    ///
    /// # Arguments
    /// * size_x - New size x, in pixels
    /// * size_y - New size x, in pixels
    ////
    pub fn set_size2u(&mut self, size_x: u32, size_y: u32) -> () {
        unsafe {
            ffi::sfRenderWindow_setSize(self.render_window,
                                        Vector2u::new(size_x, size_y))
        }
    }

    /// Get the current position of the mouse relatively to a render window
    ///
    /// This function returns the current position of the mouse
    /// cursor relative to the given render window.
    ///
    /// Return the position of the mouse cursor, relative to the given render window
    ////
    pub fn get_mouse_position(&self) -> Vector2i {
        unsafe {
            ffi::sfMouse_getPositionRenderWindow(self.render_window)
        }
    }

    /// Set the current position of the mouse relatively to a render window
    ///
    /// This function sets the current position of the mouse cursor relative
    /// to the given render window
    ///
    /// # Arguments
    /// * `position` - the positon to set
    ////
    pub fn set_mouse_position(&mut self, position: &Vector2i) -> () {
        unsafe {
            ffi::sfMouse_setPositionRenderWindow(*position, self.render_window)
        }
    }

    /// Copy the current contents of a render window to an image
    ///
    /// This is a slow operation, whose main purpose is to make
    /// screenshots of the application. If you want to update an
    /// image with the contents of the window and then use it for
    /// drawing, you should rather use a [Texture](struct.Texture.html) and its
    /// [update(Window)](struct.Texture.html#method.update_from_window) function.
    /// You can also draw things directly to a texture with the
    /// RenderWindow.
    ///
    /// Return a new image containing the captured contents
    ////
    pub fn capture(&mut self) -> Option<Image> {
        let img = unsafe { ffi::sfRenderWindow_capture(self.render_window) };
        if img.is_null() {
            None
        } else {
            Some(Wrappable::wrap(img))
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *mut ffi::sfRenderWindow {
        self.render_window
    }
}

impl RenderTarget for RenderWindow{

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
    fn push_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_pushGLStates(self.render_window)
        }
    }

    /// Restore the previously saved OpenGL render states and matrices
    fn pop_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_popGLStates(self.render_window)
        }
    }

    /// Reset the internal OpenGL states so that the target is ready for drawing
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering, if you choose not to use
    /// push_GL_states/pop_GL_states. It makes sure that all OpenGL
    /// states needed by SFML are set, so that subsequent draw()
    /// calls will work as expected.
    fn reset_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_resetGLStates(self.render_window)
        }
    }


    /// Change the current active view of a render window
    ///
    /// # Arguments
    /// * view - The new view
    ////
    fn set_view(&mut self, view: &View) -> () {
        unsafe {
            ffi::sfRenderWindow_setView(self.render_window,
                                        view.unwrap())
        }
    }

    /// Get the current active view of a render window
    ///
    /// Return the current active view
    ////
    fn get_view(&self) -> View {
        unsafe{
            Wrappable::wrap(ffi::sfRenderWindow_getView(self.render_window))
        }
    }

    /// Get the default view of a render window
    ///
    /// Return the default view of the render window
    ////
    fn get_default_view(&self) -> View {
        unsafe{
            Wrappable::wrap(ffi::sfRenderWindow_getDefaultView(self.render_window))
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
    fn map_pixel_to_coords(&self,
                           point: &Vector2i,
                           view: &View) -> Vector2f {
        unsafe {
            ffi::sfRenderWindow_mapPixelToCoords(self.render_window,
                                                 *point,
                                                 view.unwrap())
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
    /// [map_pixel_to_coords](#method.map_pixel_to_coords) function if you want to use a custom view.
    ///
    /// # Arguments
    /// * point - Pixel to convert
    ///
    /// Return the converted point, in "world" units
    fn map_pixel_to_coords_current_view(&self,
                                            point: &Vector2i) -> Vector2f {
        let view = unsafe {ffi::sfRenderWindow_getView(self.render_window)};
        unsafe {
            ffi::sfRenderWindow_mapPixelToCoords(self.render_window,
                                                 *point,
                                                 view)
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
    fn map_coords_to_pixel(&self,
                               point: &Vector2f,
                               view: &View) -> Vector2i {
        unsafe {
            ffi::sfRenderWindow_mapCoordsToPixel(self.render_window,
                                                 *point,
                                                 view.unwrap())
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
    fn map_coords_to_pixel_current_view(&self,
                                            point: &Vector2f) -> Vector2i {
        let currView =
            unsafe { ffi::sfRenderWindow_getView(self.render_window) };
        unsafe {
            ffi::sfRenderWindow_mapCoordsToPixel(self.render_window,
                                                 *point,
                                                 currView)
        }
    }

    /// Get the viewport of a view applied to this target
    ///
    /// # Arguments
    /// * view - Target view
    ///
    /// Return the viewport rectangle, expressed in pixels in the current target
    fn get_viewport(&self, view: &View) -> IntRect {
        unsafe {
            ffi::sfRenderWindow_getViewport(self.render_window, view.unwrap())
        }
    }

    /// Get the size of the rendering region of a window
    ///
    /// The size doesn't include the titlebar and borders of the window.
    ///
    /// Return the size in pixels
    fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfRenderWindow_getSize(self.render_window)
        }
    }

    /// Draw a drawable object to the render target
    ///
    /// # Arguments
    /// * object - Object to draw
    fn draw<T: Drawable>(&mut self, object: &T) -> () {
        object.draw(self);
    }

    /// Draw a drawable object to the render-target with a RenderStates
    ///
    /// # Arguments
    /// * object - Object to draw
    /// * renderStates - The renderStates to associate to the object
    fn draw_with_renderstates<T: Drawable>(&mut self,
                                                object: &T,
                                                render_states: &mut RenderStates) {
        object.draw_rs(self, render_states);
    }

    /// Draw a drawable object to the render-target with a RenderStates
    ///
    /// # Arguments
    /// * object - Object to draw
    /// * renderStates - The renderStates to associate to the object
    fn draw_with_renderstates_rc<T: Drawable>(&mut self,
                                                   object: &T,
                                                   render_states: &mut rc::RenderStates) {
        object.draw_rs_rc(self, render_states);
    }

    /// Draw a Text
    fn draw_text(&self, text: &Text) -> () {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window,
                                         text.unwrap(),
                                         ptr::null_mut())
        }
    }

    /// Draw a Text
    fn draw_text_rc(&self, text: &rc::Text) -> () {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window,
                                         text.unwrap(),
                                         ptr::null_mut())
        }
    }

    // /// Draw a Shape
    // fn draw_shape(&self, shape: &Shape) -> () {
    //     unsafe {
    //         ffi::sfRenderWindow_drawShape(self.render_window,
    //                                       shape.unwrap(),
    //                                       ptr::null_mut())
    //     }
    // }

    // /// Draw a Shape
    // fn draw_shape_rc(&self, shape: &rc::Shape) -> () {
    //     unsafe {
    //         ffi::sfRenderWindow_drawShape(self.render_window,
    //                                       shape.unwrap(),
    //                                       ptr::null_mut())
    //     }
    // }

    /// Draw a sprite
    fn draw_sprite(&self, sprite: &Sprite) -> () {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window,
                                           sprite.unwrap(),
                                           ptr::null_mut())
        }
    }

    /// Draw a sprite
    fn draw_sprite_rc(&self, sprite: &rc::Sprite) -> () {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window,
                                           sprite.unwrap(),
                                           ptr::null_mut())
        }
    }

    /// Draw a CircleShape
    fn draw_circle_shape(&self, circle_shape: &CircleShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window,
                                                circle_shape.unwrap(),
                                                ptr::null_mut())
        }
    }

    /// Draw a CircleShape
    fn draw_circle_shape_rc(&self, circle_shape: &rc::CircleShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window,
                                                circle_shape.unwrap(),
                                                ptr::null_mut())
        }
    }

    /// Draw a RectangleShape
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window,
                                                   rectangle_shape.unwrap(),
                                                   ptr::null_mut())
        }
    }

    /// Draw a RectangleShape
    fn draw_rectangle_shape_rc(&self, rectangle_shape: &rc::RectangleShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window,
                                                   rectangle_shape.unwrap(),
                                                   ptr::null_mut())
        }
    }

    /// Draw a ConvexShape
    fn draw_convex_shape(&self, convex_shape: &ConvexShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window,
                                                convex_shape.unwrap(),
                                                ptr::null_mut())
        }
    }

    /// Draw a ConvexShape
    fn draw_convex_shape_rc(&self, convex_shape: &rc::ConvexShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window,
                                                convex_shape.unwrap(),
                                                ptr::null_mut())
        }
    }

    /// Draw a VertexArray
    fn draw_vertex_array(&self, vertex_array: &VertexArray) -> () {
        unsafe {
            ffi::sfRenderWindow_drawVertexArray(self.render_window,
                                                vertex_array.unwrap(),
                                                ptr::null_mut())
        }
    }

    /// Draw a Text with a RenderStates
    fn draw_text_rs(&self,
                        text: &Text,
                        render_states: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window,
                                         text.unwrap(),
                                         render_states.unwrap())
        }
    }

    /// Draw a Text with a RenderStates
    fn draw_text_rs_rc(&self,
                           text: &rc::Text,
                           render_states: &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window,
                                         text.unwrap(),
                                         render_states.unwrap())
        }
    }

    // /// Draw a Shape with a RenderStates
    // fn draw_shape_rs(&self,
    //                      shape: &Shape,
    //                      render_states: &mut RenderStates) -> () {
    //     unsafe {
    //         ffi::sfRenderWindow_drawShape(self.render_window,
    //                                       shape.unwrap(),
    //                                       render_states.unwrap())
    //     }
    // }

    // /// Draw a Shape with a RenderStates
    // fn draw_shape_rs_rc(&self,
    //                         shape: &rc::Shape,
    //                         render_states: &mut rc::RenderStates) -> () {
    //     unsafe {
    //         ffi::sfRenderWindow_drawShape(self.render_window,
    //                                       shape.unwrap(),
    //                                       render_states.unwrap())
    //     }
    // }

    /// Draw a sprite with a RenderStates
    fn draw_sprite_rs(&self,
                          sprite: &Sprite,
                          render_states: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window,
                                           sprite.unwrap(),
                                           render_states.unwrap())
        }
    }

    /// Draw a sprite with a RenderStates
    fn draw_sprite_rs_rc(&self,
                             sprite: &rc::Sprite,
                             render_states: &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window,
                                           sprite.unwrap(),
                                           render_states.unwrap())
        }
    }

    /// Draw a CircleShape with a RenderStates
    fn draw_circle_shape_rs(&self,
                                circle_shape: &CircleShape,
                                render_states: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window,
                                                circle_shape.unwrap(),
                                                render_states.unwrap())
        }
    }

    /// Draw a CircleShape with a RenderStates
    fn draw_circle_shape_rs_rc(&self,
                                   circle_shape: &rc::CircleShape,
                                   render_states: &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window,
                                                circle_shape.unwrap(),
                                                render_states.unwrap())
        }
    }

    /// Draw a RectangleShape with a RenderStates
    fn draw_rectangle_shape_rs(&self,
                                   rectangle_shape: &RectangleShape,
                                   render_states: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window,
                                                   rectangle_shape.unwrap(),
                                                   render_states.unwrap())
        }
    }

    /// Draw a RectangleShape with a RenderStates
    fn draw_rectangle_shape_rs_rc(&self,
                                      rectangle_shape: &rc::RectangleShape,
                                      render_states: &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window,
                                                   rectangle_shape.unwrap(),
                                                   render_states.unwrap())
        }
    }

    /// Draw a ConvexShape with a RenderStates
    fn draw_convex_shape_rs(&self,
                                convex_shape: &ConvexShape,
                                render_states: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window,
                                                convex_shape.unwrap(),
                                                render_states.unwrap())
        }
    }

    /// Draw a ConvexShape with a RenderStates
    fn draw_convex_shape_rs_rc(&self,
                                   convex_shape: &rc::ConvexShape,
                                   render_states: &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window,
                                                convex_shape.unwrap(),
                                                render_states.unwrap())
        }
    }

    /// Draw a VertexArray with a RenderStates
    fn draw_vertex_array_rs(&self,
                                vertex_array: &VertexArray,
                                render_states: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawVertexArray(self.render_window,
                                                vertex_array.unwrap(),
                                                render_states.unwrap())
        }
    }

    /// Draw a VertexArray with a RenderStates
    fn draw_vertex_array_rs_rc(&self,
                                   vertex_array: &VertexArray,
                                   render_states: &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawVertexArray(self.render_window,
                                                vertex_array.unwrap(),
                                                render_states.unwrap())
        }
    }

    /// draw primitives
    fn draw_primitives_rs(&self,
                          vertices: &[Vertex],
                          ty: PrimitiveType,
                          rs: &mut RenderStates) {

        let len = vertices.len() as u32;
        unsafe {
            ffi::sfRenderWindow_drawPrimitives(self.render_window,
                                               &vertices[0],
                                               len,
                                               ty,
                                               rs.unwrap());
        }
    }

    /// draw primitives
    fn draw_primitives(&self,
                       vertices: &[Vertex],
                       ty: PrimitiveType) {

        let len = vertices.len() as u32;
        unsafe {
            ffi::sfRenderWindow_drawPrimitives(self.render_window,
                                               &vertices[0],
                                               len,
                                               ty,
                                               ptr::null_mut());
        }
    }

    /// Clear window with the given color
    fn clear(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfRenderWindow_clear(self.render_window, *color)
        }
    }

}

impl Iterator for Events {
    type Item = event::Event;

    fn next(&mut self) -> Option<event::Event> {
        let mut event = event::raw::sfEvent { data: [032; 6] };
        match unsafe { ffi::sfRenderWindow_pollEvent(self.render_window, &mut event) } {
            SFFALSE     => None,
            SFTRUE      => Some(event::raw::get_wrapped_event(&mut event))
        }
    }
}

#[unsafe_destructor]
impl Drop for RenderWindow {
    /// Destructor for class RenderWindow. Destroy all the ressource.
    fn drop(&mut self) {
        unsafe {
            ffi::sfRenderWindow_destroy(self.render_window);
        }
    }
}
