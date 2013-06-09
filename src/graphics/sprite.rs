/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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
* Drawable representation of a texture
*
* Sprite is a drawable class that allows to easily display a texture (or a part of it) on a render target.
*
*/

use std::libc::{c_float};
use graphics::color;
use graphics::texture;
use graphics::drawable::Drawable;
use graphics::render_window;
use graphics::render_texture;
use system::vector2;
use graphics::rect::{FloatRect, IntRect};
use graphics::transform::Transform;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_void, c_float};
    use rsfml::sfTypes::{sfBool};
    use graphics::color;
    use graphics::texture;
    use system::vector2;
    use graphics::rect::{IntRect, FloatRect};
    use graphics::transform::Transform;

    pub struct sfSprite {
        This : *c_void,
        Texture : *texture::csfml::sfTexture,
        Transform : Transform,
        InverseTransform : Transform
    }

    pub extern "C" {
        fn sfSprite_create() -> *sfSprite;
        fn sfSprite_copy(sprite : *sfSprite) -> *sfSprite;
        fn sfSprite_destroy(sprite : *sfSprite) -> ();
        fn sfSprite_setPosition(sprite : *sfSprite, position : vector2::Vector2f) -> ();
        fn sfSprite_setRotation(sprite : *sfSprite, angle : c_float) -> ();
        fn sfSprite_setScale(sprite : *sfSprite, scale : vector2::Vector2f) -> ();
        fn sfSprite_setOrigin(sprite : *sfSprite, origin : vector2::Vector2f) -> ();
        fn sfSprite_getPosition(sprite : *sfSprite) -> vector2::Vector2f;
        fn sfSprite_getRotation(sprite : *sfSprite) -> c_float;
        fn sfSprite_getScale(sprite : *sfSprite) -> vector2::Vector2f;
        fn sfSprite_getOrigin(sprite : *sfSprite) -> vector2::Vector2f;
        fn sfSprite_move(sprite : *sfSprite, offset : vector2::Vector2f) -> ();
        fn sfSprite_rotate(sprite : *sfSprite, angle : c_float) -> ();
        fn sfSprite_scale(sprite : *sfSprite, factors : vector2::Vector2f) -> ();
        fn sfSprite_getTransform(sprite : *sfSprite) -> Transform;
        fn sfSprite_getInverseTransform(sprite : *sfSprite) -> Transform;
        fn sfSprite_setTexture(sprite : *sfSprite, texture : *texture::csfml::sfTexture, resetRect : sfBool) -> ();
        fn sfSprite_setTextureRect(sprite : *sfSprite, rectangle : IntRect) -> ();
        fn sfSprite_setColor(sprite : *sfSprite, color : color::Color) -> ();
        fn sfSprite_getTexture(sprite : *sfSprite) -> *texture::csfml::sfTexture;
        fn sfSprite_getTextureRect(sprite : *sfSprite) -> IntRect;
        fn sfSprite_getColor(sprite : *sfSprite) -> color::Color;
        fn sfSprite_getLocalBounds(sprite : *sfSprite) -> FloatRect;
        fn sfSprite_getGlobalBounds(sprite : *sfSprite) -> FloatRect;
    }
}

#[doc(hidden)]
pub struct Sprite {
    priv sprite : *csfml::sfSprite
}

impl Sprite {
    /**
    * Create a new sprite
    */
    pub fn new() -> Sprite {
        Sprite { sprite : unsafe {csfml::sfSprite_create()} }
    }

    /**
    * Copy an existing sprite
    */
    pub fn new_copy(sprite : &Sprite) -> Sprite {
        Sprite { sprite : unsafe {csfml::sfSprite_copy(sprite.unwrap())}}
    }

    /**
    * Set the orientation of a sprite
    */
    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            csfml::sfSprite_setRotation(self.sprite, angle as c_float)
        }
    }

    /**
    * Get the orientation of a sprite
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfSprite_getRotation(self.sprite) as float
        }
    }

    /**
    * Rotate a sprite
    */
    pub fn rotate(&self, angle : float) -> () {
        unsafe {
            csfml::sfSprite_rotate(self.sprite, angle as c_float)
        }
    }
    
    /**
    * Change the source texture of a sprite
    */
    pub fn set_texture(&self, texture : &texture::Texture, resetRect : bool) -> (){
        match resetRect {
            true        => unsafe {csfml::sfSprite_setTexture(self.sprite, texture.unwrap(), 1)},
            false       => unsafe {csfml::sfSprite_setTexture(self.sprite, texture.unwrap(), 0)}
        }
    }

    /**
    * Set the global color of a sprite
    */
    pub fn set_color(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfSprite_setColor(self.sprite, *color)
        }
    }
    
    /**
    * Get the source texture of a sprite
    */
    pub fn get_texture(&self) -> texture::Texture {
        unsafe {
            texture::Texture::wrap(csfml::sfSprite_getTexture(self.sprite))
        }
    }

    /**
    * Get the global color of a sprite
    */
    pub fn get_color(&self) -> color::Color {
        unsafe {csfml::sfSprite_getColor(self.sprite)}
    }
    
    pub fn set_position(&self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfSprite_setPosition(self.sprite, *position)
        }
    }

    pub fn scale(&self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfSprite_scale(self.sprite, *factors)
        }
    }

    pub fn get_scale(&self) -> vector2::Vector2f {
        unsafe {csfml::sfSprite_getScale(self.sprite)}
    }

    pub fn get_origin(&self) -> vector2::Vector2f {
        unsafe {csfml::sfSprite_getOrigin(self.sprite)}
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfSprite_move(self.sprite, *offset)
        }
    }

    pub fn set_scale(&self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfSprite_setScale(self.sprite, *scale)
        }
    }
    
    pub fn set_origin(&self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfSprite_setOrigin(self.sprite, *origin)
        }
    }

    pub fn get_position(&self) -> vector2::Vector2f {
        unsafe {csfml::sfSprite_getPosition(self.sprite)}
    }

    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfSprite_getLocalBounds(self.sprite)
        }
    }

    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfSprite_getGlobalBounds(self.sprite)
        }
    }

    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            csfml::sfSprite_getTextureRect(self.sprite)
        }
    }

    pub fn set_texture_rect(&self, rect : &IntRect) -> () {
        unsafe {
            csfml::sfSprite_setTextureRect(self.sprite, *rect)
        }
    }

    pub fn get_transform(&self) -> Transform {
        unsafe {
            csfml::sfSprite_getTransform(self.sprite)
        }
    }

    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            csfml::sfSprite_getInverseTransform(self.sprite)
        }
    }

    #[doc(hidden)]
    pub fn wrap(sprite : *csfml::sfSprite) -> Sprite {
        Sprite { sprite : sprite }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfSprite {
        self.sprite
    }
}

#[doc(hidden)]
impl Drawable for Sprite {
    /**
    * Draw the sprite in the RenderWindow
    */
    pub fn draw_in_render_window(&self, renderWindow : &render_window::RenderWindow) -> () {
        renderWindow.draw_sprite(self)
    }

    pub fn draw_in_render_texture(&self, renderTexture : &render_texture::RenderTexture) -> () {
        renderTexture.draw_sprite(self)
    }
}

impl Drop for Sprite {
    /**
    * Destroy an existing sprite
    */
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfSprite_destroy(self.sprite)
        }
    }
}