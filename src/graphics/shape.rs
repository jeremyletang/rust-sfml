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
*
*
*
*/

use std::libc::{c_void, c_float, c_uint};
use std::ptr;
use std::cast;
use std::io;
use std::sys;

use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use graphics::render_states::RenderStates;
use traits::drawable::Drawable;
use traits::wrappable::Wrappable;
use graphics::texture::Texture;
use system::vector2::Vector2f;
use graphics::color::Color;
use graphics::transform::Transform;
use graphics::rect::{IntRect, FloatRect};

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_void, c_float, c_uint};

    use rsfml::sfTypes::sfBool;
    use graphics::texture;
    use graphics::color::Color;
    use graphics::transform::Transform;
    use system::vector2::Vector2f;
    use graphics::rect::{IntRect, FloatRect};

    pub struct sfShape {
        This : *c_void,
        Texture : *texture::ffi::sfTexture,
        Transform : Transform,
        InverseTransform : Transform
    }

    extern "C" {
        pub fn sfShape_create(getPointCount : *u8, getPoint : *u8, userData : *c_void) -> *sfShape;
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
        pub fn sfShape_setTexture(shape : *sfShape, texture : *texture::ffi::sfTexture, resetRect : sfBool) -> ();
        pub fn sfShape_setTextureRect(shape : *sfShape, rect : IntRect) -> ();
        pub fn sfShape_setFillColor(shape : *sfShape, color : Color) -> ();
        pub fn sfShape_setOutlineColor(shape : *sfShape, color : Color) -> ();
        pub fn sfShape_setOutlineThickness(shape : *sfShape, thickness : c_float) -> ();
        pub fn sfShape_getTexture(shape : *sfShape) -> *texture::ffi::sfTexture;
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

pub trait AbstractShape {
    pub fn get_point_count(&self) -> u32;
    pub fn get_point(&self, point : u32) -> Vector2f;
}

pub struct WrapObj {
    shape_impl : @AbstractShape
}

pub struct Shape {
    priv shape : *ffi::sfShape,
    priv wrap_obj : WrapObj
}

#[doc(hidden)]
extern fn get_point_count_callback(obj : *c_void) -> u32 {
    let shape = unsafe { cast::transmute::<*c_void, &WrapObj>(obj) };
    io::println("point_count");
    io::println(fmt!("POINT COUNT : %d", shape.shape_impl.get_point_count() as int ));
    shape.shape_impl.get_point_count()
}

#[doc(hidden)]
extern fn get_point_callback(point : u32, obj : *c_void) -> Vector2f {
    let shape = unsafe { cast::transmute::<*c_void, &WrapObj>(obj) };
    io::println("point");
    shape.shape_impl.get_point(point)
}


impl Shape {

    pub fn new(abstractShape : @AbstractShape) -> Option<Shape> {
        let w_o = WrapObj { shape_impl : abstractShape};
        io::println(fmt!("POINT COUNT : %d", w_o.shape_impl.get_point_count() as int ));
        io::println(fmt!("WrapObj : %d", sys::size_of::<&WrapObj>() as int));
        io::println(fmt!("*c_void : %d", sys::size_of::<*c_void>() as int));
        let sp = unsafe { ffi::sfShape_create(get_point_count_callback, get_point_callback, ptr::to_unsafe_ptr(&w_o) as *c_void) };
        if ptr::is_null(sp) {
            None
        }
        else {
            Some(Shape {
                shape : sp,
                wrap_obj : w_o
            })
        }
    }

    pub fn set_position(&mut self, position : &Vector2f) -> () {
        unsafe {
            ffi::sfShape_setPosition(self.shape, *position)
        }
    }

    pub fn set_position2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfShape_setPosition(self.shape, Vector2f::new(x, y))
        }
    }

    pub fn set_rotation(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfShape_setRotation(self.shape, angle as c_float)
        }
    }

    pub fn set_scale(&mut self, scale : &Vector2f) -> () {
        unsafe {
            ffi::sfShape_setScale(self.shape, *scale)
        }
    }
    
    pub fn set_scale2f(&mut self, scaleX : f32, scaleY : f32) -> () {
        unsafe {
            ffi::sfShape_setScale(self.shape, Vector2f::new(scaleX, scaleY))
        }
    }

    pub fn set_origin(&mut self, origin : &Vector2f) -> () {
        unsafe {
            ffi::sfShape_setOrigin(self.shape, *origin)
        }
    }

    pub fn set_origin2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfShape_setOrigin(self.shape, Vector2f::new(x, y))
        }
    }

    pub fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfShape_getPosition(self.shape)
        }
    }

    pub fn get_rotation(&self) -> float {
        unsafe {
            ffi::sfShape_getRotation(self.shape) as float
        }
    }

    pub fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfShape_getScale(self.shape)
        }
    }

    pub fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfShape_getOrigin(self.shape)
        }
    }

    pub fn move(&mut self, offset : &Vector2f) -> () {
        unsafe {
            ffi::sfShape_move(self.shape, *offset)
        }
    }

    pub fn move2f(&mut self, offsetX : f32, offsetY : f32) -> () {
        unsafe {
            ffi::sfShape_move(self.shape, Vector2f::new(offsetX, offsetY))
        }
    }

    pub fn rotate(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfShape_rotate(self.shape, angle as c_float)
        }
    }

    pub fn scale(&mut self, factors : &Vector2f) -> () {
        unsafe {
            ffi::sfShape_scale(self.shape, *factors)
        }
    }

    pub fn scale2f(&mut self, factorX : f32, factorY : f32) -> () {
        unsafe {
            ffi::sfShape_scale(self.shape, Vector2f::new(factorX, factorY))
        }
    }

    pub fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfShape_getTransform(self.shape)
        }
    }

    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfShape_getInverseTransform(self.shape)
        }
    }

    pub fn set_texture(&mut self, texture : &Texture, resetRect : bool) -> () {
        unsafe {
            match resetRect {
                true    => ffi::sfShape_setTexture(self.shape, texture.unwrap(), 1),
                false   => ffi::sfShape_setTexture(self.shape, texture.unwrap(), 0)
            }
        }
    }

    pub fn set_texture_rect(&mut self, rect : &IntRect) -> () {
        unsafe {
            ffi::sfShape_setTextureRect(self.shape, *rect)
        }
    }

    pub fn set_fill_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfShape_setFillColor(self.shape, *color)
        }
    }

    pub fn set_outline_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfShape_setOutlineColor(self.shape, *color)
        }
    }

    pub fn set_outline_thickness(&mut self, thickness : float) -> () {
        unsafe {
            ffi::sfShape_setOutlineThickness(self.shape, thickness as c_float)
        }
    }

    pub fn get_texture(&self) -> Option<Texture> {
        let tex = unsafe { ffi::sfShape_getTexture(self.shape) };
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(Wrappable::wrap(tex))
        }
    }

    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfShape_getTextureRect(self.shape)
        }
    }

    pub fn get_fill_color(&self) -> Color {
        unsafe {
            ffi::sfShape_getFillColor(self.shape)
        }
    }

    pub fn get_outline_color(&self) -> Color {
        unsafe {
            ffi::sfShape_getOutlineColor(self.shape)
        }
    }

    pub fn get_outline_thickness(&self) -> float {
        unsafe {
            ffi::sfShape_getOutlineThickness(self.shape) as float
        }
    }
    
    pub fn get_point_count(&self) -> uint {
        unsafe {
            ffi::sfShape_getPointCount(self.shape) as uint
        }
    }

    pub fn get_point(&self, index : uint) -> Vector2f {
        unsafe {
            ffi::sfShape_getPoint(self.shape, index as c_uint)
        }
    }

    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfShape_getLocalBounds(self.shape)
        }
    }

    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfShape_getGlobalBounds(self.shape)
        }
    }
    
    pub fn update(&mut self) -> () {
        unsafe {
            ffi::sfShape_update(self.shape)
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *ffi::sfShape {
        self.shape
    }
}

impl Drawable for Shape {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_shape(self)
    }

    pub fn draw_in_render_window_rs(&self, renderWindow : &RenderWindow, renderStates : &mut RenderStates) -> () {
        renderWindow.draw_shape_rs(self, renderStates)
    }
    
    pub fn draw_in_render_texture(&self, renderTexture : &RenderTexture) -> () {
        renderTexture.draw_shape(self)
    }

    pub fn draw_in_render_texture_rs(&self, renderTexture : &RenderTexture, renderStates : &mut RenderStates) -> () {
        renderTexture.draw_shape_rs(self, renderStates)
    }
}

#[unsafe_destructor]
impl Drop for Shape {
    fn drop(&self) -> () {
        unsafe {
            ffi::sfShape_destroy(self.shape)
        }
    }
}