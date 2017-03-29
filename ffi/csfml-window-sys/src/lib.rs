#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate csfml_system_sys;

use csfml_system_sys::*;

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfContext([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfWindow([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfJoystickIdentification {
    pub name: *const ::std::os::raw::c_char,
    pub vendorId: ::std::os::raw::c_uint,
    pub productId: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_sfJoystickIdentification() {
    assert_eq!(::std::mem::size_of::<sfJoystickIdentification>() , 16usize ,
               concat ! (
               "Size of: " , stringify ! ( sfJoystickIdentification ) ));
    assert_eq! (::std::mem::align_of::<sfJoystickIdentification>() , 8usize ,
                concat ! (
                "Alignment of " , stringify ! ( sfJoystickIdentification ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickIdentification ) ) . name as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! (
                sfJoystickIdentification ) , "::" , stringify ! ( name ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickIdentification ) ) . vendorId
                as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! (
                sfJoystickIdentification ) , "::" , stringify ! ( vendorId )
                ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickIdentification ) ) . productId
                as * const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! (
                sfJoystickIdentification ) , "::" , stringify ! ( productId )
                ));
}
impl Clone for sfJoystickIdentification {
    fn clone(&self) -> Self { *self }
}
pub const sfJoystickCount: _bindgen_ty_1 = _bindgen_ty_1::sfJoystickCount;
pub const sfJoystickButtonCount: _bindgen_ty_1 =
    _bindgen_ty_1::sfJoystickButtonCount;
pub const sfJoystickAxisCount: _bindgen_ty_1 = _bindgen_ty_1::sfJoystickCount;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _bindgen_ty_1 { sfJoystickCount = 8, sfJoystickButtonCount = 32, }
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfJoystickAxis {
    sfJoystickX = 0,
    sfJoystickY = 1,
    sfJoystickZ = 2,
    sfJoystickR = 3,
    sfJoystickU = 4,
    sfJoystickV = 5,
    sfJoystickPovX = 6,
    sfJoystickPovY = 7,
}
extern "C" {
    pub fn sfJoystick_isConnected(joystick: ::std::os::raw::c_uint) -> sfBool;
}
extern "C" {
    pub fn sfJoystick_getButtonCount(joystick: ::std::os::raw::c_uint)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfJoystick_hasAxis(joystick: ::std::os::raw::c_uint,
                              axis: sfJoystickAxis) -> sfBool;
}
extern "C" {
    pub fn sfJoystick_isButtonPressed(joystick: ::std::os::raw::c_uint,
                                      button: ::std::os::raw::c_uint)
     -> sfBool;
}
extern "C" {
    pub fn sfJoystick_getAxisPosition(joystick: ::std::os::raw::c_uint,
                                      axis: sfJoystickAxis) -> f32;
}
extern "C" {
    pub fn sfJoystick_getIdentification(joystick: ::std::os::raw::c_uint)
     -> sfJoystickIdentification;
}
extern "C" {
    pub fn sfJoystick_update();
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfKeyCode {
    sfKeyUnknown = -1,
    sfKeyA = 0,
    sfKeyB = 1,
    sfKeyC = 2,
    sfKeyD = 3,
    sfKeyE = 4,
    sfKeyF = 5,
    sfKeyG = 6,
    sfKeyH = 7,
    sfKeyI = 8,
    sfKeyJ = 9,
    sfKeyK = 10,
    sfKeyL = 11,
    sfKeyM = 12,
    sfKeyN = 13,
    sfKeyO = 14,
    sfKeyP = 15,
    sfKeyQ = 16,
    sfKeyR = 17,
    sfKeyS = 18,
    sfKeyT = 19,
    sfKeyU = 20,
    sfKeyV = 21,
    sfKeyW = 22,
    sfKeyX = 23,
    sfKeyY = 24,
    sfKeyZ = 25,
    sfKeyNum0 = 26,
    sfKeyNum1 = 27,
    sfKeyNum2 = 28,
    sfKeyNum3 = 29,
    sfKeyNum4 = 30,
    sfKeyNum5 = 31,
    sfKeyNum6 = 32,
    sfKeyNum7 = 33,
    sfKeyNum8 = 34,
    sfKeyNum9 = 35,
    sfKeyEscape = 36,
    sfKeyLControl = 37,
    sfKeyLShift = 38,
    sfKeyLAlt = 39,
    sfKeyLSystem = 40,
    sfKeyRControl = 41,
    sfKeyRShift = 42,
    sfKeyRAlt = 43,
    sfKeyRSystem = 44,
    sfKeyMenu = 45,
    sfKeyLBracket = 46,
    sfKeyRBracket = 47,
    sfKeySemiColon = 48,
    sfKeyComma = 49,
    sfKeyPeriod = 50,
    sfKeyQuote = 51,
    sfKeySlash = 52,
    sfKeyBackSlash = 53,
    sfKeyTilde = 54,
    sfKeyEqual = 55,
    sfKeyDash = 56,
    sfKeySpace = 57,
    sfKeyReturn = 58,
    sfKeyBack = 59,
    sfKeyTab = 60,
    sfKeyPageUp = 61,
    sfKeyPageDown = 62,
    sfKeyEnd = 63,
    sfKeyHome = 64,
    sfKeyInsert = 65,
    sfKeyDelete = 66,
    sfKeyAdd = 67,
    sfKeySubtract = 68,
    sfKeyMultiply = 69,
    sfKeyDivide = 70,
    sfKeyLeft = 71,
    sfKeyRight = 72,
    sfKeyUp = 73,
    sfKeyDown = 74,
    sfKeyNumpad0 = 75,
    sfKeyNumpad1 = 76,
    sfKeyNumpad2 = 77,
    sfKeyNumpad3 = 78,
    sfKeyNumpad4 = 79,
    sfKeyNumpad5 = 80,
    sfKeyNumpad6 = 81,
    sfKeyNumpad7 = 82,
    sfKeyNumpad8 = 83,
    sfKeyNumpad9 = 84,
    sfKeyF1 = 85,
    sfKeyF2 = 86,
    sfKeyF3 = 87,
    sfKeyF4 = 88,
    sfKeyF5 = 89,
    sfKeyF6 = 90,
    sfKeyF7 = 91,
    sfKeyF8 = 92,
    sfKeyF9 = 93,
    sfKeyF10 = 94,
    sfKeyF11 = 95,
    sfKeyF12 = 96,
    sfKeyF13 = 97,
    sfKeyF14 = 98,
    sfKeyF15 = 99,
    sfKeyPause = 100,
    sfKeyCount = 101,
}
extern "C" {
    pub fn sfKeyboard_isKeyPressed(key: sfKeyCode) -> sfBool;
}
extern "C" {
    pub fn sfKeyboard_setVirtualKeyboardVisible(visible: sfBool);
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfMouseButton {
    sfMouseLeft = 0,
    sfMouseRight = 1,
    sfMouseMiddle = 2,
    sfMouseXButton1 = 3,
    sfMouseXButton2 = 4,
    sfMouseButtonCount = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfMouseWheel {
    sfMouseVerticalWheel = 0,
    sfMouseHorizontalWheel = 1,
}
extern "C" {
    pub fn sfMouse_isButtonPressed(button: sfMouseButton) -> sfBool;
}
extern "C" {
    pub fn sfMouse_getPosition(relativeTo: *const sfWindow) -> sfVector2i;
}
extern "C" {
    pub fn sfMouse_setPosition(position: sfVector2i,
                               relativeTo: *const sfWindow);
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfSensorType {
    sfSensorAccelerometer = 0,
    sfSensorGyroscope = 1,
    sfSensorMagnetometer = 2,
    sfSensorGravity = 3,
    sfSensorUserAcceleration = 4,
    sfSensorOrientation = 5,
    sfSensorCount = 6,
}
extern "C" {
    pub fn sfSensor_isAvailable(sensor: sfSensorType) -> sfBool;
}
extern "C" {
    pub fn sfSensor_setEnabled(sensor: sfSensorType, enabled: sfBool);
}
extern "C" {
    pub fn sfSensor_getValue(sensor: sfSensorType) -> sfVector3f;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfEventType {
    sfEvtClosed = 0,
    sfEvtResized = 1,
    sfEvtLostFocus = 2,
    sfEvtGainedFocus = 3,
    sfEvtTextEntered = 4,
    sfEvtKeyPressed = 5,
    sfEvtKeyReleased = 6,
    sfEvtMouseWheelMoved = 7,
    sfEvtMouseWheelScrolled = 8,
    sfEvtMouseButtonPressed = 9,
    sfEvtMouseButtonReleased = 10,
    sfEvtMouseMoved = 11,
    sfEvtMouseEntered = 12,
    sfEvtMouseLeft = 13,
    sfEvtJoystickButtonPressed = 14,
    sfEvtJoystickButtonReleased = 15,
    sfEvtJoystickMoved = 16,
    sfEvtJoystickConnected = 17,
    sfEvtJoystickDisconnected = 18,
    sfEvtTouchBegan = 19,
    sfEvtTouchMoved = 20,
    sfEvtTouchEnded = 21,
    sfEvtSensorChanged = 22,
    sfEvtCount = 23,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfKeyEvent {
    pub type_: sfEventType,
    pub code: sfKeyCode,
    pub alt: sfBool,
    pub control: sfBool,
    pub shift: sfBool,
    pub system: sfBool,
}
#[test]
fn bindgen_test_layout_sfKeyEvent() {
    assert_eq!(::std::mem::size_of::<sfKeyEvent>() , 24usize , concat ! (
               "Size of: " , stringify ! ( sfKeyEvent ) ));
    assert_eq! (::std::mem::align_of::<sfKeyEvent>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfKeyEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfKeyEvent ) ) . type_ as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfKeyEvent ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfKeyEvent ) ) . code as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfKeyEvent ) , "::" ,
                stringify ! ( code ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfKeyEvent ) ) . alt as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfKeyEvent ) , "::" ,
                stringify ! ( alt ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfKeyEvent ) ) . control as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfKeyEvent ) , "::" ,
                stringify ! ( control ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfKeyEvent ) ) . shift as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( sfKeyEvent ) , "::" ,
                stringify ! ( shift ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfKeyEvent ) ) . system as * const _ as
                usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( sfKeyEvent ) , "::" ,
                stringify ! ( system ) ));
}
impl Clone for sfKeyEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfTextEvent {
    pub type_: sfEventType,
    pub unicode: sfUint32,
}
#[test]
fn bindgen_test_layout_sfTextEvent() {
    assert_eq!(::std::mem::size_of::<sfTextEvent>() , 8usize , concat ! (
               "Size of: " , stringify ! ( sfTextEvent ) ));
    assert_eq! (::std::mem::align_of::<sfTextEvent>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfTextEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfTextEvent ) ) . type_ as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfTextEvent ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfTextEvent ) ) . unicode as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfTextEvent ) , "::" ,
                stringify ! ( unicode ) ));
}
impl Clone for sfTextEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfMouseMoveEvent {
    pub type_: sfEventType,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfMouseMoveEvent() {
    assert_eq!(::std::mem::size_of::<sfMouseMoveEvent>() , 12usize , concat !
               ( "Size of: " , stringify ! ( sfMouseMoveEvent ) ));
    assert_eq! (::std::mem::align_of::<sfMouseMoveEvent>() , 4usize , concat !
                ( "Alignment of " , stringify ! ( sfMouseMoveEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseMoveEvent ) ) . type_ as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseMoveEvent ) ,
                "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseMoveEvent ) ) . x as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseMoveEvent ) ,
                "::" , stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseMoveEvent ) ) . y as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseMoveEvent ) ,
                "::" , stringify ! ( y ) ));
}
impl Clone for sfMouseMoveEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfMouseButtonEvent {
    pub type_: sfEventType,
    pub button: sfMouseButton,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfMouseButtonEvent() {
    assert_eq!(::std::mem::size_of::<sfMouseButtonEvent>() , 16usize , concat
               ! ( "Size of: " , stringify ! ( sfMouseButtonEvent ) ));
    assert_eq! (::std::mem::align_of::<sfMouseButtonEvent>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( sfMouseButtonEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseButtonEvent ) ) . type_ as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseButtonEvent ) ,
                "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseButtonEvent ) ) . button as *
                const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseButtonEvent ) ,
                "::" , stringify ! ( button ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseButtonEvent ) ) . x as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseButtonEvent ) ,
                "::" , stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseButtonEvent ) ) . y as * const _
                as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseButtonEvent ) ,
                "::" , stringify ! ( y ) ));
}
impl Clone for sfMouseButtonEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfMouseWheelEvent {
    pub type_: sfEventType,
    pub delta: ::std::os::raw::c_int,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfMouseWheelEvent() {
    assert_eq!(::std::mem::size_of::<sfMouseWheelEvent>() , 16usize , concat !
               ( "Size of: " , stringify ! ( sfMouseWheelEvent ) ));
    assert_eq! (::std::mem::align_of::<sfMouseWheelEvent>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( sfMouseWheelEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelEvent ) ) . type_ as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelEvent ) ,
                "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelEvent ) ) . delta as * const
                _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelEvent ) ,
                "::" , stringify ! ( delta ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelEvent ) ) . x as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelEvent ) ,
                "::" , stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelEvent ) ) . y as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelEvent ) ,
                "::" , stringify ! ( y ) ));
}
impl Clone for sfMouseWheelEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfMouseWheelScrollEvent {
    pub type_: sfEventType,
    pub wheel: sfMouseWheel,
    pub delta: f32,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfMouseWheelScrollEvent() {
    assert_eq!(::std::mem::size_of::<sfMouseWheelScrollEvent>() , 20usize ,
               concat ! (
               "Size of: " , stringify ! ( sfMouseWheelScrollEvent ) ));
    assert_eq! (::std::mem::align_of::<sfMouseWheelScrollEvent>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( sfMouseWheelScrollEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelScrollEvent ) ) . type_ as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelScrollEvent
                ) , "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelScrollEvent ) ) . wheel as *
                const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelScrollEvent
                ) , "::" , stringify ! ( wheel ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelScrollEvent ) ) . delta as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelScrollEvent
                ) , "::" , stringify ! ( delta ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelScrollEvent ) ) . x as *
                const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelScrollEvent
                ) , "::" , stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfMouseWheelScrollEvent ) ) . y as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( sfMouseWheelScrollEvent
                ) , "::" , stringify ! ( y ) ));
}
impl Clone for sfMouseWheelScrollEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfJoystickMoveEvent {
    pub type_: sfEventType,
    pub joystickId: ::std::os::raw::c_uint,
    pub axis: sfJoystickAxis,
    pub position: f32,
}
#[test]
fn bindgen_test_layout_sfJoystickMoveEvent() {
    assert_eq!(::std::mem::size_of::<sfJoystickMoveEvent>() , 16usize , concat
               ! ( "Size of: " , stringify ! ( sfJoystickMoveEvent ) ));
    assert_eq! (::std::mem::align_of::<sfJoystickMoveEvent>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( sfJoystickMoveEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickMoveEvent ) ) . type_ as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickMoveEvent ) ,
                "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickMoveEvent ) ) . joystickId as *
                const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickMoveEvent ) ,
                "::" , stringify ! ( joystickId ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickMoveEvent ) ) . axis as * const
                _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickMoveEvent ) ,
                "::" , stringify ! ( axis ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickMoveEvent ) ) . position as *
                const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickMoveEvent ) ,
                "::" , stringify ! ( position ) ));
}
impl Clone for sfJoystickMoveEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfJoystickButtonEvent {
    pub type_: sfEventType,
    pub joystickId: ::std::os::raw::c_uint,
    pub button: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_sfJoystickButtonEvent() {
    assert_eq!(::std::mem::size_of::<sfJoystickButtonEvent>() , 12usize ,
               concat ! ( "Size of: " , stringify ! ( sfJoystickButtonEvent )
               ));
    assert_eq! (::std::mem::align_of::<sfJoystickButtonEvent>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( sfJoystickButtonEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickButtonEvent ) ) . type_ as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickButtonEvent )
                , "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickButtonEvent ) ) . joystickId as
                * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickButtonEvent )
                , "::" , stringify ! ( joystickId ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickButtonEvent ) ) . button as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickButtonEvent )
                , "::" , stringify ! ( button ) ));
}
impl Clone for sfJoystickButtonEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfJoystickConnectEvent {
    pub type_: sfEventType,
    pub joystickId: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_sfJoystickConnectEvent() {
    assert_eq!(::std::mem::size_of::<sfJoystickConnectEvent>() , 8usize ,
               concat ! ( "Size of: " , stringify ! ( sfJoystickConnectEvent )
               ));
    assert_eq! (::std::mem::align_of::<sfJoystickConnectEvent>() , 4usize ,
                concat ! (
                "Alignment of " , stringify ! ( sfJoystickConnectEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickConnectEvent ) ) . type_ as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickConnectEvent
                ) , "::" , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfJoystickConnectEvent ) ) . joystickId
                as * const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfJoystickConnectEvent
                ) , "::" , stringify ! ( joystickId ) ));
}
impl Clone for sfJoystickConnectEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfSizeEvent {
    pub type_: sfEventType,
    pub width: ::std::os::raw::c_uint,
    pub height: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_sfSizeEvent() {
    assert_eq!(::std::mem::size_of::<sfSizeEvent>() , 12usize , concat ! (
               "Size of: " , stringify ! ( sfSizeEvent ) ));
    assert_eq! (::std::mem::align_of::<sfSizeEvent>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfSizeEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSizeEvent ) ) . type_ as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSizeEvent ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSizeEvent ) ) . width as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSizeEvent ) , "::" ,
                stringify ! ( width ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSizeEvent ) ) . height as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSizeEvent ) , "::" ,
                stringify ! ( height ) ));
}
impl Clone for sfSizeEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfTouchEvent {
    pub type_: sfEventType,
    pub finger: ::std::os::raw::c_uint,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfTouchEvent() {
    assert_eq!(::std::mem::size_of::<sfTouchEvent>() , 16usize , concat ! (
               "Size of: " , stringify ! ( sfTouchEvent ) ));
    assert_eq! (::std::mem::align_of::<sfTouchEvent>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfTouchEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfTouchEvent ) ) . type_ as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfTouchEvent ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfTouchEvent ) ) . finger as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfTouchEvent ) , "::" ,
                stringify ! ( finger ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfTouchEvent ) ) . x as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfTouchEvent ) , "::" ,
                stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfTouchEvent ) ) . y as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfTouchEvent ) , "::" ,
                stringify ! ( y ) ));
}
impl Clone for sfTouchEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfSensorEvent {
    pub type_: sfEventType,
    pub sensorType: sfSensorType,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[test]
fn bindgen_test_layout_sfSensorEvent() {
    assert_eq!(::std::mem::size_of::<sfSensorEvent>() , 20usize , concat ! (
               "Size of: " , stringify ! ( sfSensorEvent ) ));
    assert_eq! (::std::mem::align_of::<sfSensorEvent>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfSensorEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSensorEvent ) ) . type_ as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSensorEvent ) , "::"
                , stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSensorEvent ) ) . sensorType as * const
                _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSensorEvent ) , "::"
                , stringify ! ( sensorType ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSensorEvent ) ) . x as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSensorEvent ) , "::"
                , stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSensorEvent ) ) . y as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSensorEvent ) , "::"
                , stringify ! ( y ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfSensorEvent ) ) . z as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( sfSensorEvent ) , "::"
                , stringify ! ( z ) ));
}
impl Clone for sfSensorEvent {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfEvent {
    pub type_: __BindgenUnionField<sfEventType>,
    pub size: __BindgenUnionField<sfSizeEvent>,
    pub key: __BindgenUnionField<sfKeyEvent>,
    pub text: __BindgenUnionField<sfTextEvent>,
    pub mouseMove: __BindgenUnionField<sfMouseMoveEvent>,
    pub mouseButton: __BindgenUnionField<sfMouseButtonEvent>,
    pub mouseWheel: __BindgenUnionField<sfMouseWheelEvent>,
    pub mouseWheelScroll: __BindgenUnionField<sfMouseWheelScrollEvent>,
    pub joystickMove: __BindgenUnionField<sfJoystickMoveEvent>,
    pub joystickButton: __BindgenUnionField<sfJoystickButtonEvent>,
    pub joystickConnect: __BindgenUnionField<sfJoystickConnectEvent>,
    pub touch: __BindgenUnionField<sfTouchEvent>,
    pub sensor: __BindgenUnionField<sfSensorEvent>,
    pub bindgen_union_field: [u32; 6usize],
}
#[test]
fn bindgen_test_layout_sfEvent() {
    assert_eq!(::std::mem::size_of::<sfEvent>() , 24usize , concat ! (
               "Size of: " , stringify ! ( sfEvent ) ));
    assert_eq! (::std::mem::align_of::<sfEvent>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfEvent ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . type_ as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( type_ ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . size as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . key as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( key ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . text as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( text ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . mouseMove as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( mouseMove ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . mouseButton as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( mouseButton ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . mouseWheel as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( mouseWheel ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . mouseWheelScroll as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( mouseWheelScroll ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . joystickMove as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( joystickMove ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . joystickButton as * const _
                as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( joystickButton ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . joystickConnect as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( joystickConnect ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . touch as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( touch ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfEvent ) ) . sensor as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfEvent ) , "::" ,
                stringify ! ( sensor ) ));
}
impl Clone for sfEvent {
    fn clone(&self) -> Self { *self }
}
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: f64,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(::std::mem::size_of::<max_align_t>() , 32usize , concat ! (
               "Size of: " , stringify ! ( max_align_t ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce1 as * const _ as usize } , 0usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce1 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const max_align_t ) ) .
                __clang_max_align_nonce2 as * const _ as usize } , 16usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( max_align_t ) , "::" ,
                stringify ! ( __clang_max_align_nonce2 ) ));
}
impl Clone for max_align_t {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfVideoMode {
    pub width: ::std::os::raw::c_uint,
    pub height: ::std::os::raw::c_uint,
    pub bitsPerPixel: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_sfVideoMode() {
    assert_eq!(::std::mem::size_of::<sfVideoMode>() , 12usize , concat ! (
               "Size of: " , stringify ! ( sfVideoMode ) ));
    assert_eq! (::std::mem::align_of::<sfVideoMode>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfVideoMode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfVideoMode ) ) . width as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfVideoMode ) , "::" ,
                stringify ! ( width ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfVideoMode ) ) . height as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfVideoMode ) , "::" ,
                stringify ! ( height ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfVideoMode ) ) . bitsPerPixel as * const
                _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfVideoMode ) , "::" ,
                stringify ! ( bitsPerPixel ) ));
}
impl Clone for sfVideoMode {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn sfVideoMode_getDesktopMode() -> sfVideoMode;
}
extern "C" {
    pub fn sfVideoMode_getFullscreenModes(count: *mut usize)
     -> *const sfVideoMode;
}
extern "C" {
    pub fn sfVideoMode_isValid(mode: sfVideoMode) -> sfBool;
}
pub type sfWindowHandle = ::std::os::raw::c_ulong;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfWindowStyle {
    sfNone = 0,
    sfTitlebar = 1,
    sfResize = 2,
    sfClose = 4,
    sfFullscreen = 8,
    sfDefaultStyle = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfContextAttribute {
    sfContextDefault = 0,
    sfContextCore = 1,
    sfContextDebug = 4,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfContextSettings {
    pub depthBits: ::std::os::raw::c_uint,
    pub stencilBits: ::std::os::raw::c_uint,
    pub antialiasingLevel: ::std::os::raw::c_uint,
    pub majorVersion: ::std::os::raw::c_uint,
    pub minorVersion: ::std::os::raw::c_uint,
    pub attributeFlags: sfUint32,
    pub sRgbCapable: sfBool,
}
#[test]
fn bindgen_test_layout_sfContextSettings() {
    assert_eq!(::std::mem::size_of::<sfContextSettings>() , 28usize , concat !
               ( "Size of: " , stringify ! ( sfContextSettings ) ));
    assert_eq! (::std::mem::align_of::<sfContextSettings>() , 4usize , concat
                ! ( "Alignment of " , stringify ! ( sfContextSettings ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfContextSettings ) ) . depthBits as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfContextSettings ) ,
                "::" , stringify ! ( depthBits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfContextSettings ) ) . stencilBits as *
                const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfContextSettings ) ,
                "::" , stringify ! ( stencilBits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfContextSettings ) ) . antialiasingLevel
                as * const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfContextSettings ) ,
                "::" , stringify ! ( antialiasingLevel ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfContextSettings ) ) . majorVersion as *
                const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfContextSettings ) ,
                "::" , stringify ! ( majorVersion ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfContextSettings ) ) . minorVersion as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( sfContextSettings ) ,
                "::" , stringify ! ( minorVersion ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfContextSettings ) ) . attributeFlags as
                * const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( sfContextSettings ) ,
                "::" , stringify ! ( attributeFlags ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfContextSettings ) ) . sRgbCapable as *
                const _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( sfContextSettings ) ,
                "::" , stringify ! ( sRgbCapable ) ));
}
impl Clone for sfContextSettings {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn sfWindow_create(mode: sfVideoMode,
                           title: *const ::std::os::raw::c_char,
                           style: sfUint32,
                           settings: *const sfContextSettings)
     -> *mut sfWindow;
}
extern "C" {
    pub fn sfWindow_createUnicode(mode: sfVideoMode, title: *const sfUint32,
                                  style: sfUint32,
                                  settings: *const sfContextSettings)
     -> *mut sfWindow;
}
extern "C" {
    pub fn sfWindow_createFromHandle(handle: sfWindowHandle,
                                     settings: *const sfContextSettings)
     -> *mut sfWindow;
}
extern "C" {
    pub fn sfWindow_destroy(window: *mut sfWindow);
}
extern "C" {
    pub fn sfWindow_close(window: *mut sfWindow);
}
extern "C" {
    pub fn sfWindow_isOpen(window: *const sfWindow) -> sfBool;
}
extern "C" {
    pub fn sfWindow_getSettings(window: *const sfWindow) -> sfContextSettings;
}
extern "C" {
    pub fn sfWindow_pollEvent(window: *mut sfWindow, event: *mut sfEvent)
     -> sfBool;
}
extern "C" {
    pub fn sfWindow_waitEvent(window: *mut sfWindow, event: *mut sfEvent)
     -> sfBool;
}
extern "C" {
    pub fn sfWindow_getPosition(window: *const sfWindow) -> sfVector2i;
}
extern "C" {
    pub fn sfWindow_setPosition(window: *mut sfWindow, position: sfVector2i);
}
extern "C" {
    pub fn sfWindow_getSize(window: *const sfWindow) -> sfVector2u;
}
extern "C" {
    pub fn sfWindow_setSize(window: *mut sfWindow, size: sfVector2u);
}
extern "C" {
    pub fn sfWindow_setTitle(window: *mut sfWindow,
                             title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn sfWindow_setUnicodeTitle(window: *mut sfWindow,
                                    title: *const sfUint32);
}
extern "C" {
    pub fn sfWindow_setIcon(window: *mut sfWindow,
                            width: ::std::os::raw::c_uint,
                            height: ::std::os::raw::c_uint,
                            pixels: *const sfUint8);
}
extern "C" {
    pub fn sfWindow_setVisible(window: *mut sfWindow, visible: sfBool);
}
extern "C" {
    pub fn sfWindow_setVerticalSyncEnabled(window: *mut sfWindow,
                                           enabled: sfBool);
}
extern "C" {
    pub fn sfWindow_setMouseCursorVisible(window: *mut sfWindow,
                                          visible: sfBool);
}
extern "C" {
    pub fn sfWindow_setMouseCursorGrabbed(window: *mut sfWindow,
                                          grabbed: sfBool);
}
extern "C" {
    pub fn sfWindow_setKeyRepeatEnabled(window: *mut sfWindow,
                                        enabled: sfBool);
}
extern "C" {
    pub fn sfWindow_setFramerateLimit(window: *mut sfWindow,
                                      limit: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfWindow_setJoystickThreshold(window: *mut sfWindow,
                                         threshold: f32);
}
extern "C" {
    pub fn sfWindow_setActive(window: *mut sfWindow, active: sfBool)
     -> sfBool;
}
extern "C" {
    pub fn sfWindow_requestFocus(window: *mut sfWindow);
}
extern "C" {
    pub fn sfWindow_hasFocus(window: *const sfWindow) -> sfBool;
}
extern "C" {
    pub fn sfWindow_display(window: *mut sfWindow);
}
extern "C" {
    pub fn sfWindow_getSystemHandle(window: *const sfWindow)
     -> sfWindowHandle;
}
extern "C" {
    pub fn sfContext_create() -> *mut sfContext;
}
extern "C" {
    pub fn sfContext_destroy(context: *mut sfContext);
}
extern "C" {
    pub fn sfContext_setActive(context: *mut sfContext, active: sfBool)
     -> sfBool;
}
extern "C" {
    pub fn sfContext_getSettings(context: *const sfContext)
     -> sfContextSettings;
}
extern "C" {
    pub fn sfTouch_isDown(finger: ::std::os::raw::c_uint) -> sfBool;
}
extern "C" {
    pub fn sfTouch_getPosition(finger: ::std::os::raw::c_uint,
                               relativeTo: *const sfWindow) -> sfVector2i;
}
