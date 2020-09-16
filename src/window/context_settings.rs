use csfml_window_sys as ffi;
use std::os::raw::c_uint;

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
/// its context, with [`Window::settings`].
///
/// [`Window::settings`]: crate::window::Window::settings
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct ContextSettings(pub(crate) ffi::sfContextSettings);

impl ContextSettings {
    /// Non-debug, compatibility context (this and the core attribute are mutually exclusive).
    pub const ATTRIB_DEFAULT: u32 = 0;
    /// Core attribute.
    pub const ATTRIB_CORE: u32 = 1;
    /// Debug attribute.
    pub const ATTRIB_DEBUG: u32 = 1 << 2;
    /// Bits of the depth buffer.
    #[must_use]
    pub fn depth_bits(&self) -> c_uint {
        self.0.depthBits
    }
    /// Set [`depth_bits`](Self::depth_bits).
    pub fn set_depth_bits(&mut self, value: c_uint) {
        self.0.depthBits = value;
    }
    /// Bits of the stencil buffer.
    #[must_use]
    pub fn stencil_bits(&self) -> c_uint {
        self.0.stencilBits
    }
    /// Set [`stencil_bits`](Self::stencil_bits)
    pub fn set_stencil_bits(&mut self, value: c_uint) {
        self.0.stencilBits = value;
    }
    /// Level of antialiasing.
    #[must_use]
    pub fn antialiasing_level(&self) -> c_uint {
        self.0.antialiasingLevel
    }
    /// Set [`antialiasing_level`](Self::antialiasing_level)
    pub fn set_antialiasing_level(&mut self, value: c_uint) {
        self.0.antialiasingLevel = value;
    }
    /// Major number of the context version to create.
    #[must_use]
    pub fn major_version(&self) -> c_uint {
        self.0.majorVersion
    }
    /// Set [`major_version`](Self::major_version)
    pub fn set_major_version(&mut self, value: c_uint) {
        self.0.majorVersion = value;
    }
    /// Minor number of the context version to create.
    #[must_use]
    pub fn minor_version(&self) -> c_uint {
        self.0.minorVersion
    }
    /// Set [`minor_version`](Self::minor_version)
    pub fn set_minor_version(&mut self, value: c_uint) {
        self.0.minorVersion = value;
    }
    /// The attribute flags to create the context with.
    #[must_use]
    pub fn attribute_flags(&self) -> u32 {
        self.0.attributeFlags
    }
    /// Set [`attribute_flags`](Self::attribute_flags)
    pub fn set_attribute_flags(&mut self, flags: u32) {
        self.0.attributeFlags = flags;
    }
    /// Whether the context framebuffer is sRGB capable.
    #[must_use]
    pub fn srgb_capable(&self) -> bool {
        self.0.sRgbCapable != 0
    }
    /// Set [`srgb_capable`](Self::srgb_capable)
    pub fn set_srgb_capable(&mut self, value: bool) {
        self.0.sRgbCapable = value as i32;
    }
}

impl Default for ContextSettings {
    /// Creates a `ContextSettings` with the following values:
    ///
    /// - Depth bits: 0
    /// - Stencil bits: 0
    /// - Antialiasing level: 0
    /// - Major version: 1
    /// - Minor version: 1
    /// - Attribute flags: [`ATTRIB_DEFAULT`](Self::ATTRIB_DEFAULT)
    /// - SRGB capable: false
    fn default() -> Self {
        Self(ffi::sfContextSettings {
            depthBits: 0,
            stencilBits: 0,
            antialiasingLevel: 0,
            majorVersion: 1,
            minorVersion: 1,
            attributeFlags: Self::ATTRIB_DEFAULT,
            sRgbCapable: csfml_system_sys::sfFalse,
        })
    }
}
