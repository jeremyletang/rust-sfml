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
* Loading, manipulating and saving images.
*
*
*
*/

use std::libc::{c_uint};
use std::ptr;
use std::vec;

use traits::wrappable::Wrappable;
use system::vector2::Vector2u;
use graphics::color::Color;
use graphics::rect::IntRect;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_void, c_uint, c_char};

    use rsfml::sfTypes::{sfBool};
    use graphics::color;
    use system::vector2;
    use graphics::rect::IntRect;

    pub struct sfImage {
        This : *c_void
    }

    extern "C" {
        pub fn sfImage_create(width : c_uint, height : c_uint) -> *sfImage;
        pub fn sfImage_createFromColor(width : c_uint, height : c_uint, color : color::Color) -> *sfImage;
        pub fn sfImage_createFromPixels(width : c_uint, height : c_uint, pixels : *u8) -> *sfImage;
        pub fn sfImage_createFromFile(filename : *c_char) -> *sfImage;
        //fn sfImage_createFromMemory(data : *c_void, size : size_t) -> *sfImage;
        //fn sfImage_createFromStream(stream : *sfInputStream) -> *sfImage;
        pub fn sfImage_copy(image : *sfImage) -> *sfImage;
        pub fn sfImage_destroy(image : *sfImage) -> ();
        pub fn sfImage_saveToFile(image : *sfImage, filename : *c_char) -> sfBool;
        pub fn sfImage_getSize(image : *sfImage) -> vector2::Vector2u;
        pub fn sfImage_createMaskFromColor(image : *sfImage, color : color::Color, alpha : u8) -> ();
        pub fn sfImage_copyImage(image : *sfImage, source : *sfImage, destX : c_uint, destY : c_uint, sourceRect : IntRect, applyAlpha : sfBool) -> ();
        pub fn sfImage_setPixel(image : *sfImage, x : c_uint, y : c_uint, color : color::Color) -> ();
        pub fn sfImage_getPixel(image : *sfImage, x : c_uint, y : c_uint) -> color::Color;
        pub fn sfImage_getPixelsPtr(image : *sfImage) -> *u8;
        pub fn sfImage_flipHorizontally(image : *sfImage) -> ();
        pub fn sfImage_flipVertically(image : *sfImage) -> ();
    }
}

#[doc(hidden)]
pub struct Image {
    priv image : *ffi::sfImage
}

