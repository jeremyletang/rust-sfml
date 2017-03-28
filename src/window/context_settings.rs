use csfml_window_sys::sfContextSettings;
use std::os::raw::c_uint;
use system::raw_conv::{FromRaw, Raw};
use system::{SfBool, SF_FALSE};

/// Non-debug, compatibility context (this and the core attribute are mutually exclusive).
pub const CONTEXT_DEFAULT: u32 = 0;
/// Core attribute.
pub const CONTEXT_CORE: u32 = 1;
/// Debug attribute.
pub const CONTEXT_DEBUG: u32 = 1 << 2;

/// Structure defining the settings of the OpenGL context attached to a window.
///
/// `ContextSettings` allows to define several advanced settings of the OpenGL context attached
/// to a window.
///
/// All these settings with the exception of the compatibility flag and anti-aliasing level
/// have no impact on the regular SFML rendering (graphics module), so you may need to use
/// this structure only if you're using SFML as a windowing system for custom OpenGL rendering.
///
/// The `depth_bits` and `stencil_bits` members define the number of bits per pixel requested for
/// the (respectively) depth and stencil buffers.
///
/// `antialiasing_level` represents the requested number of multisampling levels for anti-aliasing.
///
/// `major_version` and `minor_version` define the version of the OpenGL context that you want.
/// Only versions greater or equal to 3.0 are relevant; versions lesser than 3.0 are all handled
/// the same way (i.e. you can use any version < 3.0 if you don't want an OpenGL 3 context).
///
/// When requesting a context with a version greater or equal to 3.2, you have the option of
/// specifying whether the context should follow the core or compatibility profile of
/// all newer (>= 3.2) OpenGL specifications. For versions 3.0 and 3.1 there is only the
/// core profile. By default a compatibility context is created. You only need to specify the
/// core flag if you want a core profile context to use with your own OpenGL rendering.
/// Warning: The graphics module will not function if you request a core profile context.
/// Make sure the attributes are set to Default if you want to use the graphics module.
///
/// Setting the debug attribute flag will request a context with additional debugging features
/// enabled. Depending on the system, this might be required for advanced OpenGL debugging.
/// OpenGL debugging is disabled by default.
///
/// Special Note for OS X: Apple only supports choosing between either a
/// legacy context (OpenGL 2.1) or a core context (OpenGL version depends on the
/// operating system version but is at least 3.2). Compatibility contexts are not supported.
/// Further information is available on the OpenGL Capabilities Tables page.
/// OS X also currently does not support debug contexts.
///
/// Please note that these values are only a hint. No failure will be reported if one or more
/// of these values are not supported by the system; instead, SFML will try to find the closest
/// valid match. You can then retrieve the settings that the window actually used to create
/// its context, with `Window::settings()`.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct ContextSettings {
    /// Bits of the depth buffer.
    pub depth_bits: c_uint,
    /// Bits of the stencil buffer.
    pub stencil_bits: c_uint,
    /// Level of antialiasing.
    pub antialiasing_level: c_uint,
    /// Major number of the context version to create.
    pub major_version: c_uint,
    /// Minor number of the context version to create.
    pub minor_version: c_uint,
    /// The attribute flags to create the context with.
    pub attribute_flags: u32,
    /// Whether the context framebuffer is sRGB capable.
    pub srgb_capable: SfBool,
}

impl Default for ContextSettings {
    /// Creates a `ContextSettings` with the following values:
    ///
    /// ```
    /// # use sfml::window::{ContextSettings, CONTEXT_DEFAULT};
    /// # use sfml::system::SF_FALSE;
    /// let values = ContextSettings {
    ///     depth_bits: 0,
    ///     stencil_bits: 0,
    ///     antialiasing_level: 0,
    ///     major_version: 1,
    ///     minor_version: 1,
    ///     attribute_flags: CONTEXT_DEFAULT,
    ///     srgb_capable: SF_FALSE,
    /// };
    /// assert_eq!(ContextSettings::default(), values);
    /// ```
    fn default() -> ContextSettings {
        ContextSettings {
            depth_bits: 0,
            stencil_bits: 0,
            antialiasing_level: 0,
            major_version: 1,
            minor_version: 1,
            attribute_flags: CONTEXT_DEFAULT,
            srgb_capable: SF_FALSE,
        }
    }
}

impl Raw for ContextSettings {
    type Raw = sfContextSettings;

    fn raw(&self) -> Self::Raw {
        unsafe { ::std::mem::transmute(*self) }
    }
}

impl FromRaw for ContextSettings {
    type RawFrom = sfContextSettings;
    unsafe fn from_raw(raw: Self::RawFrom) -> Self {
        ::std::mem::transmute(raw)
    }
}
