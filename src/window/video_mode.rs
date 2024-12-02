use crate::{
    cpp::{CppVector, CppVectorItem},
    ffi::window as ffi,
    window::thread_safety,
};

/// `VideoMode` defines a video mode (width, height, bpp)
///
/// A video mode is defined by a width and a height (in pixels) and a depth (in bits per pixel).
///
/// Video modes are used to setup windows at creation time.
///
/// The main usage of video modes is for fullscreen mode: indeed you must use one of the valid
/// video modes allowed by the OS (which are defined by what the monitor and
/// the graphics card support), otherwise your window creation will just fail.
///
/// `VideoMode` provides an associated function for retrieving the list of all the video modes
/// supported by the system: [`VideoMode::fullscreen_modes`].
///
/// A custom video mode can also be checked directly for fullscreen compatibility
/// with its [`VideoMode::is_valid`] function.
///
/// Additionally, `VideoMode` provides a static function to get the mode currently used by
/// the desktop: [`VideoMode::desktop_mode`]. This allows to build windows with the same size or
/// pixel depth as the current resolution.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VideoMode {
    /// Video mode width, in pixels
    pub width: std::ffi::c_uint,
    /// Video mode height, in pixels
    pub height: std::ffi::c_uint,
    /// Video mode pixel depth, in bits per pixels
    pub bits_per_pixel: std::ffi::c_uint,
}

impl VideoMode {
    /// Constructs a new `VideoMode` from the given parameters.
    #[must_use]
    pub const fn new(width: u32, height: u32, bits_per_pixel: u32) -> Self {
        Self {
            width,
            height,
            bits_per_pixel,
        }
    }

    /// Tell whether or not a video mode is valid
    ///
    /// The validity of video modes is only relevant when using
    /// fullscreen windows; otherwise any video mode can be used
    /// with no restriction.
    ///
    /// return true if the video mode is valid for fullscreen mode
    #[must_use]
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::sfVideoMode_isValid(*self) }
    }

    /// Static Method, get the current desktop video mode
    ///
    /// return the current desktop video mode
    #[must_use]
    pub fn desktop_mode() -> Self {
        thread_safety::set_window_thread();

        unsafe { ffi::sfVideoMode_getDesktopMode() }
    }

    /// Static Method, retrieve all the video modes supported in fullscreen mode
    ///
    /// When creating a fullscreen window, the video mode is restricted
    /// to be compatible with what the graphics driver and monitor
    /// support. This function returns the complete list of all video
    /// modes that can be used in fullscreen mode.
    /// The returned array is sorted from best to worst, so that
    /// the first element will always give the best mode (higher
    /// `width`, `height` and `bits_per_pixel`).
    ///
    /// Return a vector containing all the supported `VideoMode`s
    #[must_use]
    pub fn fullscreen_modes() -> &'static CppVector<Self> {
        unsafe { &*ffi::sfVideoMode_getFullscreenModes() }
    }
}

impl From<(u32, u32)> for VideoMode {
    /// Constructs a `VideoMode` from `(w, h)`. Bit depth is 32.
    fn from((w, h): (u32, u32)) -> Self {
        Self::new(w, h, 32)
    }
}

impl From<[u32; 2]> for VideoMode {
    /// Constructs a `VideoMode` from `[w, h]`. Bit depth is 32.
    fn from([w, h]: [u32; 2]) -> Self {
        Self::new(w, h, 32)
    }
}

impl Default for VideoMode {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

unsafe impl CppVectorItem for VideoMode {
    fn get_data(vec: &crate::cpp::CppVector<Self>) -> *const Self {
        unsafe { ffi::sfVideoModeVector_getData(vec) }
    }

    fn get_len(vec: &crate::cpp::CppVector<Self>) -> usize {
        unsafe { ffi::sfVideoModeVector_getLength(vec) }
    }
    // We never get a video mode vector we must drop, so this is no-op
    fn del(_vec: &mut CppVector<Self>) {}
}
