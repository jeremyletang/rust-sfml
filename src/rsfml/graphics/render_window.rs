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

/*!
* Window that can serve as a target for 2D drawing.
*
* RenderWindow is the main class of the Graphics module.
* It defines an OS window that can be painted using the other classes of the graphics module.
*/

use std::rc::Rc;
use std::cell::RefCell;
use std::libc::{c_float, c_uint, c_int};
use std::{ptr, cast};

use traits::{Drawable, Wrappable};
use window::{ContextSettings, VideoMode, 
    event, keyboard, joystick, mouse, WindowStyle};
use system::vector2::{Vector2f, Vector2i, Vector2u};
use graphics::{Text, Color, Sprite, CircleShape,
    RectangleShape, ConvexShape, RenderStates,
    View, Image, IntRect, VertexArray, Shape, rc};

use ffi::sfml_types::{SfBool, SFTRUE, SFFALSE};
use ffi = ffi::graphics::render_window;

/**
* Window that can serve as a target for 2D drawing.
*
* RenderWindow is the main class of the Graphics module.
* It defines an OS window that can be painted using the other classes of the graphics module.
*/
pub struct RenderWindow {
    #[doc(hidden)]
    priv render_window :    *ffi::sfRenderWindow,
    #[doc(hidden)]
    priv event :            ffi::sfEvent,
    #[doc(hidden)]
    priv title_length :     uint,
    #[doc(hidden)]
    priv current_view :     Rc<RefCell<View>>,
    #[doc(hidden)]
    priv default_view :     Rc<RefCell<View>>
}

/// An iterator over all the events in the events queue (internally call poll_event)
pub struct Events {
    #[doc(hidden)]
    priv render_window: *ffi::sfRenderWindow,
    #[doc(hidden)]
    priv event :            ffi::sfEvent,
}

impl RenderWindow {
    /**
    * Construct a new render window
    *
    * This function creates the render window with the size and pixel
    * depth defined in mode. An optional style can be passed to
    * customize the look and behaviour of the window (borders,
    * title bar, resizable, closable, ...). If style contains
    * sfFullscreen, then mode must be a valid video mode.
    *
    * The fourth parameter is a pointer to a structure specifying
    * advanced OpenGL context settings such as antialiasing,
    * depth-buffer bits, etc.
    * 
    * # Arguments
    * * mode - Video mode to use (defines the width, height and depth of the rendering area of the render window)
    * * title - Title of the render window
    * * style - Window style
    * * settings - Additional settings for the underlying OpenGL context
    *
    * Return Some(RenderWindow) or None
    */
    pub fn new(mode : VideoMode,
        title : &str,
        style : WindowStyle,
        settings : &ContextSettings) -> Option<RenderWindow> {
        
        let mut sf_render_win: *ffi::sfRenderWindow = ptr::null();
        unsafe {
            title.with_c_str(|c_str| {
                sf_render_win = ffi::sfRenderWindow_create(mode.unwrap(), c_str, style as u32, settings); 
            });
        }
        let sf_ev = ffi::sfEvent {
            typeEvent : 0, 
            p1 :        0, 
            p2 :        0, 
            p3 :        0 as c_float, 
            p4 :        0, 
            p5 :        0
        };
        if sf_render_win.is_null() {
            None
        }
        else {
            let raw_def_view = unsafe { ffi::sfRenderWindow_getDefaultView(sf_render_win) };
            if raw_def_view.is_null() {
                None
            }
            else {
                let def_view = Rc::new(RefCell::new(Wrappable::wrap(raw_def_view)));
                Some (RenderWindow {
                    render_window :     sf_render_win, 
                    event :             sf_ev, 
                    title_length :      title.len(),
                    current_view :      def_view.clone(),
                    default_view :      def_view.clone()
                })
            }
        }
    }

