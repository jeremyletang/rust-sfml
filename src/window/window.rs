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
 * Window manipulation
 *
 * Provides OpenGL-based windows, and abstractions for events and input handling.
 */

use std::libc::{c_uint, c_float};
use std::str;
use std::vec;
use std::ptr;
use std::cast;

use window::context_settings::ContextSettings;
use window::video_mode::*;
use rsfml::sfTypes::{sfBool};
use window::event;
use window::keyboard;
use system::vector2;
use window::joystick;
use window::mouse;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_void, c_uint, c_char, c_float};    

    use rsfml::sfTypes::{sfBool};
    use window::context_settings::ContextSettings;
    use window::video_mode::*;    
    use system::vector2;

    pub struct sfWindow {
        This : *c_void
    }
    
    pub struct sfEvent {
        typeEvent : c_uint,
        p1 : c_uint,
        p2 : c_uint,
        p3 : c_float,
        p4 : c_uint,
        p5 : c_uint
    }

    pub enum sfEventType {
        sfEvtClosed, // 0
        sfEvtResized, // 1
        sfEvtLostFocus, // 2
        sfEvtGainedFocus, // 3
        sfEvtTextEntered, // 4
        sfEvtKeyPressed, // 5
        sfEvtKeyReleased, // 6
        sfEvtMouseWheelMoved, // 7
        sfEvtMouseButtonPressed, // 8
        sfEvtMouseButtonReleased, // 9
        sfEvtMouseMoved, // 10
        sfEvtMouseEntered, // 11
        sfEvtMouseLeft, // 12
        sfEvtJoystickButtonPressed, // 13
        sfEvtJoystickButtonReleased, // 14
        sfEvtJoystickMoved, // 15
        sfEvtJoystickConnected, // 16
        sfEvtJoystickDisconnected // 17
    }

    pub extern "C" {
        fn sfWindow_create(mode : csfml::sfVideoMode, title : *c_char, style : c_uint, settings : *ContextSettings) -> *sfWindow;
        fn sfWindow_createUnicode(mode : csfml::sfVideoMode, title : *u32, style : c_uint, setting : *ContextSettings) -> *sfWindow;
        //fn sfWindow_createFromHandle(handle : sfWindowHandle, settings : *sfContextSettings) -> *sfWindow;
        fn sfWindow_close(window : *sfWindow) -> ();
        fn sfWindow_destroy(window : *sfWindow) -> ();
        fn sfWindow_isOpen(window : *sfWindow) -> sfBool;
        fn sfWindow_getSettings(window : *sfWindow) -> ContextSettings;
        fn sfWindow_setTitle(window : *sfWindow, title : *c_char) -> ();
        fn sfWindow_setUnicodeTitle(window : *sfWindow, title : *u32) -> ();
        fn sfWindow_setIcon(window : *sfWindow, width : c_uint, height : c_uint, pixel : *u8) -> (); 
        fn sfWindow_setVisible(window : *sfWindow, visible : sfBool) -> ();
        fn sfWindow_setMouseCursorVisible(window : *sfWindow, visible : sfBool) -> ();
        fn sfWindow_setVerticalSyncEnabled(window : *sfWindow, enabled : sfBool) -> ();
        fn sfWindow_setKeyRepeatEnabled(window : *sfWindow, enabled : sfBool) -> ();
        fn sfWindow_setActive(window : *sfWindow, active : sfBool) -> sfBool;
        fn sfWindow_display(window : *sfWindow) -> ();
        fn sfWindow_setFramerateLimit(window : *sfWindow, limit : c_uint) -> ();
        fn sfWindow_setJoystickThreshold(window : *sfWindow, threshold : c_float) -> ();
        fn sfWindow_getPosition(window : *sfWindow) -> vector2::Vector2i;
        fn sfWindow_setPosition(window : *sfWindow, position : vector2::Vector2i) -> ();
        fn sfWindow_getSize(window : *sfWindow) -> vector2::Vector2u;
        fn sfWindow_setSize(window : *sfWindow, size : vector2::Vector2u) -> ();
        fn sfWindow_pollEvent(window : *sfWindow, event : *sfEvent) -> sfBool;
        fn sfWindow_waitEvent(window : *sfWindow, event : *sfEvent) -> sfBool;
        //fn sfWindow_getSystemHandle(window : *sfWindow) -> sfWindowHandle;
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
pub struct Window {
    priv window : *csfml::sfWindow,
    priv event : csfml::sfEvent,
    priv titleLength : uint
}

impl Window { 
    priv fn get_wrapped_event(&self) ->event::Event {
            match self.event.typeEvent as c_uint {
            0   => event::Closed,
            1   => event::Resized{width : self.event.p1 as int, height : self.event.p2 as int},
            2   => event::LostFocus,
            3   => event::GainedFocus,
            4   => event::TextEntered{code : self.event.p1 as char},
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
                let k : keyboard::Key = unsafe {cast::transmute(self.event.p1 as int)};
                event::KeyPressed{code : k, alt : al, ctrl : ct, shift :sh, system : sy}
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
                let k : keyboard::Key = unsafe {cast::transmute(self.event.p1 as int)};
                event::KeyReleased{code : k, alt : al, ctrl : ct, shift :sh, system : sy}
            },
            7   => event::MouseWheelMoved{delta : self.event.p1 as int, x : self.event.p2 as int, y : self.event.p3 as int},
            8   => {
                let button : mouse::MouseButton = unsafe {cast::transmute(self.event.p1 as int)};
                event::MouseButtonPressed{button : button, x : self.event.p2 as int, y : self.event.p3 as int}
            },
            9   => {
                let button : mouse::MouseButton = unsafe {cast::transmute(self.event.p1 as int)};
                event::MouseButtonReleased{button : button, x : self.event.p2 as int, y : self.event.p3 as int}
            },
            10  => event::MouseMoved{x : self.event.p1 as int, y : self.event.p2 as int},
            11  => event::MouseEntered,
            12  => event::MouseLeft,
            13  => event::JoystickButtonPressed{joystickid : self.event.p1 as int, button : self.event.p2 as int},
            14  => event::JoystickButtonReleased{joystickid : self.event.p1 as int, button : self.event.p2 as int},
            15  => {
                let ax : joystick::Axis = unsafe {cast::transmute(self.event.p2 as int)};
                event::JoystickMoved{joystickid : self.event.p1 as uint, axis : ax, position : self.event.p3 as float}
            },
            16  => event::JoystickConnected{joystickid : self.event.p1 as uint},
            17  => event::JoystickDisconnected{joystickid : self.event.p1 as uint},
            _ => event::NoEvent
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
    pub fn poll_event(&self) -> event::Event {
        let haveEvent : bool =  unsafe {
            match csfml::sfWindow_pollEvent(self.window, &self.event) {
                0       => false,
                _       => true
            }
        };
        if haveEvent == false {
            return event::NoEvent;
        }
        self.get_wrapped_event()
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
    pub fn wait_event(&self) -> event::Event {
        let haveEvent : bool =  unsafe {
            match csfml::sfWindow_waitEvent(self.window, &self.event) {
                0       => false,
                _       => true
            }
        };
        if haveEvent == false {
            return event::NoEvent;
        }
        self.get_wrapped_event()
    }
    
    /**
    * Construct a new window
    *
    * This function creates the window with the size and pixel
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
    * * mode - Video mode to use (defines the width, height and depth of the rendering area of the window)
    * * title - Title of the window
    * * style - Window style
    * * settings - Additional settings for the underlying OpenGL context
    *
    * Return a new Window object
    */
    pub fn new(mode : VideoMode, title : ~str, style : WindowStyle, settings : &ContextSettings) -> Option<Window> {
        let mut sfWin: *csfml::sfWindow = ptr::null();
        do str::as_c_str(title) |title_buf| {
            unsafe { sfWin = csfml::sfWindow_create(VideoMode::unwrap(mode), title_buf, style as u32, settings); }
        };
        let sfEv : csfml::sfEvent = csfml::sfEvent {typeEvent : 0, p1 : 0, p2 : 0, p3 : 0 as c_float, p4 : 0, p5 : 0};//{0, 0, 0, 0 as float, 0, 0};
        if sfWin == ptr::null() {
            None
        }
        else {
        Some (Window { window : sfWin, event : sfEv, titleLength : title.len()})
        }
    }

    /**
    * Construct a new window (with a UTF-32 title)
    *
    * This function creates the window with the size and pixel
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
    * * mode - Video mode to use (defines the width, height and depth of the rendering area of the window)
    * * title - Title of the window (UTF-32)
    * * style - Window style
    * * settings - Additional settings for the underlying OpenGL context
    *
    * Return a new Window object
    */
    pub fn new_with_unicode(mode : VideoMode, title : ~[u32], style : WindowStyle, settings : &ContextSettings) -> Option<Window> {
        let sfWin: *csfml::sfWindow;
        unsafe { sfWin = csfml::sfWindow_createUnicode(VideoMode::unwrap(mode), vec::raw::to_ptr(title), style as u32, settings); }
        let sfEv : csfml::sfEvent = csfml::sfEvent {typeEvent : 0, p1 : 0, p2 : 0, p3 : 0 as c_float, p4 : 0, p5 : 0};//{0, 0, 0, 0 as float, 0, 0};
        if sfWin == ptr::null() {
            None
        }
        else {
        Some (Window { window : sfWin, event : sfEv, titleLength : title.len()})
        }
    }

    /**
    * Change the title of a window (with a UTF-32 string)
    *
    * # Arguments
    * * title - New title
    */
    pub fn set_unicode_title(&self, title : ~[u32]) -> () {
        unsafe {
            csfml::sfWindow_setUnicodeTitle(self.window, vec::raw::to_ptr(title))
        }
    }
    /**
    * Change a window's icon
    * pixels must be an array of width x height pixels in 32-bits RGBA format.
    *
    * # Arguments
    * * width - Icon's width, in pixels
    * * height - Icon's height, in pixels
    * * pixels - Vector of pixels
    */
    pub fn set_icon(&self, width : uint, height : uint, pixels : ~[u8]) -> () {
        unsafe {
            csfml::sfWindow_setIcon(self.window, width as c_uint, height as c_uint, vec::raw::to_ptr(pixels))
        }
    }
    
    /**
    * Close a window and destroy all the attached resources
    *
    * After calling this method, the Window object remains
    * valid.
    * All other functions such as poll_event or display
    * will still work (i.e. you don't have to test is_open
    * every time), and will have no effect on closed windows.
    */
    pub fn close(&self) -> () {
        unsafe {
            csfml::sfWindow_close(self.window);
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
            tmp = csfml::sfWindow_isOpen(self.window);
        }
        match tmp {
            0 => false,
            _ => true
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
        unsafe {csfml::sfWindow_getSettings(self.window)}
    }

    /**
    * Change the title of a window
    *
    * # Arguments
    * * title - New title
    */
    pub fn set_title(&self, title : ~str) -> () {
        do str::as_c_str(title) |title_buf| {
            unsafe {
                csfml::sfWindow_setTitle(self.window, title_buf);
            }
        }
    }

    /**
    * Show or hide a window
    *
    * # Arguments
    * * visible - true to show the window, false to hide it
    */
    pub fn set_visible(&self, visible : bool) -> () {
        let tmp : sfBool = 
            match visible {
                true    => 1,
                _       => 0
            };
        unsafe {
            csfml::sfWindow_setVisible(self.window, tmp);
        }
    }
    
    /**
    * Show or hide the mouse cursor
    *
    * # Arguments
    * * visible - true to show, false to hide
    */
    pub fn set_mouse_cursor_visible(&self, visible : bool) -> () {
        let tmp : sfBool = 
            match visible {
                true    => 1,
                _       => 0
            };
        unsafe {
            csfml::sfWindow_setMouseCursorVisible(self.window, tmp);
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
    pub fn set_vertical_sync_enabled(&self, enabled : bool) -> () {
        let tmp : sfBool = 
            match enabled {
                true    => 1,
                _       => 0
            };
        unsafe {
            csfml::sfWindow_setVerticalSyncEnabled(self.window, tmp);
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
    pub fn set_key_repeat_enabled(&self, enabled : bool) -> () {
        let tmp : sfBool = 
            match enabled {
                true    => 1,
                _       => 0
            };
        unsafe {
            csfml::sfWindow_setKeyRepeatEnabled(self.window, tmp);
        }
    }
    
    /**
    * Activate or deactivate a window as the current target for OpenGL rendering
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
    pub fn set_active(&self, enabled : bool) -> bool {
        let tmp : sfBool = 
            match enabled {
                true    => 1,
                _       => 0
            };
        let res : sfBool = unsafe {
            csfml::sfWindow_setActive(self.window, tmp)
        };
        match res {
            1   => true,
            _   => false
        }
    }
    
    /**
    * Display on screen what has been rendered to the window so far
    *
    * This function is typically called after all OpenGL rendering
    * has been done for the current frame, in order to show
    * it on screen.
    */
    pub fn display(&self) -> () {
        unsafe {
            csfml::sfWindow_display(self.window)
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
    pub fn set_framerate_limit(&self, limit : uint) -> () {
        unsafe {
            csfml::sfWindow_setFramerateLimit(self.window, limit as c_uint)
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
    pub fn set_joystick_threshold(&self, threshold : float) -> () {
        unsafe {
            csfml::sfWindow_setJoystickThreshold(self.window, threshold as c_float)
        }
    }
    
    /**
    *  Get the position of a window
    *
    * Return the position in pixels
    */
    pub fn get_position(&self) -> vector2::Vector2i {
        unsafe {
            csfml::sfWindow_getPosition(self.window)
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
    pub fn set_position(&self, position : &vector2::Vector2i) -> () {
        unsafe {
            csfml::sfWindow_setPosition(self.window, *position)
        }
    }
    
    /**
    * Get the size of the rendering region of a window
    *
    * The size doesn't include the titlebar and borders of the window.
    *
    * Return the size in pixels
    */
    pub fn get_size(&self) -> vector2::Vector2u {
        unsafe {
            csfml::sfWindow_getSize(self.window)
        }
    }
 
    /**
    * Change the size of the rendering region of a window
    *
    * # Arguments
    * * size - New size, in pixels
    */
    pub fn set_size(&self, size : &vector2::Vector2u) -> () {
        unsafe {
            csfml::sfWindow_setSize(self.window, *size)
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfWindow {
        self.window
    }
}

impl Drop for Window {
    /**
    *   Destructor for class Window. Destroy all the ressource.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfWindow_destroy(self.window);
        }
    }
}
