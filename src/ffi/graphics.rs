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

pub mod render_window {
    use std::libc::{c_uint, c_float, c_char};

    use system::vector2::{Vector2f, Vector2i, Vector2u};
    use window::ContextSettings;
    use graphics::{Color, IntRect};

    use ffi::window::video_mode::sfVideoMode;
    use ffi::graphics::text::sfText;
    use ffi::graphics::sprite::sfSprite;
    use ffi::graphics::circle_shape::sfCircleShape;
    use ffi::graphics::rectangle_shape::sfRectangleShape;
    use ffi::graphics::convex_shape::sfConvexShape;
    use ffi::graphics::render_states::sfRenderStates;
    use ffi::graphics::view::sfView;
    use ffi::graphics::image::sfImage;
    use ffi::graphics::shape::sfShape;
    use ffi::graphics::vertex_array::sfVertexArray;
    use ffi::sfml_types::SfBool;

    pub struct sfRenderWindow;

    pub struct sfEvent {
        pub typeEvent : c_uint,
        pub p1 :        c_uint,
        pub p2 :        c_uint,
        pub p3 :        c_float,
        pub p4 :        c_uint,
        pub p5 :        c_uint
    }

    extern "C" {
        pub fn sfRenderWindow_create(mode : sfVideoMode, title : *c_char, style : c_uint, settings : *ContextSettings) -> *sfRenderWindow;
        pub fn sfRenderWindow_createUnicode(mode : sfVideoMode, title : *u32, style : c_uint, settings : *ContextSettings) -> *sfRenderWindow;
        //fn sfRenderWindow_createFromHandle(handle : sfWindowHandle, settings : *sfContextSettings) -> *sfRenderWindow;
        pub fn sfRenderWindow_destroy(renderWindow : *sfRenderWindow) -> ();
        pub fn sfRenderWindow_close(renderWindow : *sfRenderWindow) -> ();
        pub fn sfRenderWindow_isOpen(renderWindow : *sfRenderWindow) -> SfBool;
        pub fn sfRenderWindow_getSettings(renderWindow : *sfRenderWindow) -> ContextSettings;
        pub fn sfRenderWindow_pollEvent(renderWindow : *sfRenderWindow, event : *sfEvent) -> SfBool;
        pub fn sfRenderWindow_waitEvent(renderWindow : *sfRenderWindow, event : *sfEvent) -> SfBool;
        pub fn sfRenderWindow_getPosition(renderWindow : *sfRenderWindow) -> Vector2i;
        pub fn sfRenderWindow_setPosition(renderWindow : *sfRenderWindow, position : Vector2i) -> ();
        pub fn sfRenderWindow_getSize(renderWindow : *sfRenderWindow) -> Vector2u;
        pub fn sfRenderWindow_setSize(renderWindow : *sfRenderWindow, size : Vector2u) -> ();
        pub fn sfRenderWindow_setTitle(renderWindow : *sfRenderWindow, title : *c_char) -> ();
        pub fn sfRenderWindow_setUnicodeTitle(renderWindow : *sfRenderWindow, title : *u32) -> ();
        pub fn sfRenderWindow_setIcon(renderWindow : *sfRenderWindow, width : c_uint, height : c_uint, pixels : *u8) -> ();
        pub fn sfRenderWindow_setVisible(renderWindow : *sfRenderWindow, visible : SfBool) -> ();
        pub fn sfRenderWindow_setMouseCursorVisible(renderWindow : *sfRenderWindow, show : SfBool) -> ();
        pub fn sfRenderWindow_setVerticalSyncEnabled(renderWindow : *sfRenderWindow, enabled : SfBool) -> ();
        pub fn sfRenderWindow_setKeyRepeatEnabled(renderWindow : *sfRenderWindow, enabled : SfBool) -> ();
        pub fn sfRenderWindow_setActive(renderWindow : *sfRenderWindow, active : SfBool) -> SfBool;
        pub fn sfRenderWindow_display(renderWindow : *sfRenderWindow) -> ();
        pub fn sfRenderWindow_setFramerateLimit(renderWindow : *sfRenderWindow, limit : c_uint) -> ();
        pub fn sfRenderWindow_setJoystickThreshold(renderWindow : *sfRenderWindow, treshold : c_float) -> ();
        // fn sfRenderWindow_getSystemHandle(renderWindow : *sfRenderWindow) -> sfWindowHandle;
        pub fn sfRenderWindow_clear(renderWindow : *sfRenderWindow, color : Color) -> ();
        pub fn sfRenderWindow_setView(renderWindow : *sfRenderWindow, view : *sfView) -> ();
        pub fn sfRenderWindow_getView(renderWindow : *sfRenderWindow) -> *sfView;
        pub fn sfRenderWindow_getDefaultView(renderWindow : *sfRenderWindow) -> *sfView;
        pub fn sfRenderWindow_getViewport(renderWindow : *sfRenderWindow, view : *sfView) -> IntRect;
        pub fn sfRenderWindow_mapPixelToCoords(renderWindow : *sfRenderWindow, point : Vector2i, view : *sfView) -> Vector2f;
        pub fn sfRenderWindow_mapCoordsToPixel(renderWindow : *sfRenderWindow, point : Vector2f, view : *sfView) -> Vector2i;
        pub fn sfRenderWindow_drawSprite(renderWindow : *sfRenderWindow, object : *sfSprite, states : *sfRenderStates) -> ();
        pub fn sfRenderWindow_drawText(renderWindow : *sfRenderWindow, object : *sfText, states : *sfRenderStates) -> ();
        pub fn sfRenderWindow_drawShape(renderWindow : *sfRenderWindow, object : *sfShape, states : *sfRenderStates) -> ();
        pub fn sfRenderWindow_drawCircleShape(renderWindow : *sfRenderWindow, object : *sfCircleShape, states : *sfRenderStates) -> ();
        pub fn sfRenderWindow_drawConvexShape(renderWindow : *sfRenderWindow, object : *sfConvexShape, states : *sfRenderStates) -> ();
        pub fn sfRenderWindow_drawRectangleShape(renderWindow : *sfRenderWindow, object : *sfRectangleShape, states : *sfRenderStates) -> ();
        pub fn sfRenderWindow_drawVertexArray(renderWindow : *sfRenderWindow, object : *sfVertexArray, states : *sfRenderStates) -> ();
        // fn sfRenderWindow_drawPrimitives(renderWindow : *sfRenderWindow, vertices : *sfVertex, vertexCount : c_uint, ttype : sfPrimitiveType, states : *sfRenderStates) -> ();  
        pub fn sfRenderWindow_pushGLStates(renderWindow : *sfRenderWindow) -> ();
        pub fn sfRenderWindow_popGLStates(renderWindow : *sfRenderWindow) -> ();
        pub fn sfRenderWindow_resetGLStates(renderWindow : *sfRenderWindow) -> ();
        pub fn sfRenderWindow_capture(renderWindow : *sfRenderWindow) -> *sfImage;
        pub fn sfMouse_getPositionRenderWindow(relativeTo : *sfRenderWindow) -> Vector2i;
        pub fn sfMouse_setPositionRenderWindow(position : Vector2i, relativeTo : *sfRenderWindow) -> ();
    }
}