    /**
    * Construct a new render window (with a UTF-32 title)
    *
    * This function creates the render window with the size and pixel
    * depth defined in mode. An optional style can be passed to
    * customize the look and behaviour of the render window (borders,
    * title bar, resizable, closable, ...). If style contains
    * sfFullscreen, then mode must be a valid video mode.
    *
    * The fourth parameter is a pointer to a structure specifying
    * advanced OpenGL context settings such as antialiasing,
    * depth-buffer bits, etc.
    * 
    * # Arguments
    * * mode - Video mode to use (defines the width, height and depth of the rendering area of the render window)
    * * title - Title of the render window (UTF-32)
    * * style - Window style
    * * settings - Additional settings for the underlying OpenGL context
    *
    * Return Some(RenderWindow) or None
    */
    pub fn new_with_unicode(mode : VideoMode,
        title : ~[u32],
        style : WindowStyle,
        settings : &ContextSettings) -> Option<RenderWindow> {
        
        let sf_render_win: *ffi::sfRenderWindow;
        unsafe { 
            sf_render_win = ffi::sfRenderWindow_createUnicode(mode.unwrap(), title.as_ptr(), style as u32, settings);
        }
        let sf_ev = ffi::sfEvent {
            typeEvent : 0, 
            p1 :        0, 
            p2 :        0, 
            p3 :        0 as c_float, 
            p4 :        0, 
            p5 :        0
        };
        if sf_render_win.is_null() {
            None
        }
        else {
            let raw_def_view = unsafe { ffi::sfRenderWindow_getDefaultView(sf_render_win) };
            if raw_def_view.is_null() {
                None
            }
            else {
                let def_view = Rc::new(RefCell::new(Wrappable::wrap(raw_def_view)));
                Some (RenderWindow {
                    render_window :     sf_render_win,
                    event :             sf_ev,
                    title_length :      title.len(),
                    current_view :      def_view.clone(),
                    default_view :      def_view.clone()
                })
            } 
        }
    }

    /**
    * Change the title of a render window (with a UTF-32 string)
    *
    * # Arguments
    * * title - New title
    */
    pub fn set_unicode_title(&mut self, title : ~[u32]) -> () {
        unsafe {
            self.title_length = title.len();
            ffi::sfRenderWindow_setUnicodeTitle(self.render_window, title.as_ptr())
        }
    }

    /**
    * Change a render window's icon
    * pixels must be an array of width x height pixels in 32-bits RGBA format.
    *
    * # Arguments
    * * width - Icon's width, in pixels
    * * height - Icon's height, in pixels
    * * pixels - Vector of pixels
    */
    pub fn set_icon(&mut self, width : uint, height : uint, pixels : ~[u8]) -> () {
        unsafe {
            ffi::sfRenderWindow_setIcon(self.render_window, width as c_uint, height as c_uint, pixels.as_ptr())
        }
    }

    /// Return an iterator over all the event currently in the events queue.
    pub fn events(&self) -> Events {
        Events {
            render_window: self.render_window.clone(),
            event : ffi::sfEvent {
                typeEvent : 0,
                p1 :        0,
                p2 :        0,
                p3 :        0 as c_float,
                p4 :        0,
                p5 :        0
            }
        }
    }

    /**
    *  Pop the event on top of event queue, if any, and return it
    *
    * This function is not blocking: if there's no pending event then
    * it will return false and leave \a event unmodified.
    * Note that more than one event may be present in the event queue,
    * thus you should always call this function in a loop
    * to make sure that you process every pending event.
    *
    * Return the event if an event was returned, or NoEvent if the event queue was empty
    */
    pub fn poll_event(&mut self) -> event::Event {
        let haveEvent : bool =  unsafe {
            match ffi::sfRenderWindow_pollEvent(self.render_window, &self.event) {
                SFFALSE     => false,
                SFTRUE      => true
            }
        };
        if haveEvent == false {
            event::NoEvent
        }
        else {
            get_wrapped_event(&self.event)
        }
    }

    /**
    * Wait for an event and return it
    *
    * This function is blocking: if there's no pending event then
    * it will wait until an event is received.
    * After this function returns (and no error occured),
    * the event object is always valid and filled properly.
    * This function is typically used when you have a thread that
    * is dedicated to events handling: you want to make this thread
    * sleep as long as no new event is received.
    *
    * Return the event or NoEvent if an error has occured
    */
    pub fn wait_event(&mut self) -> event::Event {
        let haveEvent : bool =  unsafe {
            match ffi::sfRenderWindow_waitEvent(self.render_window, &self.event) {
                SFFALSE     => false,
                SFTRUE      => true
            }
        };
        if haveEvent == false {
            event::NoEvent
        }
        else {
            get_wrapped_event(&self.event)
        }
    }

