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

#![allow(non_snake_case)]

use libc::{c_float, c_char, c_uint, c_int, c_uchar, size_t, c_void};
use ffi::SfBool;
use ffi::window::sfWindow;
use system::{Vector2f, Vector2i, Vector2u, Vector3f, InputStream};
use window::{ContextSettings, VideoMode};
use window::raw_event::sfEvent;
use graphics::{BlendMode, FloatRect, Glyph, IntRect, Transform, Color, Vertex, PrimitiveType};

foreign_type! {
	sfRenderWindow, sfRenderWindow_destroy;
	sfCircleShape, sfCircleShape_destroy;
//	sfConvexShape, sfConvexShape_destroy;
	sfFont, sfFont_destroy;
	sfImage, sfImage_destroy;
	sfRectangleShape, sfRectangleShape_destroy;
	sfShader, sfShader_destroy;
	sfRenderTexture, sfRenderTexture_destroy;
	sfShape, sfShape_destroy;
	sfSprite, sfSprite_destroy;
	sfText, sfText_destroy;
	sfTexture, sfTexture_destroy;
	sfTransformable, sfTransformable_destroy;
	sfView, sfView_destroy;
}

#[repr(C)]
pub struct sfFontInfo {
    pub family: *const c_char
}

#[repr(C)]
pub struct sfRenderStates {
    pub blendMode: BlendMode,
    pub transform: Transform,
    pub texture: *const sfTexture,
    pub shader: *const sfShader
}

