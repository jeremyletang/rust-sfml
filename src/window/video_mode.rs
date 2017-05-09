use csfml_window_sys as ffi;
use sf_bool_ext::SfBoolExt;

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
/// supported by the system: `fullscreen_modes()`.
///
/// A custom video mode can also be checked directly for fullscreen compatibility
/// with its `is_valid()` function.
///
/// Additionally, `VideoMode` provides a static function to get the mode currently used by
/// the desktop: `desktop_mode`. This allows to build windows with the same size or
/// pixel depth as the current resolution.
///
/// # Usage example
///
/// ```
/// use sfml::window::{VideoMode, Window, style};
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
///                           style::CLOSE,
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
    ///
    /// Return a new VideoMode initialized
    pub fn new(width: u32, height: u32, bits_per_pixel: u32) -> VideoMode {
        VideoMode {
            width: width,
            height: height,
            bits_per_pixel: bits_per_pixel,
        }
    }

    /// Tell whether or not a video mode is valid
    ///
    /// The validity of video modes is only relevant when using
    /// fullscreen windows; otherwise any video mode can be used
    /// with no restriction.
    ///
    /// return true if the video mode is valid for fullscreen mode
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::sfVideoMode_isValid(self.raw()) }.to_bool()
    }

    /// Static Method, get the current desktop video mode
    ///
    /// return the urrent desktop video mode
    pub fn desktop_mode() -> VideoMode {
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
    /// width, height and bits_per_pixel).
    ///
    /// Return a vector containing all the supported VideoMode
    pub fn fullscreen_modes() -> Vec<VideoMode> {
        let mut size = 0;
        let tab = unsafe { ffi::sfVideoMode_getFullscreenModes(&mut size) };
        if size == 0 {
            return Vec::new();
        }

        let size = size as u32;

        let tab_slice: &[ffi::sfVideoMode] =
            unsafe { ::std::slice::from_raw_parts(tab, size as usize) };

        let mut ret_tab = Vec::with_capacity(size as usize);

        for sf_video_mode in tab_slice.iter() {
            ret_tab.push(Self::from_raw(*sf_video_mode));
        }

        ret_tab
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