pub mod circle_shape {

    use std::libc::{c_void, c_float, c_uint};

    use system::vector2::Vector2f;
    use graphics::{Color, Transform, IntRect, FloatRect};

    use ffi::graphics::texture::sfTexture;
    use ffi::sfml_types::SfBool;

    pub struct sfCircleShape {
        this :              *c_void,
        texture :           *sfTexture,
        transform :         Transform,
        inverseTransform :  Transform
    }

    extern "C" {
        pub fn sfCircleShape_create() -> *sfCircleShape;
        pub fn sfCircleShape_copy(shape : *sfCircleShape) -> *sfCircleShape;
        pub fn sfCircleShape_destroy(shape : *sfCircleShape) -> ();
        pub fn sfCircleShape_setPosition(shape : *sfCircleShape, position : Vector2f) -> ();
        pub fn sfCircleShape_setRotation(shape : *sfCircleShape, angle : c_float) -> ();
        pub fn sfCircleShape_setScale(shape : *sfCircleShape, scale : Vector2f) -> ();
        pub fn sfCircleShape_setOrigin(shape : *sfCircleShape, origin : Vector2f) -> ();
        pub fn sfCircleShape_getPosition(shape : *sfCircleShape) -> Vector2f;
        pub fn sfCircleShape_getRotation(shape : *sfCircleShape) -> c_float;
        pub fn sfCircleShape_getScale(shape : *sfCircleShape) -> Vector2f;
        pub fn sfCircleShape_getOrigin(shape : *sfCircleShape) -> Vector2f;
        pub fn sfCircleShape_move(shape : *sfCircleShape, offset : Vector2f) -> ();
        pub fn sfCircleShape_rotate(shape : *sfCircleShape, angle : c_float) -> ();
        pub fn sfCircleShape_scale(shape : *sfCircleShape, factors : Vector2f) -> ();
        pub fn sfCircleShape_getTransform(shape : *sfCircleShape) -> Transform;
        pub fn sfCircleShape_getInverseTransform(shape : *sfCircleShape) -> Transform;
        pub fn sfCircleShape_setTexture(shape : *sfCircleShape, texture : *sfTexture, reset_rect : SfBool) -> ();
        pub fn sfCircleShape_setTextureRect(shape : *sfCircleShape, rect : IntRect) -> ();
        pub fn sfCircleShape_setFillColor(shape : *sfCircleShape, color : Color) -> ();
        pub fn sfCircleShape_setOutlineColor(shape : *sfCircleShape, color : Color) -> ();
        pub fn sfCircleShape_setOutlineThickness(shape : *sfCircleShape, thickness : c_float) -> ();
        pub fn sfCircleShape_getTexture(shape : *sfCircleShape) -> *sfTexture;
        pub fn sfCircleShape_getTextureRect(shape : *sfCircleShape) -> IntRect;
        pub fn sfCircleShape_getFillColor(shape : *sfCircleShape) -> Color;
        pub fn sfCircleShape_getOutlineColor(shape : *sfCircleShape) -> Color;
        pub fn sfCircleShape_getOutlineThickness(shape : *sfCircleShape) -> c_float;
        pub fn sfCircleShape_getPointCount(shape : *sfCircleShape) -> c_uint;
        pub fn sfCircleShape_getPoint(shape : *sfCircleShape, index : c_uint) -> ();
        pub fn sfCircleShape_setRadius(shape : *sfCircleShape, radius : c_float) -> ();
        pub fn sfCircleShape_getRadius(shape : *sfCircleShape) -> c_float;
        pub fn sfCircleShape_setPointCount(shape : *sfCircleShape, count : c_uint) -> ();
        pub fn sfCircleShape_getLocalBounds(shape : *sfCircleShape) -> FloatRect;
        pub fn sfCircleShape_getGlobalBounds(shape : *sfCircleShape) -> FloatRect;
    }
}

pub mod color {

    use graphics::Color;

    extern "C" {
        pub fn sfColor_fromRGB(red : u8, green : u8, blue : u8) -> Color;
        pub fn sfColor_fromRGBA(red : u8, green : u8, blue : u8, alpha : u8) -> Color;
        pub fn sfColor_add(color1 : Color, color2 : Color) -> Color;
        pub fn sfColor_modulate(color1 : Color, color2 : Color) -> Color;
    }
}

pub mod convex_shape {

    use std::libc::{c_uint, c_void, c_float};

    use system::vector2::Vector2f;
    use graphics::{Color, Transform, FloatRect, IntRect};

    use ffi::graphics::texture::sfTexture;
    use ffi::sfml_types::SfBool;

