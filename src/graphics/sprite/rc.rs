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

//! Drawable representation of a texture
//!
//! Sprite is a drawable class that allows to easily
//! display a texture (or a part of it) on a render target.

use std::rc::Rc;
use std::cell::RefCell;
use libc::{c_float};
use std::ptr;

use traits::{Drawable, Wrappable};
use graphics::{FloatRect, IntRect, Color, Texture,
               RenderTarget, Transform, rc};
use system::vector2::Vector2f;

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi::graphics::sprite as ffi;

/// Drawable representation of a texture
///
/// Sprite is a drawable class that allows to easily
/// display a texture (or a part of it) on a render target.
pub struct Sprite {
    #[doc(hidden)]
    sprite: *mut ffi::sfSprite,
    #[doc(hidden)]
    texture: Option<Rc<RefCell<Texture>>>
}

impl Sprite {
    /// Create a new sprite
    ///
    /// Return Some(Sprite) or None
    pub fn new() -> Option<Sprite> {
        let sp = unsafe { ffi::sfSprite_create() };
        if sp.is_null() {
            None
        } else {
            Some(Sprite {
                    sprite: sp,
                    texture: None
                })
        }
    }

    /// Create a new sprite with a texture
    ///
    /// Return Some(Sprite) or None
    pub fn new_with_texture(texture: Rc<RefCell<Texture>>) -> Option<Sprite> {
        let sp = unsafe { ffi::sfSprite_create() };
        if sp.is_null() {
            None
        } else {
            unsafe {
                ffi::sfSprite_setTexture(sp,
                                         (*texture).borrow().unwrap(),
                                         SFTRUE);
            }
            Some(Sprite {
                    sprite: sp,
                    texture: Some(texture)
                })
        }
    }

    /// Copy an existing sprite
    ///
    /// Return Some(Sprite) or None
    pub fn clone_opt(&self) -> Option<Sprite> {
        let sp = unsafe { ffi::sfSprite_copy(self.sprite) };
        if sp.is_null() {
            None
        } else {
            Some(Sprite {
                    sprite: sp,
                    texture: self.texture.clone()
                })
        }
    }

