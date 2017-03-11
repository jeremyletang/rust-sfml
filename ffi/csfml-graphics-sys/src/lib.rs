#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate csfml_system_sys;
extern crate csfml_window_sys;

use csfml_system_sys::*;
use csfml_window_sys::*;

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

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfBlendFactor {
    sfBlendFactorZero = 0,
    sfBlendFactorOne = 1,
    sfBlendFactorSrcColor = 2,
    sfBlendFactorOneMinusSrcColor = 3,
    sfBlendFactorDstColor = 4,
    sfBlendFactorOneMinusDstColor = 5,
    sfBlendFactorSrcAlpha = 6,
    sfBlendFactorOneMinusSrcAlpha = 7,
    sfBlendFactorDstAlpha = 8,
    sfBlendFactorOneMinusDstAlpha = 9,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfBlendEquation {
    sfBlendEquationAdd = 0,
    sfBlendEquationSubtract = 1,
    sfBlendEquationReverseSubtract = 2,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfBlendMode {
    pub colorSrcFactor: sfBlendFactor,
    pub colorDstFactor: sfBlendFactor,
    pub colorEquation: sfBlendEquation,
    pub alphaSrcFactor: sfBlendFactor,
    pub alphaDstFactor: sfBlendFactor,
    pub alphaEquation: sfBlendEquation,
}
#[test]
fn bindgen_test_layout_sfBlendMode() {
    assert_eq!(::std::mem::size_of::<sfBlendMode>() , 24usize , concat ! (
               "Size of: " , stringify ! ( sfBlendMode ) ));
    assert_eq! (::std::mem::align_of::<sfBlendMode>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfBlendMode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfBlendMode ) ) . colorSrcFactor as *
                const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfBlendMode ) , "::" ,
                stringify ! ( colorSrcFactor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfBlendMode ) ) . colorDstFactor as *
                const _ as usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfBlendMode ) , "::" ,
                stringify ! ( colorDstFactor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfBlendMode ) ) . colorEquation as *
                const _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfBlendMode ) , "::" ,
                stringify ! ( colorEquation ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfBlendMode ) ) . alphaSrcFactor as *
                const _ as usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfBlendMode ) , "::" ,
                stringify ! ( alphaSrcFactor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfBlendMode ) ) . alphaDstFactor as *
                const _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( sfBlendMode ) , "::" ,
                stringify ! ( alphaDstFactor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfBlendMode ) ) . alphaEquation as *
                const _ as usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( sfBlendMode ) , "::" ,
                stringify ! ( alphaEquation ) ));
}
impl Clone for sfBlendMode {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "sfBlendAlpha"]
    pub static sfBlendAlpha: sfBlendMode;
}
extern "C" {
    #[link_name = "sfBlendAdd"]
    pub static sfBlendAdd: sfBlendMode;
}
extern "C" {
    #[link_name = "sfBlendMultiply"]
    pub static sfBlendMultiply: sfBlendMode;
}
extern "C" {
    #[link_name = "sfBlendNone"]
    pub static sfBlendNone: sfBlendMode;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfColor {
    pub r: sfUint8,
    pub g: sfUint8,
    pub b: sfUint8,
    pub a: sfUint8,
}
#[test]
fn bindgen_test_layout_sfColor() {
    assert_eq!(::std::mem::size_of::<sfColor>() , 4usize , concat ! (
               "Size of: " , stringify ! ( sfColor ) ));
    assert_eq! (::std::mem::align_of::<sfColor>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( sfColor ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfColor ) ) . r as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfColor ) , "::" ,
                stringify ! ( r ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfColor ) ) . g as * const _ as usize } ,
                1usize , concat ! (
                "Alignment of field: " , stringify ! ( sfColor ) , "::" ,
                stringify ! ( g ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfColor ) ) . b as * const _ as usize } ,
                2usize , concat ! (
                "Alignment of field: " , stringify ! ( sfColor ) , "::" ,
                stringify ! ( b ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfColor ) ) . a as * const _ as usize } ,
                3usize , concat ! (
                "Alignment of field: " , stringify ! ( sfColor ) , "::" ,
                stringify ! ( a ) ));
}
impl Clone for sfColor {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "sfBlack"]
    pub static mut sfBlack: sfColor;
}
extern "C" {
    #[link_name = "sfWhite"]
    pub static mut sfWhite: sfColor;
}
extern "C" {
    #[link_name = "sfRed"]
    pub static mut sfRed: sfColor;
}
extern "C" {
    #[link_name = "sfGreen"]
    pub static mut sfGreen: sfColor;
}
extern "C" {
    #[link_name = "sfBlue"]
    pub static mut sfBlue: sfColor;
}
extern "C" {
    #[link_name = "sfYellow"]
    pub static mut sfYellow: sfColor;
}
extern "C" {
    #[link_name = "sfMagenta"]
    pub static mut sfMagenta: sfColor;
}
extern "C" {
    #[link_name = "sfCyan"]
    pub static mut sfCyan: sfColor;
}
extern "C" {
    #[link_name = "sfTransparent"]
    pub static mut sfTransparent: sfColor;
}
extern "C" {
    pub fn sfColor_fromRGB(red: sfUint8, green: sfUint8, blue: sfUint8)
     -> sfColor;
}
extern "C" {
    pub fn sfColor_fromRGBA(red: sfUint8, green: sfUint8, blue: sfUint8,
                            alpha: sfUint8) -> sfColor;
}
extern "C" {
    pub fn sfColor_fromInteger(color: sfUint32) -> sfColor;
}
extern "C" {
    pub fn sfColor_toInteger(color: sfColor) -> sfUint32;
}
extern "C" {
    pub fn sfColor_add(color1: sfColor, color2: sfColor) -> sfColor;
}
extern "C" {
    pub fn sfColor_subtract(color1: sfColor, color2: sfColor) -> sfColor;
}
extern "C" {
    pub fn sfColor_modulate(color1: sfColor, color2: sfColor) -> sfColor;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfFloatRect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}
