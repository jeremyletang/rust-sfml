// Generated by rust-sfml-bindgen
// https://github.com/crumblingstatue/rust-sfml-bindgen

unsafe extern "C" {

// Clipboard.cpp
pub fn sfClipboard_getUnicodeString() -> *mut sfString;
pub fn sfClipboard_setUnicodeString(text: *const u32);
// Cursor.cpp
pub fn sfCursor_new() -> *mut sfCursor;
pub fn sfCursor_del(cursor: *mut sfCursor);
pub fn sfCursor_loadFromPixels(cursor: *mut sfCursor, pixels: *const u8, size: sfVector2u, hotspot: sfVector2u) -> bool;
pub fn sfCursor_loadFromSystem(cursor: *mut sfCursor, type_: sfCursorType) -> bool;
// Joystick.cpp
pub fn sfJoystick_isConnected(joystick: c_uint) -> bool;
pub fn sfJoystick_getButtonCount(joystick: c_uint) -> c_uint;
pub fn sfJoystick_hasAxis(joystick: c_uint, axis: sfJoystickAxis) -> bool;
pub fn sfJoystick_isButtonPressed(joystick: c_uint, button: c_uint) -> bool;
pub fn sfJoystick_getAxisPosition(joystick: c_uint, axis: sfJoystickAxis) -> f32;
pub fn sfJoystick_getIdentification(joystick: c_uint) -> *mut sfJoystickIdentification;
pub fn sfJoystickIdentification_del(ident: *mut sfJoystickIdentification);
pub fn sfJoystickIdentification_getVendorId(ident: *const sfJoystickIdentification) -> c_uint;
pub fn sfJoystickIdentification_getProductId(ident: *const sfJoystickIdentification) -> c_uint;
pub fn sfJoystickIdentification_getName(ident: *const sfJoystickIdentification) -> *const sfString;
pub fn sfJoystick_update();
// Keyboard.cpp
pub fn sfKeyboard_isKeyPressed(key: sfKeyboardKey) -> bool;
pub fn sfKeyboard_setVirtualKeyboardVisible(visible: bool);
// Mouse.cpp
pub fn sfMouse_isButtonPressed(button: sfMouseButton) -> bool;
pub fn sfMouse_getPosition() -> sfVector2i;
pub fn sfMouse_getPositionRelativeTo(relativeTo: *const sfWindow) -> sfVector2i;
pub fn sfMouse_setPosition(position: sfVector2i);
pub fn sfMouse_setPositionRelativeTo(pos: sfVector2i, relativeTo: *const sfWindow);
// Sensor.cpp
pub fn sfSensor_isAvailable(sensor: sfSensorType) -> bool;
pub fn sfSensor_setEnabled(sensor: sfSensorType, enabled: bool);
pub fn sfSensor_getValue(sensor: sfSensorType) -> sfVector3f;
// Touch.cpp
pub fn sfTouch_isDown(finger: c_uint) -> bool;
pub fn sfTouch_getPosition(finger: c_uint) -> sfVector2i;
pub fn sfTouch_getPositionRelativeTo(finger: c_uint, relativeTo: *const sfWindow) -> sfVector2i;
// VideoMode.cpp
pub fn sfVideoMode_getDesktopMode() -> sfVideoMode;
pub fn sfVideoMode_getFullscreenModes() -> *const sfVideoModeVector;
pub fn sfVideoMode_isValid(mode: sfVideoMode) -> bool;
pub fn sfVideoModeVector_getLength(vec: *const sfVideoModeVector) -> usize;
pub fn sfVideoModeVector_index(vec: *const sfVideoModeVector, index: usize) -> *const sfVideoMode;
// Window.cpp
pub fn sfWindow_new() -> *mut sfWindow;
pub fn sfWindow_del(window: *mut sfWindow);
pub fn sfWindow_create_mtss(window: *mut sfWindow, mode: sfVideoMode, title: *const u32, style: u32, settings: *const sfContextSettings);
pub fn sfWindow_create_handle_settings(window: *mut sfWindow, handle: sfWindowHandle, settings: *const sfContextSettings);
pub fn sfWindow_close(window: *mut sfWindow);
pub fn sfWindow_isOpen(window: *const sfWindow) -> bool;
pub fn sfWindow_getSettings(window: *const sfWindow) -> *const sfContextSettings;
pub fn sfWindow_pollEvent(window: *mut sfWindow, event: *mut sfEvent) -> bool;
pub fn sfWindow_waitEvent(window: *mut sfWindow, event: *mut sfEvent) -> bool;
pub fn sfWindow_getPosition(window: *const sfWindow) -> sfVector2i;
pub fn sfWindow_setPosition(window: *mut sfWindow, position: sfVector2i);
pub fn sfWindow_getSize(window: *const sfWindow) -> sfVector2u;
pub fn sfWindow_setSize(window: *mut sfWindow, size: sfVector2u);
pub fn sfWindow_setUnicodeTitle(window: *mut sfWindow, title: *const u32);
pub fn sfWindow_setIcon(window: *mut sfWindow, width: c_uint, height: c_uint, pixels: *const u8);
pub fn sfWindow_setVisible(window: *mut sfWindow, visible: bool);
pub fn sfWindow_setMouseCursorVisible(window: *mut sfWindow, visible: bool);
pub fn sfWindow_setMouseCursorGrabbed(window: *mut sfWindow, grabbed: bool);
pub fn sfWindow_setMouseCursor(window: *mut sfWindow, cursor: *const sfCursor);
pub fn sfWindow_setVerticalSyncEnabled(window: *mut sfWindow, enabled: bool);
pub fn sfWindow_setKeyRepeatEnabled(window: *mut sfWindow, enabled: bool);
pub fn sfWindow_setActive(window: *mut sfWindow, active: bool) -> bool;
pub fn sfWindow_requestFocus(window: *mut sfWindow);
pub fn sfWindow_hasFocus(window: *const sfWindow) -> bool;
pub fn sfWindow_display(window: *mut sfWindow);
pub fn sfWindow_setFramerateLimit(window: *mut sfWindow, limit: c_uint);
pub fn sfWindow_setJoystickThreshold(window: *mut sfWindow, threshold: f32);
pub fn sfWindow_getSystemHandle(window: *const sfWindow) -> sfWindowHandle;
pub fn sfContext_new() -> *mut sfContext;
pub fn sfContext_del(context: *mut sfContext);
pub fn sfContext_setActive(context: *mut sfContext, active: bool) -> bool;
pub fn sfContext_getSettings(context: *const sfContext) -> *const sfContextSettings;
pub fn sfContext_getActiveContextId() -> u64;
pub fn sfContext_getFunction(name: *const c_char) -> sfGlFunctionPointer;

}