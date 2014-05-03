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

//! Loading, manipulating and saving images.

use libc::c_uint;
use std::vec::Vec;

use traits::Wrappable;
use system::vector2::Vector2u;
use graphics::{Color, IntRect};

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi = ffi::graphics::image;

/// Loading, manipulating and saving images.
pub struct Image {
    #[doc(hidden)]
    image : *ffi::sfImage
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
     * Return Some(Image) or None
     */
    pub fn new(width : uint, height : uint) -> Option<Image> {
        let image = unsafe { ffi::sfImage_create(width as c_uint,
                                                 height as c_uint) };
        if image.is_null() {
            None
        } else {
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
     * Return Some(Image) or None
     */
    pub fn new_from_color(width : uint,
                          height : uint,
                          color : &Color) -> Option<Image> {
        let image =
            unsafe { ffi::sfImage_createFromColor(width as c_uint,
                                                  height as c_uint, *color) };
        if image.is_null() {
            None
        } else {
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
     *
     * Return Some(Image) or None
     */
    pub fn new_from_file(filename : ~str) -> Option<Image> {
        let image = unsafe {
            let c_filename = filename.to_c_str().unwrap();
            ffi::sfImage_createFromFile(c_filename)
        };
        if image.is_null() {
            None
        } else {
            Some(Image {
                    image : image
                })
        }
    }

    /**
     * Copy an existing image
     *
     * Return Some(Image) or None
     */
    pub fn clone_opt(&self) -> Option<Image> {
        let image = unsafe { ffi::sfImage_copy(self.image) };
        if image.is_null() {
            None
        } else {
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
     * Return Some(Image) or None
     */
    pub fn create_from_pixels(width : uint,
                              height : uint,
                              pixels : Vec<u8>) -> Option<Image> {
        let image =
            unsafe { ffi::sfImage_createFromPixels(width as c_uint,
                                                   height as c_uint,
                                                   pixels.as_ptr()) };
        if image.is_null() {
            None
        } else {
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
    pub fn save_to_file(&self, filename : &str) -> bool {
        let mut return_value = false;
        unsafe {
            filename.with_c_str(|c_str| {
                    match ffi::sfImage_saveToFile(self.image, c_str) {
                        SFFALSE => return_value = false,
                        SFTRUE  => return_value = true
                    }
                });
        }
        return_value
    }

    /**
     * Return the size of an image
     *
     * Return the size in pixels
     */
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
    pub fn get_pixel(&self, x : uint, y : uint) -> Color {
        unsafe {
            ffi::sfImage_getPixel(self.image, x as c_uint, y as c_uint)
        }
    }

    /**
     * Flip an image horizontally (left <-> right)
     */
    pub fn flip_horizontally(&mut self) -> () {
        unsafe {
            ffi::sfImage_flipHorizontally(self.image)
        }
    }

    /**
     * Flip an image vertically (top <-> bottom)
     */
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
    pub fn copy_image(&mut self,
                      source : &Image,
                      destX : uint,
                      destY : uint,
                      source_rect : &IntRect,
                      apply_alpha : bool) -> () {
        unsafe {
            match apply_alpha {
                true        =>  ffi::sfImage_copyImage(self.image,
                                                       source.unwrap(),
                                                       destX as c_uint,
                                                       destY as c_uint,
                                                       *source_rect,
                                                       SFFALSE),
                false       =>  ffi::sfImage_copyImage(self.image,
                                                       source.unwrap(),
                                                       destX as c_uint,
                                                       destY as c_uint,
                                                       *source_rect,
                                                       SFTRUE)
            }
        }
    }
}

impl Clone for Image {
    /// Return a new Image or fail! if there is not enough memory
    fn clone(&self) -> Image {
        let image = unsafe { ffi::sfImage_copy(self.image) };
        if image.is_null() {
            fail!("Not enough memory to clone Image")
        } else {
            Image {
                image : image
            }
        }
    }
}

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
    /// Destroy an existing image
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfImage_destroy(self.image)
        }
    }
}