    /**
    * Close a render window and destroy all the attached resources
    *
    * After calling this method, the Window object remains
    * valid.
    * All other functions such as poll_event or display
    * will still work (i.e. you don't have to test is_open
    * every time), and will have no effect on closed windows.
    */
    pub fn close(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_close(self.render_window);
        }
    }

    /**
    * Tell whether or not a window is opened
    *
    * This function returns whether or not the window exists.
    * Note that a hidden window (set_visible(false)) will return
    * true.
    */
    pub fn is_open(&self) -> bool {
        let tmp : SfBool;
        unsafe {
            tmp = ffi::sfRenderWindow_isOpen(self.render_window);
        }
        match tmp {
            SFFALSE => false,
            SFTRUE  => true
        }
    }

    /**
    * Display on screen what has been rendered to the window so far
    *
    * This function is typically called after all OpenGL rendering
    * has been done for the current frame, in order to show
    * it on screen.
    */
    pub fn display(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_display(self.render_window)
        }
    }

    /**
    * Limit the framerate to a maximum fixed frequency
    *
    * If a limit is set, the window will use a small delay after
    * each call to sfWindow_display to ensure that the current frame
    * lasted long enough to match the framerate limit.
    *
    * # Arguments
    * * limit - Framerate limit, in frames per seconds (use 0 to disable limit)
    */
    pub fn set_framerate_limit(&mut self, limit : uint) -> () {
        unsafe {
            ffi::sfRenderWindow_setFramerateLimit(self.render_window, limit as c_uint)
        }
    }
    
    /**
    * Get the settings of the OpenGL context of a window
    *
    * Note that these settings may be different from what was
    * passed to the sfWindow_create function,
    * if one or more settings were not supported. In this case,
    * SFML chose the closest match.
    *
    * Return a structure containing the OpenGL context settings
    */
    pub fn get_settings(&self) -> ContextSettings {
        unsafe {
            ffi::sfRenderWindow_getSettings(self.render_window)
        }
    }

    /**
    * Change the title of a window
    *
    * # Arguments
    * * title - New title
    */
    pub fn set_title(&mut self, title : &str) -> () {
        unsafe {
            title.with_c_str(|c_str| {
                ffi::sfRenderWindow_setTitle(self.render_window, c_str);
            });
        }
        self.title_length = title.len();
    }

    /**
    * Show or hide a window
    *
    * # Arguments
    * * visible - true to show the window, false to hide it
    */
    pub fn set_visible(&mut self, visible : bool) -> () {
        let tmp : SfBool =
            match visible {
                true    => SFTRUE,
                false   => SFFALSE
            };
        unsafe {
            ffi::sfRenderWindow_setVisible(self.render_window, tmp);
        }
    }

    /**
    * Show or hide the mouse cursor
    *
    * # Arguments
    * * visible - true to show, false to hide
    */
    pub fn set_mouse_cursor_visible(&mut self, visible : bool) -> () {
        let tmp : SfBool =
            match visible {
                true    => SFTRUE,
                false   => SFFALSE
            };
        unsafe {
            ffi::sfRenderWindow_setMouseCursorVisible(self.render_window, tmp);
        }
    }

    /**
    * Enable or disable vertical synchronization
    *
    * Activating vertical synchronization will limit the number
    * of frames displayed to the refresh rate of the monitor.
    * This can avoid some visual artifacts, and limit the framerate
    * to a good value (but not constant across different computers).
    *
    * # Arguments
    * * enabled - true to enable v-sync, false to deactivate
    */
    pub fn set_vertical_sync_enabled(&mut self, enabled : bool) -> () {
        let tmp : SfBool =
            match enabled {
                true    => SFTRUE,
                false   => SFFALSE
            };
        unsafe {
            ffi::sfRenderWindow_setVerticalSyncEnabled(self.render_window, tmp);
        }
    }

    /**
    * Enable or disable automatic key-repeat
    *
    * If key repeat is enabled, you will receive repeated
    * KeyPress events while keeping a key pressed. If it is disabled,
    * you will only get a single event when the key is pressed.
    *
    * Key repeat is enabled by default.
    *
    * # Arguments
    * * enabled - true to enable, false to disable
    */
    pub fn set_key_repeat_enabled(&mut self, enabled : bool) -> () {
        let tmp : SfBool =
            match enabled {
                true    => SFTRUE,
                false   => SFFALSE
            };
        unsafe {
            ffi::sfRenderWindow_setKeyRepeatEnabled(self.render_window, tmp);
        }
    }

    /**
    * Activate or deactivate a render window as the current target for OpenGL rendering
    *
    * A window is active only on the current thread, if you want to
    * make it active on another thread you have to deactivate it
    * on the previous thread first if it was active.
    * Only one window can be active on a thread at a time, thus
    * the window previously active (if any) automatically gets deactivated.
    *
    * # Arguments
    * * active - true to activate, false to deactivate
    *
    * Return true if operation was successful, false otherwise
    */
    pub fn set_active(&mut self, enabled : bool) -> bool {
        let tmp : SfBool = match enabled {
            true    => SFTRUE,
            false   => SFFALSE
        };
        let res : SfBool = unsafe {
            ffi::sfRenderWindow_setActive(self.render_window, tmp)
        };
        match res {
            SFTRUE      => true,
            SFFALSE     => false
        }
    }

    /**
    * Change the joystick threshold
    *
    * The joystick threshold is the value below which
    * no JoyMoved event will be generated.
    *
    * # Arguments
    * * threshold - New threshold, in the range [0, 100]
    */
    pub fn set_joystick_threshold(&mut self, threshold : f32) -> () {
        unsafe {
            ffi::sfRenderWindow_setJoystickThreshold(self.render_window, threshold as c_float)
        }
    }

    /**
    *  Get the position of a window
    *
    * Return the position in pixels
    */
    pub fn get_position(&self) -> Vector2i {
        unsafe {
            ffi::sfRenderWindow_getPosition(self.render_window)
        }
    }

    /**
    * Change the position of a window on screen
    *
    * This function only works for top-level windows
    * (i.e. it will be ignored for windows created from
    * the handle of a child window/control).
    *
    * # Arguments
    * * position - New position of the window, in pixels
    */
    pub fn set_position(&mut self, position : &Vector2i) -> () {
        unsafe {
            ffi::sfRenderWindow_setPosition(self.render_window, *position)
        }
    }

    /**
    * Get the size of the rendering region of a window
    *
    * The size doesn't include the titlebar and borders of the window.
    *
    * Return the size in pixels
    */
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfRenderWindow_getSize(self.render_window)
        }
    }

    /**
    * Change the size of the rendering region of a window
    *
    * # Arguments
    * * size - New size, in pixels
    */
    pub fn set_size(&mut self, size : &Vector2u) -> () {
        unsafe {
            ffi::sfRenderWindow_setSize(self.render_window, *size)
        }
    }

    /**
    * Change the size of the rendering region of a window
    *
    * # Arguments
    * * size_x - New size x, in pixels
    * * size_y - New size x, in pixels
    */
    pub fn set_size2f(&mut self, size_x : u32, size_y : u32) -> () {
        unsafe {
            ffi::sfRenderWindow_setSize(self.render_window, Vector2u::new(size_x, size_y))
        }
    }

    /**
    * Save the current OpenGL render states and matrices
    *
    * This function can be used when you mix SFML drawing
    * and direct OpenGL rendering. Combined with popGLStates,
    * it ensures that: 
    * SFML's internal states are not messed up by your OpenGL code
    * and that your OpenGL states are not modified by a call to a SFML function
    *
    * Note that this function is quite expensive: it saves all the
    * possible OpenGL states and matrices, even the ones you
    * don't care about. Therefore it should be used wisely.
    * It is provided for convenience, but the best results will
    * be achieved if you handle OpenGL states yourself (because
    * you know which states have really changed, and need to be
    * saved and restored). Take a look at the resetGLStates
    * function if you do so.
    *
    */
    pub fn push_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_pushGLStates(self.render_window)
        }
    }

    /**
    * Restore the previously saved OpenGL render states and matrices
    */
    pub fn pop_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_popGLStates(self.render_window)
        }
    }

    /**
    * Reset the internal OpenGL states so that the target is ready for drawing
    *
    * This function can be used when you mix SFML drawing
    * and direct OpenGL rendering, if you choose not to use
    * pushGLStates/popGLStates. It makes sure that all OpenGL
    * states needed by SFML are set, so that subsequent sfRenderWindow_draw*()
    * calls will work as expected.
    */
    pub fn reset_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderWindow_resetGLStates(self.render_window)
        }
    }

    /**
    * Get the current position of the mouse relatively to a render window
    *
    * This function returns the current position of the mouse
    * cursor relative to the given render window.
    *
    * Return the position of the mouse cursor, relative to the given render window
    */
    pub fn get_mouse_position(&self) -> Vector2i {
        unsafe {
            ffi::sfMouse_getPositionRenderWindow(self.render_window)
        }
    }

    /**
    * Set the current position of the mouse relatively to a render-window
    *
    * This function sets the current position of the mouse cursor relative to the given render-window
    *
    * # Arguments
    * * relativeTo - Reference render window
    */
    pub fn set_mouse_position(&mut self, position : &Vector2i) -> () {
        unsafe {
            ffi::sfMouse_setPositionRenderWindow(*position, self.render_window)
        }
    }

    /**
    * Draw a drawable object to the render-target
    *
    * # Arguments
    * * object - Object to draw
    */
    pub fn draw<T : Drawable>(&mut self, object : &T) -> () {
        object.draw_in_render_window(self);
    }

    /**
    * Draw a drawable object to the render-target with a RenderStates
    *
    * # Arguments 
    * * object - Object to draw
    * * renderStates - The renderStates to associate to the object
    */
    pub fn draw_with_renderstates<T : Drawable>(&mut self, object : &T, render_states : &mut RenderStates) -> () {
        object.draw_in_render_window_rs(self, render_states);
    }

    /**
    * Draw a drawable object to the render-target with a RenderStates
    *
    * # Arguments 
    * * object - Object to draw
    * * renderStates - The renderStates to associate to the object
    */
    pub fn draw_with_renderstates_rc<T : Drawable>(&mut self, object : &T, render_states : &mut rc::RenderStates) -> () {
        object.draw_in_render_window_rs_rc(self, render_states);
    }

    /// Draw a Text
    pub fn draw_text(&self, text : &Text) -> () {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window, text.unwrap(), ptr::null())
        }
    }

    /// Draw a Text
    pub fn draw_text_rc(&self, text : &rc::Text) -> () {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window, text.unwrap(), ptr::null())
        }
    }

    /// Draw a Shape
    pub fn draw_shape(&self, shape : &Shape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawShape(self.render_window, shape.unwrap(), ptr::null())
        }
    }

    /// Draw a Shape
    pub fn draw_shape_rc(&self, shape : &rc::Shape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawShape(self.render_window, shape.unwrap(), ptr::null())
        }
    }

    /// Draw a sprite
    pub fn draw_sprite(&self, sprite : &Sprite) -> () {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window, sprite.unwrap(), ptr::null())
        }
    }

    /// Draw a sprite
    pub fn draw_sprite_rc(&self, sprite : &rc::Sprite) -> () {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window, sprite.unwrap(), ptr::null())
        }
    }

    /// Draw a CircleShape
    pub fn draw_circle_shape(&self, circle_shape : &CircleShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window, circle_shape.unwrap(), ptr::null())
        }
    }

    /// Draw a CircleShape
    pub fn draw_circle_shape_rc(&self, circle_shape : &rc::CircleShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window, circle_shape.unwrap(), ptr::null())
        }
    }

    /// Draw a RectangleShape
    pub fn draw_rectangle_shape(&self, rectangle_shape : &RectangleShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window, rectangle_shape.unwrap(), ptr::null())
        }
    }

    /// Draw a RectangleShape
    pub fn draw_rectangle_shape_rc(&self, rectangle_shape : &rc::RectangleShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window, rectangle_shape.unwrap(), ptr::null())
        }
    }

    /// Draw a ConvexShape
    pub fn draw_convex_shape(&self, convex_shape : &ConvexShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window, convex_shape.unwrap(), ptr::null())
        }
    }

    /// Draw a ConvexShape
    pub fn draw_convex_shape_rc(&self, convex_shape : &rc::ConvexShape) -> () {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window, convex_shape.unwrap(), ptr::null())
        }
    }

    /// Draw a VertexArray
    pub fn draw_vertex_array(&self, vertex_array : &VertexArray) -> () {
        unsafe {
            ffi::sfRenderWindow_drawVertexArray(self.render_window, vertex_array.unwrap(), ptr::null())
        }
    }

    /// Draw a Text with a RenderStates
    pub fn draw_text_rs(&self, text : &Text, render_states : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window, text.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a Text with a RenderStates
    pub fn draw_text_rs_rc(&self, text : &rc::Text, render_states : &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawText(self.render_window, text.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a Shape with a RenderStates
    pub fn draw_shape_rs(&self, shape : &Shape, render_states : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawShape(self.render_window, shape.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a Shape with a RenderStates
    pub fn draw_shape_rs_rc(&self, shape : &rc::Shape, render_states : &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawShape(self.render_window, shape.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a sprite with a RenderStates
    pub fn draw_sprite_rs(&self, sprite : &Sprite, render_states : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window, sprite.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a sprite with a RenderStates
    pub fn draw_sprite_rs_rc(&self, sprite : &rc::Sprite, render_states : &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawSprite(self.render_window, sprite.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a CircleShape with a RenderStates
    pub fn draw_circle_shape_rs(&self, circle_shape : &CircleShape, render_states : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window, circle_shape.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a CircleShape with a RenderStates
    pub fn draw_circle_shape_rs_rc(&self, circle_shape : &rc::CircleShape, render_states : &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawCircleShape(self.render_window, circle_shape.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a RectangleShape with a RenderStates
    pub fn draw_rectangle_shape_rs(&self, rectangle_shape : &RectangleShape, render_states : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window, rectangle_shape.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a RectangleShape with a RenderStates
    pub fn draw_rectangle_shape_rs_rc(&self, rectangle_shape : &rc::RectangleShape, render_states : &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawRectangleShape(self.render_window, rectangle_shape.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a ConvexShape with a RenderStates
    pub fn draw_convex_shape_rs(&self, convex_shape : &ConvexShape, render_states : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window, convex_shape.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a ConvexShape with a RenderStates
    pub fn draw_convex_shape_rs_rc(&self, convex_shape : &rc::ConvexShape, render_states : &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawConvexShape(self.render_window, convex_shape.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a VertexArray with a RenderStates
    pub fn draw_vertex_array_rs(&self, vertex_array : &VertexArray, render_states : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawVertexArray(self.render_window, vertex_array.unwrap(), render_states.unwrap())
        }
    }

    /// Draw a VertexArray with a RenderStates
    pub fn draw_vertex_array_rs_rc(&self, vertex_array : &VertexArray, render_states : &mut rc::RenderStates) -> () {
        unsafe {
            ffi::sfRenderWindow_drawVertexArray(self.render_window, vertex_array.unwrap(), render_states.unwrap())
        }
    }

    /// Clear window with the given color
    pub fn clear(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfRenderWindow_clear(self.render_window, *color)
        }
    }
    
    /**
    * Copy the current contents of a render window to an image
    *
    * This is a slow operation, whose main purpose is to make
    * screenshots of the application. If you want to update an
    * image with the contents of the window and then use it for
    * drawing, you should rather use a Texture and its
    * update(Window) function.
    * You can also draw things directly to a texture with the
    * sfRenderWindow class.
    *
    * Return a new image containing the captured contents
    */
    pub fn capture(&mut self) -> Option<Image> {
        let img = unsafe { ffi::sfRenderWindow_capture(self.render_window) };
        if img.is_null() {
            None
        }
        else {
            Some(Wrappable::wrap(img))
        }
    }

    /**
    * Change the current active view of a render window
    *
    * # Arguments
    * * view - The new view
    */
    pub fn set_view(&mut self, view : Rc<RefCell<View>>) -> () {
        self.current_view = view;
        unsafe {
            ffi::sfRenderWindow_setView(self.render_window, self.current_view.borrow().with(|v| v.unwrap()))  
        }
    }

    /**
    * Get the current active view of a render window
    *
    * Return the current active view
    */
    pub fn get_view(&self) -> Rc<RefCell<View>> {
        self.current_view.clone()
    }

    /**
    * Get the default view of a render window
    *
    * Return the default view of the render window
    */
    pub fn get_default_view(&self) -> Rc<RefCell<View>> {
        self.default_view.clone()
    }
    
    /**
    * Convert a point from window coordinates to world coordinates
    *
    * This function finds the 2D position that matches the
    * given pixel of the render-window. In other words, it does
    * the inverse of what the graphics card does, to find the
    * initial position of a rendered pixel.
    *
    * Initially, both coordinate systems (world units and target pixels)
    * match perfectly. But if you define a custom view or resize your
    * render window, this assertion is not true anymore, ie. a point
    * located at (10, 50) in your render-window may map to the point
    * (150, 75) in your 2D world -- if the view is translated by (140, 25).
    * 
    * This function is typically used to find which point (or object) is
    * located below the mouse cursor.
    * 
    * This version uses a custom view for calculations, see the
    * map_pixel_to_coords_current_view function if you want to use the current view of the
    * render-window.
    *
    * # Arguments
    * * point - Pixel to convert
    * * view - The view to use for converting the point
    * 
    * Return the converted point, in "world" units
    */
    pub fn map_pixel_to_coords(&self, point : &Vector2i, view : &View) -> Vector2f {
        unsafe {
            ffi::sfRenderWindow_mapPixelToCoords(self.render_window, *point, view.unwrap())
        }
    }

    /**
    * Convert a point from window coordinates to world coordinates
    *
    * This function finds the 2D position that matches the
    * given pixel of the render-window. In other words, it does
    * the inverse of what the graphics card does, to find the
    * initial position of a rendered pixel.
    *
    * Initially, both coordinate systems (world units and target pixels)
    * match perfectly. But if you define a custom view or resize your
    * render window, this assertion is not true anymore, ie. a point
    * located at (10, 50) in your render-window may map to the point
    * (150, 75) in your 2D world -- if the view is translated by (140, 25).
    * 
    * This function is typically used to find which point (or object) is
    * located below the mouse cursor.
    * 
    * This version uses the current view for calculations, see the
    * map_pixel_to_coords function if you want to use a custom view.
    *
    * # Arguments
    * * point - Pixel to convert
    * 
    * Return the converted point, in "world" units
    */
    pub fn map_pixel_to_coords_current_view(&self, point : &Vector2i) -> Vector2f {
        let view = unsafe {ffi::sfRenderWindow_getView(self.render_window)};
        unsafe {
            ffi::sfRenderWindow_mapPixelToCoords(self.render_window, *point, view)
        }
    }

    /**
    * Convert a point from world coordinates to window coordinates
    *
    * This function finds the pixel of the render-window that matches
    * the given 2D point. In other words, it goes through the same process
    * as the graphics card, to compute the final position of a rendered point.
    *
    * Initially, both coordinate systems (world units and target pixels)
    * match perfectly. But if you define a custom view or resize your
    * render window, this assertion is not true anymore, ie. a point
    * located at (150, 75) in your 2D world may map to the pixel
    * (10, 50) of your render-window -- if the view is translated by (140, 25).
    * 
    * This version uses a custom view for calculations, see
    * map_coords_to_pixel_current_view if you want to use the current view of the
    * render-window.
    *
    * # Arguments
    * * point - Point to convert
    * * view - The view to use for converting the point
    */
    pub fn map_coords_to_pixel(&self, point : &Vector2f, view : &View) -> Vector2i {
        unsafe {
            ffi::sfRenderWindow_mapCoordsToPixel(self.render_window, *point, view.unwrap())
        }
    }

    /**
    * Convert a point from world coordinates to window coordinates
    *
    * This function finds the pixel of the render-window that matches
    * the given 2D point. In other words, it goes through the same process
    * as the graphics card, to compute the final position of a rendered point.
    *
    * Initially, both coordinate systems (world units and target pixels)
    * match perfectly. But if you define a custom view or resize your
    * render window, this assertion is not true anymore, ie. a point
    * located at (150, 75) in your 2D world may map to the pixel
    * (10, 50) of your render-window -- if the view is translated by (140, 25).
    * 
    * This version uses the current view for calculations, see 
    * map_coords_to_pixel if you want to use a custom view.
    *
    * # Arguments
    * * point - Point to convert
    */
    pub fn map_coords_to_pixel_current_view(&self, point : &Vector2f) -> Vector2i {
        let currView = unsafe { ffi::sfRenderWindow_getView(self.render_window) };
        unsafe {
            ffi::sfRenderWindow_mapCoordsToPixel(self.render_window, *point, currView)
        }
    }
    
    /**
    * Get the viewport of a view applied to this target
    *
    * # Arguments
    * * view - Target view
    *
    * Return the viewport rectangle, expressed in pixels in the current target
    */
    pub fn get_viewport(&self, view : &View) -> IntRect {
        unsafe {
            ffi::sfRenderWindow_getViewport(self.render_window, view.unwrap())
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *ffi::sfRenderWindow {
        self.render_window
    }
}

#[doc(hidden)]
fn get_wrapped_event(event: &ffi::sfEvent) ->event::Event {
    match event.typeEvent as c_uint {
        0   => event::Closed,
        1   => event::Resized{ width : event.p1 as int, height : event.p2 as int },
        2   => event::LostFocus,
        3   => event::GainedFocus,
        4   => {
            event::TextEntered { 
                code : (event.p1 as u8) as char
            }
        },
        5   => {
            let al : bool = match event.p2 {
                0 => false,
                _ => true
            };
            let ct : bool = match event.p3 as int{
                0 => false,
                _ => true
            };
            let sh : bool = match event.p4  {
                0 => false,
                _ => true
            };
            let sy : bool = match event.p5 {
                0 => false,
                _ => true
            };
            let k : keyboard::Key = unsafe { cast::transmute(event.p1 as i64) };
            event::KeyPressed{ 
                code : k, 
                alt : al, 
                ctrl : ct, 
                shift :sh, 
                system : sy 
            }
        },
        6   => {
            let al : bool = match event.p2 {
                0 => false,
                _ => true
            };
            let ct : bool = match event.p3 as int{
                0 => false,
                _ => true
            };
            let sh : bool = match event.p4  {
                0 => false,
                _ => true
            };
            let sy : bool = match event.p5 {
                0 => false,
                _ => true
            };
            let k : keyboard::Key = unsafe { cast::transmute(event.p1 as i64) };
            event::KeyReleased {
                code : k, 
                alt : al, 
                ctrl : ct, 
                shift :sh, 
                system : sy 
            }
        },
        7   => {
            event::MouseWheelMoved{
                delta : unsafe { cast::transmute::<c_uint, c_int>(event.p1) }  as int,
                x :     unsafe { cast::transmute::<c_uint, c_int>(event.p2) }  as int,
                y :     unsafe { cast::transmute::<c_float, c_int>(event.p3) } as int
            }
        },
        8   => {
            let button : mouse::MouseButton = unsafe {cast::transmute(event.p1 as i8)};
            event::MouseButtonPressed{
                button : button,
                x :      unsafe { cast::transmute::<c_uint, c_int>(event.p2) as int },
                y :      unsafe { cast::transmute::<c_float, c_int>(event.p3) as int }
            }
        },
        9   => {
            let button : mouse::MouseButton = unsafe { cast::transmute(event.p1 as i8) };
            event::MouseButtonReleased{
                button : button,
                x :      unsafe { cast::transmute::<c_uint, c_int>(event.p2) as int },
                y :      unsafe { cast::transmute::<c_float, c_int>(event.p3) as int }
            }
        },
        10  => { 
            event::MouseMoved {
                x : unsafe { cast::transmute::<c_uint, c_int>(event.p1) } as int,
                y : unsafe { cast::transmute::<c_uint, c_int>(event.p2) } as int
            }
        },
        11  => event::MouseEntered,
        12  => event::MouseLeft,
        13  => {
            event::JoystickButtonPressed {
                joystickid : event.p1 as int, 
                button : event.p2 as int
            }
        },
        14  => { 
            event::JoystickButtonReleased{
                joystickid : event.p1 as int, 
                button : event.p2 as int
            }
        },
        15  => {
            let ax : joystick::Axis = unsafe { cast::transmute(event.p2 as i8) };
            event::JoystickMoved{
                joystickid : event.p1 as uint, 
                axis : ax, 
                position : event.p3 as f32
            }
        },
        16  => { 
            event::JoystickConnected{
                joystickid : event.p1 as uint
            }
        },
        17  => { 
            event::JoystickDisconnected{
                joystickid : event.p1 as uint
            }
        },
        _ => event::NoEvent
    }
}

impl Iterator<event::Event> for Events {
    fn next(&mut self) -> Option<event::Event> {
        match unsafe { ffi::sfRenderWindow_pollEvent(self.render_window, &self.event) } {
            SFFALSE     => None,
            SFTRUE      => Some(get_wrapped_event(&self.event))
        }
    }
} 


#[unsafe_destructor]
impl Drop for RenderWindow {
    /**
    *   Destructor for class RenderWindow. Destroy all the ressource.
    */
    fn drop(&mut self) {
        unsafe {
            ffi::sfRenderWindow_destroy(self.render_window);
        }
    }
}
