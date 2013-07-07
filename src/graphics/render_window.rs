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
*
*/

use std::libc::{c_float, c_uint, c_int};
use std::str;
use std::ptr;
use std::cast;
use std::vec;

use rsfml::sfTypes::{sfBool};
use window::video_mode::*;
use window::context_settings::ContextSettings;
use window::event;
use window::keyboard;
use system::vector2::{Vector2f, Vector2i, Vector2u};
use window::joystick;
use window::mouse;
use graphics::text::Text;
use graphics::drawable::Drawable;
use graphics::color::Color;
use graphics::sprite::Sprite;
use graphics::circle_shape::CircleShape;
use graphics::rectangle_shape::RectangleShape;
use graphics::convex_shape::ConvexShape;
//use graphics::transform;
//use graphics::render_states;
use graphics::view::View;
use graphics::image::Image;
use graphics::rect::IntRect;
use graphics::vertex_array::VertexArray;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_uint, c_float, c_void, c_char};
    use system::vector2::{Vector2f, Vector2i, Vector2u};
    use rsfml::sfTypes::{sfBool};
    use window::video_mode::*;
    use window::context_settings::ContextSettings;
    use graphics::text::csfml::sfText;
    use graphics::render_states;
    use graphics::color::Color;
    use graphics::sprite::csfml::sfSprite;
    use graphics::circle_shape::csfml::sfCircleShape;
    use graphics::rectangle_shape::csfml::sfRectangleShape;
    use graphics::convex_shape::csfml::sfConvexShape;
    use graphics::view::csfml::sfView;
    use graphics::image::csfml::sfImage;
    use graphics::rect::IntRect;
    use graphics::vertex_array::csfml::sfVertexArray;

    pub struct sfRenderWindow {
        This : *c_void,
        DefaultView : sfView,
        CurrentView : sfView
    }

    pub struct sfEvent {
        typeEvent : c_uint,
        p1 : c_uint,
        p2 : c_uint,
        p3 : c_float,
        p4 : c_uint,
        p5 : c_uint
    }

    pub extern "C" {
        fn sfRenderWindow_create(mode : csfml::sfVideoMode, title : *c_char, style : c_uint, settings : *ContextSettings) -> *sfRenderWindow;
        fn sfRenderWindow_createUnicode(mode : csfml::sfVideoMode, title : *u32, style : c_uint, settings : *ContextSettings) -> *sfRenderWindow;
        //fn sfRenderWindow_createFromHandle(handle : sfWindowHandle, settings : *sfContextSettings) -> *sfRenderWindow;
        fn sfRenderWindow_destroy(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_close(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_isOpen(renderWindow : *sfRenderWindow) -> sfBool;
        fn sfRenderWindow_getSettings(renderWindow : *sfRenderWindow) -> ContextSettings;
        fn sfRenderWindow_pollEvent(renderWindow : *sfRenderWindow, event : *sfEvent) -> sfBool;
        fn sfRenderWindow_waitEvent(renderWindow : *sfRenderWindow, event : *sfEvent) -> sfBool;
        fn sfRenderWindow_getPosition(renderWindow : *sfRenderWindow) -> Vector2i;
        fn sfRenderWindow_setPosition(renderWindow : *sfRenderWindow, position : Vector2i) -> ();
        fn sfRenderWindow_getSize(renderWindow : *sfRenderWindow) -> Vector2u;
        fn sfRenderWindow_setSize(renderWindow : *sfRenderWindow, size : Vector2u) -> ();
        fn sfRenderWindow_setTitle(renderWindow : *sfRenderWindow, title : *c_char) -> ();
        fn sfRenderWindow_setUnicodeTitle(renderWindow : *sfRenderWindow, title : *u32) -> ();
        fn sfRenderWindow_setIcon(renderWindow : *sfRenderWindow, width : c_uint, height : c_uint, pixels : *u8) -> ();
        fn sfRenderWindow_setVisible(renderWindow : *sfRenderWindow, visible : sfBool) -> ();
        fn sfRenderWindow_setMouseCursorVisible(renderWindow : *sfRenderWindow, show : sfBool) -> ();
        fn sfRenderWindow_setVerticalSyncEnabled(renderWindow : *sfRenderWindow, enabled : sfBool) -> ();
        fn sfRenderWindow_setKeyRepeatEnabled(renderWindow : *sfRenderWindow, enabled : sfBool) -> ();
        fn sfRenderWindow_setActive(renderWindow : *sfRenderWindow, active : sfBool) -> sfBool;
        fn sfRenderWindow_display(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_setFramerateLimit(renderWindow : *sfRenderWindow, limit : c_uint) -> ();
        fn sfRenderWindow_setJoystickThreshold(renderWindow : *sfRenderWindow, treshold : c_float) -> ();
        // fn sfRenderWindow_getSystemHandle(renderWindow : *sfRenderWindow) -> sfWindowHandle;
        fn sfRenderWindow_clear(renderWindow : *sfRenderWindow, color : Color) -> ();
        fn sfRenderWindow_setView(renderWindow : *sfRenderWindow, view : *sfView) -> ();
        fn sfRenderWindow_getView(renderWindow : *sfRenderWindow) -> *sfView;
        fn sfRenderWindow_getDefaultView(renderWindow : *sfRenderWindow) -> *sfView;
        fn sfRenderWindow_getViewport(renderWindow : *sfRenderWindow, view : *sfView) -> IntRect;
        fn sfRenderWindow_mapPixelToCoords(renderWindow : *sfRenderWindow, point : Vector2i, view : *sfView) -> Vector2f;
        fn sfRenderWindow_mapCoordsToPixel(renderWindow : *sfRenderWindow, point : Vector2f, view : *sfView) -> Vector2i;
        fn sfRenderWindow_drawSprite(renderWindow : *sfRenderWindow, object : *sfSprite, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderWindow_drawText(renderWindow : *sfRenderWindow, object : *sfText, states : *render_states::csfml::sfRenderStates) -> ();
        // fn sfRenderWindow_drawShape(renderWindow : *sfRenderWindow, object : *sfShape, states : *sfRenderStates) -> ();
        fn sfRenderWindow_drawCircleShape(renderWindow : *sfRenderWindow, object : *sfCircleShape, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderWindow_drawConvexShape(renderWindow : *sfRenderWindow, object : *sfConvexShape, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderWindow_drawRectangleShape(renderWindow : *sfRenderWindow, object : *sfRectangleShape, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderWindow_drawVertexArray(renderWindow : *sfRenderWindow, object : *sfVertexArray, states : *render_states::csfml::sfRenderStates) -> ();
        // fn sfRenderWindow_drawPrimitives(renderWindow : *sfRenderWindow, vertices : *sfVertex, vertexCount : c_uint, ttype : sfPrimitiveType, states : *sfRenderStates) -> ();  
        fn sfRenderWindow_pushGLStates(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_popGLStates(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_resetGLStates(renderWindow : *sfRenderWindow) -> ();
        fn sfRenderWindow_capture(renderWindow : *sfRenderWindow) -> *sfImage;
        fn sfMouse_getPositionRenderWindow(relativeTo : *sfRenderWindow) -> Vector2i;
        fn sfMouse_setPositionRenderWindow(position : Vector2i, relativeTo : *sfRenderWindow) -> ();
    }    
}

/// Enumeration of window creation styles
pub enum WindowStyle {
    pub sfNone = 0,
    pub sfTitlebar = 1,
    pub sfResize = 2,
    pub sfClose = 4,
    pub sfFullscreen = 8,
    pub sfDefaultStyle = 7
}

#[doc(hidden)]
pub struct RenderWindow {
    priv renderWindow : *csfml::sfRenderWindow,
    priv event : csfml::sfEvent,
    priv titleLength : uint,
    priv currentView : @mut View,
    priv defaultView : @mut View
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
    * Return a new RenderWindow object
    */
    pub fn new(mode : VideoMode, title : ~str, style : WindowStyle, settings : &ContextSettings) -> Option<RenderWindow> {
        let mut sfRenderWin: *csfml::sfRenderWindow = ptr::null();
        do str::as_c_str(title) |title_buf| {
            unsafe { 
                sfRenderWin = csfml::sfRenderWindow_create(VideoMode::unwrap(mode), title_buf, style as u32, settings); 
            }
        };
        let sfEv = csfml::sfEvent {
            typeEvent : 0, 
            p1 : 0, 
            p2 : 0, 
            p3 : 0 as c_float, 
            p4 : 0, 
            p5 : 0
        };
        if ptr::is_null(sfRenderWin) {
            None
        }
        else {
            let defView = unsafe { csfml::sfRenderWindow_getDefaultView(sfRenderWin) };
            if ptr::is_null(defView) {
                None
            }
            else {
                Some (RenderWindow {
                    renderWindow : sfRenderWin, 
                    event : sfEv, 
                    titleLength : title.len(),
                    currentView : @mut View::wrap(defView),
                    defaultView : @mut View::wrap(defView)
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
    * Return a new RenderWindow object
    */
    pub fn new_with_unicode(mode : VideoMode, title : ~[u32], style : WindowStyle, settings : &ContextSettings) -> Option<RenderWindow> {
        let sfRenderWin: *csfml::sfRenderWindow;
        unsafe { 
            sfRenderWin = csfml::sfRenderWindow_createUnicode(VideoMode::unwrap(mode), vec::raw::to_ptr(title), style as u32, settings); 
        }
        let sfEv = csfml::sfEvent {
            typeEvent : 0, 
            p1 : 0, 
            p2 : 0, 
            p3 : 0 as c_float, 
            p4 : 0, 
            p5 : 0};
        if ptr::is_null(sfRenderWin) {
            None
        }
        else {
            let defView = unsafe { csfml::sfRenderWindow_getDefaultView(sfRenderWin) };
            if ptr::is_null(defView) {
                None
            }
            else {
                Some (RenderWindow {
                    renderWindow : sfRenderWin, 
                    event : sfEv, 
                    titleLength : title.len(),
                    currentView : @mut View::wrap(defView),
                    defaultView : @mut View::wrap(defView)
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
            self.titleLength = title.len();
            csfml::sfRenderWindow_setUnicodeTitle(self.renderWindow, vec::raw::to_ptr(title))
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
            csfml::sfRenderWindow_setIcon(self.renderWindow, width as c_uint, height as c_uint, vec::raw::to_ptr(pixels))
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
            match csfml::sfRenderWindow_pollEvent(self.renderWindow, &self.event) {
                0       => false,
                _       => true
            }
        };
        if haveEvent == false {
            event::NoEvent
        }
        else {
            self.get_wrapped_event()
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
            match csfml::sfRenderWindow_waitEvent(self.renderWindow, &self.event) {
                0       => false,
                _       => true
            }
        };
        if haveEvent == false {
            event::NoEvent
        }
        else {
            self.get_wrapped_event()
        }
    }
    
    pub fn get_wrapped_event(&self) ->event::Event {
        match self.event.typeEvent as c_uint {
            0   => event::Closed,
            1   => event::Resized{ width : self.event.p1 as int, height : self.event.p2 as int },
            2   => event::LostFocus,
            3   => event::GainedFocus,
            4   => {
                event::TextEntered { 
                    code : self.event.p1 as char 
                }
            },
            5   => {
                let al : bool = match self.event.p2 {
                    0 => false,
                    _ => true
                };
                let ct : bool = match self.event.p3 as int{
                    0 => false,
                    _ => true
                };
                let sh : bool = match self.event.p4  {
                    0 => false,
                    _ => true
                };
                let sy : bool = match self.event.p5 {
                    0 => false,
                    _ => true
                };
                let k : keyboard::Key = unsafe { cast::transmute(self.event.p1 as int) };
                event::KeyPressed{ 
                    code : k, 
                    alt : al, 
                    ctrl : ct, 
                    shift :sh, 
                    system : sy 
                }
            },
            6   => {
                let al : bool = match self.event.p2 {
                    0 => false,
                    _ => true
                };
                let ct : bool = match self.event.p3 as int{
                    0 => false,
                    _ => true
                };
                let sh : bool = match self.event.p4  {
                    0 => false,
                    _ => true
                };
                let sy : bool = match self.event.p5 {
                    0 => false,
                    _ => true
                };
                let k : keyboard::Key = unsafe { cast::transmute(self.event.p1 as int) };
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
                    delta : unsafe { cast::transmute::<c_uint, c_int>(self.event.p1) }  as int,
                    x :     unsafe { cast::transmute::<c_uint, c_int>(self.event.p2) }  as int,
                    y :     unsafe { cast::transmute::<c_float, c_int>(self.event.p3) } as int
                }
            },
            8   => {
                let button : mouse::MouseButton = unsafe {cast::transmute(self.event.p1 as int)};
                event::MouseButtonPressed{
                    button : button,
                    x :      unsafe { cast::transmute::<c_uint, c_int>(self.event.p2) as int },
                    y :      unsafe { cast::transmute::<c_float, c_int>(self.event.p3) as int }
                }
            },
            9   => {
                let button : mouse::MouseButton = unsafe { cast::transmute(self.event.p1 as int) };
                event::MouseButtonReleased{
                    button : button,
                    x :      unsafe { cast::transmute::<c_uint, c_int>(self.event.p2) as int },
                    y :      unsafe { cast::transmute::<c_float, c_int>(self.event.p3) as int }
                }
            },
            10  => { 
                event::MouseMoved {
                    x : unsafe { cast::transmute::<c_uint, c_int>(self.event.p1) } as int,
                    y : unsafe { cast::transmute::<c_uint, c_int>(self.event.p2) } as int
                }
            },
            11  => event::MouseEntered,
            12  => event::MouseLeft,
            13  => {
                event::JoystickButtonPressed {
                    joystickid : self.event.p1 as int, 
                    button : self.event.p2 as int
                }
            },
            14  => { 
                event::JoystickButtonReleased{
                    joystickid : self.event.p1 as int, 
                    button : self.event.p2 as int
                }
            },
            15  => {
                let ax : joystick::Axis = unsafe { cast::transmute(self.event.p2 as int) };
                event::JoystickMoved{
                    joystickid : self.event.p1 as uint, 
                    axis : ax, 
                    position : self.event.p3 as float
                }
            },
            16  => { 
                event::JoystickConnected{
                    joystickid : self.event.p1 as uint
                }
            },
            17  => { 
                event::JoystickDisconnected{
                    joystickid : self.event.p1 as uint
                }
            },
            _ => event::NoEvent
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
            csfml::sfRenderWindow_close(self.renderWindow);
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
        let tmp : sfBool;
        unsafe {
            tmp = csfml::sfRenderWindow_isOpen(self.renderWindow);
        }
        match tmp {
            0 => false,
            _ => true
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
            csfml::sfRenderWindow_display(self.renderWindow)
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
            csfml::sfRenderWindow_setFramerateLimit(self.renderWindow, limit as c_uint)
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
            csfml::sfRenderWindow_getSettings(self.renderWindow)
        }
    }

    /**
    * Change the title of a window
    *
    * # Arguments
    * * title - New title
    */
    pub fn set_title(&mut self, title : ~str) -> () {
        do str::as_c_str(title) |title_buf| {
            unsafe {
                self.titleLength = title.len();
                csfml::sfRenderWindow_setTitle(self.renderWindow, title_buf);
            }
        }
    }
    
    /**
    * Show or hide a window
    *
    * # Arguments
    * * visible - true to show the window, false to hide it
    */
    pub fn set_visible(&mut self, visible : bool) -> () {
        let tmp : sfBool =
            match visible {
                true    => 1,
                false   => 0
            };
        unsafe {
            csfml::sfRenderWindow_setVisible(self.renderWindow, tmp);
        }
    }

    /**
    * Show or hide the mouse cursor
    *
    * # Arguments
    * * visible - true to show, false to hide
    */
    pub fn set_mouse_cursor_visible(&mut self, visible : bool) -> () {
        let tmp : sfBool =
            match visible {
                true    => 1,
                false   => 0
            };
        unsafe {
            csfml::sfRenderWindow_setMouseCursorVisible(self.renderWindow, tmp);
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
        let tmp : sfBool =
            match enabled {
                true    => 1,
                false   => 0
            };
        unsafe {
            csfml::sfRenderWindow_setVerticalSyncEnabled(self.renderWindow, tmp);
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
        let tmp : sfBool =
            match enabled {
                true    => 1,
                false   => 0
            };
        unsafe {
            csfml::sfRenderWindow_setKeyRepeatEnabled(self.renderWindow, tmp);
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
        let tmp : sfBool = match enabled {
            true    => 1,
            false   => 0
        };
        let res : sfBool = unsafe {
            csfml::sfRenderWindow_setActive(self.renderWindow, tmp)
        };
        match res {
            1   => true,
            _   => false
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
    pub fn set_joystick_threshold(&mut self, threshold : float) -> () {
        unsafe {
            csfml::sfRenderWindow_setJoystickThreshold(self.renderWindow, threshold as c_float)
        }
    }

    /**
    *  Get the position of a window
    *
    * Return the position in pixels
    */
    pub fn get_position(&self) -> Vector2i {
        unsafe {
            csfml::sfRenderWindow_getPosition(self.renderWindow)
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
            csfml::sfRenderWindow_setPosition(self.renderWindow, *position)
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
            csfml::sfRenderWindow_getSize(self.renderWindow)
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
            csfml::sfRenderWindow_setSize(self.renderWindow, *size)
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
            csfml::sfRenderWindow_pushGLStates(self.renderWindow)
        }
    }

    /**
    * Restore the previously saved OpenGL render states and matrices
    */
    pub fn pop_GL_states(&mut self) -> () {
        unsafe {
            csfml::sfRenderWindow_popGLStates(self.renderWindow)
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
            csfml::sfRenderWindow_resetGLStates(self.renderWindow)
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
            csfml::sfMouse_getPositionRenderWindow(self.renderWindow)
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
            csfml::sfMouse_setPositionRenderWindow(*position, self.renderWindow)
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

    /// Draw a Text
    pub fn draw_text(&self, text : &Text) -> () {
        unsafe {
            csfml::sfRenderWindow_drawText(self.renderWindow, text.unwrap(), ptr::null())
        }
    }

    /// Draw a sprite
    pub fn draw_sprite(&self, sprite : &Sprite) -> () {
        unsafe {
            csfml::sfRenderWindow_drawSprite(self.renderWindow, sprite.unwrap(), ptr::null())
        }
    }

    /// Draw a CircleShape
    pub fn draw_circle_shape(&self, circleShape : &CircleShape) -> () {
        unsafe {
            csfml::sfRenderWindow_drawCircleShape(self.renderWindow, circleShape.unwrap(), ptr::null())
        }
    }

    /// Draw a RectangleShape
    pub fn draw_rectangle_shape(&self, rectangleShape : &RectangleShape) -> () {
        unsafe {
            csfml::sfRenderWindow_drawRectangleShape(self.renderWindow, rectangleShape.unwrap(), ptr::null())
        }
    }

    /// Draw a ConvexShape
    pub fn draw_convex_shape(&self, convexShape : &ConvexShape) -> () {
        unsafe {
            csfml::sfRenderWindow_drawConvexShape(self.renderWindow, convexShape.unwrap(), ptr::null())
        }
    }

    /// Draw a VertexArray
    pub fn draw_vertex_array(&self, vertexArray : &VertexArray) -> () {
        unsafe {
            csfml::sfRenderWindow_drawVertexArray(self.renderWindow, vertexArray.unwrap(), ptr::null())
        }
    }

    /// Clear window with the given color
    pub fn clear(&mut self, color : &Color) -> () {
        unsafe {
            csfml::sfRenderWindow_clear(self.renderWindow, *color)
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
        let img = unsafe { csfml::sfRenderWindow_capture(self.renderWindow) };
        if ptr::is_null(img) {
            None
        }
        else {
            Some(Image::wrap(img))
        }
    }
    
    /**
    * Change the current active view of a render window
    *
    * # Arguments
    * * view - The new view
    */
    pub fn set_view(&mut self, view : @mut View) -> () {
        self.currentView = view;
        unsafe {
            csfml::sfRenderWindow_setView(self.renderWindow, view.unwrap())
        }
    }
    
    /**
    * Get the current active view of a render window
    *
    * Return the current active view
    */
    pub fn get_view(&self) -> @mut View {
        self.currentView
    }
    
    /**
    * Get the default view of a render window
    *
    * Return the default view of the render window
    */
    pub fn get_default_view(&self) -> @mut View {
        self.defaultView
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
            csfml::sfRenderWindow_mapPixelToCoords(self.renderWindow, *point, view.unwrap())
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
        let view = unsafe {csfml::sfRenderWindow_getView(self.renderWindow)};
        unsafe {
            csfml::sfRenderWindow_mapPixelToCoords(self.renderWindow, *point, view)
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
            csfml::sfRenderWindow_mapCoordsToPixel(self.renderWindow, *point, view.unwrap())
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
        let currView = unsafe { csfml::sfRenderWindow_getView(self.renderWindow) };
        unsafe {
            csfml::sfRenderWindow_mapCoordsToPixel(self.renderWindow, *point, currView)
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
            csfml::sfRenderWindow_getViewport(self.renderWindow, view.unwrap())
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfRenderWindow {
        self.renderWindow
    }
}

#[unsafe_destructor]
impl Drop for RenderWindow {
    /**
    *   Destructor for class RenderWindow. Destroy all the ressource.
    */
    fn drop(&self) {
        unsafe {
            csfml::sfRenderWindow_destroy(self.renderWindow);
        }
    }
}