    /// Set the orientation of a sprite
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a sprite Sprite object is 0.
    ///
    /// # Arguments
    /// * angle - New rotation, in degrees
    pub fn set_rotation(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfSprite_setRotation(self.sprite, angle as c_float)
        }
    }

    /// Get the orientation of a sprite
    ///
    /// The rotation is always in the range [0, 360].
    ///
    /// Return the current rotation, in degrees
    pub fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfSprite_getRotation(self.sprite) as f32
        }
    }

    /// Rotate a sprite
    ///
    /// This function adds to the current rotation of the object,
    /// unlike sfSprite_setRotation which overwrites it.
    ///
    /// # Arguments
    /// * angle - Angle of rotation, in degrees
    pub fn rotate(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfSprite_rotate(self.sprite, angle as c_float)
        }
    }

    /// Change the source texture of a sprite
    ///
    /// The texture argument refers to a texture that must
    /// exist as long as the sprite uses it. Indeed, the sprite
    /// doesn't store its own copy of the texture, but rather keeps
    /// a pointer to the one that you passed to this function.
    /// If the source texture is destroyed and the sprite tries to
    /// use it, the behaviour is undefined.
    /// If reset_rect is true, the TextureRect property of
    /// the sprite is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    ///
    /// # Arguments
    /// * texture - New texture
    /// * reset_rect - Should the texture rect be reset to the size of the new texture?
    pub fn set_texture(&mut self,
                       texture: Rc<RefCell<Texture>>,
                       reset_rect: bool) -> (){
        unsafe {
            match reset_rect {
                true  =>
                    ffi::sfSprite_setTexture(self.sprite,
                                             (*texture).borrow().unwrap(),
                                             SFTRUE),
                false =>
                    ffi::sfSprite_setTexture(self.sprite,
                                             (*texture).borrow().unwrap(),
                                             SFFALSE)
            }
        }
        self.texture = Some(texture);
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    pub fn disable_texture(&mut self) -> () {
        self.texture = None;
        unsafe {
            ffi::sfSprite_setTexture(self.sprite, ptr::null_mut(), SFTRUE)
        }
    }

    /// Set the global color of a sprite
    ///
    /// This color is modulated (multiplied) with the sprite's
    /// texture. It can be used to colorize the sprite, or change
    /// its global opacity.
    /// By default, the sprite's color is opaque white.
    ///
    /// # Arguments
    /// * color - New color of the sprite
    pub fn set_color(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfSprite_setColor(self.sprite, *color)
        }
    }

    /// Get the source texture of a sprite
    ///
    /// If the sprite has no source texture, None is returned.
    /// You can't
    /// modify the texture when you retrieve it with this function.
    ///
    /// Return an Option to the sprite's texture
    pub fn get_texture(&self) -> Option<Rc<RefCell<Texture>>> {
        self.texture.clone()
    }

    /// Get the global color of a sprite
    ///
    /// Return the global color of the sprite
    pub fn get_color(&self) -> Color {
        unsafe {
            ffi::sfSprite_getColor(self.sprite)
        }
    }

    /// Set the position of a sprite
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a sprite Sprite object is (0, 0).
    ///
    /// # Arguments
    /// * position - New position
    pub fn set_position(&mut self, position: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_setPosition(self.sprite, *position)
        }
    }

    /// Set the position of a sprite
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a sprite Sprite object is (0, 0).
    ///
    /// # Arguments
    /// * x - New x coordinate
    /// * y - New y coordinate
    pub fn set_position2f(&mut self, x: f32, y: f32) -> () {
        unsafe {
            ffi::sfSprite_setPosition(self.sprite, Vector2f::new(x, y))
        }
    }

    /// Scale a sprite
    ///
    /// This function multiplies the current scale of the object,
    /// unlike setScale which overwrites it.
    ///
    /// # Arguments
    /// * factors - Scale factors
    pub fn scale(&mut self, factors: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_scale(self.sprite, *factors)
        }
    }

    /// Scale a sprite
    ///
    /// This function multiplies the current scale of the object,
    /// unlike setScale which overwrites it.
    ///
    /// # Arguments
    /// * factor_x - Scale x factor
    /// * factor_y - Scale y factor
    pub fn scale2f(&mut self, factor_x: f32, factor_y: f32) -> () {
        unsafe {
            ffi::sfSprite_scale(self.sprite, Vector2f::new(factor_x, factor_y))
        }
    }

    /// Get the current scale of a sprite
    ///
    /// Return the current scale factors
    pub fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfSprite_getScale(self.sprite)
        }
    }

    /// Get the local origin of a sprite
    ///
    /// Return the current origin
    pub fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfSprite_getOrigin(self.sprite)
        }
    }

    /// Move a sprite by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike sfSprite_setPosition which overwrites it.
    ///
    /// # Arguments
    /// * offset - Offset
    pub fn move_(&mut self, offset: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_move(self.sprite, *offset)
        }
    }

    /// Move a sprite by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike sfSprite_setPosition which overwrites it.
    ///
    /// # Arguments
    /// * offsetX - Offset x
    /// * offsetY - Offset y
    pub fn move2f(&mut self, offset_x: f32, offset_y: f32) -> () {
        unsafe {
            ffi::sfSprite_move(self.sprite, Vector2f::new(offset_x, offset_y))
        }
    }

    /// Set the scale factors of a sprite
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a sprite Sprite object is (1, 1).
    ///
    /// # Arguments
    /// * scale - New scale factors
    pub fn set_scale(&mut self, scale: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_setScale(self.sprite, *scale)
        }
    }

    /// Set the scale factors of a sprite
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a sprite Sprite object is (1, 1).
    ///
    /// # Arguments
    /// * scale_x - New x scale factor
    /// * scale_y - New y scale factor
    pub fn set_scale2f(&mut self, scale_x: f32, scale_y: f32) -> () {
        unsafe {
            ffi::sfSprite_setScale(self.sprite, Vector2f::new(scale_x, scale_y))
        }
    }

    /// Set the local origin of a sprite
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a sprite Sprite object is (0, 0).
    ///
    /// # Arguments
    /// * origin - New origin
    pub fn set_origin(&mut self, origin: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_setOrigin(self.sprite, *origin)
        }
    }

    /// Set the local origin of a sprite
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a sprite Sprite object is (0, 0).
    ///
    /// # Arguments
    /// * x - New x origin coordinate
    /// * y - New y origin coordinate
    pub fn set_origin2f(&mut self, x: f32, y: f32) -> () {
        unsafe {
            ffi::sfSprite_setOrigin(self.sprite, Vector2f::new(x, y))
        }
    }

    /// Get the position of a sprite
    ///
    /// Return the current position
    pub fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfSprite_getPosition(self.sprite)
        }
    }

    /// Get the local bounding rectangle of a sprite
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    ///
    /// Return the local bounding rectangle of the entity
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfSprite_getLocalBounds(self.sprite)
        }
    }

    /// Get the global bounding rectangle of a sprite
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    ///
    /// Return the global bounding rectangle of the entity
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfSprite_getGlobalBounds(self.sprite)
        }
    }

    /// Get the sub-rectangle of the texture displayed by a sprite
    ///
    /// Return the texture rectangle of the sprite
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfSprite_getTextureRect(self.sprite)
        }
    }

    /// Set the sub-rectangle of the texture that a sprite will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    ///
    /// # Arguments
    /// * rectangle - Rectangle defining the region of the texture to display
    pub fn set_texture_rect(&mut self, rect: &IntRect) -> () {
        unsafe {
            ffi::sfSprite_setTextureRect(self.sprite, *rect)
        }
    }

    /// Get the combined transform of a sprite
    ///
    /// Return the transform combining the position/rotation/scale/origin of the object
    pub fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfSprite_getTransform(self.sprite)
        }
    }

    /// Get the inverse of the combined transform of a sprite
    ///
    /// Return the inverse of the combined transformations applied to the object
    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfSprite_getInverseTransform(self.sprite)
        }
    }
}

impl Clone for Sprite {
    /// Return a new Sprite or panic! if there is not enough memory
    fn clone(&self) -> Sprite {
        let sp = unsafe { ffi::sfSprite_copy(self.sprite) };
        if sp.is_null() {
            panic!("Not enough memory to clone RectangleShape")
        } else {
            Sprite {
                sprite: sp,
                texture: self.texture.clone()
            }
        }
    }
}

impl Wrappable<*mut ffi::sfSprite> for Sprite {
    fn wrap(sprite: *mut ffi::sfSprite) -> Sprite {
        Sprite {
            sprite: sprite,
            texture: None
        }
    }

    fn unwrap(&self) -> *mut ffi::sfSprite {
        self.sprite
    }
}

impl Drawable for Sprite {
    fn draw<RT:RenderTarget>(&self, render_target: &mut RT) -> () {
        render_target.draw_sprite_rc(self)
    }

    fn draw_rs_rc<RT:RenderTarget>(&self,
                                   render_target: &mut RT,
                                   render_states: &mut rc::RenderStates) -> () {
        render_target.draw_sprite_rs_rc(self, render_states)
    }
}



impl Drop for Sprite {
    /// Destroy an existing sprite
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfSprite_destroy(self.sprite)
        }
    }
}
