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
    
    use std::libc::{c_uint, c_float, c_void, size_t, c_char};
    use graphics::transform;
    use system::vector2::Vector2f;
    use graphics::font;
    use graphics::color;
    use graphics::rect::FloatRect;
    use graphics::transform::Transform;

    pub struct sfText {
        This :          c_void,
        font :          *c_void,
        transform :     transform::Transform,
        transform2 :    transform::Transform
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
    Regular =       0,
    Bold =          1,
    Italic =        2,
    Underlined =    4
}

pub struct Text<'self> {
    #[doc(hidden)]
    priv text :             *ffi::sfText,
    #[doc(hidden)]
    priv string_length :    uint,
    #[doc(hidden)]
    priv font :             Option<&'self Font>
}

impl<'self> Text<'self> {
    /**
    * Create a new text
    *
    * Return a new Option on Text object, or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new() -> Option<Text<'self>> {
        let text  = unsafe { ffi::sfText_create() };
        if ptr::is_null(text) {
            None
        }
        else {
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
    * Return a new Option on Text object, or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_init(string : ~str, font : &'self Font, character_size : uint) ->Option<Text<'self>> {
        let text = unsafe { ffi::sfText_create() };
        if ptr::is_null(text) {
            None
        }
        else {
            unsafe {
                let c_string = string.to_c_str().unwrap();
                ffi::sfText_setString(text, c_string);
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_string(&mut self, string : ~str) -> () {
        unsafe {
            let c_string = string.to_c_str().unwrap();
            ffi::sfText_setString(self.text, c_string)
        }
        self.string_length = string.len()
    }

    /**
    * Get the string of a text (returns an ANSI string)
    *
    * Return a string as a locale-dependant ANSI string
    */
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_unicode_string(&self) -> ~[u32] {
        unsafe {
            let mut return_unicode : ~[u32] = ~[];
            let string : *mut u32 = ffi::sfText_getUnicodeString(self.text);
            let cvec = CVec(string, self.string_length);
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_font(&mut self, font : &'self Font) -> () {
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    pub fn get_font(&self) -> Option<&'self Font> {
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_unicode_string(&mut self, string : ~[u32]) -> () {
        unsafe {
            self.string_length = string.len();
            ffi::sfText_setUnicodeString(self.text, vec::raw::to_ptr(string))
        }
    }

    /**
    * Get the combined transform of a text
    *
    * Return the transform combining the position/rotation/scale/origin of the object
    */
    #[fixed_stack_segment] #[inline(never)]
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfText_getInverseTransform(self.text)
        }
    }
}

impl<'self> Wrappable<*ffi::sfText> for Text<'self> {
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

impl<'self> Drawable for Text<'self> {
    fn draw_in_render_window(&self, render_window : &RenderWindow) -> () {
        render_window.draw_text(self)
    }

    fn draw_in_render_window_rs(&self, render_window : &RenderWindow, render_states : &mut RenderStates) -> () {
        render_window.draw_text_rs(self, render_states)
    }

    fn draw_in_render_texture(&self, renderTexture : &RenderTexture) -> () {
        renderTexture.draw_text(self)
    }

    fn draw_in_render_texture_rs(&self, render_texture : &RenderTexture, render_states : &mut RenderStates) -> () {
        render_texture.draw_text_rs(self, render_states)
    }
}

#[unsafe_destructor]
impl<'self> Drop for Text<'self> {
    /**
    *   Destructor for class Text. Destroy all the ressource.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) {
        unsafe {
            ffi::sfText_destroy(self.text);
        }
    }
}