    pub struct sfConvexShape {
        this :              *c_void,
        texture :           *sfTexture,
        transform :         Transform,
        inverseTransform :  Transform
    }

    extern "C" {
        pub fn sfConvexShape_create() -> *sfConvexShape;
        pub fn sfConvexShape_copy(shape : *sfConvexShape) -> *sfConvexShape;
        pub fn sfConvexShape_destroy(shape : *sfConvexShape) -> ();
        pub fn sfConvexShape_setPosition(shape : *sfConvexShape, position : Vector2f) -> ();
        pub fn sfConvexShape_setRotation(shape : *sfConvexShape, angle : c_float) -> ();
        pub fn sfConvexShape_setScale(shape : *sfConvexShape, scale : Vector2f) -> ();
        pub fn sfConvexShape_setOrigin(shape : *sfConvexShape, origin : Vector2f) -> ();
        pub fn sfConvexShape_getPosition(shape : *sfConvexShape) -> Vector2f;
        pub fn sfConvexShape_getRotation(shape : *sfConvexShape) -> c_float;
        pub fn sfConvexShape_getScale(shape : *sfConvexShape) -> Vector2f;
        pub fn sfConvexShape_getOrigin(shape : *sfConvexShape) -> Vector2f;
        pub fn sfConvexShape_move(shape : *sfConvexShape, offset : Vector2f) -> ();
        pub fn sfConvexShape_rotate(shape : *sfConvexShape, angle : c_float) -> ();
        pub fn sfConvexShape_scale(shape : *sfConvexShape, factors : Vector2f) -> ();
        pub fn sfConvexShape_getTransform(shape : *sfConvexShape) -> Transform;
        pub fn sfConvexShape_getInverseTransform(shape : *sfConvexShape) -> Transform;
        pub fn sfConvexShape_setTexture(shape : *sfConvexShape, texture : *sfTexture, reset_rect : SfBool) -> ();
        pub fn sfConvexShape_setTextureRect(shape : *sfConvexShape, rect : IntRect) -> ();
        pub fn sfConvexShape_setFillColor(shape : *sfConvexShape, color : Color) -> ();
        pub fn sfConvexShape_setOutlineColor(shape : *sfConvexShape, color : Color) -> ();
        pub fn sfConvexShape_setOutlineThickness(shape : *sfConvexShape, thickness : c_float) -> ();
        pub fn sfConvexShape_getTexture(shape : *sfConvexShape) -> *sfTexture;
        pub fn sfConvexShape_getTextureRect(shape : *sfConvexShape) -> IntRect;
        pub fn sfConvexShape_getFillColor(shape : *sfConvexShape) -> Color;
        pub fn sfConvexShape_getOutlineColor(shape : *sfConvexShape) -> Color;
        pub fn sfConvexShape_getOutlineThickness(shape : *sfConvexShape) -> c_float;
        pub fn sfConvexShape_getPointCount(shape : *sfConvexShape) -> c_uint;
        pub fn sfConvexShape_getPoint(shape : *sfConvexShape, index : c_uint) -> Vector2f;
        pub fn sfConvexShape_setPointCount(shape : *sfConvexShape, count : c_uint) -> ();
        pub fn sfConvexShape_setPoint(shape : *sfConvexShape, index : c_uint, point : Vector2f) -> ();
        pub fn sfConvexShape_getLocalBounds(shape : *sfConvexShape) -> FloatRect;
        pub fn sfConvexShape_getGlobalBounds(shape : *sfConvexShape) -> FloatRect;
    }
}

pub mod font {
    use std::libc::{c_void, c_uint, c_int, c_char};

    use graphics::Glyph;

    use ffi::graphics::texture::sfTexture;
    use ffi::sfml_types::SfBool;

    pub struct sfFont {
        this : *c_void
    }

    extern "C" {
        pub fn sfFont_createFromFile(filename : *c_char) -> *sfFont;
        pub fn sfFont_copy(font : *sfFont) -> *sfFont;
        // fn sfFont_createFromMemory(data : *c_void, sizeInBytes : size_t) -> *sfFont;
        // fn sfFont_createFromStream(stream : *sfInputStream) -> *sfFont;
        pub fn sfFont_destroy(font : *sfFont) -> ();
        pub fn sfFont_getGlyph(font : *sfFont, codepoint : u32, characterSize : c_uint, bold :SfBool) -> Glyph;
        pub fn sfFont_getKerning(font : *sfFont, first : u32, second : u32, characterSize : c_uint) -> c_int;
        pub fn sfFont_getLineSpacing(font : *sfFont, characterSize : c_uint) -> c_int;
        pub fn sfFont_getTexture(font : *sfFont, characterSize : c_uint) -> *sfTexture;
    }
}

pub mod image {
    use std::libc::{c_void, c_uint, c_char};

    use graphics::{Color, IntRect};
    use system::vector2::Vector2u;

    use ffi::sfml_types::SfBool;

    pub struct sfImage {
        this : *c_void
    }

    extern "C" {
        pub fn sfImage_create(width : c_uint, height : c_uint) -> *sfImage;
        pub fn sfImage_createFromColor(width : c_uint, height : c_uint, color : Color) -> *sfImage;
        pub fn sfImage_createFromPixels(width : c_uint, height : c_uint, pixels : *u8) -> *sfImage;
        pub fn sfImage_createFromFile(filename : *c_char) -> *sfImage;
        //fn sfImage_createFromMemory(data : *c_void, size : size_t) -> *sfImage;
        //fn sfImage_createFromStream(stream : *sfInputStream) -> *sfImage;
        pub fn sfImage_copy(image : *sfImage) -> *sfImage;
        pub fn sfImage_destroy(image : *sfImage) -> ();
        pub fn sfImage_saveToFile(image : *sfImage, filename : *c_char) -> SfBool;
        pub fn sfImage_getSize(image : *sfImage) -> Vector2u;
        pub fn sfImage_createMaskFromColor(image : *sfImage, color : Color, alpha : u8) -> ();
        pub fn sfImage_copyImage(image : *sfImage, source : *sfImage, destX : c_uint, destY : c_uint, sourceRect : IntRect, applyAlpha : SfBool) -> ();
        pub fn sfImage_setPixel(image : *sfImage, x : c_uint, y : c_uint, color : Color) -> ();
        pub fn sfImage_getPixel(image : *sfImage, x : c_uint, y : c_uint) -> Color;
        pub fn sfImage_getPixelsPtr(image : *sfImage) -> *u8;
        pub fn sfImage_flipHorizontally(image : *sfImage) -> ();
        pub fn sfImage_flipVertically(image : *sfImage) -> ();
    }
}