impl Image {
    /**
    * Create an image
    *
    * This image is filled with black pixels.
    *
    * # Arguments
    * * width - Width of the image
    * * height - Height of the image
    * 
    * Return a new Image object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new(width : uint, height : uint) -> Option<Image> {
        let image = unsafe { ffi::sfImage_create(width as c_uint, height as c_uint) };
        if ptr::is_null(image) {
            None
        }
        else {
            Some(Image { 
                image : image
            })
        }
    }

    /**
    * Create an image and fill it with a unique color
    *
    * # Arguments
    * * width - Width of the image
    * * height - Height of the image
    * * color - Fill color
    *
    * Return a new Image object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_from_color(width : uint, height : uint, color : &Color) -> Option<Image> {
        let image = unsafe { ffi::sfImage_createFromColor(width as c_uint, height as c_uint, *color) };
        if ptr::is_null(image) {
            None
        }
        else {
            Some(Image { 
                image : image
            })
        }
    }
    
    /**
    * Create an image from a file on disk
    *
    * The supported image formats are bmp, png, tga, jpg, gif,
    * psd, hdr and pic. Some format options are not supported,
    * like progressive jpeg.
    * If this function fails, the image is left unchanged.
    *
    * # Arguments
    * * filename - Path of the image file to load
    * Return a new Image object, or NULL if it failed
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_from_file(filename : ~str) -> Option<Image> {
        let image = unsafe { 
            let c_filename = filename.to_c_str().unwrap();
            ffi::sfImage_createFromFile(c_filename)
        };
        if ptr::is_null(image) {
            None
        }
        else {
            Some(Image {
                image : image
            }) 
        }
    }
    
    /**
    * Copy an existing image
    *
    * Return copied object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn clone(&self) -> Option<Image> {
        let image = unsafe { ffi::sfImage_copy(self.image) };
        if ptr::is_null(image) {
            None
        }
        else {
            Some(Image {
                image : image
            })
        }
        
    }

    /**
    * Create an image from an vector of pixels
    *
    * The pixel vector is assumed to contain 32-bits RGBA pixels,
    * and have the given width and height. If not, this is
    * an undefined behaviour.
    *
    * # Arguments
    * * width - Width of the image
    * * height - Height of the image
    * * pixels - Vector of pixels to copy to the image 
    *
    * Return A new Image object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn create_from_pixels(width : uint, height : uint, pixels : ~[u8]) -> Option<Image> {
        let image = unsafe { ffi::sfImage_createFromPixels(width as c_uint, height as c_uint, vec::raw::to_ptr(pixels)) };
        if ptr::is_null(image) {
            None
        }
        else {
            Some(Image {
                image : image
            })
        }
    }

    /**
    * Save an image to a file on disk
    *
    * The format of the image is automatically deduced from
    * the extension. The supported image formats are bmp, png,
    * tga and jpg. The destination file is overwritten
    * if it already exists. This function fails if the image is empty.
    *
    * # Arguments
    * * filename - Path of the file to save
    * 
    * Return true if saving was successful
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn save_to_file(&self, filename : ~str) -> bool {
        unsafe {
            let c_filename = filename.to_c_str().unwrap();
            match ffi::sfImage_saveToFile(self.image, c_filename) {
                0 => false,
                _ => true
            }
        }
    }
    
    /**
    * Return the size of an image 
    * 
    * Return the size in pixels
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfImage_getSize(self.image)
        }
    }
    
    /**
    * Create a transparency mask from a specified color-key
    *
    * This function sets the alpha value of every pixel matching
    * the given color to alpha (0 by default), so that they
    * become transparent.
    *
    * # Arguments
    * * color - Color to make transparent
    * * alpha - Alpha value to assign to transparent pixels
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn create_mask_from_color(&self, color : &Color, alpha : u8) -> () {
        unsafe {
            ffi::sfImage_createMaskFromColor(self.image, *color, alpha)
        }
    }
    
    /**
    * Change the color of a pixel in an image
    *
    * This function doesn't check the validity of the pixel
    * coordinates, using out-of-range values will result in
    * an undefined behaviour.
    *
    * # Arguments
    * * x - X coordinate of pixel to change
    * * y - Y coordinate of pixel to change
    * * color - New color of the pixel
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_pixel(&mut self, x : uint, y : uint, color : &Color) -> () {
        unsafe {
            ffi::sfImage_setPixel(self.image, x as c_uint, y as c_uint, *color)
        }
    }

    /**
    * Get the color of a pixel in an image
    *
    * This function doesn't check the validity of the pixel
    * coordinates, using out-of-range values will result in
    * an undefined behaviour.
    *
    * # Arguments
    * * x - X coordinate of pixel to get
    * * y - Y coordinate of pixel to get
    *
    * Return the Color of the pixel at coordinates (x, y)
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_pixel(&self, x : uint, y : uint) -> Color {
        unsafe {
            ffi::sfImage_getPixel(self.image, x as c_uint, y as c_uint)
        }
    }

    /**
    * Flip an image horizontally (left <-> right)
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn flip_horizontally(&mut self) -> () {
        unsafe {
            ffi::sfImage_flipHorizontally(self.image)
        }
    }
    
    /**
    * Flip an image vertically (top <-> bottom)
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn flip_vertically(&mut self) -> () {
        unsafe {
            ffi::sfImage_flipVertically(self.image)
        }
    }

    /**
    * Copy pixels from an image onto another
    *
    * This function does a slow pixel copy and should not be
    * used intensively. It can be used to prepare a complex
    * static image from several others, but if you need this
    * kind of feature in real-time you'd better use sfRenderTexture.
    *
    * If sourceRect is empty, the whole image is copied.
    * If applyAlpha is set to true, the transparency of
    * source pixels is applied. If it is false, the pixels are
    * copied unchanged with their alpha value.
    *
    * # Arguments
    * * source - Source image to copy
    * * destX - X coordinate of the destination position
    * * destY - Y coordinate of the destination position
    * * sourceRect - Sub-rectangle of the source image to copy
    * * applyAlpha - Should the copy take in account the source transparency?
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn copy_image(&mut self, source : &Image, destX : uint, destY : uint, source_rect : &IntRect, apply_alpha : bool) -> () {
        unsafe {
            match apply_alpha {
                true        =>  ffi::sfImage_copyImage(self.image, source.unwrap(), destX as c_uint, destY as c_uint, *source_rect, 1),
                false       =>  ffi::sfImage_copyImage(self.image, source.unwrap(), destX as c_uint, destY as c_uint, *source_rect, 0)
            }
        }
    }

}

#[doc(hidden)]
impl Wrappable<*ffi::sfImage> for Image {
    fn wrap(image : *ffi::sfImage) -> Image {
        Image {
            image : image
        }
    }
    
    fn unwrap(&self) -> *ffi::sfImage {
        self.image
    }
}

impl Drop for Image {
    /**
    * Destroy an existing image
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&self) -> () {
        unsafe {
            ffi::sfImage_destroy(self.image)
        }
    }
}
