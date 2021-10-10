use crate::ffi::window as ffi;

pub use ffi::sfContextSettings as ContextSettings;

impl ContextSettings {
    /// Non-debug, compatibility context (this and the core attribute are mutually exclusive).
    pub const ATTRIB_DEFAULT: u32 = 0;
    /// Core attribute.
    pub const ATTRIB_CORE: u32 = 1;
    /// Debug attribute.
    pub const ATTRIB_DEBUG: u32 = 1 << 2;
}

impl Default for ContextSettings {
    /// Creates a `ContextSettings` with the following values:
    ///
    /// - Depth bits: 0
    /// - Stencil bits: 0
    /// - Antialiasing level: 0
    /// - Major version: 1
    /// - Minor version: 1
    /// - Attribute flags: [`ATTRIB_DEFAULT`](ContextSettings::ATTRIB_DEFAULT)
    /// - SRGB capable: false
    fn default() -> Self {
        Self {
            depth_bits: 0,
            stencil_bits: 0,
            antialiasing_level: 0,
            major_version: 1,
            minor_version: 1,
            attribute_flags: Self::ATTRIB_DEFAULT,
            srgb_capable: false,
        }
    }
}