pub mod rect {
    use std::libc::{c_int};
    use graphics::{FloatRect, IntRect};

    use ffi::sfml_types::{SfBool};

    extern "C" {
        pub fn sfIntRect_contains(rect : *IntRect, x : c_int, y : c_int) -> SfBool;
        pub fn sfIntRect_intersects(rect1 : *IntRect, rect2 : *IntRect, intersectons : *IntRect) -> SfBool;
        pub fn sfFloatRect_intersects(rect1 : *FloatRect, rect2 : *FloatRect, intersectons : *FloatRect) -> SfBool;
        pub fn sfFloatRect_contains(rect : *FloatRect, x : f32, y : f32) -> SfBool;
    }
}

pub mod rectangle_shape {
    use std::libc::{c_void, c_float, c_uint};

    use system::vector2::Vector2f;
    use graphics::{Color, Transform, FloatRect, IntRect};

    use ffi::graphics::texture::sfTexture;
    use ffi::sfml_types::SfBool;

    pub struct sfRectangleShape {
        this :              *c_void,
        texture :           *sfTexture,
        transform :         Transform,
        inverseTransform :  Transform
    }

    extern "C" {
        pub fn sfRectangleShape_create() -> *sfRectangleShape;
        pub fn sfRectangleShape_copy(shape : *sfRectangleShape) -> *sfRectangleShape;
        pub fn sfRectangleShape_destroy(shape : *sfRectangleShape) -> ();
        pub fn sfRectangleShape_setPosition(shape : *sfRectangleShape, position : Vector2f) -> ();
        pub fn sfRectangleShape_setRotation(shape : *sfRectangleShape, angle : c_float) -> ();
        pub fn sfRectangleShape_setScale(shape : *sfRectangleShape, scale : Vector2f) -> ();
        pub fn sfRectangleShape_setOrigin(shape : *sfRectangleShape, origin : Vector2f) -> ();
        pub fn sfRectangleShape_getPosition(shape : *sfRectangleShape) -> Vector2f;
        pub fn sfRectangleShape_getRotation(shape : *sfRectangleShape) -> c_float;
        pub fn sfRectangleShape_getScale(shape : *sfRectangleShape) -> Vector2f;
        pub fn sfRectangleShape_getOrigin(shape : *sfRectangleShape) -> Vector2f;
        pub fn sfRectangleShape_move(shape : *sfRectangleShape, offset : Vector2f) -> ();
        pub fn sfRectangleShape_rotate(shape : *sfRectangleShape, angle : c_float) -> ();
        pub fn sfRectangleShape_scale(shape : *sfRectangleShape, factors : Vector2f) -> ();
        pub fn sfRectangleShape_getTransform(shape : *sfRectangleShape) -> Transform;
        pub fn sfRectangleShape_getInverseTransform(shape : *sfRectangleShape) -> Transform;
        pub fn sfRectangleShape_setTexture(shape : *sfRectangleShape, texture : *sfTexture, reset_rect : SfBool) -> ();
        pub fn sfRectangleShape_setTextureRect(shape : *sfRectangleShape, rect : IntRect) -> ();
        pub fn sfRectangleShape_setFillColor(shape : *sfRectangleShape, color : Color) -> ();
        pub fn sfRectangleShape_setOutlineColor(shape : *sfRectangleShape, color : Color) -> ();
        pub fn sfRectangleShape_setOutlineThickness(shape : *sfRectangleShape, thickness : c_float) -> ();
        pub fn sfRectangleShape_getTexture(shape : *sfRectangleShape) -> *sfTexture;
        pub fn sfRectangleShape_getTextureRect(shape : *sfRectangleShape) -> IntRect;
        pub fn sfRectangleShape_getFillColor(shape : *sfRectangleShape) -> Color;
        pub fn sfRectangleShape_getOutlineColor(shape : *sfRectangleShape) -> Color;
        pub fn sfRectangleShape_getOutlineThickness(shape : *sfRectangleShape) -> c_float;
        pub fn sfRectangleShape_getPointCount(shape : *sfRectangleShape) -> c_uint;
        pub fn sfRectangleShape_getPoint(shape : *sfRectangleShape, index : c_uint) -> Vector2f;
        pub fn sfRectangleShape_setSize(shape : *sfRectangleShape, size : Vector2f) -> ();
        pub fn sfRectangleShape_getSize(shape : *sfRectangleShape) -> Vector2f;
        pub fn sfRectangleShape_getLocalBounds(shape : *sfRectangleShape) -> FloatRect;
        pub fn sfRectangleShape_getGlobalBounds(shape : *sfRectangleShape) -> FloatRect;
    }
}

pub mod render_states {
    use graphics::Transform;

    use ffi::graphics::shader::sfShader;
    use ffi::graphics::texture::sfTexture;

    pub struct sfRenderStates {
        pub blendMode : i32,
        pub transform : Transform,
        pub texture :   *sfTexture,
        pub shader :    *sfShader
    }
}

pub mod shader {

    use std::libc::{c_void, c_float, c_char};

    use graphics::{Transform, Color};
    use system::vector2::Vector2f;
    use system::vector3::Vector3f;

