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

use libc::{c_int, c_uint, c_float, c_char, size_t};
use system::{Vector2i, Vector2u, Vector3f};
use window::{ContextSettings, VideoMode, Sensor};
use window::raw_event::sfEvent;
use ffi::SfBool;

foreign_type! {
	sfWindow, sfWindow_destroy;
	sfContext, sfContext_destroy;
}

#[repr(C)]
pub struct sfJoystickIdentification {
	pub name: *const c_char,
	pub vendor_id: c_uint,
	pub product_id: c_uint
}

#[cfg_attr(any(target_os="macos", target_os="linux", target_os="windows"), link(name="csfml-window"))]
extern "C" {
	//pub fn sfWindow_create(mode: VideoMode, title: *const c_char, style: c_uint, settings: *const ContextSettings) -> *mut sfWindow;
	pub fn sfWindow_createUnicode(mode: VideoMode, title: *const u32, style: c_uint, setting: *const ContextSettings) -> *mut sfWindow;
	//fn sfWindow_createFromHandle(handle: sfWindowHandle, settings: *mut sfContextSettings) -> *mut sfWindow;
	pub fn sfWindow_close(window: *mut sfWindow) -> ();
	pub fn sfWindow_destroy(window: *mut sfWindow) -> ();
	pub fn sfWindow_isOpen(window: *const sfWindow) -> SfBool;
	pub fn sfWindow_getSettings(window: *const sfWindow) -> ContextSettings;
	//pub fn sfWindow_setTitle(window: *mut sfWindow, title: *const c_char) -> ();
	pub fn sfWindow_setUnicodeTitle(window: *mut sfWindow, title: *const u32) -> ();
	pub fn sfWindow_setIcon(window: *mut sfWindow, width: c_uint, height: c_uint, pixel: *const u8) -> ();
	pub fn sfWindow_setVisible(window: *mut sfWindow, visible: SfBool) -> ();
	pub fn sfWindow_setMouseCursorVisible(window: *mut sfWindow, visible: SfBool) -> ();
	pub fn sfWindow_setVerticalSyncEnabled(window: *mut sfWindow, enabled: SfBool) -> ();
	pub fn sfWindow_setKeyRepeatEnabled(window: *mut sfWindow, enabled: SfBool) -> ();
	pub fn sfWindow_setActive(window: *mut sfWindow, active: SfBool) -> SfBool;
	pub fn sfWindow_requestFocus(window: *mut sfWindow) -> ();
	pub fn sfWindow_hasFocus(window: *const sfWindow) -> SfBool;
	pub fn sfWindow_display(window: *mut sfWindow) -> ();
	pub fn sfWindow_setFramerateLimit(window: *mut sfWindow, limit: c_uint) -> ();
	pub fn sfWindow_setJoystickThreshold(window: *mut sfWindow, threshold: c_float) -> ();
	pub fn sfWindow_getPosition(window: *const sfWindow) -> Vector2i;
	pub fn sfWindow_setPosition(window: *mut sfWindow, position: Vector2i) -> ();
	pub fn sfWindow_getSize(window: *const sfWindow) -> Vector2u;
	pub fn sfWindow_setSize(window: *mut sfWindow, size: Vector2u) -> ();
	pub fn sfWindow_pollEvent(window: *mut sfWindow, event: *mut sfEvent) -> SfBool;
	pub fn sfWindow_waitEvent(window: *mut sfWindow, event: *mut sfEvent) -> SfBool;
	pub fn sfMouse_getPosition(relativeTo: *const sfWindow) -> Vector2i;
	pub fn sfMouse_setPosition(position: Vector2i, relativeTo: *const sfWindow) -> ();
	//fn sfWindow_getSystemHandle(window: *mut sfWindow) -> sfWindowHandle;

	pub fn sfContext_create() -> *mut sfContext;
	pub fn sfContext_destroy(context: *mut sfContext) -> ();
	pub fn sfContext_setActive(context: *mut sfContext, active: SfBool) -> ();

	pub fn sfJoystick_isConnected(joystick: c_uint) -> SfBool;
	pub fn sfJoystick_getButtonCount(joystick: c_uint) -> c_uint;
	pub fn sfJoystick_hasAxis(joystick: c_uint, axis: c_uint) -> SfBool;
	pub fn sfJoystick_isButtonPressed(joystick: c_uint, button: c_uint) -> SfBool;
	pub fn sfJoystick_getAxisPosition(joystick: c_uint, axis: c_uint) -> c_float;
	pub fn sfJoystick_getIdentification(joystick: c_uint) -> sfJoystickIdentification;
	pub fn sfJoystick_update() -> ();

	pub fn sfKeyboard_isKeyPressed(key: c_int) -> SfBool;

	pub fn sfMouse_isButtonPressed(button: c_uint) -> SfBool;

	pub fn sfSensor_isAvailable(sensor: Sensor) -> SfBool;
	pub fn sfSensor_setEnabled(sensor: Sensor, enabled: SfBool);
	pub fn sfSensor_getValue(sensor: Sensor) -> Vector3f;

	pub fn sfTouch_isDown(finger: c_uint) -> SfBool;
	pub fn sfTouch_getPosition(finger: c_uint, relative_to: *const sfWindow) -> Vector2i;

	pub fn sfVideoMode_getDesktopMode() -> VideoMode;
	pub fn sfVideoMode_getFullscreenModes(count: *mut size_t) -> *const VideoMode;
	pub fn sfVideoMode_isValid(mode: VideoMode) -> SfBool;
}
