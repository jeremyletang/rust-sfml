use csfml_system_sys::{sfBool, sfFalse, sfTrue};

/// Boolean type defined for purposes of interfacing with CSFML.
///
/// It implements `From<bool>` and `Into<bool>`, so you can easily convert
/// between the two types.
///
/// # Example
/// ```ignore
/// use sfml::window::ContextSettings;
/// let mut context_settings = ContextSettings::default();
/// // We can't use `true` directly, but we can use the `.into()` method.
/// context_settings.srgb_capable = true.into();
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Bool(sfBool);

impl Bool {
    /// The `false` boolean value.
    pub const FALSE: Bool = Bool(sfFalse);
    /// The `true` boolean value.
    pub const TRUE: Bool = Bool(sfTrue);
}

impl From<bool> for Bool {
    fn from(src: bool) -> Self {
        if src { Self::TRUE } else { Self::FALSE }
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        self == Self::TRUE
    }
}
