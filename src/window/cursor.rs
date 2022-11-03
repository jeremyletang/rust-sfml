use crate::{
    ffi::window::{self as ffi},
    sf_box::{Dispose, SfBox},
    system::Vector2u,
};

decl_opaque! {
/// Defines the appearance of a system cursor.
///
/// Warning: Features related to Cursor are not supported on iOS and Android.
///
/// This type abstracts the operating system resources associated with either a
/// native system cursor or a custom cursor.
///
/// After loading the cursor the graphical appearance with either
/// [`Cursor::from_pixels`] or [`Cursor::from_system`], the cursor can be changed with
/// [`Window::set_mouse_cursor`].
///
/// The behaviour is undefined if the cursor is destroyed while in use by the window.
///
/// [`Window::set_mouse_cursor`]: crate::window::Window::set_mouse_cursor
Cursor;
}

impl Dispose for Cursor {
    unsafe fn dispose(&mut self) {
        ffi::sfCursor_destroy(self);
    }
}

impl Cursor {
    /// Create a cursor with the provided image.
    ///
    /// `pixels` must be an array of width by height pixels in 32-bit RGBA format.
    /// If not, this will cause undefined behavior.
    ///
    /// If the width or height are 0, the current cursor is left unchanged and
    /// the function will return false.
    ///
    /// In addition to specifying the pixel data, you can also specify the location of
    /// the hotspot of the cursor. The hotspot is the pixel coordinate within the cursor image which
    /// will be located exactly where the mouse pointer position is.
    /// Any mouse actions that are performed will return the window/screen location of the hotspot.
    ///
    /// # Warning
    /// On Unix, the pixels are mapped into a monochrome bitmap:
    /// pixels with an alpha channel to 0 are transparent, black if the RGB channel are
    /// close to zero, and white otherwise.
    ///
    /// # Parameters
    /// - `pixels`: Array of pixels of the image
    /// - `size`: Width and height of the image
    /// - `hotspot`: (x,y) location of the hotspot
    ///
    /// # Returns
    /// `true` if the cursor was successfully loaded; `false` otherwise
    ///
    /// # Safety
    ///
    /// `pixels` must be an array of width by height pixels in 32-bit RGBA format.
    /// If not, this will cause undefined behavior.
    ///
    /// Binding maintainer's note:
    ///
    /// > I noticed that on at least Linux X11, if the size of the image is not a power of 2,
    /// > the image is loaded in a wrong way that doesn't respect the dimensions. This is also
    /// > why I decided to leave this function unsafe.
    #[must_use]
    pub unsafe fn from_pixels(
        pixels: &[u8],
        size: Vector2u,
        hotspot: Vector2u,
    ) -> Option<SfBox<Self>> {
        let cursor = ffi::sfCursor_createFromPixels(pixels.as_ptr(), size, hotspot);
        SfBox::new(cursor)
    }

    /// Create a native system cursor.
    ///
    /// Refer to the list of cursor available on each system (see `CursorType`) to
    /// know whether a given cursor is expected to load successfully or
    /// is not supported by the operating system.
    ///
    /// # Parameters
    /// - type: Native system cursor type
    ///
    /// # Returns
    /// true if and only if the corresponding cursor is natively supported by
    /// the operating system; false otherwise.
    #[must_use]
    pub fn from_system(type_: Type) -> Option<SfBox<Self>> {
        unsafe {
            let cursor = ffi::sfCursor_createFromSystem(type_);
            SfBox::new(cursor)
        }
    }
}
pub use ffi::sfCursorType as Type;
