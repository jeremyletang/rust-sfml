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

use std::libc::{c_uint, c_float, c_int};
use std::{vec, ptr, cast};

use traits::wrappable::Wrappable;
use window::{event, keyboard, joystick, mouse};
use window::video_mode::*;
use window::context_settings::ContextSettings;
use system::vector2::{Vector2i, Vector2u};
use sfml_types::*;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_void, c_uint, c_float, c_char};    

    use sfml_types::SfBool;
    use window::context_settings::ContextSettings;
    use window::video_mode::*;    
    use system::vector2::{Vector2i, Vector2u};

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

    extern "C" {
        pub fn sfWindow_create(mode : ffi::sfVideoMode, title : *c_char, style : c_uint, settings : *ContextSettings) -> *sfWindow;
        pub fn sfWindow_createUnicode(mode : ffi::sfVideoMode, title : *u32, style : c_uint, setting : *ContextSettings) -> *sfWindow;
        //fn sfWindow_createFromHandle(handle : sfWindowHandle, settings : *sfContextSettings) -> *sfWindow;
        pub fn sfWindow_close(window : *sfWindow) -> ();
        pub fn sfWindow_destroy(window : *sfWindow) -> ();
        pub fn sfWindow_isOpen(window : *sfWindow) -> SfBool;
        pub fn sfWindow_getSettings(window : *sfWindow) -> ContextSettings;
        pub fn sfWindow_setTitle(window : *sfWindow, title : *c_char) -> ();
        pub fn sfWindow_setUnicodeTitle(window : *sfWindow, title : *u32) -> ();
        pub fn sfWindow_setIcon(window : *sfWindow, width : c_uint, height : c_uint, pixel : *u8) -> (); 
        pub fn sfWindow_setVisible(window : *sfWindow, visible : SfBool) -> ();
        pub fn sfWindow_setMouseCursorVisible(window : *sfWindow, visible : SfBool) -> ();
        pub fn sfWindow_setVerticalSyncEnabled(window : *sfWindow, enabled : SfBool) -> ();
        pub fn sfWindow_setKeyRepeatEnabled(window : *sfWindow, enabled : SfBool) -> ();
        pub fn sfWindow_setActive(window : *sfWindow, active : SfBool) -> SfBool;
        pub fn sfWindow_display(window : *sfWindow) -> ();
        pub fn sfWindow_setFramerateLimit(window : *sfWindow, limit : c_uint) -> ();
        pub fn sfWindow_setJoystickThreshold(window : *sfWindow, threshold : c_float) -> ();
        pub fn sfWindow_getPosition(window : *sfWindow) -> Vector2i;
        pub fn sfWindow_setPosition(window : *sfWindow, position : Vector2i) -> ();
        pub fn sfWindow_getSize(window : *sfWindow) -> Vector2u;
        pub fn sfWindow_setSize(window : *sfWindow, size : Vector2u) -> ();
        pub fn sfWindow_pollEvent(window : *sfWindow, event : *sfEvent) -> SfBool;
        pub fn sfWindow_waitEvent(window : *sfWindow, event : *sfEvent) -> SfBool;
        //fn sfWindow_getSystemHandle(window : *sfWindow) -> sfWindowHandle;
    }
}

/// Enumeration of window creation styles
pub enum WindowStyle {
    sfNone = 0,
    sfTitlebar = 1,
    sfResize = 2,
    sfClose = 4,
    sfFullscreen = 8,
    sfDefaultStyle = 7
}

#[doc(hidden)]
pub struct Window {
    priv window : *ffi::sfWindow,
    priv event : ffi::sfEvent,
    priv title_length : uint
}

