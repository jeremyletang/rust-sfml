/*!
* Loading, manipulating and saving images.
*
*
*
*/

use core::libc::{c_uint};
//use system::vector2;
use system::vector2::Vector2u;
use graphics::color::Color;
use graphics::rect::IntRect;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_char, c_void, c_uint};
    use rsfml::sfTypes::{sfBool};
    use graphics::color;
    use system::vector2;
    use graphics::rect::IntRect;

    pub struct sfImage {
        This : *c_void
    }

    pub extern "C" {
        fn sfImage_create(width : c_uint, height : c_uint) -> *sfImage;
        fn sfImage_createFromColor(width : c_uint, height : c_uint, color : color::Color) -> *sfImage;
        fn sfImage_createFromPixels(width : c_uint, height : c_uint, pixels : *u8) -> *sfImage;
        fn sfImage_createFromFile(filename : *c_char) -> *sfImage;
        //fn sfImage_createFromMemory(data : *c_void, size : size_t) -> *sfImage;
        //fn sfImage_createFromStream(stream : *sfInputStream) -> *sfImage;
        fn sfImage_copy(image : *sfImage) -> *sfImage;
        fn sfImage_destroy(image : *sfImage) -> ();
        fn sfImage_saveToFile(image : *sfImage, filename : *c_char) -> sfBool;
        fn sfImage_getSize(image : *sfImage) -> vector2::Vector2u;
        fn sfImage_createMaskFromColor(image : *sfImage, color : color::Color, alpha : u8) -> ();
        fn sfImage_copyImage(image : *sfImage, source : *sfImage, destX : c_uint, destY : c_uint, sourceRect : IntRect, applyAlpha : sfBool) -> ();
        fn sfImage_setPixel(image : *sfImage, x : c_uint, y : c_uint, color : color::Color) -> ();
        fn sfImage_getPixel(image : *sfImage, x : c_uint, y : c_uint) -> color::Color;
        fn sfImage_getPixelsPtr(image : *sfImage) -> *u8;
        fn sfImage_flipHorizontally(image : *sfImage) -> ();
        fn sfImage_flipVertically(image : *sfImage) -> ();
    }
}

#[doc(hidden)]
pub struct Image {
    priv image : *csfml::sfImage
}

impl Image {
    /**
    * Create an image
    */
    pub fn new(width : uint, height : uint) -> Image {
        Image { image : unsafe {csfml::sfImage_create(width as c_uint, height as c_uint)} }
    }

    /**
    * Create an image and fill it with a unique color
    */
    pub fn new_from_color(width : uint, height : uint, color : &Color) -> Image {
        Image { image : unsafe {csfml::sfImage_createFromColor(width as c_uint, height as c_uint, *color)} }
    }
    
    /**
    * Create an image from a file on disk
    */
    pub fn new_from_file(filename : ~str) -> Image {
        do str::as_c_str(filename) |filebuf| {
            Image { image : unsafe {csfml::sfImage_createFromFile(filebuf)} }
        }
    }
    
    /**
    * Copy an existing image 
    */
    pub fn new_copy(image : &Image) -> Image {
        Image { image : unsafe {csfml::sfImage_copy(image.unwrap())} }
    }

    pub fn create_from_pixels(width : uint, height : uint, pixels : ~[u8]) -> Image {
        unsafe {
            Image { image : csfml::sfImage_createFromPixels(width as c_uint, height as c_uint, vec::raw::to_ptr(pixels))}
        }
    }

    /**
    * Save an image to a file on disk
    */
    pub fn save_to_file(&self, filename : ~str) -> bool {
        do str::as_c_str(filename) |filebuf| {
            match unsafe {csfml::sfImage_saveToFile(self.image, filebuf)} {
                0 => false,
                _ => true
            }
        }
    }
    
    /**
    * Return the size of an image 
    */
    pub fn get_size(&self) -> Vector2u {
        unsafe {csfml::sfImage_getSize(self.image)}
    }
    
    /**
    * Create a transparency mask from a specified color-key
    */
    pub fn create_mask_from_color(&self, color : &Color, alpha : u8) -> () {
        unsafe {
            csfml::sfImage_createMaskFromColor(self.image, *color, alpha)
        }
    }
    
    /**
    * Change the color of a pixel in an image
    */
    pub fn set_pixel(&self, x : uint, y : uint, color : &Color) -> () {
        unsafe {
            csfml::sfImage_setPixel(self.image, x as c_uint, y as c_uint, *color)
        }
    }

    /**
    * Get the color of a pixel in an image
    */
    pub fn get_pixel(&self, x : uint, y : uint) -> Color {
        unsafe {csfml::sfImage_getPixel(self.image, x as c_uint, y as c_uint)}
    }

    /**
    * Flip an image horizontally (left <-> right)
    */
    pub fn flip_horizontally(&self) -> () {
        unsafe {
            csfml::sfImage_flipHorizontally(self.image)
        }
    }
    
    /**
    * Flip an image vertically (top <-> bottom)
    */
    pub fn flip_vertically(&self) -> () {
        unsafe {
            csfml::sfImage_flipVertically(self.image)
        }
    }

    pub fn copy_image(&self, source : &Image, destX : uint, destY : uint, sourceRect : &IntRect, applyAlpha : bool) -> () {
        match applyAlpha {
            true        =>  unsafe { csfml::sfImage_copyImage(self.image, source.unwrap(), destX as c_uint, destY as c_uint, *sourceRect, 1) },
            false       =>  unsafe { csfml::sfImage_copyImage(self.image, source.unwrap(), destX as c_uint, destY as c_uint, *sourceRect, 0) }
        }
    }

    #[doc(hidden)]
    pub fn wrap(image : *csfml::sfImage) -> Image {
        Image { image : image }
    }
    
    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfImage {
        self.image
    }
}

impl Drop for Image {
    /**
    * Destroy an existing image
    */
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfImage_destroy(self.image)
        }
    }
}