#[test]
fn bindgen_test_layout_sfFloatRect() {
    assert_eq!(::std::mem::size_of::<sfFloatRect>() , 16usize , concat ! (
               "Size of: " , stringify ! ( sfFloatRect ) ));
    assert_eq! (::std::mem::align_of::<sfFloatRect>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfFloatRect ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfFloatRect ) ) . left as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfFloatRect ) , "::" ,
                stringify ! ( left ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfFloatRect ) ) . top as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfFloatRect ) , "::" ,
                stringify ! ( top ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfFloatRect ) ) . width as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfFloatRect ) , "::" ,
                stringify ! ( width ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfFloatRect ) ) . height as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfFloatRect ) , "::" ,
                stringify ! ( height ) ));
}
impl Clone for sfFloatRect {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfIntRect {
    pub left: ::std::os::raw::c_int,
    pub top: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfIntRect() {
    assert_eq!(::std::mem::size_of::<sfIntRect>() , 16usize , concat ! (
               "Size of: " , stringify ! ( sfIntRect ) ));
    assert_eq! (::std::mem::align_of::<sfIntRect>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfIntRect ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfIntRect ) ) . left as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfIntRect ) , "::" ,
                stringify ! ( left ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfIntRect ) ) . top as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfIntRect ) , "::" ,
                stringify ! ( top ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfIntRect ) ) . width as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfIntRect ) , "::" ,
                stringify ! ( width ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfIntRect ) ) . height as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfIntRect ) , "::" ,
                stringify ! ( height ) ));
}
impl Clone for sfIntRect {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn sfFloatRect_contains(rect: *const sfFloatRect, x: f32, y: f32)
     -> sfBool;
}
extern "C" {
    pub fn sfIntRect_contains(rect: *const sfIntRect,
                              x: ::std::os::raw::c_int,
                              y: ::std::os::raw::c_int) -> sfBool;
}
extern "C" {
    pub fn sfFloatRect_intersects(rect1: *const sfFloatRect,
                                  rect2: *const sfFloatRect,
                                  intersection: *mut sfFloatRect) -> sfBool;
}
extern "C" {
    pub fn sfIntRect_intersects(rect1: *const sfIntRect,
                                rect2: *const sfIntRect,
                                intersection: *mut sfIntRect) -> sfBool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfCircleShape([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfConvexShape([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfFont([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfImage([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfShader([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfRectangleShape([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfRenderTexture([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfRenderWindow([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfShape([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfSprite([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfText([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfTexture([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfTransformable([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfVertexArray([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sfView([u8; 0]);
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfTransform {
    pub matrix: [f32; 9usize],
}
#[test]
fn bindgen_test_layout_sfTransform() {
    assert_eq!(::std::mem::size_of::<sfTransform>() , 36usize , concat ! (
               "Size of: " , stringify ! ( sfTransform ) ));
    assert_eq! (::std::mem::align_of::<sfTransform>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfTransform ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfTransform ) ) . matrix as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfTransform ) , "::" ,
                stringify ! ( matrix ) ));
}
impl Clone for sfTransform {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "sfTransform_Identity"]
    pub static sfTransform_Identity: sfTransform;
}
extern "C" {
    pub fn sfTransform_fromMatrix(a00: f32, a01: f32, a02: f32, a10: f32,
                                  a11: f32, a12: f32, a20: f32, a21: f32,
                                  a22: f32) -> sfTransform;
}
extern "C" {
    pub fn sfTransform_getMatrix(transform: *const sfTransform,
                                 matrix: *mut f32);
}
extern "C" {
    pub fn sfTransform_getInverse(transform: *const sfTransform)
     -> sfTransform;
}
extern "C" {
    pub fn sfTransform_transformPoint(transform: *const sfTransform,
                                      point: sfVector2f) -> sfVector2f;
}
extern "C" {
    pub fn sfTransform_transformRect(transform: *const sfTransform,
                                     rectangle: sfFloatRect) -> sfFloatRect;
}
extern "C" {
    pub fn sfTransform_combine(transform: *mut sfTransform,
                               other: *const sfTransform);
}
extern "C" {
    pub fn sfTransform_translate(transform: *mut sfTransform, x: f32, y: f32);
}
extern "C" {
    pub fn sfTransform_rotate(transform: *mut sfTransform, angle: f32);
}
extern "C" {
    pub fn sfTransform_rotateWithCenter(transform: *mut sfTransform,
                                        angle: f32, centerX: f32,
                                        centerY: f32);
}
extern "C" {
    pub fn sfTransform_scale(transform: *mut sfTransform, scaleX: f32,
                             scaleY: f32);
}
extern "C" {
    pub fn sfTransform_scaleWithCenter(transform: *mut sfTransform,
                                       scaleX: f32, scaleY: f32, centerX: f32,
                                       centerY: f32);
}
extern "C" {
    pub fn sfCircleShape_create() -> *mut sfCircleShape;
}
extern "C" {
    pub fn sfCircleShape_copy(shape: *const sfCircleShape)
     -> *mut sfCircleShape;
}
extern "C" {
    pub fn sfCircleShape_destroy(shape: *mut sfCircleShape);
}
extern "C" {
    pub fn sfCircleShape_setPosition(shape: *mut sfCircleShape,
                                     position: sfVector2f);
}
extern "C" {
    pub fn sfCircleShape_setRotation(shape: *mut sfCircleShape, angle: f32);
}
extern "C" {
    pub fn sfCircleShape_setScale(shape: *mut sfCircleShape,
                                  scale: sfVector2f);
}
extern "C" {
    pub fn sfCircleShape_setOrigin(shape: *mut sfCircleShape,
                                   origin: sfVector2f);
}
extern "C" {
    pub fn sfCircleShape_getPosition(shape: *const sfCircleShape)
     -> sfVector2f;
}
extern "C" {
    pub fn sfCircleShape_getRotation(shape: *const sfCircleShape) -> f32;
}
extern "C" {
    pub fn sfCircleShape_getScale(shape: *const sfCircleShape) -> sfVector2f;
}
extern "C" {
    pub fn sfCircleShape_getOrigin(shape: *const sfCircleShape) -> sfVector2f;
}
extern "C" {
    pub fn sfCircleShape_move(shape: *mut sfCircleShape, offset: sfVector2f);
}
extern "C" {
    pub fn sfCircleShape_rotate(shape: *mut sfCircleShape, angle: f32);
}
extern "C" {
    pub fn sfCircleShape_scale(shape: *mut sfCircleShape,
                               factors: sfVector2f);
}
extern "C" {
    pub fn sfCircleShape_getTransform(shape: *const sfCircleShape)
     -> sfTransform;
}
extern "C" {
    pub fn sfCircleShape_getInverseTransform(shape: *const sfCircleShape)
     -> sfTransform;
}
extern "C" {
    pub fn sfCircleShape_setTexture(shape: *mut sfCircleShape,
                                    texture: *const sfTexture,
                                    resetRect: sfBool);
}
extern "C" {
    pub fn sfCircleShape_setTextureRect(shape: *mut sfCircleShape,
                                        rect: sfIntRect);
}
extern "C" {
    pub fn sfCircleShape_setFillColor(shape: *mut sfCircleShape,
                                      color: sfColor);
}
extern "C" {
    pub fn sfCircleShape_setOutlineColor(shape: *mut sfCircleShape,
                                         color: sfColor);
}
extern "C" {
    pub fn sfCircleShape_setOutlineThickness(shape: *mut sfCircleShape,
                                             thickness: f32);
}
extern "C" {
    pub fn sfCircleShape_getTexture(shape: *const sfCircleShape)
     -> *const sfTexture;
}
extern "C" {
    pub fn sfCircleShape_getTextureRect(shape: *const sfCircleShape)
     -> sfIntRect;
}
extern "C" {
    pub fn sfCircleShape_getFillColor(shape: *const sfCircleShape) -> sfColor;
}
extern "C" {
    pub fn sfCircleShape_getOutlineColor(shape: *const sfCircleShape)
     -> sfColor;
}
extern "C" {
    pub fn sfCircleShape_getOutlineThickness(shape: *const sfCircleShape)
     -> f32;
}
extern "C" {
    pub fn sfCircleShape_getPointCount(shape: *const sfCircleShape) -> usize;
}
extern "C" {
    pub fn sfCircleShape_getPoint(shape: *const sfCircleShape, index: usize)
     -> sfVector2f;
}
extern "C" {
    pub fn sfCircleShape_setRadius(shape: *mut sfCircleShape, radius: f32);
}
extern "C" {
    pub fn sfCircleShape_getRadius(shape: *const sfCircleShape) -> f32;
}
extern "C" {
    pub fn sfCircleShape_setPointCount(shape: *mut sfCircleShape,
                                       count: usize);
}
extern "C" {
    pub fn sfCircleShape_getLocalBounds(shape: *const sfCircleShape)
     -> sfFloatRect;
}
extern "C" {
    pub fn sfCircleShape_getGlobalBounds(shape: *const sfCircleShape)
     -> sfFloatRect;
}
extern "C" {
    pub fn sfConvexShape_create() -> *mut sfConvexShape;
}
extern "C" {
    pub fn sfConvexShape_copy(shape: *const sfConvexShape)
     -> *mut sfConvexShape;
}
extern "C" {
    pub fn sfConvexShape_destroy(shape: *mut sfConvexShape);
}
extern "C" {
    pub fn sfConvexShape_setPosition(shape: *mut sfConvexShape,
                                     position: sfVector2f);
}
extern "C" {
    pub fn sfConvexShape_setRotation(shape: *mut sfConvexShape, angle: f32);
}
extern "C" {
    pub fn sfConvexShape_setScale(shape: *mut sfConvexShape,
                                  scale: sfVector2f);
}
extern "C" {
    pub fn sfConvexShape_setOrigin(shape: *mut sfConvexShape,
                                   origin: sfVector2f);
}
extern "C" {
    pub fn sfConvexShape_getPosition(shape: *const sfConvexShape)
     -> sfVector2f;
}
extern "C" {
    pub fn sfConvexShape_getRotation(shape: *const sfConvexShape) -> f32;
}
extern "C" {
    pub fn sfConvexShape_getScale(shape: *const sfConvexShape) -> sfVector2f;
}
extern "C" {
    pub fn sfConvexShape_getOrigin(shape: *const sfConvexShape) -> sfVector2f;
}
extern "C" {
    pub fn sfConvexShape_move(shape: *mut sfConvexShape, offset: sfVector2f);
}
extern "C" {
    pub fn sfConvexShape_rotate(shape: *mut sfConvexShape, angle: f32);
}
extern "C" {
    pub fn sfConvexShape_scale(shape: *mut sfConvexShape,
                               factors: sfVector2f);
}
extern "C" {
    pub fn sfConvexShape_getTransform(shape: *const sfConvexShape)
     -> sfTransform;
}
extern "C" {
    pub fn sfConvexShape_getInverseTransform(shape: *const sfConvexShape)
     -> sfTransform;
}
extern "C" {
    pub fn sfConvexShape_setTexture(shape: *mut sfConvexShape,
                                    texture: *const sfTexture,
                                    resetRect: sfBool);
}
extern "C" {
    pub fn sfConvexShape_setTextureRect(shape: *mut sfConvexShape,
                                        rect: sfIntRect);
}
extern "C" {
    pub fn sfConvexShape_setFillColor(shape: *mut sfConvexShape,
                                      color: sfColor);
}
extern "C" {
    pub fn sfConvexShape_setOutlineColor(shape: *mut sfConvexShape,
                                         color: sfColor);
}
extern "C" {
    pub fn sfConvexShape_setOutlineThickness(shape: *mut sfConvexShape,
                                             thickness: f32);
}
extern "C" {
    pub fn sfConvexShape_getTexture(shape: *const sfConvexShape)
     -> *const sfTexture;
}
extern "C" {
    pub fn sfConvexShape_getTextureRect(shape: *const sfConvexShape)
     -> sfIntRect;
}
extern "C" {
    pub fn sfConvexShape_getFillColor(shape: *const sfConvexShape) -> sfColor;
}
extern "C" {
    pub fn sfConvexShape_getOutlineColor(shape: *const sfConvexShape)
     -> sfColor;
}
extern "C" {
    pub fn sfConvexShape_getOutlineThickness(shape: *const sfConvexShape)
     -> f32;
}
extern "C" {
    pub fn sfConvexShape_getPointCount(shape: *const sfConvexShape) -> usize;
}
extern "C" {
    pub fn sfConvexShape_getPoint(shape: *const sfConvexShape, index: usize)
     -> sfVector2f;
}
extern "C" {
    pub fn sfConvexShape_setPointCount(shape: *mut sfConvexShape,
                                       count: usize);
}
extern "C" {
    pub fn sfConvexShape_setPoint(shape: *mut sfConvexShape, index: usize,
                                  point: sfVector2f);
}
extern "C" {
    pub fn sfConvexShape_getLocalBounds(shape: *const sfConvexShape)
     -> sfFloatRect;
}
extern "C" {
    pub fn sfConvexShape_getGlobalBounds(shape: *const sfConvexShape)
     -> sfFloatRect;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfFontInfo {
    pub family: *const ::std::os::raw::c_schar,
}
#[test]
fn bindgen_test_layout_sfFontInfo() {
    assert_eq!(::std::mem::size_of::<sfFontInfo>() , 8usize , concat ! (
               "Size of: " , stringify ! ( sfFontInfo ) ));
    assert_eq! (::std::mem::align_of::<sfFontInfo>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( sfFontInfo ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfFontInfo ) ) . family as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfFontInfo ) , "::" ,
                stringify ! ( family ) ));
}
impl Clone for sfFontInfo {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlyph {
    pub advance: f32,
    pub bounds: sfFloatRect,
    pub textureRect: sfIntRect,
}
#[test]
fn bindgen_test_layout_sfGlyph() {
    assert_eq!(::std::mem::size_of::<sfGlyph>() , 36usize , concat ! (
               "Size of: " , stringify ! ( sfGlyph ) ));
    assert_eq! (::std::mem::align_of::<sfGlyph>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlyph ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlyph ) ) . advance as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlyph ) , "::" ,
                stringify ! ( advance ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlyph ) ) . bounds as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlyph ) , "::" ,
                stringify ! ( bounds ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlyph ) ) . textureRect as * const _ as
                usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlyph ) , "::" ,
                stringify ! ( textureRect ) ));
}
impl Clone for sfGlyph {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn sfFont_createFromFile(filename: *const ::std::os::raw::c_schar)
     -> *mut sfFont;
}
extern "C" {
    pub fn sfFont_createFromMemory(data: *const ::std::os::raw::c_void,
                                   sizeInBytes: usize) -> *mut sfFont;
}
extern "C" {
    pub fn sfFont_createFromStream(stream: *mut sfInputStream) -> *mut sfFont;
}
extern "C" {
    pub fn sfFont_copy(font: *const sfFont) -> *mut sfFont;
}
extern "C" {
    pub fn sfFont_destroy(font: *mut sfFont);
}
extern "C" {
    pub fn sfFont_getGlyph(font: *const sfFont, codePoint: sfUint32,
                           characterSize: ::std::os::raw::c_uint,
                           bold: sfBool, outlineThickness: f32) -> sfGlyph;
}
extern "C" {
    pub fn sfFont_getKerning(font: *const sfFont, first: sfUint32,
                             second: sfUint32,
                             characterSize: ::std::os::raw::c_uint) -> f32;
}
extern "C" {
    pub fn sfFont_getLineSpacing(font: *const sfFont,
                                 characterSize: ::std::os::raw::c_uint)
     -> f32;
}
extern "C" {
    pub fn sfFont_getUnderlinePosition(font: *const sfFont,
                                       characterSize: ::std::os::raw::c_uint)
     -> f32;
}
extern "C" {
    pub fn sfFont_getUnderlineThickness(font: *const sfFont,
                                        characterSize: ::std::os::raw::c_uint)
     -> f32;
}
extern "C" {
    pub fn sfFont_getTexture(font: *mut sfFont,
                             characterSize: ::std::os::raw::c_uint)
     -> *const sfTexture;
}
extern "C" {
    pub fn sfFont_getInfo(font: *const sfFont) -> sfFontInfo;
}
extern "C" {
    pub fn sfImage_create(width: ::std::os::raw::c_uint,
                          height: ::std::os::raw::c_uint) -> *mut sfImage;
}
extern "C" {
    pub fn sfImage_createFromColor(width: ::std::os::raw::c_uint,
                                   height: ::std::os::raw::c_uint,
                                   color: sfColor) -> *mut sfImage;
}
extern "C" {
    pub fn sfImage_createFromPixels(width: ::std::os::raw::c_uint,
                                    height: ::std::os::raw::c_uint,
                                    pixels: *const sfUint8) -> *mut sfImage;
}
extern "C" {
    pub fn sfImage_createFromFile(filename: *const ::std::os::raw::c_schar)
     -> *mut sfImage;
}
extern "C" {
    pub fn sfImage_createFromMemory(data: *const ::std::os::raw::c_void,
                                    size: usize) -> *mut sfImage;
}
extern "C" {
    pub fn sfImage_createFromStream(stream: *mut sfInputStream)
     -> *mut sfImage;
}
extern "C" {
    pub fn sfImage_copy(image: *const sfImage) -> *mut sfImage;
}
extern "C" {
    pub fn sfImage_destroy(image: *mut sfImage);
}
extern "C" {
    pub fn sfImage_saveToFile(image: *const sfImage,
                              filename: *const ::std::os::raw::c_schar)
     -> sfBool;
}
extern "C" {
    pub fn sfImage_getSize(image: *const sfImage) -> sfVector2u;
}
extern "C" {
    pub fn sfImage_createMaskFromColor(image: *mut sfImage, color: sfColor,
                                       alpha: sfUint8);
}
extern "C" {
    pub fn sfImage_copyImage(image: *mut sfImage, source: *const sfImage,
                             destX: ::std::os::raw::c_uint,
                             destY: ::std::os::raw::c_uint,
                             sourceRect: sfIntRect, applyAlpha: sfBool);
}
extern "C" {
    pub fn sfImage_setPixel(image: *mut sfImage, x: ::std::os::raw::c_uint,
                            y: ::std::os::raw::c_uint, color: sfColor);
}
extern "C" {
    pub fn sfImage_getPixel(image: *const sfImage, x: ::std::os::raw::c_uint,
                            y: ::std::os::raw::c_uint) -> sfColor;
}
extern "C" {
    pub fn sfImage_getPixelsPtr(image: *const sfImage) -> *const sfUint8;
}
extern "C" {
    pub fn sfImage_flipHorizontally(image: *mut sfImage);
}
extern "C" {
    pub fn sfImage_flipVertically(image: *mut sfImage);
}
pub const sfPrimitiveType_sfLinesStrip: sfPrimitiveType =
    sfPrimitiveType::sfLineStrip;
pub const sfPrimitiveType_sfTrianglesStrip: sfPrimitiveType =
    sfPrimitiveType::sfTriangleStrip;
pub const sfPrimitiveType_sfTrianglesFan: sfPrimitiveType =
    sfPrimitiveType::sfTriangleFan;
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfPrimitiveType {
    sfPoints = 0,
    sfLines = 1,
    sfLineStrip = 2,
    sfTriangles = 3,
    sfTriangleStrip = 4,
    sfTriangleFan = 5,
    sfQuads = 6,
}
extern "C" {
    pub fn sfRectangleShape_create() -> *mut sfRectangleShape;
}
extern "C" {
    pub fn sfRectangleShape_copy(shape: *const sfRectangleShape)
     -> *mut sfRectangleShape;
}
extern "C" {
    pub fn sfRectangleShape_destroy(shape: *mut sfRectangleShape);
}
extern "C" {
    pub fn sfRectangleShape_setPosition(shape: *mut sfRectangleShape,
                                        position: sfVector2f);
}
extern "C" {
    pub fn sfRectangleShape_setRotation(shape: *mut sfRectangleShape,
                                        angle: f32);
}
extern "C" {
    pub fn sfRectangleShape_setScale(shape: *mut sfRectangleShape,
                                     scale: sfVector2f);
}
extern "C" {
    pub fn sfRectangleShape_setOrigin(shape: *mut sfRectangleShape,
                                      origin: sfVector2f);
}
extern "C" {
    pub fn sfRectangleShape_getPosition(shape: *const sfRectangleShape)
     -> sfVector2f;
}
extern "C" {
    pub fn sfRectangleShape_getRotation(shape: *const sfRectangleShape)
     -> f32;
}
extern "C" {
    pub fn sfRectangleShape_getScale(shape: *const sfRectangleShape)
     -> sfVector2f;
}
extern "C" {
    pub fn sfRectangleShape_getOrigin(shape: *const sfRectangleShape)
     -> sfVector2f;
}
extern "C" {
    pub fn sfRectangleShape_move(shape: *mut sfRectangleShape,
                                 offset: sfVector2f);
}
extern "C" {
    pub fn sfRectangleShape_rotate(shape: *mut sfRectangleShape, angle: f32);
}
extern "C" {
    pub fn sfRectangleShape_scale(shape: *mut sfRectangleShape,
                                  factors: sfVector2f);
}
extern "C" {
    pub fn sfRectangleShape_getTransform(shape: *const sfRectangleShape)
     -> sfTransform;
}
extern "C" {
    pub fn sfRectangleShape_getInverseTransform(shape:
                                                    *const sfRectangleShape)
     -> sfTransform;
}
extern "C" {
    pub fn sfRectangleShape_setTexture(shape: *mut sfRectangleShape,
                                       texture: *const sfTexture,
                                       resetRect: sfBool);
}
extern "C" {
    pub fn sfRectangleShape_setTextureRect(shape: *mut sfRectangleShape,
                                           rect: sfIntRect);
}
extern "C" {
    pub fn sfRectangleShape_setFillColor(shape: *mut sfRectangleShape,
                                         color: sfColor);
}
extern "C" {
    pub fn sfRectangleShape_setOutlineColor(shape: *mut sfRectangleShape,
                                            color: sfColor);
}
extern "C" {
    pub fn sfRectangleShape_setOutlineThickness(shape: *mut sfRectangleShape,
                                                thickness: f32);
}
extern "C" {
    pub fn sfRectangleShape_getTexture(shape: *const sfRectangleShape)
     -> *const sfTexture;
}
extern "C" {
    pub fn sfRectangleShape_getTextureRect(shape: *const sfRectangleShape)
     -> sfIntRect;
}
extern "C" {
    pub fn sfRectangleShape_getFillColor(shape: *const sfRectangleShape)
     -> sfColor;
}
extern "C" {
    pub fn sfRectangleShape_getOutlineColor(shape: *const sfRectangleShape)
     -> sfColor;
}
extern "C" {
    pub fn sfRectangleShape_getOutlineThickness(shape:
                                                    *const sfRectangleShape)
     -> f32;
}
extern "C" {
    pub fn sfRectangleShape_getPointCount(shape: *const sfRectangleShape)
     -> usize;
}
extern "C" {
    pub fn sfRectangleShape_getPoint(shape: *const sfRectangleShape,
                                     index: usize) -> sfVector2f;
}
extern "C" {
    pub fn sfRectangleShape_setSize(shape: *mut sfRectangleShape,
                                    size: sfVector2f);
}
extern "C" {
    pub fn sfRectangleShape_getSize(shape: *const sfRectangleShape)
     -> sfVector2f;
}
extern "C" {
    pub fn sfRectangleShape_getLocalBounds(shape: *const sfRectangleShape)
     -> sfFloatRect;
}
extern "C" {
    pub fn sfRectangleShape_getGlobalBounds(shape: *const sfRectangleShape)
     -> sfFloatRect;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfRenderStates {
    pub blendMode: sfBlendMode,
    pub transform: sfTransform,
    pub texture: *const sfTexture,
    pub shader: *const sfShader,
}
#[test]
fn bindgen_test_layout_sfRenderStates() {
    assert_eq!(::std::mem::size_of::<sfRenderStates>() , 80usize , concat ! (
               "Size of: " , stringify ! ( sfRenderStates ) ));
    assert_eq! (::std::mem::align_of::<sfRenderStates>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( sfRenderStates ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfRenderStates ) ) . blendMode as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfRenderStates ) , "::"
                , stringify ! ( blendMode ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfRenderStates ) ) . transform as * const
                _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( sfRenderStates ) , "::"
                , stringify ! ( transform ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfRenderStates ) ) . texture as * const _
                as usize } , 64usize , concat ! (
                "Alignment of field: " , stringify ! ( sfRenderStates ) , "::"
                , stringify ! ( texture ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfRenderStates ) ) . shader as * const _
                as usize } , 72usize , concat ! (
                "Alignment of field: " , stringify ! ( sfRenderStates ) , "::"
                , stringify ! ( shader ) ));
}
impl Clone for sfRenderStates {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfVertex {
    pub position: sfVector2f,
    pub color: sfColor,
    pub texCoords: sfVector2f,
}
#[test]
fn bindgen_test_layout_sfVertex() {
    assert_eq!(::std::mem::size_of::<sfVertex>() , 20usize , concat ! (
               "Size of: " , stringify ! ( sfVertex ) ));
    assert_eq! (::std::mem::align_of::<sfVertex>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfVertex ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfVertex ) ) . position as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfVertex ) , "::" ,
                stringify ! ( position ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfVertex ) ) . color as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfVertex ) , "::" ,
                stringify ! ( color ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfVertex ) ) . texCoords as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfVertex ) , "::" ,
                stringify ! ( texCoords ) ));
}
impl Clone for sfVertex {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn sfRenderTexture_create(width: ::std::os::raw::c_uint,
                                  height: ::std::os::raw::c_uint,
                                  depthBuffer: sfBool)
     -> *mut sfRenderTexture;
}
extern "C" {
    pub fn sfRenderTexture_destroy(renderTexture: *mut sfRenderTexture);
}
extern "C" {
    pub fn sfRenderTexture_getSize(renderTexture: *const sfRenderTexture)
     -> sfVector2u;
}
extern "C" {
    pub fn sfRenderTexture_setActive(renderTexture: *mut sfRenderTexture,
                                     active: sfBool) -> sfBool;
}
extern "C" {
    pub fn sfRenderTexture_display(renderTexture: *mut sfRenderTexture);
}
extern "C" {
    pub fn sfRenderTexture_clear(renderTexture: *mut sfRenderTexture,
                                 color: sfColor);
}
extern "C" {
    pub fn sfRenderTexture_setView(renderTexture: *mut sfRenderTexture,
                                   view: *const sfView);
}
extern "C" {
    pub fn sfRenderTexture_getView(renderTexture: *const sfRenderTexture)
     -> *const sfView;
}
extern "C" {
    pub fn sfRenderTexture_getDefaultView(renderTexture:
                                              *const sfRenderTexture)
     -> *const sfView;
}
extern "C" {
    pub fn sfRenderTexture_getViewport(renderTexture: *const sfRenderTexture,
                                       view: *const sfView) -> sfIntRect;
}
extern "C" {
    pub fn sfRenderTexture_mapPixelToCoords(renderTexture:
                                                *const sfRenderTexture,
                                            point: sfVector2i,
                                            view: *const sfView)
     -> sfVector2f;
}
extern "C" {
    pub fn sfRenderTexture_mapCoordsToPixel(renderTexture:
                                                *const sfRenderTexture,
                                            point: sfVector2f,
                                            view: *const sfView)
     -> sfVector2i;
}
extern "C" {
    pub fn sfRenderTexture_drawSprite(renderTexture: *mut sfRenderTexture,
                                      object: *const sfSprite,
                                      states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderTexture_drawText(renderTexture: *mut sfRenderTexture,
                                    object: *const sfText,
                                    states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderTexture_drawShape(renderTexture: *mut sfRenderTexture,
                                     object: *const sfShape,
                                     states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderTexture_drawCircleShape(renderTexture:
                                               *mut sfRenderTexture,
                                           object: *const sfCircleShape,
                                           states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderTexture_drawConvexShape(renderTexture:
                                               *mut sfRenderTexture,
                                           object: *const sfConvexShape,
                                           states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderTexture_drawRectangleShape(renderTexture:
                                                  *mut sfRenderTexture,
                                              object: *const sfRectangleShape,
                                              states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderTexture_drawVertexArray(renderTexture:
                                               *mut sfRenderTexture,
                                           object: *const sfVertexArray,
                                           states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderTexture_drawPrimitives(renderTexture: *mut sfRenderTexture,
                                          vertices: *const sfVertex,
                                          vertexCount: usize,
                                          type_: sfPrimitiveType,
                                          states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderTexture_pushGLStates(renderTexture: *mut sfRenderTexture);
}
extern "C" {
    pub fn sfRenderTexture_popGLStates(renderTexture: *mut sfRenderTexture);
}
extern "C" {
    pub fn sfRenderTexture_resetGLStates(renderTexture: *mut sfRenderTexture);
}
extern "C" {
    pub fn sfRenderTexture_getTexture(renderTexture: *const sfRenderTexture)
     -> *const sfTexture;
}
extern "C" {
    pub fn sfRenderTexture_setSmooth(renderTexture: *mut sfRenderTexture,
                                     smooth: sfBool);
}
extern "C" {
    pub fn sfRenderTexture_isSmooth(renderTexture: *const sfRenderTexture)
     -> sfBool;
}
extern "C" {
    pub fn sfRenderTexture_setRepeated(renderTexture: *mut sfRenderTexture,
                                       repeated: sfBool);
}
extern "C" {
    pub fn sfRenderTexture_isRepeated(renderTexture: *const sfRenderTexture)
     -> sfBool;
}
extern "C" {
    pub fn sfRenderTexture_generateMipmap(renderTexture: *mut sfRenderTexture)
     -> sfBool;
}
extern "C" {
    pub fn sfRenderWindow_create(mode: sfVideoMode,
                                 title: *const ::std::os::raw::c_schar,
                                 style: sfUint32,
                                 settings: *const sfContextSettings)
     -> *mut sfRenderWindow;
}
extern "C" {
    pub fn sfRenderWindow_createUnicode(mode: sfVideoMode,
                                        title: *const sfUint32,
                                        style: sfUint32,
                                        settings: *const sfContextSettings)
     -> *mut sfRenderWindow;
}
extern "C" {
    pub fn sfRenderWindow_createFromHandle(handle: sfWindowHandle,
                                           settings: *const sfContextSettings)
     -> *mut sfRenderWindow;
}
extern "C" {
    pub fn sfRenderWindow_destroy(renderWindow: *mut sfRenderWindow);
}
extern "C" {
    pub fn sfRenderWindow_close(renderWindow: *mut sfRenderWindow);
}
extern "C" {
    pub fn sfRenderWindow_isOpen(renderWindow: *const sfRenderWindow)
     -> sfBool;
}
extern "C" {
    pub fn sfRenderWindow_getSettings(renderWindow: *const sfRenderWindow)
     -> sfContextSettings;
}
extern "C" {
    pub fn sfRenderWindow_pollEvent(renderWindow: *mut sfRenderWindow,
                                    event: *mut sfEvent) -> sfBool;
}
extern "C" {
    pub fn sfRenderWindow_waitEvent(renderWindow: *mut sfRenderWindow,
                                    event: *mut sfEvent) -> sfBool;
}
extern "C" {
    pub fn sfRenderWindow_getPosition(renderWindow: *const sfRenderWindow)
     -> sfVector2i;
}
extern "C" {
    pub fn sfRenderWindow_setPosition(renderWindow: *mut sfRenderWindow,
                                      position: sfVector2i);
}
extern "C" {
    pub fn sfRenderWindow_getSize(renderWindow: *const sfRenderWindow)
     -> sfVector2u;
}
extern "C" {
    pub fn sfRenderWindow_setSize(renderWindow: *mut sfRenderWindow,
                                  size: sfVector2u);
}
extern "C" {
    pub fn sfRenderWindow_setTitle(renderWindow: *mut sfRenderWindow,
                                   title: *const ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfRenderWindow_setUnicodeTitle(renderWindow: *mut sfRenderWindow,
                                          title: *const sfUint32);
}
extern "C" {
    pub fn sfRenderWindow_setIcon(renderWindow: *mut sfRenderWindow,
                                  width: ::std::os::raw::c_uint,
                                  height: ::std::os::raw::c_uint,
                                  pixels: *const sfUint8);
}
extern "C" {
    pub fn sfRenderWindow_setVisible(renderWindow: *mut sfRenderWindow,
                                     visible: sfBool);
}
extern "C" {
    pub fn sfRenderWindow_setVerticalSyncEnabled(renderWindow:
                                                     *mut sfRenderWindow,
                                                 enabled: sfBool);
}
extern "C" {
    pub fn sfRenderWindow_setMouseCursorVisible(renderWindow:
                                                    *mut sfRenderWindow,
                                                show: sfBool);
}
extern "C" {
    pub fn sfRenderWindow_setMouseCursorGrabbed(renderWindow:
                                                    *mut sfRenderWindow,
                                                grabbed: sfBool);
}
extern "C" {
    pub fn sfRenderWindow_setKeyRepeatEnabled(renderWindow:
                                                  *mut sfRenderWindow,
                                              enabled: sfBool);
}
extern "C" {
    pub fn sfRenderWindow_setFramerateLimit(renderWindow: *mut sfRenderWindow,
                                            limit: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfRenderWindow_setJoystickThreshold(renderWindow:
                                                   *mut sfRenderWindow,
                                               threshold: f32);
}
extern "C" {
    pub fn sfRenderWindow_setActive(renderWindow: *mut sfRenderWindow,
                                    active: sfBool) -> sfBool;
}
extern "C" {
    pub fn sfRenderWindow_requestFocus(renderWindow: *mut sfRenderWindow);
}
extern "C" {
    pub fn sfRenderWindow_hasFocus(renderWindow: *const sfRenderWindow)
     -> sfBool;
}
extern "C" {
    pub fn sfRenderWindow_display(renderWindow: *mut sfRenderWindow);
}
extern "C" {
    pub fn sfRenderWindow_getSystemHandle(renderWindow: *const sfRenderWindow)
     -> sfWindowHandle;
}
extern "C" {
    pub fn sfRenderWindow_clear(renderWindow: *mut sfRenderWindow,
                                color: sfColor);
}
extern "C" {
    pub fn sfRenderWindow_setView(renderWindow: *mut sfRenderWindow,
                                  view: *const sfView);
}
extern "C" {
    pub fn sfRenderWindow_getView(renderWindow: *const sfRenderWindow)
     -> *const sfView;
}
extern "C" {
    pub fn sfRenderWindow_getDefaultView(renderWindow: *const sfRenderWindow)
     -> *const sfView;
}
extern "C" {
    pub fn sfRenderWindow_getViewport(renderWindow: *const sfRenderWindow,
                                      view: *const sfView) -> sfIntRect;
}
extern "C" {
    pub fn sfRenderWindow_mapPixelToCoords(renderWindow:
                                               *const sfRenderWindow,
                                           point: sfVector2i,
                                           view: *const sfView) -> sfVector2f;
}
extern "C" {
    pub fn sfRenderWindow_mapCoordsToPixel(renderWindow:
                                               *const sfRenderWindow,
                                           point: sfVector2f,
                                           view: *const sfView) -> sfVector2i;
}
extern "C" {
    pub fn sfRenderWindow_drawSprite(renderWindow: *mut sfRenderWindow,
                                     object: *const sfSprite,
                                     states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderWindow_drawText(renderWindow: *mut sfRenderWindow,
                                   object: *const sfText,
                                   states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderWindow_drawShape(renderWindow: *mut sfRenderWindow,
                                    object: *const sfShape,
                                    states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderWindow_drawCircleShape(renderWindow: *mut sfRenderWindow,
                                          object: *const sfCircleShape,
                                          states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderWindow_drawConvexShape(renderWindow: *mut sfRenderWindow,
                                          object: *const sfConvexShape,
                                          states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderWindow_drawRectangleShape(renderWindow:
                                                 *mut sfRenderWindow,
                                             object: *const sfRectangleShape,
                                             states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderWindow_drawVertexArray(renderWindow: *mut sfRenderWindow,
                                          object: *const sfVertexArray,
                                          states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderWindow_drawPrimitives(renderWindow: *mut sfRenderWindow,
                                         vertices: *const sfVertex,
                                         vertexCount: usize,
                                         type_: sfPrimitiveType,
                                         states: *const sfRenderStates);
}
extern "C" {
    pub fn sfRenderWindow_pushGLStates(renderWindow: *mut sfRenderWindow);
}
extern "C" {
    pub fn sfRenderWindow_popGLStates(renderWindow: *mut sfRenderWindow);
}
extern "C" {
    pub fn sfRenderWindow_resetGLStates(renderWindow: *mut sfRenderWindow);
}
extern "C" {
    pub fn sfRenderWindow_capture(renderWindow: *const sfRenderWindow)
     -> *mut sfImage;
}
extern "C" {
    pub fn sfMouse_getPositionRenderWindow(relativeTo: *const sfRenderWindow)
     -> sfVector2i;
}
extern "C" {
    pub fn sfMouse_setPositionRenderWindow(position: sfVector2i,
                                           relativeTo: *const sfRenderWindow);
}
extern "C" {
    pub fn sfTouch_getPositionRenderWindow(finger: ::std::os::raw::c_uint,
                                           relativeTo: *const sfRenderWindow)
     -> sfVector2i;
}
pub type sfGlslVec2 = sfVector2f;
pub type sfGlslIvec2 = sfVector2i;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlslBvec2 {
    pub x: sfBool,
    pub y: sfBool,
}
#[test]
fn bindgen_test_layout_sfGlslBvec2() {
    assert_eq!(::std::mem::size_of::<sfGlslBvec2>() , 8usize , concat ! (
               "Size of: " , stringify ! ( sfGlslBvec2 ) ));
    assert_eq! (::std::mem::align_of::<sfGlslBvec2>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlslBvec2 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec2 ) ) . x as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec2 ) , "::" ,
                stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec2 ) ) . y as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec2 ) , "::" ,
                stringify ! ( y ) ));
}
impl Clone for sfGlslBvec2 {
    fn clone(&self) -> Self { *self }
}
pub type sfGlslVec3 = sfVector3f;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlslIvec3 {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub z: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfGlslIvec3() {
    assert_eq!(::std::mem::size_of::<sfGlslIvec3>() , 12usize , concat ! (
               "Size of: " , stringify ! ( sfGlslIvec3 ) ));
    assert_eq! (::std::mem::align_of::<sfGlslIvec3>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlslIvec3 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslIvec3 ) ) . x as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslIvec3 ) , "::" ,
                stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslIvec3 ) ) . y as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslIvec3 ) , "::" ,
                stringify ! ( y ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslIvec3 ) ) . z as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslIvec3 ) , "::" ,
                stringify ! ( z ) ));
}
impl Clone for sfGlslIvec3 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlslBvec3 {
    pub x: sfBool,
    pub y: sfBool,
    pub z: sfBool,
}
#[test]
fn bindgen_test_layout_sfGlslBvec3() {
    assert_eq!(::std::mem::size_of::<sfGlslBvec3>() , 12usize , concat ! (
               "Size of: " , stringify ! ( sfGlslBvec3 ) ));
    assert_eq! (::std::mem::align_of::<sfGlslBvec3>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlslBvec3 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec3 ) ) . x as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec3 ) , "::" ,
                stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec3 ) ) . y as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec3 ) , "::" ,
                stringify ! ( y ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec3 ) ) . z as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec3 ) , "::" ,
                stringify ! ( z ) ));
}
impl Clone for sfGlslBvec3 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlslVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[test]
fn bindgen_test_layout_sfGlslVec4() {
    assert_eq!(::std::mem::size_of::<sfGlslVec4>() , 16usize , concat ! (
               "Size of: " , stringify ! ( sfGlslVec4 ) ));
    assert_eq! (::std::mem::align_of::<sfGlslVec4>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlslVec4 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslVec4 ) ) . x as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslVec4 ) , "::" ,
                stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslVec4 ) ) . y as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslVec4 ) , "::" ,
                stringify ! ( y ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslVec4 ) ) . z as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslVec4 ) , "::" ,
                stringify ! ( z ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslVec4 ) ) . w as * const _ as usize
                } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslVec4 ) , "::" ,
                stringify ! ( w ) ));
}
impl Clone for sfGlslVec4 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlslIvec4 {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub z: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sfGlslIvec4() {
    assert_eq!(::std::mem::size_of::<sfGlslIvec4>() , 16usize , concat ! (
               "Size of: " , stringify ! ( sfGlslIvec4 ) ));
    assert_eq! (::std::mem::align_of::<sfGlslIvec4>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlslIvec4 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslIvec4 ) ) . x as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslIvec4 ) , "::" ,
                stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslIvec4 ) ) . y as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslIvec4 ) , "::" ,
                stringify ! ( y ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslIvec4 ) ) . z as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslIvec4 ) , "::" ,
                stringify ! ( z ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslIvec4 ) ) . w as * const _ as usize
                } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslIvec4 ) , "::" ,
                stringify ! ( w ) ));
}
impl Clone for sfGlslIvec4 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlslBvec4 {
    pub x: sfBool,
    pub y: sfBool,
    pub z: sfBool,
    pub w: sfBool,
}
#[test]
fn bindgen_test_layout_sfGlslBvec4() {
    assert_eq!(::std::mem::size_of::<sfGlslBvec4>() , 16usize , concat ! (
               "Size of: " , stringify ! ( sfGlslBvec4 ) ));
    assert_eq! (::std::mem::align_of::<sfGlslBvec4>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlslBvec4 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec4 ) ) . x as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec4 ) , "::" ,
                stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec4 ) ) . y as * const _ as usize
                } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec4 ) , "::" ,
                stringify ! ( y ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec4 ) ) . z as * const _ as usize
                } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec4 ) , "::" ,
                stringify ! ( z ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslBvec4 ) ) . w as * const _ as usize
                } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslBvec4 ) , "::" ,
                stringify ! ( w ) ));
}
impl Clone for sfGlslBvec4 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlslMat3 {
    pub array: [f32; 9usize],
}
#[test]
fn bindgen_test_layout_sfGlslMat3() {
    assert_eq!(::std::mem::size_of::<sfGlslMat3>() , 36usize , concat ! (
               "Size of: " , stringify ! ( sfGlslMat3 ) ));
    assert_eq! (::std::mem::align_of::<sfGlslMat3>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlslMat3 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslMat3 ) ) . array as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslMat3 ) , "::" ,
                stringify ! ( array ) ));
}
impl Clone for sfGlslMat3 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct sfGlslMat4 {
    pub array: [f32; 16usize],
}
#[test]
fn bindgen_test_layout_sfGlslMat4() {
    assert_eq!(::std::mem::size_of::<sfGlslMat4>() , 64usize , concat ! (
               "Size of: " , stringify ! ( sfGlslMat4 ) ));
    assert_eq! (::std::mem::align_of::<sfGlslMat4>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( sfGlslMat4 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const sfGlslMat4 ) ) . array as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( sfGlslMat4 ) , "::" ,
                stringify ! ( array ) ));
}
impl Clone for sfGlslMat4 {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    pub fn sfShader_createFromFile(vertexShaderFilename:
                                       *const ::std::os::raw::c_schar,
                                   geometryShaderFilename:
                                       *const ::std::os::raw::c_schar,
                                   fragmentShaderFilename:
                                       *const ::std::os::raw::c_schar)
     -> *mut sfShader;
}
extern "C" {
    pub fn sfShader_createFromMemory(vertexShader:
                                         *const ::std::os::raw::c_schar,
                                     geometryShader:
                                         *const ::std::os::raw::c_schar,
                                     fragmentShader:
                                         *const ::std::os::raw::c_schar)
     -> *mut sfShader;
}
extern "C" {
    pub fn sfShader_createFromStream(vertexShaderStream: *mut sfInputStream,
                                     geometryShaderStream: *mut sfInputStream,
                                     fragmentShaderStream: *mut sfInputStream)
     -> *mut sfShader;
}
extern "C" {
    pub fn sfShader_destroy(shader: *mut sfShader);
}
extern "C" {
    pub fn sfShader_setFloatUniform(shader: *mut sfShader,
                                    name: *const ::std::os::raw::c_schar,
                                    x: f32);
}
extern "C" {
    pub fn sfShader_setVec2Uniform(shader: *mut sfShader,
                                   name: *const ::std::os::raw::c_schar,
                                   vector: sfGlslVec2);
}
extern "C" {
    pub fn sfShader_setVec3Uniform(shader: *mut sfShader,
                                   name: *const ::std::os::raw::c_schar,
                                   vector: sfGlslVec3);
}
extern "C" {
    pub fn sfShader_setVec4Uniform(shader: *mut sfShader,
                                   name: *const ::std::os::raw::c_schar,
                                   vector: sfGlslVec4);
}
extern "C" {
    pub fn sfShader_setColorUniform(shader: *mut sfShader,
                                    name: *const ::std::os::raw::c_schar,
                                    color: sfColor);
}
extern "C" {
    pub fn sfShader_setIntUniform(shader: *mut sfShader,
                                  name: *const ::std::os::raw::c_schar,
                                  x: ::std::os::raw::c_int);
}
extern "C" {
    pub fn sfShader_setIvec2Uniform(shader: *mut sfShader,
                                    name: *const ::std::os::raw::c_schar,
                                    vector: sfGlslIvec2);
}
extern "C" {
    pub fn sfShader_setIvec3Uniform(shader: *mut sfShader,
                                    name: *const ::std::os::raw::c_schar,
                                    vector: sfGlslIvec3);
}
extern "C" {
    pub fn sfShader_setIvec4Uniform(shader: *mut sfShader,
                                    name: *const ::std::os::raw::c_schar,
                                    vector: sfGlslIvec4);
}
extern "C" {
    pub fn sfShader_setIntColorUniform(shader: *mut sfShader,
                                       name: *const ::std::os::raw::c_schar,
                                       color: sfColor);
}
extern "C" {
    pub fn sfShader_setBoolUniform(shader: *mut sfShader,
                                   name: *const ::std::os::raw::c_schar,
                                   x: sfBool);
}
extern "C" {
    pub fn sfShader_setBvec2Uniform(shader: *mut sfShader,
                                    name: *const ::std::os::raw::c_schar,
                                    vector: sfGlslBvec2);
}
extern "C" {
    pub fn sfShader_setBvec3Uniform(shader: *mut sfShader,
                                    name: *const ::std::os::raw::c_schar,
                                    vector: sfGlslBvec3);
}
extern "C" {
    pub fn sfShader_setBvec4Uniform(shader: *mut sfShader,
                                    name: *const ::std::os::raw::c_schar,
                                    vector: sfGlslBvec4);
}
extern "C" {
    pub fn sfShader_setMat3Uniform(shader: *mut sfShader,
                                   name: *const ::std::os::raw::c_schar,
                                   matrix: *const sfGlslMat3);
}
extern "C" {
    pub fn sfShader_setMat4Uniform(shader: *mut sfShader,
                                   name: *const ::std::os::raw::c_schar,
                                   matrix: *const sfGlslMat4);
}
extern "C" {
    pub fn sfShader_setTextureUniform(shader: *mut sfShader,
                                      name: *const ::std::os::raw::c_schar,
                                      texture: *const sfTexture);
}
extern "C" {
    pub fn sfShader_setCurrentTextureUniform(shader: *mut sfShader,
                                             name:
                                                 *const ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfShader_setFloatUniformArray(shader: *mut sfShader,
                                         name: *const ::std::os::raw::c_schar,
                                         scalarArray: *const f32,
                                         length: usize);
}
extern "C" {
    pub fn sfShader_setVec2UniformArray(shader: *mut sfShader,
                                        name: *const ::std::os::raw::c_schar,
                                        vectorArray: *const sfGlslVec2,
                                        length: usize);
}
extern "C" {
    pub fn sfShader_setVec3UniformArray(shader: *mut sfShader,
                                        name: *const ::std::os::raw::c_schar,
                                        vectorArray: *const sfGlslVec3,
                                        length: usize);
}
extern "C" {
    pub fn sfShader_setVec4UniformArray(shader: *mut sfShader,
                                        name: *const ::std::os::raw::c_schar,
                                        vectorArray: *const sfGlslVec4,
                                        length: usize);
}
extern "C" {
    pub fn sfShader_setMat3UniformArray(shader: *mut sfShader,
                                        name: *const ::std::os::raw::c_schar,
                                        matrixArray: *const sfGlslMat3,
                                        length: usize);
}
extern "C" {
    pub fn sfShader_setMat4UniformArray(shader: *mut sfShader,
                                        name: *const ::std::os::raw::c_schar,
                                        matrixArray: *const sfGlslMat4,
                                        length: usize);
}
extern "C" {
    pub fn sfShader_setFloatParameter(shader: *mut sfShader,
                                      name: *const ::std::os::raw::c_schar,
                                      x: f32);
}
extern "C" {
    pub fn sfShader_setFloat2Parameter(shader: *mut sfShader,
                                       name: *const ::std::os::raw::c_schar,
                                       x: f32, y: f32);
}
extern "C" {
    pub fn sfShader_setFloat3Parameter(shader: *mut sfShader,
                                       name: *const ::std::os::raw::c_schar,
                                       x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn sfShader_setFloat4Parameter(shader: *mut sfShader,
                                       name: *const ::std::os::raw::c_schar,
                                       x: f32, y: f32, z: f32, w: f32);
}
extern "C" {
    pub fn sfShader_setVector2Parameter(shader: *mut sfShader,
                                        name: *const ::std::os::raw::c_schar,
                                        vector: sfVector2f);
}
extern "C" {
    pub fn sfShader_setVector3Parameter(shader: *mut sfShader,
                                        name: *const ::std::os::raw::c_schar,
                                        vector: sfVector3f);
}
extern "C" {
    pub fn sfShader_setColorParameter(shader: *mut sfShader,
                                      name: *const ::std::os::raw::c_schar,
                                      color: sfColor);
}
extern "C" {
    pub fn sfShader_setTransformParameter(shader: *mut sfShader,
                                          name:
                                              *const ::std::os::raw::c_schar,
                                          transform: sfTransform);
}
extern "C" {
    pub fn sfShader_setTextureParameter(shader: *mut sfShader,
                                        name: *const ::std::os::raw::c_schar,
                                        texture: *const sfTexture);
}
extern "C" {
    pub fn sfShader_setCurrentTextureParameter(shader: *mut sfShader,
                                               name:
                                                   *const ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfShader_getNativeHandle(shader: *const sfShader)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfShader_bind(shader: *const sfShader);
}
extern "C" {
    pub fn sfShader_isAvailable() -> sfBool;
}
extern "C" {
    pub fn sfShader_isGeometryAvailable() -> sfBool;
}
pub type sfShapeGetPointCountCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *mut ::std::os::raw::c_void)
                              -> usize>;
pub type sfShapeGetPointCallback =
    ::std::option::Option<unsafe extern "C" fn(arg1: usize,
                                               arg2:
                                                   *mut ::std::os::raw::c_void)
                              -> sfVector2f>;
extern "C" {
    pub fn sfShape_create(getPointCount: sfShapeGetPointCountCallback,
                          getPoint: sfShapeGetPointCallback,
                          userData: *mut ::std::os::raw::c_void)
     -> *mut sfShape;
}
extern "C" {
    pub fn sfShape_destroy(shape: *mut sfShape);
}
extern "C" {
    pub fn sfShape_setPosition(shape: *mut sfShape, position: sfVector2f);
}
extern "C" {
    pub fn sfShape_setRotation(shape: *mut sfShape, angle: f32);
}
extern "C" {
    pub fn sfShape_setScale(shape: *mut sfShape, scale: sfVector2f);
}
extern "C" {
    pub fn sfShape_setOrigin(shape: *mut sfShape, origin: sfVector2f);
}
extern "C" {
    pub fn sfShape_getPosition(shape: *const sfShape) -> sfVector2f;
}
extern "C" {
    pub fn sfShape_getRotation(shape: *const sfShape) -> f32;
}
extern "C" {
    pub fn sfShape_getScale(shape: *const sfShape) -> sfVector2f;
}
extern "C" {
    pub fn sfShape_getOrigin(shape: *const sfShape) -> sfVector2f;
}
extern "C" {
    pub fn sfShape_move(shape: *mut sfShape, offset: sfVector2f);
}
extern "C" {
    pub fn sfShape_rotate(shape: *mut sfShape, angle: f32);
}
extern "C" {
    pub fn sfShape_scale(shape: *mut sfShape, factors: sfVector2f);
}
extern "C" {
    pub fn sfShape_getTransform(shape: *const sfShape) -> sfTransform;
}
extern "C" {
    pub fn sfShape_getInverseTransform(shape: *const sfShape) -> sfTransform;
}
extern "C" {
    pub fn sfShape_setTexture(shape: *mut sfShape, texture: *const sfTexture,
                              resetRect: sfBool);
}
extern "C" {
    pub fn sfShape_setTextureRect(shape: *mut sfShape, rect: sfIntRect);
}
extern "C" {
    pub fn sfShape_setFillColor(shape: *mut sfShape, color: sfColor);
}
extern "C" {
    pub fn sfShape_setOutlineColor(shape: *mut sfShape, color: sfColor);
}
extern "C" {
    pub fn sfShape_setOutlineThickness(shape: *mut sfShape, thickness: f32);
}
extern "C" {
    pub fn sfShape_getTexture(shape: *const sfShape) -> *const sfTexture;
}
extern "C" {
    pub fn sfShape_getTextureRect(shape: *const sfShape) -> sfIntRect;
}
extern "C" {
    pub fn sfShape_getFillColor(shape: *const sfShape) -> sfColor;
}
extern "C" {
    pub fn sfShape_getOutlineColor(shape: *const sfShape) -> sfColor;
}
extern "C" {
    pub fn sfShape_getOutlineThickness(shape: *const sfShape) -> f32;
}
extern "C" {
    pub fn sfShape_getPointCount(shape: *const sfShape) -> usize;
}
extern "C" {
    pub fn sfShape_getPoint(shape: *const sfShape, index: usize)
     -> sfVector2f;
}
extern "C" {
    pub fn sfShape_getLocalBounds(shape: *const sfShape) -> sfFloatRect;
}
extern "C" {
    pub fn sfShape_getGlobalBounds(shape: *const sfShape) -> sfFloatRect;
}
extern "C" {
    pub fn sfShape_update(shape: *mut sfShape);
}
extern "C" {
    pub fn sfSprite_create() -> *mut sfSprite;
}
extern "C" {
    pub fn sfSprite_copy(sprite: *const sfSprite) -> *mut sfSprite;
}
extern "C" {
    pub fn sfSprite_destroy(sprite: *mut sfSprite);
}
extern "C" {
    pub fn sfSprite_setPosition(sprite: *mut sfSprite, position: sfVector2f);
}
extern "C" {
    pub fn sfSprite_setRotation(sprite: *mut sfSprite, angle: f32);
}
extern "C" {
    pub fn sfSprite_setScale(sprite: *mut sfSprite, scale: sfVector2f);
}
extern "C" {
    pub fn sfSprite_setOrigin(sprite: *mut sfSprite, origin: sfVector2f);
}
extern "C" {
    pub fn sfSprite_getPosition(sprite: *const sfSprite) -> sfVector2f;
}
extern "C" {
    pub fn sfSprite_getRotation(sprite: *const sfSprite) -> f32;
}
extern "C" {
    pub fn sfSprite_getScale(sprite: *const sfSprite) -> sfVector2f;
}
extern "C" {
    pub fn sfSprite_getOrigin(sprite: *const sfSprite) -> sfVector2f;
}
extern "C" {
    pub fn sfSprite_move(sprite: *mut sfSprite, offset: sfVector2f);
}
extern "C" {
    pub fn sfSprite_rotate(sprite: *mut sfSprite, angle: f32);
}
extern "C" {
    pub fn sfSprite_scale(sprite: *mut sfSprite, factors: sfVector2f);
}
extern "C" {
    pub fn sfSprite_getTransform(sprite: *const sfSprite) -> sfTransform;
}
extern "C" {
    pub fn sfSprite_getInverseTransform(sprite: *const sfSprite)
     -> sfTransform;
}
extern "C" {
    pub fn sfSprite_setTexture(sprite: *mut sfSprite,
                               texture: *const sfTexture, resetRect: sfBool);
}
extern "C" {
    pub fn sfSprite_setTextureRect(sprite: *mut sfSprite,
                                   rectangle: sfIntRect);
}
extern "C" {
    pub fn sfSprite_setColor(sprite: *mut sfSprite, color: sfColor);
}
extern "C" {
    pub fn sfSprite_getTexture(sprite: *const sfSprite) -> *const sfTexture;
}
extern "C" {
    pub fn sfSprite_getTextureRect(sprite: *const sfSprite) -> sfIntRect;
}
extern "C" {
    pub fn sfSprite_getColor(sprite: *const sfSprite) -> sfColor;
}
extern "C" {
    pub fn sfSprite_getLocalBounds(sprite: *const sfSprite) -> sfFloatRect;
}
extern "C" {
    pub fn sfSprite_getGlobalBounds(sprite: *const sfSprite) -> sfFloatRect;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfTextStyle {
    sfTextRegular = 0,
    sfTextBold = 1,
    sfTextItalic = 2,
    sfTextUnderlined = 4,
    sfTextStrikeThrough = 8,
}
extern "C" {
    pub fn sfText_create() -> *mut sfText;
}
extern "C" {
    pub fn sfText_copy(text: *const sfText) -> *mut sfText;
}
extern "C" {
    pub fn sfText_destroy(text: *mut sfText);
}
extern "C" {
    pub fn sfText_setPosition(text: *mut sfText, position: sfVector2f);
}
extern "C" {
    pub fn sfText_setRotation(text: *mut sfText, angle: f32);
}
extern "C" {
    pub fn sfText_setScale(text: *mut sfText, scale: sfVector2f);
}
extern "C" {
    pub fn sfText_setOrigin(text: *mut sfText, origin: sfVector2f);
}
extern "C" {
    pub fn sfText_getPosition(text: *const sfText) -> sfVector2f;
}
extern "C" {
    pub fn sfText_getRotation(text: *const sfText) -> f32;
}
extern "C" {
    pub fn sfText_getScale(text: *const sfText) -> sfVector2f;
}
extern "C" {
    pub fn sfText_getOrigin(text: *const sfText) -> sfVector2f;
}
extern "C" {
    pub fn sfText_move(text: *mut sfText, offset: sfVector2f);
}
extern "C" {
    pub fn sfText_rotate(text: *mut sfText, angle: f32);
}
extern "C" {
    pub fn sfText_scale(text: *mut sfText, factors: sfVector2f);
}
extern "C" {
    pub fn sfText_getTransform(text: *const sfText) -> sfTransform;
}
extern "C" {
    pub fn sfText_getInverseTransform(text: *const sfText) -> sfTransform;
}
extern "C" {
    pub fn sfText_setString(text: *mut sfText,
                            string: *const ::std::os::raw::c_schar);
}
extern "C" {
    pub fn sfText_setUnicodeString(text: *mut sfText,
                                   string: *const sfUint32);
}
extern "C" {
    pub fn sfText_setFont(text: *mut sfText, font: *const sfFont);
}
extern "C" {
    pub fn sfText_setCharacterSize(text: *mut sfText,
                                   size: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfText_setStyle(text: *mut sfText, style: sfUint32);
}
extern "C" {
    pub fn sfText_setColor(text: *mut sfText, color: sfColor);
}
extern "C" {
    pub fn sfText_setFillColor(text: *mut sfText, color: sfColor);
}
extern "C" {
    pub fn sfText_setOutlineColor(text: *mut sfText, color: sfColor);
}
extern "C" {
    pub fn sfText_setOutlineThickness(text: *mut sfText, thickness: f32);
}
extern "C" {
    pub fn sfText_getString(text: *const sfText)
     -> *const ::std::os::raw::c_schar;
}
extern "C" {
    pub fn sfText_getUnicodeString(text: *const sfText) -> *const sfUint32;
}
extern "C" {
    pub fn sfText_getFont(text: *const sfText) -> *const sfFont;
}
extern "C" {
    pub fn sfText_getCharacterSize(text: *const sfText)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfText_getStyle(text: *const sfText) -> sfUint32;
}
extern "C" {
    pub fn sfText_getColor(text: *const sfText) -> sfColor;
}
extern "C" {
    pub fn sfText_getFillColor(text: *const sfText) -> sfColor;
}
extern "C" {
    pub fn sfText_getOutlineColor(text: *const sfText) -> sfColor;
}
extern "C" {
    pub fn sfText_getOutlineThickness(text: *const sfText) -> f32;
}
extern "C" {
    pub fn sfText_findCharacterPos(text: *const sfText, index: usize)
     -> sfVector2f;
}
extern "C" {
    pub fn sfText_getLocalBounds(text: *const sfText) -> sfFloatRect;
}
extern "C" {
    pub fn sfText_getGlobalBounds(text: *const sfText) -> sfFloatRect;
}
extern "C" {
    pub fn sfTexture_create(width: ::std::os::raw::c_uint,
                            height: ::std::os::raw::c_uint) -> *mut sfTexture;
}
extern "C" {
    pub fn sfTexture_createFromFile(filename: *const ::std::os::raw::c_schar,
                                    area: *const sfIntRect) -> *mut sfTexture;
}
extern "C" {
    pub fn sfTexture_createFromMemory(data: *const ::std::os::raw::c_void,
                                      sizeInBytes: usize,
                                      area: *const sfIntRect)
     -> *mut sfTexture;
}
extern "C" {
    pub fn sfTexture_createFromStream(stream: *mut sfInputStream,
                                      area: *const sfIntRect)
     -> *mut sfTexture;
}
extern "C" {
    pub fn sfTexture_createFromImage(image: *const sfImage,
                                     area: *const sfIntRect)
     -> *mut sfTexture;
}
extern "C" {
    pub fn sfTexture_copy(texture: *const sfTexture) -> *mut sfTexture;
}
extern "C" {
    pub fn sfTexture_destroy(texture: *mut sfTexture);
}
extern "C" {
    pub fn sfTexture_getSize(texture: *const sfTexture) -> sfVector2u;
}
extern "C" {
    pub fn sfTexture_copyToImage(texture: *const sfTexture) -> *mut sfImage;
}
extern "C" {
    pub fn sfTexture_updateFromPixels(texture: *mut sfTexture,
                                      pixels: *const sfUint8,
                                      width: ::std::os::raw::c_uint,
                                      height: ::std::os::raw::c_uint,
                                      x: ::std::os::raw::c_uint,
                                      y: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfTexture_updateFromImage(texture: *mut sfTexture,
                                     image: *const sfImage,
                                     x: ::std::os::raw::c_uint,
                                     y: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfTexture_updateFromWindow(texture: *mut sfTexture,
                                      window: *const sfWindow,
                                      x: ::std::os::raw::c_uint,
                                      y: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfTexture_updateFromRenderWindow(texture: *mut sfTexture,
                                            renderWindow:
                                                *const sfRenderWindow,
                                            x: ::std::os::raw::c_uint,
                                            y: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn sfTexture_setSmooth(texture: *mut sfTexture, smooth: sfBool);
}
extern "C" {
    pub fn sfTexture_isSmooth(texture: *const sfTexture) -> sfBool;
}
extern "C" {
    pub fn sfTexture_setSrgb(texture: *mut sfTexture, sRgb: sfBool);
}
extern "C" {
    pub fn sfTexture_isSrgb(texture: *const sfTexture) -> sfBool;
}
extern "C" {
    pub fn sfTexture_setRepeated(texture: *mut sfTexture, repeated: sfBool);
}
extern "C" {
    pub fn sfTexture_isRepeated(texture: *const sfTexture) -> sfBool;
}
extern "C" {
    pub fn sfTexture_generateMipmap(texture: *mut sfTexture) -> sfBool;
}
extern "C" {
    pub fn sfTexture_getNativeHandle(texture: *const sfTexture)
     -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfTexture_bind(texture: *const sfTexture);
}
extern "C" {
    pub fn sfTexture_getMaximumSize() -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn sfTransformable_create() -> *mut sfTransformable;
}
extern "C" {
    pub fn sfTransformable_copy(transformable: *const sfTransformable)
     -> *mut sfTransformable;
}
extern "C" {
    pub fn sfTransformable_destroy(transformable: *mut sfTransformable);
}
extern "C" {
    pub fn sfTransformable_setPosition(transformable: *mut sfTransformable,
                                       position: sfVector2f);
}
extern "C" {
    pub fn sfTransformable_setRotation(transformable: *mut sfTransformable,
                                       angle: f32);
}
extern "C" {
    pub fn sfTransformable_setScale(transformable: *mut sfTransformable,
                                    scale: sfVector2f);
}
extern "C" {
    pub fn sfTransformable_setOrigin(transformable: *mut sfTransformable,
                                     origin: sfVector2f);
}
extern "C" {
    pub fn sfTransformable_getPosition(transformable: *const sfTransformable)
     -> sfVector2f;
}
extern "C" {
    pub fn sfTransformable_getRotation(transformable: *const sfTransformable)
     -> f32;
}
extern "C" {
    pub fn sfTransformable_getScale(transformable: *const sfTransformable)
     -> sfVector2f;
}
extern "C" {
    pub fn sfTransformable_getOrigin(transformable: *const sfTransformable)
     -> sfVector2f;
}
extern "C" {
    pub fn sfTransformable_move(transformable: *mut sfTransformable,
                                offset: sfVector2f);
}
extern "C" {
    pub fn sfTransformable_rotate(transformable: *mut sfTransformable,
                                  angle: f32);
}
extern "C" {
    pub fn sfTransformable_scale(transformable: *mut sfTransformable,
                                 factors: sfVector2f);
}
extern "C" {
    pub fn sfTransformable_getTransform(transformable: *const sfTransformable)
     -> sfTransform;
}
extern "C" {
    pub fn sfTransformable_getInverseTransform(transformable:
                                                   *const sfTransformable)
     -> sfTransform;
}
extern "C" {
    pub fn sfVertexArray_create() -> *mut sfVertexArray;
}
extern "C" {
    pub fn sfVertexArray_copy(vertexArray: *const sfVertexArray)
     -> *mut sfVertexArray;
}
extern "C" {
    pub fn sfVertexArray_destroy(vertexArray: *mut sfVertexArray);
}
extern "C" {
    pub fn sfVertexArray_getVertexCount(vertexArray: *const sfVertexArray)
     -> usize;
}
extern "C" {
    pub fn sfVertexArray_getVertex(vertexArray: *mut sfVertexArray,
                                   index: usize) -> *mut sfVertex;
}
extern "C" {
    pub fn sfVertexArray_clear(vertexArray: *mut sfVertexArray);
}
extern "C" {
    pub fn sfVertexArray_resize(vertexArray: *mut sfVertexArray,
                                vertexCount: usize);
}
extern "C" {
    pub fn sfVertexArray_append(vertexArray: *mut sfVertexArray,
                                vertex: sfVertex);
}
extern "C" {
    pub fn sfVertexArray_setPrimitiveType(vertexArray: *mut sfVertexArray,
                                          type_: sfPrimitiveType);
}
extern "C" {
    pub fn sfVertexArray_getPrimitiveType(vertexArray: *mut sfVertexArray)
     -> sfPrimitiveType;
}
extern "C" {
    pub fn sfVertexArray_getBounds(vertexArray: *mut sfVertexArray)
     -> sfFloatRect;
}
extern "C" {
    pub fn sfView_create() -> *mut sfView;
}
extern "C" {
    pub fn sfView_createFromRect(rectangle: sfFloatRect) -> *mut sfView;
}
extern "C" {
    pub fn sfView_copy(view: *const sfView) -> *mut sfView;
}
extern "C" {
    pub fn sfView_destroy(view: *mut sfView);
}
extern "C" {
    pub fn sfView_setCenter(view: *mut sfView, center: sfVector2f);
}
extern "C" {
    pub fn sfView_setSize(view: *mut sfView, size: sfVector2f);
}
extern "C" {
    pub fn sfView_setRotation(view: *mut sfView, angle: f32);
}
extern "C" {
    pub fn sfView_setViewport(view: *mut sfView, viewport: sfFloatRect);
}
extern "C" {
    pub fn sfView_reset(view: *mut sfView, rectangle: sfFloatRect);
}
extern "C" {
    pub fn sfView_getCenter(view: *const sfView) -> sfVector2f;
}
extern "C" {
    pub fn sfView_getSize(view: *const sfView) -> sfVector2f;
}
extern "C" {
    pub fn sfView_getRotation(view: *const sfView) -> f32;
}
extern "C" {
    pub fn sfView_getViewport(view: *const sfView) -> sfFloatRect;
}
extern "C" {
    pub fn sfView_move(view: *mut sfView, offset: sfVector2f);
}
extern "C" {
    pub fn sfView_rotate(view: *mut sfView, angle: f32);
}
extern "C" {
    pub fn sfView_zoom(view: *mut sfView, factor: f32);
}
