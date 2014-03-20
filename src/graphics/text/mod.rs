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
 * Graphical text
 *
 * Text is a drawable class that allows to easily
 * display some text with custom style and color on a render target.
 */

use std::libc::{c_float, c_uint, size_t};
use std::{str, cast};
use std::vec::Vec;
use std::c_vec::CVec;

use traits::{Drawable, Wrappable};
use graphics::{RenderWindow, RenderTexture, Font, FloatRect, Color, Transform,
               RenderStates, TextStyle};
use system::vector2::Vector2f;

use ffi = ffi::graphics::text;

pub mod rc;

/**
 * Graphical text
 *
 * Text is a drawable class that allows to easily
 * display some text with custom style and color on a render target.
 */
pub struct Text<'s> {
    #[doc(hidden)]
    priv text :             *ffi::sfText,
    #[doc(hidden)]
    priv string_length :    uint,
    #[doc(hidden)]
    priv font :             Option<&'s Font>
}

impl<'s> Text<'s> {
    /**
     * Create a new text
     *
     * Return Some(Text) or None
     */
    pub fn new() -> Option<Text<'s>> {
        let text  = unsafe { ffi::sfText_create() };
        if text.is_null() {
            None
        } else {
            Some(Text {
                    text :          text,
                    string_length : 0,
                    font :          None
                })
        }
    }

    /**
     * Create a new text with initialized value
     *
     * Default value for characterSize on SFML is 30.
     *
     * # Arguments
     * * string - The string of the text
     * * font - The font to display the Text
     * * characterSize - The size of the Text
     *
     * Return Some(Text) or None
     */
    pub fn new_init(string : &str,
                    font : &'s Font,
                    character_size : uint) ->Option<Text<'s>> {
        let text = unsafe { ffi::sfText_create() };
        if text.is_null() {
            None
        } else {
            unsafe {
                string.with_c_str(|c_str| {
                        ffi::sfText_setString(text, c_str)
                    });
                ffi::sfText_setFont(text, font.unwrap());
                ffi::sfText_setCharacterSize(text, character_size as c_uint)
            }
            Some(Text {
                    text :          text,
                    string_length : string.len(),
                    font :          Some(font)
                })
        }
    }

    /**
     * Set the string of a text (from an ANSI string)
     *
     * A text's string is empty by default.
     *
     * # Arguments
     * * string - New string
     */
    pub fn set_string(&mut self, string : &str) -> () {
        unsafe {
            string.with_c_str(|c_str| {
                    ffi::sfText_setString(self.text, c_str)
                });
        }
        self.string_length = string.len()
    }

    /**
     * Get the string of a text (returns an ANSI string)
     *
     * Return a string as a locale-dependant ANSI string
     */
    pub fn get_string(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sfText_getString(self.text))
        }
    }

    /**
     * Get the string of a text (returns a unicode string)
     *
     * Return a string as UTF-32
     */
    pub fn get_unicode_string(&self) -> Vec<u32> {
        unsafe {
            let mut return_unicode : Vec<u32> = Vec::new();
            let string : *mut u32 = ffi::sfText_getUnicodeString(self.text);
            let cvec = CVec::new(string, self.string_length);
            let mut d : uint = 0;
            return_unicode.push(*cvec.get(d).unwrap());
            d += 1;
            while d != 16 {
                return_unicode.push(*cvec.get(d).unwrap());
                d += 1;
            }
        return_unicode
        }
    }

    /**
     * Get the size of the characters
     *
     * Return the size of the characters
     */
    pub fn get_character_size(&self) -> uint {
        unsafe {
            ffi::sfText_getCharacterSize(self.text) as uint
        }
    }

    /**
     * Set the font of the text
     *
     * The font argument refers to a texture that must
     * exist as long as the text uses it. Indeed, the text
     * doesn't store its own copy of the font, but rather keeps
     * a pointer to the one that you passed to this function.
     * If the font is destroyed and the text tries to
     * use it, the behaviour is undefined.
     *
     * font - New font
     */
    pub fn set_font(&mut self, font : &'s Font) -> () {
        self.font = Some(font);
        unsafe {
            ffi::sfText_setFont(self.text, font.unwrap())
        }
    }

    /**
     * Set the orientation of a text
     *
     * This function completely overwrites the previous rotation.
     * See rotate to add an angle based on the previous rotation instead.
     * The default rotation of a text Text object is 0.
     *
     * # Arguments
     * * angle - New rotation, in degrees
     */
    pub fn set_rotation(&mut self, angle : f32) -> () {
        unsafe {
            ffi::sfText_setRotation(self.text, angle as c_float)
        }
    }

    /**
     * Get the orientation of a text
     *
     * The rotation is always in the range [0, 360].
     *
     * Return the current rotation, in degrees
     */
    pub fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfText_getRotation(self.text) as f32
        }
    }

    /**
     * Rotate a text
     *
     * This function adds to the current rotation of the object,
     * unlike set_rotation which overwrites it.
     *
     * # Arguments
     * * factors - Scale factors
     */
    pub fn rotate(&mut self, angle : f32) -> () {
        unsafe {
            ffi::sfText_rotate(self.text, angle as c_float)
        }
    }

    /**
     * Set the style of a text
     *
     * You can pass a combination of one or more styles, for
     * example Bold | Italic.
     * The default style is Regular.
     *
     * # Arguments
     * * style - New style
     */
    pub fn set_style(&mut self, style : TextStyle) -> () {
        unsafe {
            ffi::sfText_setStyle(self.text, style as u32)
        }
    }

    /**
     * Set the size of the characters of a text
     *
     * The default size is 30.
     *
     * # Arguments
     * * size - The new character size, in pixels
     */
    pub fn set_character_size(&mut self, size : uint) -> () {
        unsafe {
            ffi::sfText_setCharacterSize(self.text, size as c_uint)
        }
    }

    /**
     * Get the style of a text
     *
     * Return the current string style (see Style enum)
     */
    pub fn get_style(&self) -> TextStyle {
        unsafe { cast::transmute(ffi::sfText_getStyle(self.text)) }
    }

    /**
     * Get the font of a text
     * If the text has no font attached, a None is returned.
     * The returned pointer is const, which means that you can't
     * modify the font when you retrieve it with this function.
     */
    pub fn get_font(&self) -> Option<&'s Font> {
       self.font
    }

    /**
     * Set the global color of used by a text
     *
     * By default, the text's color is opaque white.
     *
     * # Arguments
     * * color - The new color of the text
     */
    pub fn set_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfText_setColor(self.text, *color)
        }
    }

    /**
     * Get the global color of a text
     *
     * Return the global color of the text
     */
    pub fn get_color(&self) -> Color {
        unsafe {
            ffi::sfText_getColor(self.text)
        }
    }

    /**
     * Scale a text
     *
     * This function multiplies the current scale of the object,
     * unlike set_Scale which overwrites it.
     *
     * # Arguments
     * * factors - Scale factors
     */
    pub fn scale(&mut self, factors : &Vector2f) -> () {
        unsafe {
            ffi::sfText_scale(self.text, *factors)
        }
    }

    /**
     * Scale a text
     *
     * This function multiplies the current scale of the object,
     * unlike set_Scale which overwrites it.
     *
     * # Arguments
     * * factor_x - Scale x factor
     * * factor_y - Scale y factor
     */
    pub fn scale2f(&mut self, factor_x : f32, factor_y : f32) -> () {
        unsafe {
            ffi::sfText_scale(self.text, Vector2f::new(factor_x, factor_y))
        }
    }

    /**
     * Set the scale factors of a text
     *
     * This function completely overwrites the previous scale.
     * See scale to add a factor based on the previous scale instead.
     * The default scale of a text Text object is (1, 1).
     *
     * # Arguments
     * * scale - The new scale factors
     */
    pub fn set_scale(&mut self, scale : &Vector2f) -> () {
        unsafe {
            ffi::sfText_setScale(self.text, *scale)
        }
    }

    /**
     * Set the scale factors of a text
     *
     * This function completely overwrites the previous scale.
     * See scale to add a factor based on the previous scale instead.
     * The default scale of a text Text object is (1, 1).
     *
     * # Arguments
     * * scale_x - The new x scale factor
     * * scale_y - The new y scale factor
     */
    pub fn set_scale2f(&mut self, scale_x : f32, scale_y : f32) -> () {
        unsafe {
            ffi::sfText_setScale(self.text, Vector2f::new(scale_x, scale_y))
        }
    }

    /**
     * Move a text by a given offset
     *
     * This function adds to the current position of the object,
     * unlike set_position which overwrites it.
     *
     * # Arguments
     * * offset - Offset
     */
    pub fn move(&mut self, offset : &Vector2f) -> () {
        unsafe {
            ffi::sfText_move(self.text, *offset)
        }
    }

    /**
     * Move a text by a given offset
     *
     * This function adds to the current position of the object,
     * unlike set_position which overwrites it.
     *
     * # Arguments
     * * offsetX - Offset x
     * * offsetY - Offset y
     */
    pub fn move2f(&mut self, offset_x : f32, offset_y : f32) -> () {
        unsafe {
            ffi::sfText_move(self.text, Vector2f::new(offset_x, offset_y))
        }
    }

    /**
     * Set the position of a text
     *
     * This function completely overwrites the previous position.
     * See move to apply an offset based on the previous position instead.
     * The default position of a text Text object is (0, 0).
     *
     * # Arguments
     * * position - The new position
     */
    pub fn set_position(&mut self, position : &Vector2f) -> () {
        unsafe {
            ffi::sfText_setPosition(self.text, *position)
        }
    }

    /**
     * Set the position of a text
     *
     * This function completely overwrites the previous position.
     * See move to apply an offset based on the previous position instead.
     * The default position of a text Text object is (0, 0).
     *
     * # Arguments
     * * x - The new x coordinate
     * * y - The new y coordinate
     */
    pub fn set_position2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfText_setPosition(self.text, Vector2f::new(x, y))
        }
    }

    /**
     * Set the local origin of a text
     *
     * The origin of an object defines the center point for
     * all transformations (position, scale, rotation).
     * The coordinates of this point must be relative to the
     * top-left corner of the object, and ignore all
     * transformations (position, scale, rotation).
     * The default origin of a text object is (0, 0).
     *
     * # Arguments
     * * origin - New origin
     */
    pub fn set_origin(&mut self, origin : &Vector2f) -> () {
        unsafe {
            ffi::sfText_setOrigin(self.text, *origin)
        }
    }

    /**
     * Set the local origin of a text
     *
     * The origin of an object defines the center point for
     * all transformations (position, scale, rotation).
     * The coordinates of this point must be relative to the
     * top-left corner of the object, and ignore all
     * transformations (position, scale, rotation).
     * The default origin of a text object is (0, 0).
     *
     * # Arguments
     * * x - New x origin coordinate
     * * y - New y origin coordinate
     */
    pub fn set_origin2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfText_setOrigin(self.text, Vector2f::new(x, y))
        }
    }

    /**
     * Get the current scale of a text
     *
     * Return the current scale factors
     */
    pub fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfText_getScale(self.text)
        }
    }

    /**
     * Get the local origin of a text
     *
     * Return the current origin
     */
    pub fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfText_getOrigin(self.text)
        }
    }

    /**
     * Return the position of the index-th character in a text
     *
     * This function computes the visual position of a character
     * from its index in the string. The returned position is
     * in global coordinates (translation, rotation, scale and
     * origin are applied).
     * If index is out of range, the position of the end of
     * the string is returned.
     *
     * # Arguments
     * * index - The index of the character
     *
     * Return the position of the character
     */
    pub fn find_character_pos(&self, index : u64) -> Vector2f {
        unsafe {
            ffi::sfText_findCharacterPos(self.text, index as size_t)
        }
    }

    /**
     * Get the position of a text
     *
     * Return the current position
     */
    pub fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfText_getPosition(self.text)
        }
    }

    /**
     * Get the local bounding rectangle of a text
     *
     * The returned rectangle is in local coordinates, which means
     * that it ignores the transformations (translation, rotation,
     * scale, ...) that are applied to the entity.
     * In other words, this function returns the bounds of the
     * entity in the entity's coordinate system.
     *
     * Return the local bounding rectangle of the entity
     */
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfText_getLocalBounds(self.text)
        }
    }

    /**
     * Get the global bounding rectangle of a text
     *
     * The returned rectangle is in global coordinates, which means
     * that it takes in account the transformations (translation,
     * rotation, scale, ...) that are applied to the entity.
     * In other words, this function returns the bounds of the
     * text in the global 2D world's coordinate system.
     *
     * Return the global bounding rectangle of the entity
     */
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfText_getGlobalBounds(self.text)
        }
    }

    /**
     * Set the string of a text (from a unicode string)
     *
     * # Arguments
     * * string - The new string
     */
    pub fn set_unicode_string(&mut self, string : Vec<u32>) -> () {
        unsafe {
            self.string_length = string.len();
            ffi::sfText_setUnicodeString(self.text, string.as_ptr())
        }
    }

    /**
     * Get the combined transform of a text
     *
     * Return the transform combining the position/rotation/scale/origin
     * of the object
     */
    pub fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfText_getTransform(self.text)
        }
    }

    /**
     * Get the inverse of the combined transform of a text
     *
     * Return the inverse of the combined transformations applied to the object
     */
    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfText_getInverseTransform(self.text)
        }
    }
}

impl<'s> Wrappable<*ffi::sfText> for Text<'s> {
    fn wrap(text : *ffi::sfText) -> Text {
        Text {
            text :          text,
            string_length : 0,
            font :          None
        }
    }

    fn unwrap(&self) -> *ffi::sfText {
        self.text
    }
}

impl<'s> Drawable for Text<'s> {
    fn draw_in_render_window(&self, render_window : &mut RenderWindow) -> () {
        render_window.draw_text(self)
    }

    fn draw_in_render_window_rs(&self,
                                render_window : &mut RenderWindow,
                                render_states : &mut RenderStates) -> () {
        render_window.draw_text_rs(self, render_states)
    }

    fn draw_in_render_texture(&self, renderTexture : &mut RenderTexture) -> () {
        renderTexture.draw_text(self)
    }

    fn draw_in_render_texture_rs(&self,
                                 render_texture : &mut RenderTexture,
                                 render_states : &mut RenderStates) -> () {
        render_texture.draw_text_rs(self, render_states)
    }
}

#[unsafe_destructor]
impl<'s> Drop for Text<'s> {
    /// Destructor for class Text. Destroy all the ressource.
    fn drop(&mut self) {
        unsafe {
            ffi::sfText_destroy(self.text);
        }
    }
}