impl Window { 
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn new(mode : VideoMode, title : ~str, style : WindowStyle, settings : &ContextSettings) -> Option<Window> {
        let sf_win: *ffi::sfWindow =
        unsafe {
            let c_title = title.to_c_str().unwrap();
            ffi::sfWindow_create(mode.unwrap(), c_title, style as u32, settings)
        };
        let sf_ev = ffi::sfEvent {
            typeEvent : 0,
            p1 : 0,
            p2 : 0,
            p3 : 0 as c_float,
            p4 : 0,
            p5 : 0
        };
        if ptr::is_null(sf_win) {
            None
        }
        else {
            Some (Window {
                window : sf_win, 
                event : sf_ev, 
                title_length : title.len()
            })
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_with_unicode(mode : VideoMode, title : ~[u32], style : WindowStyle, settings : &ContextSettings) -> Option<Window> {
        let sfWin = unsafe { ffi::sfWindow_createUnicode(mode.unwrap(), vec::raw::to_ptr(title), style as u32, settings) };
        let sfEv = ffi::sfEvent {
            typeEvent : 0,
            p1 : 0, 
            p2 : 0, 
            p3 : 0 as c_float, 
            p4 : 0, 
            p5 : 0
        };
        if ptr::is_null(sfWin) {
            None
        }
        else {
            Some (Window {
                window : sfWin,
                event : sfEv,
                title_length : title.len()
            })
        }
    }

    fn get_wrapped_event(&self) -> event::Event {
            match self.event.typeEvent as c_uint {
                0   => event::Closed,
                1   => {
                    event::Resized{
                        width : self.event.p1 as int,
                        height : self.event.p2 as int
                    }
                },
                2   => event::LostFocus,
                3   => event::GainedFocus,
                4   => {
                    event::TextEntered{
                        code : (self.event.p1 as u8) as char
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
                    event::KeyReleased{
                        code : k,
                        alt : al, 
                        ctrl : ct,
                        shift :sh,
                        system : sy
                    }
                },
                7   =>  event::MouseWheelMoved{
                    delta : unsafe { cast::transmute::<c_uint, c_int>(self.event.p1) }  as int,
                    x :     unsafe { cast::transmute::<c_uint, c_int>(self.event.p2) }  as int,
                    y :     unsafe { cast::transmute::<c_float, c_int>(self.event.p3) } as int
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
                    let button : mouse::MouseButton = unsafe {cast::transmute(self.event.p1 as int)};
                    event::MouseButtonReleased{
                        button : button,
                        x :      unsafe { cast::transmute::<c_uint, c_int>(self.event.p2) as int },
                        y :      unsafe { cast::transmute::<c_float, c_int>(self.event.p3) as int }
                    }
                },
                10  => event::MouseMoved{
                    x : unsafe { cast::transmute::<c_uint, c_int>(self.event.p1) } as int,
                    y : unsafe { cast::transmute::<c_uint, c_int>(self.event.p2) } as int
                },
                11  => event::MouseEntered,
                12  => event::MouseLeft,
                13  => {
                    event::JoystickButtonPressed{
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
                    let ax : joystick::Axis = unsafe {cast::transmute(self.event.p2 as int)};
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn poll_event(&mut self) -> event::Event {
        let haveEvent : bool =  unsafe {
            match ffi::sfWindow_pollEvent(self.window, &self.event) {
                SFFALSE     => false,
                SFTRUE      => true,
                _           => unreachable!()
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn wait_event(&mut self) -> event::Event {
        let haveEvent : bool =  unsafe {
            match ffi::sfWindow_waitEvent(self.window, &self.event) {
                SFFALSE     => false,
                SFTRUE      => true,
                _           => unreachable!()
            }
        };
        if haveEvent == false {
            return event::NoEvent;
        }
        self.get_wrapped_event()
    }

    /**
    * Change the title of a window (with a UTF-32 string)
    *
    * # Arguments
    * * title - New title
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_unicode_title(&mut self, title : ~[u32]) -> () {
        unsafe {
            ffi::sfWindow_setUnicodeTitle(self.window, vec::raw::to_ptr(title))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_icon(&mut self, width : uint, height : uint, pixels : ~[u8]) -> () {
        unsafe {
            ffi::sfWindow_setIcon(self.window, width as c_uint, height as c_uint, vec::raw::to_ptr(pixels))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn close(&mut self) -> () {
        unsafe {
            ffi::sfWindow_close(self.window);
        }
    }

    /**
    * Tell whether or not a window is opened
    *
    * This function returns whether or not the window exists.
    * Note that a hidden window (set_visible(false)) will return
    * true.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_open(&self) -> bool {
        let tmp = unsafe { ffi::sfWindow_isOpen(self.window) };
        match tmp {
            SFFALSE => false,
            SFTRUE  => true,
            _       => unreachable!()
        
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_settings(&self) -> ContextSettings {
        unsafe {ffi::sfWindow_getSettings(self.window)}
    }

    /**
    * Change the title of a window
    *
    * # Arguments
    * * title - New title
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_title(&mut self, title : ~str) -> () {
        let c_title = title.to_c_str();
        unsafe {
            ffi::sfWindow_setTitle(self.window, c_title.unwrap());
        }
    }

    /**
    * Show or hide a window
    *
    * # Arguments
    * * visible - true to show the window, false to hide it
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_visible(&mut self, visible : bool) -> () {
        unsafe {
            match visible {
                true    => ffi::sfWindow_setVisible(self.window, SFTRUE),
                false   => ffi::sfWindow_setVisible(self.window, SFFALSE)
            }
        }
    }
    
    /**
    * Show or hide the mouse cursor
    *
    * # Arguments
    * * visible - true to show, false to hide
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_mouse_cursor_visible(&mut self, visible : bool) -> () {
        unsafe { 
            match visible {
                true    => ffi::sfWindow_setMouseCursorVisible(self.window, SFTRUE),
                false   => ffi::sfWindow_setMouseCursorVisible(self.window, SFFALSE)
            }
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_vertical_sync_enabled(&mut self, enabled : bool) -> () {
        unsafe {
            match enabled {
                true    => ffi::sfWindow_setVerticalSyncEnabled(self.window, SFTRUE),
                false   => ffi::sfWindow_setVerticalSyncEnabled(self.window, SFFALSE)
            }
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_key_repeat_enabled(&mut self, enabled : bool) -> () {
        unsafe {
            match enabled {
                true    => ffi::sfWindow_setKeyRepeatEnabled(self.window, SFTRUE),
                false   => ffi::sfWindow_setKeyRepeatEnabled(self.window, SFFALSE)
            }
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_active(&mut self, enabled : bool) -> bool {
        let tmp = unsafe { match enabled {
            true    => ffi::sfWindow_setActive(self.window, SFTRUE),
            false   => ffi::sfWindow_setActive(self.window, SFFALSE)
        }};
        match tmp {
            SFTRUE   => true,
            SFFALSE  => false,
            _        => unreachable!()
        }
    }
    
    /**
    * Display on screen what has been rendered to the window so far
    *
    * This function is typically called after all OpenGL rendering
    * has been done for the current frame, in order to show
    * it on screen.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn display(&mut self) -> () {
        unsafe {
            ffi::sfWindow_display(self.window)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_framerate_limit(&mut self, limit : uint) -> () {
        unsafe {
            ffi::sfWindow_setFramerateLimit(self.window, limit as c_uint)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_joystick_threshold(&mut self, threshold : float) -> () {
        unsafe {
            ffi::sfWindow_setJoystickThreshold(self.window, threshold as c_float)
        }
    }
    
    /**
    *  Get the position of a window
    *
    * Return the position in pixels
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_position(&self) -> Vector2i {
        unsafe {
            ffi::sfWindow_getPosition(self.window)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_position(&mut self, position : &Vector2i) -> () {
        unsafe {
            ffi::sfWindow_setPosition(self.window, *position)
        }
    }
    
    /**
    * Get the size of the rendering region of a window
    *
    * The size doesn't include the titlebar and borders of the window.
    *
    * Return the size in pixels
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfWindow_getSize(self.window)
        }
    }
 
    /**
    * Change the size of the rendering region of a window
    *
    * # Arguments
    * * size - New size, in pixels
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_size(&mut self, size : &Vector2u) -> () {
        unsafe {
            ffi::sfWindow_setSize(self.window, *size)
        }
    }

    #[fixed_stack_segment] #[inline(never)]
    #[doc(hidden)]
    pub fn unwrap(&self) -> *ffi::sfWindow {
        self.window
    }
}

impl Drop for Window {
    /**
    *   Destructor for class Window. Destroy all the ressource.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&self) {
        unsafe {
            ffi::sfWindow_destroy(self.window);
        }
    }
}
