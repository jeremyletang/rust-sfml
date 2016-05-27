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

#![allow(non_upper_case_globals, non_camel_case_types)]

extern crate libc;
extern crate sfml_types;
extern crate csfml_system_sys;

use libc::{c_uint, c_float, c_char, c_int, size_t};
use sfml_types::{Vector2i, Vector2u};
use csfml_system_sys::*;

pub enum sfWindow {}

extern "C" {
    pub fn sfWindow_create(mode: sfVideoMode, title: *const c_char, style: c_uint, settings: *const sfContextSettings) -> *mut sfWindow;
    pub fn sfWindow_createUnicode(mode: sfVideoMode, title: *const u32, style: c_uint, setting: *const sfContextSettings) -> *mut sfWindow;
    //fn sfWindow_createFromHandle(handle: sfWindowHandle, settings: *mut sfContextSettings) -> *mut sfWindow;
    pub fn sfWindow_close(window: *mut sfWindow);
    pub fn sfWindow_destroy(window: *mut sfWindow);
    pub fn sfWindow_isOpen(window: *mut sfWindow) -> sfBool;
    pub fn sfWindow_getSettings(window: *mut sfWindow) -> sfContextSettings;
    pub fn sfWindow_setTitle(window: *mut sfWindow, title: *const c_char);
    pub fn sfWindow_setUnicodeTitle(window: *mut sfWindow, title: *const u32);
    pub fn sfWindow_setIcon(window: *mut sfWindow, width: c_uint, height: c_uint, pixel: *const u8);
    pub fn sfWindow_setVisible(window: *mut sfWindow, visible: sfBool);
    pub fn sfWindow_setMouseCursorVisible(window: *mut sfWindow, visible: sfBool);
    pub fn sfWindow_setVerticalSyncEnabled(window: *mut sfWindow, enabled: sfBool);
    pub fn sfWindow_setKeyRepeatEnabled(window: *mut sfWindow, enabled: sfBool);
    pub fn sfWindow_setActive(window: *mut sfWindow, active: sfBool) -> sfBool;
    pub fn sfWindow_display(window: *mut sfWindow);
    pub fn sfWindow_setFramerateLimit(window: *mut sfWindow, limit: c_uint);
    pub fn sfWindow_setJoystickThreshold(window: *mut sfWindow, threshold: c_float);
    pub fn sfWindow_getPosition(window: *mut sfWindow) -> Vector2i;
    pub fn sfWindow_setPosition(window: *mut sfWindow, position: Vector2i);
    pub fn sfWindow_getSize(window: *mut sfWindow) -> Vector2u;
    pub fn sfWindow_setSize(window: *mut sfWindow, size: Vector2u);
    pub fn sfWindow_pollEvent(window: *mut sfWindow, event: *mut sfEvent) -> sfBool;
    pub fn sfWindow_waitEvent(window: *mut sfWindow, event: *mut sfEvent) -> sfBool;
    pub fn sfMouse_getPosition(relativeTo: *mut sfWindow) -> Vector2i;
    pub fn sfMouse_setPosition(position: Vector2i, relativeTo: *mut sfWindow);
    //fn sfWindow_getSystemHandle(window: *mut sfWindow) -> sfWindowHandle;
}

pub enum sfContext {}

extern "C" {
    pub fn sfContext_create() -> *mut sfContext;
    pub fn sfContext_destroy(context: *mut sfContext);
    pub fn sfContext_setActive(context: *mut sfContext, active: sfBool);
}

extern "C" {
    pub fn sfJoystick_isConnected(joystick: c_uint) -> sfBool;
    pub fn sfJoystick_getButtonCount(joystick: c_uint) -> c_uint;
    pub fn sfJoystick_hasAxis(joystick: c_uint, axis: c_uint) -> sfBool;
    pub fn sfJoystick_isButtonPressed(joystick: c_uint, button: c_uint) -> sfBool;
    pub fn sfJoystick_getAxisPosition(joystick: c_uint, axis: c_uint) -> c_float;
    pub fn sfJoystick_update();
}

extern "C" {
    pub fn sfKeyboard_isKeyPressed(key: c_int) -> sfBool;
}

extern "C" {
    pub fn sfMouse_isButtonPressed(button: c_uint) -> sfBool;
}

