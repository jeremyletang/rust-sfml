use crate::{ffi, sf_bool_ext::SfBoolExt, window::thread_safety};

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
///
/// # Usage example
///
#[cfg_attr(feature = "ci-headless", doc = "```no_run")]
#[cfg_attr(not(feature = "ci-headless"), doc = "```")]
/// use sfml::window::{VideoMode, Window, Style};
///
/// // Display the list of all the video modes available for fullscreen
/// let modes = VideoMode::fullscreen_modes();
///
/// for mode in modes {
///     println!("{:?}", mode);
/// }
///
/// // Create a window with the same pixel depth as the desktop
/// let desktop = VideoMode::desktop_mode();
/// let _window = Window::new(VideoMode::new(1024, 768, desktop.bits_per_pixel),
///                           "SFML window",
///                           Style::CLOSE,
///                           &Default::default());
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct VideoMode {
    /// Video mode width, in pixels.
    pub width: u32,
    /// Video mode height, in pixels.
    pub height: u32,
    /// Video mode pixel depth, in bits per pixels.
    pub bits_per_pixel: u32,
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
        unsafe { ffi::sfVideoMode_isValid(self.raw()) }.into_bool()
    }

    /// Static Method, get the current desktop video mode
    ///
    /// return the urrent desktop video mode
    #[must_use]
    pub fn desktop_mode() -> Self {
        thread_safety::set_window_thread();

        unsafe { Self::from_raw(ffi::sfVideoMode_getDesktopMode()) }
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
    pub fn fullscreen_modes() -> &'static ffi::sfVideoModeVector {
        unsafe { &*ffi::sfVideoMode_getFullscreenModes() }
    }
    pub(crate) fn raw(&self) -> ffi::sfVideoMode {
        ffi::sfVideoMode {
            width: self.width,
            height: self.height,
            bitsPerPixel: self.bits_per_pixel,
        }
    }
    fn from_raw(raw: ffi::sfVideoMode) -> Self {
        Self::new(raw.width, raw.height, raw.bitsPerPixel)
    }
}

impl From<(u32, u32)> for VideoMode {
    /// Constructs a `VideoMode` from `(w, h)`. Bit depth is 32.
    fn from(src: (u32, u32)) -> Self {
        Self::new(src.0, src.1, 32)
    }
}

impl Default for VideoMode {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}
