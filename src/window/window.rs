/*!
 * Window manipulation
 *
 * Provides OpenGL-based windows, and abstractions for events and input handling.
 */

use window::context_settings::ContextSettings;
use window::video_mode::*;
use rsfml::sfTypes::{sfBool};
use core::libc::{c_uint, c_float};
use window::event;
use window::keyboard;
use system::vector2;
use window::joystick;
use window::mouse;

#[doc(hidden)]
pub mod csfml {
    
    use rsfml::sfTypes::{sfBool};
    use core::libc::{c_void, c_uint, c_char, c_float};
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
        //fn sfWindow_createUnicode(mode : sfVideoMode, title : *u32, style : c_uint, setting : *sfContextSettings) -> *sfWindow;
        //fn sfWindow_createFromHandle(handle : sfWindowHandle, settings : *sfContextSettings) -> *sfWindow;
        fn sfWindow_close(window : *sfWindow) -> ();
        fn sfWindow_destroy(window : *sfWindow) -> ();
        fn sfWindow_isOpen(window : *sfWindow) -> sfBool;
        fn sfWindow_getSettings(window : *sfWindow) -> ContextSettings;
        fn sfWindow_setTitle(window : *sfWindow, title : *c_char) -> ();
        //fn sfWindow_setUnicodeTitle(window : *sfWindow, title : *u32) -> ();
        //fn sfWindow_setIcon(window : *sfWindow, width : c_uint, height : c_uint, pixel : *u8) -> (); 
        fn sfWindow_setVisible(window : *sfWindow, visible : sfBool) -> ();
        fn sfWindow_setMouseCursorVisible(window : *sfWindow, visible : sfBool) -> ();
        fn sfWindow_setVerticalSyncEnabled(window : *sfWindow, enabled : sfBool) -> ();
        fn sfWindow_setKeyRepeatEnabled(window : *sfWindow, enabled : sfBool) -> ();
        fn sfWindow_setActive(window : *sfWindow, active : sfBool) -> sfBool;
        fn sfWindow_display(window : *sfWindow) -> ();
        fn sfWindow_setFramerateLimit(window : *sfWindow, limit : c_uint) -> ();
        fn sfWindow_setJoystickThreshold(window : *sfWindow, threshold : c_float) -> ();
        fn sfWindow_getPosition(window : *sfWindow) -> vector2::csfml::sfVector2i;
        fn sfWindow_setPosition(window : *sfWindow, position : vector2::csfml::sfVector2i) -> ();
        fn sfWindow_getSize(window : *sfWindow) -> vector2::csfml::sfVector2u;
        fn sfWindow_setSize(window : *sfWindow, size : vector2::csfml::sfVector2u) -> ();
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
    priv event : csfml::sfEvent
}

pub impl Window {
    
    fn get_wrapped_event(&self) ->event::Event {
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
    * Pop the event on top of event queue.
    *
    * Pop the event on top of event queue, if any, and return it, else return NoEvent.
    *
    */
    fn poll_event(&self) -> event::Event {
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
    * wait_event is blocking, it wait until a new event arrive.
    *
    */
    fn wait_event(&self) -> event::Event {
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
    
    /// Constructor for class Window. Create a window with a VideoMode, a title, style and Contextsetting.
    fn new(mode : VideoMode, title : ~str, style : WindowStyle, settings : &ContextSettings) -> Option<Window> {
        let mut sfWin: *csfml::sfWindow = ptr::null();
        do str::as_c_str(title) |title_buf| {
            unsafe { sfWin = csfml::sfWindow_create(VideoMode::unwrap_videoMode(mode), title_buf, style as u32, settings); }
        };
        let sfEv : csfml::sfEvent = csfml::sfEvent {typeEvent : 0, p1 : 0, p2 : 0, p3 : 0 as c_float, p4 : 0, p5 : 0};//{0, 0, 0, 0 as float, 0, 0};
        if sfWin == ptr::null() {
            None
        }
        else {
        Some (Window { window : sfWin, event : sfEv})
        }
    }
    
    /// Method close for class Window. Close the window and destroy attached ressources.
    fn close(&self) -> () {
        unsafe {
            csfml::sfWindow_close(self.window);
        }
    }

    /**
    *   Method is_open. Verifiy if the windows is already open.
    */
    fn is_open(&self) -> bool {
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
    *   Method for class window, get the window OpenGl context settings.
    */
    fn get_settings(&self) -> ContextSettings {
        unsafe {csfml::sfWindow_getSettings(self.window)}
    }

    /**
    *   Method for class window, set the window title.
    */
    fn set_title(&self, title : ~str) -> () {
        do str::as_c_str(title) |title_buf| {
            unsafe {
                csfml::sfWindow_setTitle(self.window, title_buf);
            }
        }
    }

    /**
    *   Method for class window, display or not the window.
    */
    fn set_visible(&self, visible : bool) -> () {
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
    *   Method for class window, set visible the mouse cursor on the window.
    */
    fn set_mouse_cursor_visible(&self, visible : bool) -> () {
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
    *   Method for class window, enable or diseable the vertical sync.
    */
    fn set_vertical_sync_enabled(&self, enabled : bool) -> () {
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
    *   Method for class window, enable or diseable the key repeat.
    */
    fn set_key_repeat_enabled(&self, enabled : bool) -> () {
        let tmp : sfBool = 
            match enabled {
                true    => 1,
                _       => 0
            };
        unsafe {
            csfml::sfWindow_setKeyRepeatEnabled(self.window, tmp);
        }
    }
    
    fn set_active(&self, enabled : bool) -> bool {
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
    *   Method for class window, display the content of the window.
    */
    fn display(&self) -> () {
        unsafe {
            csfml::sfWindow_display(self.window)
        }
    }

    /**
    *   Method for class window, set the maximal framerate of the window.
    */
    fn set_framerate_limit(&self, limit : uint) -> () {
        unsafe {
            csfml::sfWindow_setFramerateLimit(self.window, limit as c_uint)
        }
    }

    /**
    *   Method for class window, set the joystick Threshold.
    */
    fn set_joystick_threshold(&self, threshold : float) -> () {
        unsafe {
            csfml::sfWindow_setJoystickThreshold(self.window, threshold as c_float)
        }
    }
    
    /**
    *   Method for class window, get the position of the window on a Vector2i.
    */
    fn get_position(&self) -> vector2::Vector2i {
        unsafe {
            vector2::wrap_vector2i(csfml::sfWindow_getPosition(self.window))
        }
    }
    
    /**
    *   Method for class window, set the position of the window with a Vector2i.
    */
    fn set_position(&self, position: vector2::Vector2i) -> () {
        unsafe {
            csfml::sfWindow_setPosition(self.window, vector2::unwrap_vector2i(position))
        }
    }
    
    /**
    *   Method for class window, get the size of the window on a Vector2u.
    */
    fn get_size(&self) -> vector2::Vector2u {
        unsafe {
            vector2::wrap_vector2u(csfml::sfWindow_getSize(self.window))
        }
    }
 
    /**
    *   Method for class window, set the size of the window with a Vector2u
    */
    fn set_size(&self, size : vector2::Vector2u) -> () {
        unsafe {
            csfml::sfWindow_setSize(self.window, vector2::unwrap_vector2u(size))
        }
    }

    /**
    *   Method for class window, retrieve the sfWindow contained on struct Window. Used for binding.
    */
    #[doc(hidden)]
    fn get_sfWindow(&self) -> *csfml::sfWindow {
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