#[repr(C)]
pub struct sfVideoMode {
    pub width:          c_uint,
    pub height:         c_uint,
    pub bits_per_pixel: c_uint
}

impl Clone for sfVideoMode {
    fn clone(&self) -> sfVideoMode {
       sfVideoMode {
           width: self.width,
           height: self.height,
           bits_per_pixel: self.bits_per_pixel
        }
    }
}

extern "C" {
    pub fn sfVideoMode_getDesktopMode() -> sfVideoMode;
    pub fn sfVideoMode_getFullscreenModes(Count: *mut size_t) -> *mut sfVideoMode;
    pub fn sfVideoMode_isValid(mode: sfVideoMode) -> sfBool;
}

pub type sfKeyCode = ::libc::c_int;

pub type sfMouseButton = ::libc::c_uint;
pub type sfJoystickAxis = ::libc::c_uint;

pub type sfEventType = ::libc::c_uint;
pub const sfEvtClosed: ::libc::c_uint = 0;
pub const sfEvtResized: ::libc::c_uint = 1;
pub const sfEvtLostFocus: ::libc::c_uint = 2;
pub const sfEvtGainedFocus: ::libc::c_uint = 3;
pub const sfEvtTextEntered: ::libc::c_uint = 4;
pub const sfEvtKeyPressed: ::libc::c_uint = 5;
pub const sfEvtKeyReleased: ::libc::c_uint = 6;
pub const sfEvtMouseWheelMoved: ::libc::c_uint = 7;
pub const sfEvtMouseWheelScrolled: ::libc::c_uint = 8;
pub const sfEvtMouseButtonPressed: ::libc::c_uint = 9;
pub const sfEvtMouseButtonReleased: ::libc::c_uint = 10;
pub const sfEvtMouseMoved: ::libc::c_uint = 11;
pub const sfEvtMouseEntered: ::libc::c_uint = 12;
pub const sfEvtMouseLeft: ::libc::c_uint = 13;
pub const sfEvtJoystickButtonPressed: ::libc::c_uint = 14;
pub const sfEvtJoystickButtonReleased: ::libc::c_uint = 15;
pub const sfEvtJoystickMoved: ::libc::c_uint = 16;
pub const sfEvtJoystickConnected: ::libc::c_uint = 17;
pub const sfEvtJoystickDisconnected: ::libc::c_uint = 18;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfKeyEvent {
    pub type_: sfEventType,
    pub code: sfKeyCode,
    pub alt: sfBool,
    pub control: sfBool,
    pub shift: sfBool,
    pub system: sfBool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfTextEvent {
    pub type_: sfEventType,
    pub unicode: ::libc::c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfMouseMoveEvent {
    pub type_: sfEventType,
    pub x: ::libc::c_int,
    pub y: ::libc::c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfMouseButtonEvent {
    pub type_: sfEventType,
    pub button: sfMouseButton,
    pub x: ::libc::c_int,
    pub y: ::libc::c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfMouseWheelEvent {
    pub type_: sfEventType,
    pub delta: ::libc::c_int,
    pub x: ::libc::c_int,
    pub y: ::libc::c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfJoystickMoveEvent {
    pub type_: sfEventType,
    pub joystickid: ::libc::c_uint,
    pub axis: sfJoystickAxis,
    pub position: ::libc::c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfJoystickButtonEvent {
    pub type_: sfEventType,
    pub joystickid: ::libc::c_uint,
    pub button: ::libc::c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfJoystickConnectEvent {
    pub type_: sfEventType,
    pub joystickid: ::libc::c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfSizeEvent {
    pub type_: sfEventType,
    pub width: ::libc::c_uint,
    pub height: ::libc::c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sfEvent {
    pub data: [u32; 6],
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct sfContextSettings {
    /// Bits of the depth buffer.
    pub depth_bits: u32,
    /// Bits of the stencil buffer.
    pub stencil_bits: u32,
    /// Level of antialiasing.
    pub antialiasing_level: u32,
    /// Major number of the context version
    pub major_version: u32,
    /// Minor number of the context version
    pub minor_version: u32,
    /// The attribute flags to create the context with
    pub attribute_flags: u32,
}