    use ffi::graphics::texture::sfTexture;
    use ffi::sfml_types::SfBool;

    pub struct sfShader {
        this : *c_void
    }

    extern "C" {
        pub fn sfShader_createFromFile(vertexShaderFilename : *c_char, fragmentShaderFilename : *c_char) -> *sfShader;
        pub fn sfShader_createFromMemory(vertexShader : *c_char, fragmentShader : *c_char) -> *sfShader;
        //fn sfShader_createFromStream(vertexShaderStream : *sfInputStream, fragmentShaderStream : *sfInputStream) -> *sfShader;
        pub fn sfShader_destroy(shader : *sfShader)-> ();
        pub fn sfShader_setFloatParameter(shader : *sfShader, name : *c_char, x : c_float) -> ();
        pub fn sfShader_setFloat2Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float) -> ();
        pub fn sfShader_setFloat3Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float, z : c_float) -> ();
        pub fn sfShader_setFloat4Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float, z : c_float, w : c_float) -> ();
        pub fn sfShader_setVector2Parameter(shader : *sfShader, name : *c_char, vector : Vector2f) -> ();
        pub fn sfShader_setVector3Parameter(shader : *sfShader, name : *c_char, vector : Vector3f) -> ();
        pub fn sfShader_setColorParameter(shader : *sfShader, name : *c_char, color : Color) -> (); 
        pub fn sfShader_setTransformParameter(shader : *sfShader, name : *c_char, transform : Transform) -> ();
        pub fn sfShader_setTextureParameter(shader : *sfShader, name : *c_char, texture : *sfTexture) -> ();
        pub fn sfShader_setCurrentTextureParameter(shader : *sfShader, name : *c_char) -> ();
        pub fn sfShader_bind(shader : *sfShader) -> ();
        pub fn sfShader_isAvailable() -> SfBool;
   }
}

#[doc(hidden)]
pub mod render_texture {
    use std::libc::{c_void, c_uint};

    use system::vector2::{Vector2f, Vector2i, Vector2u};
    use graphics::{Color, IntRect};

    use ffi::graphics::sprite::sfSprite;
    use ffi::graphics::render_states::sfRenderStates;
    use ffi::graphics::texture::sfTexture;
    use ffi::graphics::text::sfText;
    use ffi::graphics::circle_shape::sfCircleShape;
    use ffi::graphics::rectangle_shape::sfRectangleShape;
    use ffi::graphics::vertex_array::sfVertexArray;
    use ffi::graphics::convex_shape::sfConvexShape;
    use ffi::graphics::shape::sfShape;
    use ffi::graphics::view::sfView;
    use ffi::sfml_types::SfBool;

    pub struct sfRenderTexture {
        this :          *c_void,
        target :        *sfTexture,
        defaultView :   sfView,
        currentView :   sfView
    }