#[cfg_attr(any(target_os="macos", target_os="linux", target_os="windows"), link(name="csfml-graphics"))]
extern "C" {
    //pub fn sfRenderWindow_create(mode: VideoMode, title: *const c_char, style: c_uint, settings: *const ContextSettings) -> *mut sfRenderWindow;
    pub fn sfRenderWindow_createUnicode(mode: VideoMode, title: *const u32, style: c_uint, settings: *const ContextSettings) -> *mut sfRenderWindow;
    //fn sfRenderWindow_createFromHandle(handle: sfWindowHandle, settings: *mut sfContextSettings) -> *mut sfRenderWindow;
    pub fn sfRenderWindow_destroy(renderWindow: *mut sfRenderWindow) -> ();
    pub fn sfRenderWindow_close(renderWindow: *mut sfRenderWindow) -> ();
    pub fn sfRenderWindow_isOpen(renderWindow: *const sfRenderWindow) -> SfBool;
    pub fn sfRenderWindow_getSettings(renderWindow: *const sfRenderWindow) -> ContextSettings;
    pub fn sfRenderWindow_pollEvent(renderWindow: *mut sfRenderWindow, event: *mut sfEvent) -> SfBool;
    pub fn sfRenderWindow_waitEvent(renderWindow: *mut sfRenderWindow, event: *mut sfEvent) -> SfBool;
    pub fn sfRenderWindow_getPosition(renderWindow: *const sfRenderWindow) -> Vector2i;
    pub fn sfRenderWindow_setPosition(renderWindow: *mut sfRenderWindow, position: Vector2i) -> ();
    pub fn sfRenderWindow_getSize(renderWindow: *const sfRenderWindow) -> Vector2u;
    pub fn sfRenderWindow_setSize(renderWindow: *mut sfRenderWindow, size: Vector2u) -> ();
    //pub fn sfRenderWindow_setTitle(renderWindow: *mut sfRenderWindow, title: *const c_char) -> ();
    pub fn sfRenderWindow_setUnicodeTitle(renderWindow: *mut sfRenderWindow, title: *const u32) -> ();
    pub fn sfRenderWindow_setIcon(renderWindow: *mut sfRenderWindow, width: c_uint, height: c_uint, pixels: *const u8) -> ();
    pub fn sfRenderWindow_setVisible(renderWindow: *mut sfRenderWindow, visible: SfBool) -> ();
    pub fn sfRenderWindow_setMouseCursorVisible(renderWindow: *mut sfRenderWindow, show: SfBool) -> ();
    pub fn sfRenderWindow_setVerticalSyncEnabled(renderWindow: *mut sfRenderWindow, enabled: SfBool) -> ();
    pub fn sfRenderWindow_setKeyRepeatEnabled(renderWindow: *mut sfRenderWindow, enabled: SfBool) -> ();
    pub fn sfRenderWindow_setActive(renderWindow: *mut sfRenderWindow, active: SfBool) -> SfBool;
    pub fn sfRenderWindow_requestFocus(window: *mut sfRenderWindow) -> ();
    pub fn sfRenderWindow_hasFocus(window: *const sfRenderWindow) -> SfBool;
    pub fn sfRenderWindow_display(renderWindow: *mut sfRenderWindow) -> ();
    pub fn sfRenderWindow_setFramerateLimit(renderWindow: *mut sfRenderWindow, limit: c_uint) -> ();
    pub fn sfRenderWindow_setJoystickThreshold(renderWindow: *mut sfRenderWindow, treshold: c_float) -> ();
    // fn sfRenderWindow_getSystemHandle(renderWindow: *mut sfRenderWindow) -> sfWindowHandle;
    pub fn sfRenderWindow_clear(renderWindow: *mut sfRenderWindow, color: Color) -> ();
    pub fn sfRenderWindow_setView(renderWindow: *mut sfRenderWindow, view: *const sfView) -> ();
    pub fn sfRenderWindow_getView(renderWindow: *const sfRenderWindow) -> *const sfView;
    pub fn sfRenderWindow_getDefaultView(renderWindow: *const sfRenderWindow) -> *const sfView;
    pub fn sfRenderWindow_getViewport(renderWindow: *const sfRenderWindow, view: *const sfView) -> IntRect;
    pub fn sfRenderWindow_mapPixelToCoords(renderWindow: *const sfRenderWindow, point: Vector2i, view: *const sfView) -> Vector2f;
    pub fn sfRenderWindow_mapCoordsToPixel(renderWindow: *const sfRenderWindow, point: Vector2f, view: *const sfView) -> Vector2i;
    pub fn sfRenderWindow_drawSprite(renderWindow: *mut sfRenderWindow, object: *const sfSprite, states: *const sfRenderStates) -> ();
    pub fn sfRenderWindow_drawText(renderWindow: *mut sfRenderWindow, object: *const sfText, states: *const sfRenderStates) -> ();
    pub fn sfRenderWindow_drawShape(renderWindow: *mut sfRenderWindow, object: *const sfShape, states: *const sfRenderStates) -> ();
    pub fn sfRenderWindow_drawCircleShape(renderWindow: *mut sfRenderWindow, object: *const sfCircleShape, states: *const sfRenderStates) -> ();
    //pub fn sfRenderWindow_drawConvexShape(renderWindow: *mut sfRenderWindow, object: *const sfConvexShape, states: *const sfRenderStates) -> ();
    pub fn sfRenderWindow_drawRectangleShape(renderWindow: *mut sfRenderWindow, object: *const sfRectangleShape, states: *const sfRenderStates) -> ();
    //pub fn sfRenderWindow_drawVertexArray(renderWindow: *mut sfRenderWindow, object: *const sfVertexArray, states: *const sfRenderStates) -> ();
    pub fn sfRenderWindow_drawPrimitives(renderWindow: *mut sfRenderWindow, vertices: *const Vertex, vertexCount: c_uint, ttype: PrimitiveType, states: *const sfRenderStates) -> ();
    pub fn sfRenderWindow_pushGLStates(renderWindow: *mut sfRenderWindow) -> ();
    pub fn sfRenderWindow_popGLStates(renderWindow: *mut sfRenderWindow) -> ();
    pub fn sfRenderWindow_resetGLStates(renderWindow: *mut sfRenderWindow) -> ();
    pub fn sfRenderWindow_capture(renderWindow: *const sfRenderWindow) -> *mut sfImage;
    pub fn sfMouse_getPositionRenderWindow(relativeTo: *const sfRenderWindow) -> Vector2i;
    pub fn sfMouse_setPositionRenderWindow(position: Vector2i, relativeTo: *const sfRenderWindow) -> ();
    pub fn sfTouch_getPositionRenderWindow(finger: c_uint, relativeTo: *const sfRenderWindow) -> Vector2i;

    pub fn sfCircleShape_create() -> *mut sfCircleShape;
    pub fn sfCircleShape_copy(shape: *const sfCircleShape) -> *mut sfCircleShape;
    pub fn sfCircleShape_destroy(shape: *mut sfCircleShape) -> ();
    pub fn sfCircleShape_setPosition(shape: *mut sfCircleShape, position: Vector2f) -> ();
    pub fn sfCircleShape_setRotation(shape: *mut sfCircleShape, angle: c_float) -> ();
    pub fn sfCircleShape_setScale(shape: *mut sfCircleShape, scale: Vector2f) -> ();
    pub fn sfCircleShape_setOrigin(shape: *mut sfCircleShape, origin: Vector2f) -> ();
    pub fn sfCircleShape_getPosition(shape: *const sfCircleShape) -> Vector2f;
    pub fn sfCircleShape_getRotation(shape: *const sfCircleShape) -> c_float;
    pub fn sfCircleShape_getScale(shape: *const sfCircleShape) -> Vector2f;
    pub fn sfCircleShape_getOrigin(shape: *const sfCircleShape) -> Vector2f;
    pub fn sfCircleShape_move(shape: *mut sfCircleShape, offset: Vector2f) -> ();
    pub fn sfCircleShape_rotate(shape: *mut sfCircleShape, angle: c_float) -> ();
    pub fn sfCircleShape_scale(shape: *mut sfCircleShape, factors: Vector2f) -> ();
    pub fn sfCircleShape_getTransform(shape: *const sfCircleShape) -> Transform;
    pub fn sfCircleShape_getInverseTransform(shape: *const sfCircleShape) -> Transform;
    pub fn sfCircleShape_setTexture(shape: *mut sfCircleShape, texture: *const sfTexture, reset_rect: SfBool) -> ();
    pub fn sfCircleShape_setTextureRect(shape: *mut sfCircleShape, rect: IntRect) -> ();
    pub fn sfCircleShape_setFillColor(shape: *mut sfCircleShape, color: Color) -> ();
    pub fn sfCircleShape_setOutlineColor(shape: *mut sfCircleShape, color: Color) -> ();
    pub fn sfCircleShape_setOutlineThickness(shape: *mut sfCircleShape, thickness: c_float) -> ();
    //pub fn sfCircleShape_getTexture(shape: *const sfCircleShape) -> *const sfTexture;
    pub fn sfCircleShape_getTextureRect(shape: *const sfCircleShape) -> IntRect;
    pub fn sfCircleShape_getFillColor(shape: *const sfCircleShape) -> Color;
    pub fn sfCircleShape_getOutlineColor(shape: *const sfCircleShape) -> Color;
    pub fn sfCircleShape_getOutlineThickness(shape: *const sfCircleShape) -> c_float;
    pub fn sfCircleShape_getPointCount(shape: *const sfCircleShape) -> c_uint;
    pub fn sfCircleShape_getPoint(shape: *const sfCircleShape, index: c_uint) -> Vector2f;
    pub fn sfCircleShape_setRadius(shape: *mut sfCircleShape, radius: c_float) -> ();
    pub fn sfCircleShape_getRadius(shape: *const sfCircleShape) -> c_float;
    pub fn sfCircleShape_setPointCount(shape: *mut sfCircleShape, count: c_uint) -> ();
    pub fn sfCircleShape_getLocalBounds(shape: *const sfCircleShape) -> FloatRect;
    pub fn sfCircleShape_getGlobalBounds(shape: *const sfCircleShape) -> FloatRect;

	/*
    pub fn sfConvexShape_create() -> *mut sfConvexShape;
    pub fn sfConvexShape_copy(shape: *const sfConvexShape) -> *mut sfConvexShape;
    pub fn sfConvexShape_destroy(shape: *mut sfConvexShape) -> ();
    pub fn sfConvexShape_setPosition(shape: *mut sfConvexShape, position: Vector2f) -> ();
    pub fn sfConvexShape_setRotation(shape: *mut sfConvexShape, angle: c_float) -> ();
    pub fn sfConvexShape_setScale(shape: *mut sfConvexShape, scale: Vector2f) -> ();
    pub fn sfConvexShape_setOrigin(shape: *mut sfConvexShape, origin: Vector2f) -> ();
    pub fn sfConvexShape_getPosition(shape: *const sfConvexShape) -> Vector2f;
    pub fn sfConvexShape_getRotation(shape: *const sfConvexShape) -> c_float;
    pub fn sfConvexShape_getScale(shape: *const sfConvexShape) -> Vector2f;
    pub fn sfConvexShape_getOrigin(shape: *const sfConvexShape) -> Vector2f;
    pub fn sfConvexShape_move(shape: *mut sfConvexShape, offset: Vector2f) -> ();
    pub fn sfConvexShape_rotate(shape: *mut sfConvexShape, angle: c_float) -> ();
    pub fn sfConvexShape_scale(shape: *mut sfConvexShape, factors: Vector2f) -> ();
    pub fn sfConvexShape_getTransform(shape: *const sfConvexShape) -> Transform;
    pub fn sfConvexShape_getInverseTransform(shape: *const sfConvexShape) -> Transform;
    pub fn sfConvexShape_setTexture(shape: *mut sfConvexShape, texture: *const sfTexture, reset_rect: SfBool) -> ();
    pub fn sfConvexShape_setTextureRect(shape: *mut sfConvexShape, rect: IntRect) -> ();
    pub fn sfConvexShape_setFillColor(shape: *mut sfConvexShape, color: Color) -> ();
    pub fn sfConvexShape_setOutlineColor(shape: *mut sfConvexShape, color: Color) -> ();
    pub fn sfConvexShape_setOutlineThickness(shape: *mut sfConvexShape, thickness: c_float) -> ();
    pub fn sfConvexShape_getTexture(shape: *const sfConvexShape) -> *const sfTexture;
    pub fn sfConvexShape_getTextureRect(shape: *const sfConvexShape) -> IntRect;
    pub fn sfConvexShape_getFillColor(shape: *const sfConvexShape) -> Color;
    pub fn sfConvexShape_getOutlineColor(shape: *const sfConvexShape) -> Color;
    pub fn sfConvexShape_getOutlineThickness(shape: *const sfConvexShape) -> c_float;
    pub fn sfConvexShape_getPointCount(shape: *const sfConvexShape) -> c_uint;
    pub fn sfConvexShape_getPoint(shape: *const sfConvexShape, index: c_uint) -> Vector2f;
    pub fn sfConvexShape_setPointCount(shape: *mut sfConvexShape, count: c_uint) -> ();
    pub fn sfConvexShape_setPoint(shape: *mut sfConvexShape, index: c_uint, point: Vector2f) -> ();
    pub fn sfConvexShape_getLocalBounds(shape: *const sfConvexShape) -> FloatRect;
    pub fn sfConvexShape_getGlobalBounds(shape: *const sfConvexShape) -> FloatRect;
	*/

    pub fn sfFont_createFromFile(filename: *const c_char) -> *mut sfFont;
    pub fn sfFont_copy(font: *const sfFont) -> *mut sfFont;
    pub fn sfFont_createFromMemory(data: *const c_uchar, sizeInBytes: size_t) -> *mut sfFont;
    // fn sfFont_createFromStream(stream: *mut sfInputStream) -> *mut sfFont;
    pub fn sfFont_destroy(font: *mut sfFont) -> ();
    pub fn sfFont_getGlyph(font: *const sfFont, codepoint: u32, characterSize: c_uint, bold :SfBool) -> Glyph;
    pub fn sfFont_getKerning(font: *const sfFont, first: u32, second: u32, characterSize: c_uint) -> c_float;
    pub fn sfFont_getLineSpacing(font: *const sfFont, characterSize: c_uint) -> c_float;
    pub fn sfFont_getTexture(font: *mut sfFont, characterSize: c_uint) -> *const sfTexture;
    pub fn sfFont_getUnderlinePosition(font: *const sfFont, characterSize: c_uint) -> c_float;
    pub fn sfFont_getUnderlineThickness(font: *const sfFont, characterSize: c_uint) -> c_float;
    pub fn sfFont_getInfo(font: *const sfFont) -> sfFontInfo;

    pub fn sfImage_create(width: c_uint, height: c_uint) -> *mut sfImage;
    pub fn sfImage_createFromColor(width: c_uint, height: c_uint, color: Color) -> *mut sfImage;
    pub fn sfImage_createFromPixels(width: c_uint, height: c_uint, pixels: *const u8) -> *mut sfImage;
    pub fn sfImage_createFromFile(filename: *const c_char) -> *mut sfImage;
    pub fn sfImage_createFromMemory(data: *const c_uchar, size: size_t) -> *mut sfImage;
    //fn sfImage_createFromStream(stream: *mut sfInputStream) -> *mut sfImage;
    pub fn sfImage_copy(image: *const sfImage) -> *mut sfImage;
    pub fn sfImage_destroy(image: *mut sfImage) -> ();
    pub fn sfImage_saveToFile(image: *const sfImage, filename: *const c_char) -> SfBool;
    pub fn sfImage_getSize(image: *const sfImage) -> Vector2u;
    pub fn sfImage_createMaskFromColor(image: *mut sfImage, color: Color, alpha: u8) -> ();
    pub fn sfImage_copyImage(image: *mut sfImage, source: *const sfImage, destX: c_uint, destY: c_uint, sourceRect: IntRect, applyAlpha: SfBool) -> ();
    pub fn sfImage_setPixel(image: *mut sfImage, x: c_uint, y: c_uint, color: Color) -> ();
    pub fn sfImage_getPixel(image: *const sfImage, x: c_uint, y: c_uint) -> Color;
    pub fn sfImage_getPixelsPtr(image: *const sfImage) -> *const u8;
    pub fn sfImage_flipHorizontally(image: *mut sfImage) -> ();
    pub fn sfImage_flipVertically(image: *mut sfImage) -> ();

    pub fn sfIntRect_contains(rect: *const IntRect, x: c_int, y: c_int) -> SfBool;
    pub fn sfIntRect_intersects(rect1: *const IntRect, rect2: *const IntRect, intersectons: *mut IntRect) -> SfBool;
    pub fn sfFloatRect_intersects(rect1: *const FloatRect, rect2: *const FloatRect, intersectons: *mut FloatRect) -> SfBool;
    pub fn sfFloatRect_contains(rect: *const FloatRect, x: f32, y: f32) -> SfBool;

    pub fn sfRectangleShape_create() -> *mut sfRectangleShape;
    pub fn sfRectangleShape_copy(shape: *const sfRectangleShape) -> *mut sfRectangleShape;
    pub fn sfRectangleShape_destroy(shape: *mut sfRectangleShape) -> ();
    pub fn sfRectangleShape_setPosition(shape: *mut sfRectangleShape, position: Vector2f) -> ();
    pub fn sfRectangleShape_setRotation(shape: *mut sfRectangleShape, angle: c_float) -> ();
    pub fn sfRectangleShape_setScale(shape: *mut sfRectangleShape, scale: Vector2f) -> ();
    pub fn sfRectangleShape_setOrigin(shape: *mut sfRectangleShape, origin: Vector2f) -> ();
    pub fn sfRectangleShape_getPosition(shape: *const sfRectangleShape) -> Vector2f;
    pub fn sfRectangleShape_getRotation(shape: *const sfRectangleShape) -> c_float;
    pub fn sfRectangleShape_getScale(shape: *const sfRectangleShape) -> Vector2f;
    pub fn sfRectangleShape_getOrigin(shape: *const sfRectangleShape) -> Vector2f;
    pub fn sfRectangleShape_move(shape: *mut sfRectangleShape, offset: Vector2f) -> ();
    pub fn sfRectangleShape_rotate(shape: *mut sfRectangleShape, angle: c_float) -> ();
    pub fn sfRectangleShape_scale(shape: *mut sfRectangleShape, factors: Vector2f) -> ();
    pub fn sfRectangleShape_getTransform(shape: *const sfRectangleShape) -> Transform;
    pub fn sfRectangleShape_getInverseTransform(shape: *const sfRectangleShape) -> Transform;
    pub fn sfRectangleShape_setTexture(shape: *mut sfRectangleShape, texture: *const sfTexture, reset_rect: SfBool) -> ();
    pub fn sfRectangleShape_setTextureRect(shape: *mut sfRectangleShape, rect: IntRect) -> ();
    pub fn sfRectangleShape_setFillColor(shape: *mut sfRectangleShape, color: Color) -> ();
    pub fn sfRectangleShape_setOutlineColor(shape: *mut sfRectangleShape, color: Color) -> ();
    pub fn sfRectangleShape_setOutlineThickness(shape: *mut sfRectangleShape, thickness: c_float) -> ();
    //pub fn sfRectangleShape_getTexture(shape: *const sfRectangleShape) -> *const sfTexture;
    pub fn sfRectangleShape_getTextureRect(shape: *const sfRectangleShape) -> IntRect;
    pub fn sfRectangleShape_getFillColor(shape: *const sfRectangleShape) -> Color;
    pub fn sfRectangleShape_getOutlineColor(shape: *const sfRectangleShape) -> Color;
    pub fn sfRectangleShape_getOutlineThickness(shape: *const sfRectangleShape) -> c_float;
    pub fn sfRectangleShape_getPointCount(shape: *const sfRectangleShape) -> c_uint;
    pub fn sfRectangleShape_getPoint(shape: *const sfRectangleShape, index: c_uint) -> Vector2f;
    pub fn sfRectangleShape_setSize(shape: *mut sfRectangleShape, size: Vector2f) -> ();
    pub fn sfRectangleShape_getSize(shape: *const sfRectangleShape) -> Vector2f;
    pub fn sfRectangleShape_getLocalBounds(shape: *const sfRectangleShape) -> FloatRect;
    pub fn sfRectangleShape_getGlobalBounds(shape: *const sfRectangleShape) -> FloatRect;

    pub fn sfShader_createFromFile(vertexShaderFilename: *const c_char, fragmentShaderFilename: *const c_char) -> *mut sfShader;
    pub fn sfShader_createFromMemory(vertexShader: *const c_char, fragmentShader: *const c_char) -> *mut sfShader;
    //fn sfShader_createFromStream(vertexShaderStream: *mut sfInputStream, fragmentShaderStream: *mut sfInputStream) -> *mut sfShader;
    pub fn sfShader_destroy(shader: *mut sfShader)-> ();
    pub fn sfShader_setFloatParameter(shader: *mut sfShader, name: *const c_char, x: c_float) -> ();
    pub fn sfShader_setFloat2Parameter(shader: *mut sfShader, name: *const c_char, x: c_float, y: c_float) -> ();
    pub fn sfShader_setFloat3Parameter(shader: *mut sfShader, name: *const c_char, x: c_float, y: c_float, z: c_float) -> ();
    pub fn sfShader_setFloat4Parameter(shader: *mut sfShader, name: *const c_char, x: c_float, y: c_float, z: c_float, w: c_float) -> ();
    pub fn sfShader_setVector2Parameter(shader: *mut sfShader, name: *const c_char, vector: Vector2f) -> ();
    pub fn sfShader_setVector3Parameter(shader: *mut sfShader, name: *const c_char, vector: Vector3f) -> ();
    pub fn sfShader_setColorParameter(shader: *mut sfShader, name: *const c_char, color: Color) -> ();
    pub fn sfShader_setTransformParameter(shader: *mut sfShader, name: *const c_char, transform: Transform) -> ();
    pub fn sfShader_setTextureParameter(shader: *mut sfShader, name: *const c_char, texture: *const sfTexture) -> ();
    pub fn sfShader_setCurrentTextureParameter(shader: *mut sfShader, name: *const c_char) -> ();
    pub fn sfShader_bind(shader: *mut sfShader) -> ();
    pub fn sfShader_isAvailable() -> SfBool;

    pub fn sfRenderTexture_create(width: c_uint, height: c_uint, depthBuffer: SfBool) -> *mut sfRenderTexture;
    pub fn sfRenderTexture_destroy(renderTexture: *mut sfRenderTexture) -> ();
    pub fn sfRenderTexture_getSize(renderTexture: *const sfRenderTexture) -> Vector2u;
    pub fn sfRenderTexture_setActive(renderTexture: *mut sfRenderTexture, active: SfBool) -> SfBool;
    pub fn sfRenderTexture_display(renderTexture: *mut sfRenderTexture) -> ();
    pub fn sfRenderTexture_clear(renderTexture: *mut sfRenderTexture, color: Color) -> ();
    pub fn sfRenderTexture_setView(renderTexture: *mut sfRenderTexture, view: *const sfView) -> ();
    pub fn sfRenderTexture_getView(renderTexture: *const sfRenderTexture) -> *const sfView;
    pub fn sfRenderTexture_getDefaultView(renderTexture: *const sfRenderTexture) -> *const sfView;
    pub fn sfRenderTexture_getViewport(renderTexture: *const sfRenderTexture, view: *const sfView) -> IntRect;
    pub fn sfRenderTexture_mapPixelToCoords(renderTexture: *const sfRenderTexture, point: Vector2i, view: *const sfView) -> Vector2f;
    pub fn sfRenderTexture_mapCoordsToPixel(renderTexture: *const sfRenderTexture, point: Vector2f, view: *const sfView) -> Vector2i;
    pub fn sfRenderTexture_drawSprite(renderTexture: *mut sfRenderTexture, object: *const sfSprite, states: *const sfRenderStates) -> ();
    pub fn sfRenderTexture_drawText(renderTexture: *mut sfRenderTexture, object: *const sfText, states: *const sfRenderStates) -> ();
    pub fn sfRenderTexture_drawShape(renderTexture: *mut sfRenderTexture, object: *const sfShape, states: *const sfRenderStates) -> ();
    pub fn sfRenderTexture_drawCircleShape(renderTexture: *mut sfRenderTexture, object: *const sfCircleShape, states: *const sfRenderStates) -> ();
    //pub fn sfRenderTexture_drawConvexShape(renderTexture: *mut sfRenderTexture, object: *const sfConvexShape, states: *const sfRenderStates) -> ();
    pub fn sfRenderTexture_drawRectangleShape(renderTexture: *mut sfRenderTexture, object: *const sfRectangleShape, states: *const sfRenderStates) -> ();
    //pub fn sfRenderTexture_drawVertexArray(renderTexture: *mut sfRenderTexture, object: *const sfVertexArray, states: *const sfRenderStates) -> ();
    pub fn sfRenderTexture_drawPrimitives(renderTexture: *mut sfRenderTexture, vertices: *const Vertex, vertexCount: c_uint, ttype: PrimitiveType, states: *const sfRenderStates) -> ();
    pub fn sfRenderTexture_pushGLStates(renderTexture: *mut sfRenderTexture) -> ();
    pub fn sfRenderTexture_popGLStates(renderTexture: *mut sfRenderTexture) -> ();
    pub fn sfRenderTexture_resetGLStates(renderTexture: *mut sfRenderTexture) -> ();
    pub fn sfRenderTexture_getTexture(renderTexture: *const sfRenderTexture) -> *const sfTexture;
    pub fn sfRenderTexture_setSmooth(renderTexture: *mut sfRenderTexture, smooth: SfBool) -> ();
    pub fn sfRenderTexture_isSmooth(renderTexture: *const sfRenderTexture) -> SfBool;
    pub fn sfRenderTexture_setRepeated(renderTexture: *mut sfRenderTexture, repeated: SfBool) -> ();
    pub fn sfRenderTexture_isRepeated(renderTexture: *const sfRenderTexture) -> SfBool;

    pub fn sfShape_create(getPointCount: extern "C" fn(*mut c_void) -> u32, getPoint: extern "C" fn(u32, *mut c_void) -> Vector2f, userData: *mut c_void) -> *mut sfShape;
    pub fn sfShape_destroy(shape: *mut sfShape) -> ();
    pub fn sfShape_setPosition(shape: *mut sfShape, position: Vector2f) -> ();
    pub fn sfShape_setRotation(shape: *mut sfShape, angle: c_float) -> ();
    pub fn sfShape_setScale(shape: *mut sfShape, scale: Vector2f) -> ();
    pub fn sfShape_setOrigin(shape: *mut sfShape, origin: Vector2f) -> ();
    pub fn sfShape_getPosition(shape: *const sfShape) -> Vector2f;
    pub fn sfShape_getRotation(shape: *const sfShape) -> c_float;
    pub fn sfShape_getScale(shape: *const sfShape) -> Vector2f;
    pub fn sfShape_getOrigin(shape: *const sfShape) -> Vector2f;
    pub fn sfShape_move(shape: *mut sfShape, offset: Vector2f) -> ();
    pub fn sfShape_rotate(shape: *mut sfShape, angle: c_float) -> ();
    pub fn sfShape_scale(shape: *mut sfShape, factors: Vector2f) -> ();
    pub fn sfShape_getTransform(shape: *const sfShape) -> Transform;
    pub fn sfShape_getInverseTransform(shape: *const sfShape) -> Transform;
    pub fn sfShape_setTexture(shape: *mut sfShape, texture: *const sfTexture, reset_rect: SfBool) -> ();
    pub fn sfShape_setTextureRect(shape: *mut sfShape, rect: IntRect) -> ();
    pub fn sfShape_setFillColor(shape: *mut sfShape, color: Color) -> ();
    pub fn sfShape_setOutlineColor(shape: *mut sfShape, color: Color) -> ();
    pub fn sfShape_setOutlineThickness(shape: *mut sfShape, thickness: c_float) -> ();
    //pub fn sfShape_getTexture(shape: *const sfShape) -> *const sfTexture;
    pub fn sfShape_getTextureRect(shape: *const sfShape) -> IntRect;
    pub fn sfShape_getFillColor(shape: *const sfShape) -> Color;
    pub fn sfShape_getOutlineColor(shape: *const sfShape) -> Color;
    pub fn sfShape_getOutlineThickness(shape: *const sfShape) -> c_float;
    pub fn sfShape_getPointCount(shape: *const sfShape) -> c_uint;
    pub fn sfShape_getPoint(shape: *const sfShape, index: c_uint) -> Vector2f;
    pub fn sfShape_getLocalBounds(shape: *const sfShape) -> FloatRect;
    pub fn sfShape_getGlobalBounds(shape: *const sfShape) -> FloatRect;
    pub fn sfShape_update(shape: *mut sfShape) -> ();

    pub fn sfSprite_create() -> *mut sfSprite;
    pub fn sfSprite_copy(sprite: *const sfSprite) -> *mut sfSprite;
    pub fn sfSprite_destroy(sprite: *mut sfSprite) -> ();
    pub fn sfSprite_setPosition(sprite: *mut sfSprite, position: Vector2f) -> ();
    pub fn sfSprite_setRotation(sprite: *mut sfSprite, angle: c_float) -> ();
    pub fn sfSprite_setScale(sprite: *mut sfSprite, scale: Vector2f) -> ();
    pub fn sfSprite_setOrigin(sprite: *mut sfSprite, origin: Vector2f) -> ();
    pub fn sfSprite_getPosition(sprite: *const sfSprite) -> Vector2f;
    pub fn sfSprite_getRotation(sprite: *const sfSprite) -> c_float;
    pub fn sfSprite_getScale(sprite: *const sfSprite) -> Vector2f;
    pub fn sfSprite_getOrigin(sprite: *const sfSprite) -> Vector2f;
    pub fn sfSprite_move(sprite: *mut sfSprite, offset: Vector2f) -> ();
    pub fn sfSprite_rotate(sprite: *mut sfSprite, angle: c_float) -> ();
    pub fn sfSprite_scale(sprite: *mut sfSprite, factors: Vector2f) -> ();
    pub fn sfSprite_getTransform(sprite: *const sfSprite) -> Transform;
    pub fn sfSprite_getInverseTransform(sprite: *const sfSprite) -> Transform;
    pub fn sfSprite_setTexture(sprite: *mut sfSprite, texture: *const sfTexture, reset_rect: SfBool) -> ();
    pub fn sfSprite_setTextureRect(sprite: *mut sfSprite, rectangle: IntRect) -> ();
    pub fn sfSprite_setColor(sprite: *mut sfSprite, color: Color) -> ();
    //pub fn sfSprite_getTexture(sprite: *const sfSprite) -> *const sfTexture;
    pub fn sfSprite_getTextureRect(sprite: *const sfSprite) -> IntRect;
    pub fn sfSprite_getColor(sprite: *const sfSprite) -> Color;
    pub fn sfSprite_getLocalBounds(sprite: *const sfSprite) -> FloatRect;
    pub fn sfSprite_getGlobalBounds(sprite: *const sfSprite) -> FloatRect;

    pub fn sfText_create() -> *mut sfText;
    pub fn sfText_copy(text: *const sfText) -> *mut sfText;
    pub fn sfText_destroy(text: *mut sfText) -> ();
    pub fn sfText_setPosition(text: *mut sfText, position: Vector2f) -> ();
    pub fn sfText_setRotation(text: *mut sfText, angle: c_float) -> ();
    pub fn sfText_setScale(text: *mut sfText, scale: Vector2f) -> ();
    pub fn sfText_setOrigin(text: *mut sfText, origin: Vector2f) -> ();
    pub fn sfText_getPosition(text: *const sfText) -> Vector2f;
    pub fn sfText_getRotation(text: *const sfText) -> c_float;
    pub fn sfText_getScale(text: *const sfText) -> Vector2f;
    pub fn sfText_getOrigin(text: *const sfText) -> Vector2f;
    pub fn sfText_move(text: *mut sfText, offset: Vector2f) -> ();
    pub fn sfText_rotate(text: *mut sfText, angle: c_float) -> ();
    pub fn sfText_scale(text: *mut sfText, factors: Vector2f) -> ();
    pub fn sfText_getTransform(text: *const sfText) -> Transform;
    pub fn sfText_getInverseTransform(text: *const sfText) -> Transform;
    //pub fn sfText_setString(text: *mut sfText, string: *const c_char) -> ();
    pub fn sfText_setUnicodeString(text: *mut sfText, string: *const u32 ) -> ();
    pub fn sfText_setFont(text: *mut sfText, font: *const sfFont) -> ();
    pub fn sfText_setCharacterSize(text: *mut sfText, size: c_uint) -> ();
    pub fn sfText_setStyle(text: *mut sfText, style: u32) -> ();
    pub fn sfText_setColor(text: *mut sfText, color: Color) -> ();
    //pub fn sfText_getString(text: *const sfText) -> *const c_char;
    pub fn sfText_getUnicodeString(text: *const sfText) -> *const u32;
    //pub fn sfText_getFont(text: *const sfText) -> *const sfFont;
    pub fn sfText_getCharacterSize(text: *const sfText) -> c_uint;
    pub fn sfText_getStyle(text: *const sfText) -> u32;
    pub fn sfText_getColor(text: *const sfText) -> Color;
    pub fn sfText_findCharacterPos(text: *const sfText, index: size_t) -> Vector2f;
    pub fn sfText_getLocalBounds(text: *const sfText) -> FloatRect;
    pub fn sfText_getGlobalBounds(text: *const sfText) -> FloatRect;

    pub fn sfTexture_create(width: c_uint, height: c_uint) -> *mut sfTexture;
    pub fn sfTexture_createFromFile(filename: *const c_char, area: *const IntRect) -> *mut sfTexture;
    pub fn sfTexture_createFromMemory(data: *const c_uchar, sizeInBytes: size_t , area: *const IntRect) -> *mut sfTexture;
    //fn sfTexture_createFromStream(strea;: *mut sfInputStream, area: *mut sfIntRect) -> *mut sfTexture;
    pub fn sfTexture_createFromImage(image: *const sfImage, area: *const IntRect) -> *mut sfTexture;
    pub fn sfTexture_copy(texture: *const sfTexture) -> *mut sfTexture;
    pub fn sfTexture_destroy(texture: *mut sfTexture) -> ();
    pub fn sfTexture_getSize(texture: *const sfTexture) -> Vector2u;
    pub fn sfTexture_copyToImage(texture: *const sfTexture) -> *mut sfImage;
    pub fn sfTexture_updateFromPixels(texture: *mut sfTexture, pixels: *const u8, width: c_uint, height: c_uint, x: c_uint, y: c_uint) -> ();
    pub fn sfTexture_updateFromImage(texture: *mut sfTexture, image: *const sfImage, x: c_uint, y: c_uint) -> ();
    pub fn sfTexture_updateFromWindow(texture: *mut sfTexture, window: *const sfWindow, x: c_uint, y: c_uint) -> ();
    pub fn sfTexture_updateFromRenderWindow(texture: *mut sfTexture, renderWindow: *const sfRenderWindow, x: c_uint, y: c_uint) -> ();
    pub fn sfTexture_setSmooth(texture: *mut sfTexture, smooth: SfBool) -> ();
    pub fn sfTexture_isSmooth(texture: *const sfTexture) -> SfBool;
    pub fn sfTexture_setRepeated(texture: *mut sfTexture, repeated: SfBool);
    pub fn sfTexture_isRepeated(texture: *const sfTexture) -> SfBool;
    pub fn sfTexture_bind(texture: *mut sfTexture) -> ();
    pub fn sfTexture_getMaximumSize() -> c_uint;

    pub fn sfTransform_fromMatrix(a01: f32, a02: f32, a03: f32, b01: f32, b02: f32, b03: f32, c01: f32, c02: f32, c03: f32) -> Transform;
    pub fn sfTransform_getMatrix(tranform: *const Transform, matrix: *mut f32) -> ();
    pub fn sfTransform_getInverse(transform: *const Transform) -> Transform;
    pub fn sfTransform_transformPoint(transform: *const Transform, point: Vector2f) -> Vector2f;
    pub fn sfTransform_transformRect(transform: *const Transform, rectangle: FloatRect) -> FloatRect;
    pub fn sfTransform_combine(transform: *mut Transform, other: *const Transform) -> ();
    pub fn sfTransform_translate(transform: *mut Transform, x: c_float, y: c_float) -> ();
    pub fn sfTransform_rotate(transform: *mut Transform, angle: c_float) -> ();
    pub fn sfTransform_rotateWithCenter(transform: *mut Transform, angle: c_float, center_x: c_float, center_y: c_float) -> ();
    pub fn sfTransform_scale(transform: *mut Transform, scale_x: c_float, scale_y: c_float) -> ();
    pub fn sfTransform_scaleWithCenter(transform: *mut Transform, scale_x: c_float, scale_y: c_float, center_x: c_float, center_y: c_float) -> ();

    pub fn sfTransformable_create() -> *mut sfTransformable;
    pub fn sfTransformable_copy(transformable: *const sfTransformable) -> *mut sfTransformable;
    pub fn sfTransformable_destroy(transformable: *mut sfTransformable) -> ();
    pub fn sfTransformable_setPosition(transformable: *mut sfTransformable, position: Vector2f) -> ();
    pub fn sfTransformable_setRotation(transformable: *mut sfTransformable, angle: c_float) -> ();
    pub fn sfTransformable_setScale(transformable: *mut sfTransformable, scale: Vector2f) -> ();
    pub fn sfTransformable_setOrigin(transformable: *mut sfTransformable, origin: Vector2f) -> ();
    pub fn sfTransformable_getPosition(transformable: *const sfTransformable) -> Vector2f;
    pub fn sfTransformable_getRotation(transformable: *const sfTransformable) -> c_float;
    pub fn sfTransformable_getScale(transformable: *const sfTransformable) -> Vector2f;
    pub fn sfTransformable_getOrigin(transformable: *const sfTransformable) -> Vector2f;
    pub fn sfTransformable_move(transformable: *mut sfTransformable, offset: Vector2f) -> ();
    pub fn sfTransformable_rotate(transformable: *mut sfTransformable, angle: c_float) -> ();
    pub fn sfTransformable_scale(transformable: *mut sfTransformable, factors: Vector2f) -> ();
    pub fn sfTransformable_getTransform(transformable: *const sfTransformable) -> Transform;
    pub fn sfTransformable_getInverseTransform(transformable: *const sfTransformable) -> Transform;

	/*
    pub fn sfVertexArray_create() -> *mut sfVertexArray;
    pub fn sfVertexArray_copy(vertexArray: *const sfVertexArray) -> *mut sfVertexArray;
    pub fn sfVertexArray_destroy(vertexArray: *mut sfVertexArray) -> ();
    pub fn sfVertexArray_getVertexCount(vertexArray: *mut sfVertexArray) -> c_uint;
    pub fn sfVertexArray_getVertex(vertexArray: *mut sfVertexArray, index: c_uint) -> *mut Vertex;
    pub fn sfVertexArray_clear(vertexArray: *mut sfVertexArray) -> ();
    pub fn sfVertexArray_resize(vertexArray: *mut sfVertexArray, vertexCount: c_uint) -> ();
    pub fn sfVertexArray_append(vertexArray: *mut sfVertexArray, vertex: Vertex) -> ();
    pub fn sfVertexArray_setPrimitiveType(vertexArray: *mut sfVertexArray, stype: sfPrimitiveType) -> ();
    pub fn sfVertexArray_getPrimitiveType(vertexArray: *mut sfVertexArray) -> sfPrimitiveType;
    pub fn sfVertexArray_getBounds(vertexArray: *mut sfVertexArray) -> FloatRect;
	*/

    pub fn sfView_create() -> *mut sfView;
    pub fn sfView_createFromRect(rectangle: FloatRect) -> *mut sfView;
    pub fn sfView_copy(view: *const sfView) -> *mut sfView;
    pub fn sfView_destroy(view: *mut sfView) -> ();
    pub fn sfView_setCenter(view: *mut sfView, center: Vector2f) -> ();
    pub fn sfView_setSize(view: *mut sfView, size: Vector2f) -> ();
    pub fn sfView_setRotation(view: *mut sfView, angle: c_float) -> ();
    pub fn sfView_setViewport(view: *mut sfView, viewport: FloatRect) -> ();
    pub fn sfView_reset(view: *mut sfView, rectangle: FloatRect) -> ();
    pub fn sfView_getCenter(view: *const sfView) -> Vector2f;
    pub fn sfView_getSize(view: *const sfView) -> Vector2f;
    pub fn sfView_getRotation(view: *const sfView) -> c_float;
    pub fn sfView_getViewport(view: *const sfView) -> FloatRect;
    pub fn sfView_move(view: *mut sfView, offset: Vector2f) -> ();
    pub fn sfView_rotate(view: *mut sfView, angle: c_float) -> ();
    pub fn sfView_zoom(view: *mut sfView, factor: c_float) -> ();
}

// InputStream isn't properly #[repr(C)] due to containing a PhantomData.
#[allow(improper_ctypes)]
extern "C" {
	pub fn sfFont_createFromStream(stream: *mut InputStream) -> *mut sfFont;
	pub fn sfImage_createFromStream(stream: *mut InputStream) -> *mut sfImage;
	pub fn sfShader_createFromStream(vertex_stream: *mut InputStream, fragment_stream: *mut InputStream) -> *mut sfShader;
	pub fn sfTexture_createFromStream(stream: *mut InputStream) -> *mut sfTexture;
}
