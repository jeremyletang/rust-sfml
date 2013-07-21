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
* Text is a drawable class that allows to easily display some text with custom style and color on a render target.
*
*/

use std::libc::{c_float, c_uint, size_t};
use extra::c_vec::{CVec, get};
use std::str;
use std::ptr;
use std::vec;

use traits::drawable::Drawable;
use traits::wrappable::Wrappable;
use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use system::vector2::Vector2f;
use graphics::font::Font;
use graphics::color::Color;
use graphics::rect::FloatRect;
use graphics::transform::Transform;
use graphics::render_states::RenderStates;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_uint, c_float, c_void, c_char, size_t};
    use graphics::transform;
    use system::vector2::Vector2f;
    use graphics::font;
    use graphics::color;
    use graphics::rect::FloatRect;
    use graphics::transform::Transform;

    pub struct sfText {
        This : c_void,
        font : *c_void,
        transform : transform::Transform,
        transform2 : transform::Transform
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
        pub fn sfText_setFont(text : *sfText, font : *font::ffi::sfFont) -> ();
        pub fn sfText_setCharacterSize(text : *sfText, size : c_uint) -> ();
        pub fn sfText_setStyle(text : *sfText, style : u32) -> ();
        pub fn sfText_setColor(text : *sfText, color : color::Color) -> ();
        pub fn sfText_getString(text : *sfText) -> *c_char;
        pub fn sfText_getUnicodeString(text : *sfText) -> *mut u32;
        pub fn sfText_getFont(text : *sfText) -> *font::ffi::sfFont;
        pub fn sfText_getCharacterSize(text : *sfText) -> c_uint;
        pub fn sfText_getStyle(text : *sfText) -> u32;
        pub fn sfText_getColor(text : *sfText) -> color::Color;
        pub fn sfText_findCharacterPos(text : *sfText, index : size_t) -> Vector2f;
        pub fn sfText_getLocalBounds(text : *sfText) -> FloatRect;
        pub fn sfText_getGlobalBounds(text : *sfText) -> FloatRect;
    }
}

pub enum Style {
    Regular = 0,
    Bold = 1,
    Italic = 2,
    Underlined = 4
}

pub struct Text {
    priv text : *ffi::sfText,
    priv stringLength : uint
}

impl Text {
    /**
    * Create a new text
    *
    * Return a new Option on Text object, or None
    */
    pub fn new() -> Option<Text> {
        let text  = unsafe { ffi::sfText_create() };
        if ptr::is_null(text) {
            None
        }
        else {
            Some(Text {
                text : text,
                stringLength : 0
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
    * Return a new Option on Text object, or None
    */
    pub fn new_init(string : ~str, font : &Font, characterSize : uint) ->Option<Text> {
        let text = unsafe { ffi::sfText_create() };
        if ptr::is_null(text) {
            None
        }
        else {
            unsafe {
                do str::as_c_str(string) |cstring| {
                    ffi::sfText_setString(text, cstring);
                }
                ffi::sfText_setFont(text, font.unwrap());
                ffi::sfText_setCharacterSize(text, characterSize as c_uint);
                Some(Text {
                    text : text, 
                    stringLength : string.len()
                })
            }
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
    pub fn set_string(&mut self, string : ~str) -> () {
        do str::as_c_str(string) |cstring| {
            unsafe {
                ffi::sfText_setString(self.text, cstring)
            }
        };
        self.stringLength = string.len()
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
    pub fn get_unicode_string(&self) -> ~[u32] {
        unsafe {
            let mut return_unicode : ~[u32] = ~[];
            let string : *mut u32 = ffi::sfText_getUnicodeString(self.text);
            let cvec = CVec(string, self.stringLength);
            let mut d : uint = 0;
            return_unicode.push(get(cvec, d));
            d += 1;
            while d != 16 {
                return_unicode.push(get(cvec, d));
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
    pub fn set_font(&mut self, font : &Font) -> () {
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
    pub fn set_rotation(&mut self, angle : float) -> () {
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
    pub fn get_rotation(&self) -> float {
        unsafe {
            ffi::sfText_getRotation(self.text) as float
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
    pub fn rotate(&mut self, angle : float) -> () {
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
    pub fn set_style(&mut self, style : Style) -> () {
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
    pub fn get_style(&self) -> Style {
        match unsafe { ffi::sfText_getStyle(self.text) } {
            0 => Regular,
            1 => Bold,
            2 => Italic,
            _ => Underlined
        }
    }
    
    /**
    * Get the font of a text
    * If the text has no font attached, a None is returned.
    * The returned pointer is const, which means that you can't
    * modify the font when you retrieve it with this function.
    */
    pub fn get_font(&self) -> Option<Font> {
        let fnt = unsafe { ffi::sfText_getFont(self.text) };
        if ptr::is_null(fnt) {
            None
        }
        else {
            Some(Wrappable::wrap(fnt))
        }
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
    * * factorX - Scale x factor
    * * factorY - Scale y factor
    */
    pub fn scale2f(&mut self, factorX : f32, factorY : f32) -> () {
        unsafe {
            ffi::sfText_scale(self.text, Vector2f::new(factorX, factorY))
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
    * * scaleX - The new x scale factor
    * * scaleY - The new y scale factor
    */
    pub fn set_scale2f(&mut self, scaleX : f32, scaleY : f32) -> () {
        unsafe {
            ffi::sfText_setScale(self.text, Vector2f::new(scaleX, scaleY))
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
    pub fn move2f(&mut self, offsetX : f32, offsetY : f32) -> () {
        unsafe {
            ffi::sfText_move(self.text, Vector2f::new(offsetX, offsetY))
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
    pub fn set_unicode_string(&mut self, string : ~[u32]) -> () {
        unsafe {
            self.stringLength = string.len();
            ffi::sfText_setUnicodeString(self.text, vec::raw::to_ptr(string))
        }
    }

    /**
    * Get the combined transform of a text
    *
    * Return the transform combining the position/rotation/scale/origin of the object
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

#[doc(hidden)]
impl Wrappable<*ffi::sfText> for Text {
    pub fn wrap(text : *ffi::sfText) -> Text {
        Text {
            text : text,
            stringLength : 0
        }
    } 
    pub fn unwrap(&self) -> *ffi::sfText {
        self.text
    }
}

#[doc(hidden)]
impl Drawable for Text {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_text(self)
    }

    pub fn draw_in_render_window_rs(&self, renderWindow : &RenderWindow, renderStates : &mut RenderStates) -> () {
        renderWindow.draw_text_rs(self, renderStates)
    }

    pub fn draw_in_render_texture(&self, renderTexture : &RenderTexture) -> () {
        renderTexture.draw_text(self)
    }

    pub fn draw_in_render_texture_rs(&self, renderTexture : &RenderTexture, renderStates : &mut RenderStates) -> () {
        renderTexture.draw_text_rs(self, renderStates)
    }
}

impl Drop for Text {
    /**
    *   Destructor for class Text. Destroy all the ressource.
    */
    fn drop(&self) {
        unsafe {
            ffi::sfText_destroy(self.text);
        }
    }
}