    extern "C" {
        pub fn sfRenderTexture_create(width : c_uint, height : c_uint, depthBuffer : SfBool) -> *sfRenderTexture;
        pub fn sfRenderTexture_destroy(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_getSize(renderTexture : *sfRenderTexture) -> Vector2u;
        pub fn sfRenderTexture_setActive(renderTexture : *sfRenderTexture, active : SfBool) -> SfBool;
        pub fn sfRenderTexture_display(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_clear(renderTexture : *sfRenderTexture, color : Color) -> ();
        pub fn sfRenderTexture_setView(renderTexture : *sfRenderTexture, view : *sfView) -> ();
        pub fn sfRenderTexture_getView(renderTexture : *sfRenderTexture) -> *sfView;
        pub fn sfRenderTexture_getDefaultView(renderTexture : *sfRenderTexture) -> *sfView;
        pub fn sfRenderTexture_getViewport(renderTexture : *sfRenderTexture, view : *sfView) -> IntRect;
        pub fn sfRenderTexture_mapPixelToCoords(renderTexture : *sfRenderTexture, point : Vector2i, view : *sfView) -> Vector2f;
        pub fn sfRenderTexture_mapCoordsToPixel(renderTexture : *sfRenderTexture, point : Vector2f, view : *sfView) -> Vector2i;
        pub fn sfRenderTexture_drawSprite(renderTexture : *sfRenderTexture, object : *sfSprite, states : *sfRenderStates) -> ();
        pub fn sfRenderTexture_drawText(renderTexture : *sfRenderTexture, object : *sfText, states : *sfRenderStates) -> ();
        pub fn sfRenderTexture_drawShape(renderTexture : *sfRenderTexture, object : *sfShape, states : *sfRenderStates) -> ();
        pub fn sfRenderTexture_drawCircleShape(renderTexture : *sfRenderTexture, object : *sfCircleShape, states : *sfRenderStates) -> ();
        pub fn sfRenderTexture_drawConvexShape(renderTexture : *sfRenderTexture, object : *sfConvexShape, states : *sfRenderStates) -> ();
        pub fn sfRenderTexture_drawRectangleShape(renderTexture : *sfRenderTexture, object : *sfRectangleShape, states : *sfRenderStates) -> ();
        pub fn sfRenderTexture_drawVertexArray(renderTexture : *sfRenderTexture, object : *sfVertexArray, states : *sfRenderStates) -> ();
        //fn sfRenderTexture_drawPrimitives(renderTexture : *sfRenderTexture) -> (); // a modifier
        pub fn sfRenderTexture_pushGLStates(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_popGLStates(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_resetGLStates(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_getTexture(renderTexture : *sfRenderTexture) -> *sfTexture;
        pub fn sfRenderTexture_setSmooth(renderTexture : *sfRenderTexture, smooth : SfBool) -> ();
        pub fn sfRenderTexture_isSmooth(renderTexture : *sfRenderTexture) -> SfBool;
    }
}

pub mod shape {

    use std::libc::{c_void, c_float, c_uint};

    use graphics::{Color, Transform, IntRect, FloatRect};
    use system::vector2::Vector2f;

    use ffi::graphics::texture::sfTexture;
    use ffi::sfml_types::SfBool;

    pub struct sfShape {
        this :              *c_void,
        texture :           *sfTexture,
        transform :         Transform,
        inverseTransform :  Transform
    }

    extern "C" {
        pub fn sfShape_create(getPointCount : extern "C" fn(*c_void) -> u32, getPoint : extern "C" fn(u32, *c_void) -> Vector2f, userData : *c_void) -> *sfShape;
        pub fn sfShape_destroy(shape : *sfShape) -> ();
        pub fn sfShape_setPosition(shape : *sfShape, position : Vector2f) -> ();
        pub fn sfShape_setRotation(shape : *sfShape, angle : c_float) -> ();
        pub fn sfShape_setScale(shape : *sfShape, scale : Vector2f) -> ();
        pub fn sfShape_setOrigin(shape : *sfShape, origin : Vector2f) -> ();
        pub fn sfShape_getPosition(shape : *sfShape) -> Vector2f;
        pub fn sfShape_getRotation(shape : *sfShape) -> c_float;
        pub fn sfShape_getScale(shape : *sfShape) -> Vector2f;
        pub fn sfShape_getOrigin(shape : *sfShape) -> Vector2f;
        pub fn sfShape_move(shape : *sfShape, offset : Vector2f) -> ();
        pub fn sfShape_rotate(shape : *sfShape, angle : c_float) -> ();
        pub fn sfShape_scale(shape : *sfShape, factors : Vector2f) -> ();
        pub fn sfShape_getTransform(shape : *sfShape) -> Transform;
        pub fn sfShape_getInverseTransform(shape : *sfShape) -> Transform;
        pub fn sfShape_setTexture(shape : *sfShape, texture : *sfTexture, reset_rect : SfBool) -> ();
        pub fn sfShape_setTextureRect(shape : *sfShape, rect : IntRect) -> ();
        pub fn sfShape_setFillColor(shape : *sfShape, color : Color) -> ();
        pub fn sfShape_setOutlineColor(shape : *sfShape, color : Color) -> ();
        pub fn sfShape_setOutlineThickness(shape : *sfShape, thickness : c_float) -> ();
        pub fn sfShape_getTexture(shape : *sfShape) -> *sfTexture;
        pub fn sfShape_getTextureRect(shape : *sfShape) -> IntRect;
        pub fn sfShape_getFillColor(shape : *sfShape) -> Color;
        pub fn sfShape_getOutlineColor(shape : *sfShape) -> Color;
        pub fn sfShape_getOutlineThickness(shape : *sfShape) -> c_float;
        pub fn sfShape_getPointCount(shape : *sfShape) -> c_uint;
        pub fn sfShape_getPoint(shape : *sfShape, index : c_uint) -> Vector2f;
        pub fn sfShape_getLocalBounds(shape : *sfShape) -> FloatRect;
        pub fn sfShape_getGlobalBounds(shape : *sfShape) -> FloatRect;
        pub fn sfShape_update(shape : *sfShape) -> ();
    }
}

pub mod sprite {

    use std::libc::{c_void, c_float};

    use system::vector2::Vector2f;
    use graphics::{Color, Transform, IntRect, FloatRect};

    use ffi::graphics::texture::sfTexture;
    use ffi::sfml_types::SfBool;

    pub struct sfSprite {
        this :              *c_void,
        texture :           *sfTexture,
        transform :         Transform,
        inverseTransform :  Transform
    }

    extern "C" {
        pub fn sfSprite_create() -> *sfSprite;
        pub fn sfSprite_copy(sprite : *sfSprite) -> *sfSprite;
        pub fn sfSprite_destroy(sprite : *sfSprite) -> ();
        pub fn sfSprite_setPosition(sprite : *sfSprite, position : Vector2f) -> ();
        pub fn sfSprite_setRotation(sprite : *sfSprite, angle : c_float) -> ();
        pub fn sfSprite_setScale(sprite : *sfSprite, scale : Vector2f) -> ();
        pub fn sfSprite_setOrigin(sprite : *sfSprite, origin : Vector2f) -> ();
        pub fn sfSprite_getPosition(sprite : *sfSprite) -> Vector2f;
        pub fn sfSprite_getRotation(sprite : *sfSprite) -> c_float;
        pub fn sfSprite_getScale(sprite : *sfSprite) -> Vector2f;
        pub fn sfSprite_getOrigin(sprite : *sfSprite) -> Vector2f;
        pub fn sfSprite_move(sprite : *sfSprite, offset : Vector2f) -> ();
        pub fn sfSprite_rotate(sprite : *sfSprite, angle : c_float) -> ();
        pub fn sfSprite_scale(sprite : *sfSprite, factors : Vector2f) -> ();
        pub fn sfSprite_getTransform(sprite : *sfSprite) -> Transform;
        pub fn sfSprite_getInverseTransform(sprite : *sfSprite) -> Transform;
        pub fn sfSprite_setTexture(sprite : *sfSprite, texture : *sfTexture, reset_rect : SfBool) -> ();
        pub fn sfSprite_setTextureRect(sprite : *sfSprite, rectangle : IntRect) -> ();
        pub fn sfSprite_setColor(sprite : *sfSprite, color : Color) -> ();
        pub fn sfSprite_getTexture(sprite : *sfSprite) -> *sfTexture;
        pub fn sfSprite_getTextureRect(sprite : *sfSprite) -> IntRect;
        pub fn sfSprite_getColor(sprite : *sfSprite) -> Color;
        pub fn sfSprite_getLocalBounds(sprite : *sfSprite) -> FloatRect;
        pub fn sfSprite_getGlobalBounds(sprite : *sfSprite) -> FloatRect;
    }
}

pub mod text {
    use std::libc::{c_uint, c_float, c_void, size_t, c_char};

    use system::vector2::Vector2f;
    use graphics::{Color, Transform, FloatRect};

    use ffi::graphics::font::sfFont;

    pub struct sfText {
        this :          c_void,
        font :          *c_void,
        transform :     Transform,
        transform2 :    Transform
    }

    extern "C" {
        pub fn sfText_create() -> *sfText;
        pub fn sfText_copy(text : *sfText) -> *sfText;
        pub fn sfText_destroy(text : *sfText) -> ();
        pub fn sfText_setPosition(text : *sfText, position : Vector2f) -> ();
        pub fn sfText_setRotation(text : *sfText, angle : c_float) -> ();
        pub fn sfText_setScale(text : *sfText, scale : Vector2f) -> ();
        pub fn sfText_setOrigin(text : *sfText, origin : Vector2f) -> ();
        pub fn sfText_getPosition(text : *sfText) -> Vector2f;
        pub fn sfText_getRotation(text : *sfText) -> c_float;
        pub fn sfText_getScale(text : *sfText) -> Vector2f;
        pub fn sfText_getOrigin(text : *sfText) -> Vector2f;
        pub fn sfText_move(text : *sfText, offset : Vector2f) -> ();
        pub fn sfText_rotate(text : *sfText, angle : c_float) -> ();
        pub fn sfText_scale(text : *sfText, factors : Vector2f) -> ();
        pub fn sfText_getTransform(text : *sfText) -> Transform;
        pub fn sfText_getInverseTransform(text : *sfText) -> Transform;
        pub fn sfText_setString(text : *sfText, string : *c_char) -> ();
        pub fn sfText_setUnicodeString(text : *sfText, string : *u32 ) -> ();
        pub fn sfText_setFont(text : *sfText, font : *sfFont) -> ();
        pub fn sfText_setCharacterSize(text : *sfText, size : c_uint) -> ();
        pub fn sfText_setStyle(text : *sfText, style : u32) -> ();
        pub fn sfText_setColor(text : *sfText, color : Color) -> ();
        pub fn sfText_getString(text : *sfText) -> *c_char;
        pub fn sfText_getUnicodeString(text : *sfText) -> *mut u32;
        pub fn sfText_getFont(text : *sfText) -> *sfFont;
        pub fn sfText_getCharacterSize(text : *sfText) -> c_uint;
        pub fn sfText_getStyle(text : *sfText) -> u32;
        pub fn sfText_getColor(text : *sfText) -> Color;
        pub fn sfText_findCharacterPos(text : *sfText, index : size_t) -> Vector2f;
        pub fn sfText_getLocalBounds(text : *sfText) -> FloatRect;
        pub fn sfText_getGlobalBounds(text : *sfText) -> FloatRect;
    }
}

pub mod texture {

    use std::libc::{c_uint, c_void, c_char};

    use system::vector2::Vector2u;
    use graphics::IntRect;

    use ffi::graphics::render_window::sfRenderWindow;
    use ffi::graphics::image::sfImage;
    use ffi::window::window::sfWindow;
    use ffi::sfml_types::SfBool;

    pub struct sfTexture {
        this : *c_void
    }

    extern "C" {
        pub fn sfTexture_create(width : c_uint, height : c_uint) -> *sfTexture;
        pub fn sfTexture_createFromFile(filename : *c_char, area : *IntRect) -> *sfTexture;
        //fn sfTexture_createFromMemory(data : *c_void, sizeInBytes : size_t , area : *sfIntRect) -> *sfTexture;
        //fn sfTexture_createFromStream(strea; : *sfInputStream, area : *sfIntRect) -> *sfTexture;
        pub fn sfTexture_createFromImage(image :*sfImage, area : *IntRect) -> *sfTexture;
        pub fn sfTexture_copy(texture : *sfTexture) -> *sfTexture;
        pub fn sfTexture_destroy(texture : *sfTexture) -> ();
        pub fn sfTexture_getSize(texture : *sfTexture) -> Vector2u;
        pub fn sfTexture_copyToImage(texture : *sfTexture) -> *sfImage;
        pub fn sfTexture_updateFromPixels(texture : *sfTexture, pixels : *u8, width : c_uint, height : c_uint, x : c_uint, y : c_uint) -> ();
        pub fn sfTexture_updateFromImage(texture : *sfTexture, image : *sfImage, x : c_uint, y : c_uint) -> ();
        pub fn sfTexture_updateFromWindow(texture : *sfTexture, window : *sfWindow, x : c_uint, y : c_uint) -> ();
        pub fn sfTexture_updateFromRenderWindow(texture : *sfTexture, renderWindow : *sfRenderWindow, x : c_uint, y : c_uint) -> ();
        pub fn sfTexture_setSmooth(texture : *sfTexture, smooth : SfBool) -> ();
        pub fn sfTexture_isSmooth(texture : *sfTexture) -> SfBool;
        pub fn sfTexture_setRepeated(texture : *sfTexture, repeated : SfBool);
        pub fn sfTexture_isRepeated(texture : *sfTexture) -> SfBool;
        pub fn sfTexture_bind(texture : *sfTexture) -> ();
        pub fn sfTexture_getMaximumSize() -> c_uint;
    }
}

pub mod transform {

    use std::libc::c_float;

    use system::vector2::Vector2f;
    use graphics::{Transform, FloatRect};

    extern "C" {
        pub fn sfTransform_fromMatrix(a01 : f32, a02 : f32, a03 : f32, b01 : f32, b02 : f32, b03 : f32, c01 : f32, c02 : f32, c03 : f32) -> Transform;
        pub fn sfTransform_getMatrix(tranform : *Transform, matrix : *f32) -> ();
        pub fn sfTransform_getInverse(transform : *Transform) -> Transform;
        pub fn sfTransform_transformPoint(transform : *Transform, point : Vector2f) -> Vector2f;
        pub fn sfTransform_transformRect(transform : *Transform, rectangle : FloatRect) -> FloatRect;
        pub fn sfTransform_combine(transform : *Transform, other : *Transform) -> ();
        pub fn sfTransform_translate(transform : *Transform, x : c_float, y : c_float) -> ();
        pub fn sfTransform_rotate(transform : *Transform, angle : c_float) -> ();
        pub fn sfTransform_rotateWithCenter(transform : *Transform, angle : c_float, center_x : c_float, center_y : c_float) -> ();
        pub fn sfTransform_scale(transform : *Transform, scale_x : c_float, scale_y : c_float) -> ();
        pub fn sfTransform_scaleWithCenter(transform: *Transform, scale_x : c_float, scale_y : c_float, center_x : c_float, center_y : c_float) -> ();
    }
}

pub mod transformable {

    use std::libc::{c_float, c_void};

    use system::vector2::Vector2f;
    use graphics::Transform;

    pub struct sfTransformable {
        this :              *c_void,
        transform :         Transform,
        inverseTransform :  Transform
    }

    extern "C" {
        pub fn sfTransformable_create() -> *sfTransformable;
        pub fn sfTransformable_copy(transformable : *sfTransformable) -> *sfTransformable;
        pub fn sfTransformable_destroy(transformable : *sfTransformable) -> ();
        pub fn sfTransformable_setPosition(transformable : *sfTransformable, position : Vector2f) -> ();
        pub fn sfTransformable_setRotation(transformable : *sfTransformable, angle : c_float) -> ();
        pub fn sfTransformable_setScale(transformable : *sfTransformable, scale : Vector2f) -> ();
        pub fn sfTransformable_setOrigin(transformable : *sfTransformable, origin : Vector2f) -> ();
        pub fn sfTransformable_getPosition(transformable : *sfTransformable) -> Vector2f;
        pub fn sfTransformable_getRotation(transformable : *sfTransformable) -> c_float;
        pub fn sfTransformable_getScale(transformable : *sfTransformable) -> Vector2f;
        pub fn sfTransformable_getOrigin(transformable : *sfTransformable) -> Vector2f;
        pub fn sfTransformable_move(transformable : *sfTransformable, offset : Vector2f) -> ();
        pub fn sfTransformable_rotate(transformable : *sfTransformable, angle : c_float) -> ();
        pub fn sfTransformable_scale(transformable : *sfTransformable, factors : Vector2f) -> ();
        pub fn sfTransformable_getTransform(transformable : *sfTransformable) -> Transform;
        pub fn sfTransformable_getInverseTransform(transformable : *sfTransformable) -> Transform;
    }
}

pub mod vertex_array {

    use std::libc::{c_uint, c_void};

    use graphics::{FloatRect, Vertex};

    pub type sfPrimitiveType = c_uint;
    pub static SFPOINTS :           sfPrimitiveType = 0;
    pub static SFLINES :            sfPrimitiveType = 1;
    pub static SFLINESSTRIP :       sfPrimitiveType = 2;
    pub static SFTRIANGLES :        sfPrimitiveType = 3;
    pub static SFTRIANGLESSTRIP :   sfPrimitiveType = 4;
    pub static SFTRIANGLESFAN :     sfPrimitiveType = 5;
    pub static SFQUADS :            sfPrimitiveType = 6;

    pub struct sfVertexArray {
        this : *c_void
    }

    extern "C" {
        pub fn sfVertexArray_create() -> *sfVertexArray;
        pub fn sfVertexArray_copy(vertexArray : *sfVertexArray) -> *sfVertexArray;
        pub fn sfVertexArray_destroy(vertexArray : *sfVertexArray) -> ();
        pub fn sfVertexArray_getVertexCount(vertexArray : *sfVertexArray) -> c_uint;
        pub fn sfVertexArray_getVertex(vertexArray : *sfVertexArray, index : c_uint) -> *Vertex;
        pub fn sfVertexArray_clear(vertexArray : *sfVertexArray) -> ();
        pub fn sfVertexArray_resize(vertexArray : *sfVertexArray, vertexCount : c_uint) -> ();
        pub fn sfVertexArray_append(vertexArray : *sfVertexArray, vertex : Vertex) -> ();
        pub fn sfVertexArray_setPrimitiveType(vertexArray : *sfVertexArray, stype : sfPrimitiveType) -> ();
        pub fn sfVertexArray_getPrimitiveType(vertexArray : *sfVertexArray) -> sfPrimitiveType;
        pub fn sfVertexArray_getBounds(vertexArray : *sfVertexArray) -> FloatRect;
    }
}

pub mod view {

    use std::libc::{c_float, c_void};

    use system::vector2::Vector2f;
    use graphics::FloatRect;

    pub struct sfView {
        this : *c_void
    }

    extern "C" {
        pub fn sfView_create() -> *sfView;
        pub fn sfView_createFromRect(rectangle : FloatRect) -> *sfView;
        pub fn sfView_copy(view : *sfView) -> *sfView;
        pub fn sfView_destroy(view : *sfView) -> ();
        pub fn sfView_setCenter(view : *sfView, center : Vector2f) -> ();
        pub fn sfView_setSize(view : *sfView, size : Vector2f) -> ();
        pub fn sfView_setRotation(view : *sfView, angle : c_float) -> ();
        pub fn sfView_setViewport(view : *sfView, viewport : FloatRect) -> ();
        pub fn sfView_reset(view : *sfView, rectangle : FloatRect) -> ();
        pub fn sfView_getCenter(view : *sfView) -> Vector2f;
        pub fn sfView_getSize(view : *sfView) -> Vector2f;
        pub fn sfView_getRotation(view : *sfView) -> c_float;
        pub fn sfView_getViewport(view : *sfView) -> FloatRect;
        pub fn sfView_move(view : *sfView, offset : Vector2f) -> ();
        pub fn sfView_rotate(view : *sfView, angle : c_float) -> ();
        pub fn sfView_zoom(view : *sfView, factor : c_float) -> ();
    }
}
