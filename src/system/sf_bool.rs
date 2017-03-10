use csfml_system_sys::{sfBool, sfTrue, sfFalse};

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
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
pub struct SfBool(sfBool);

/// The `false` boolean value.
pub const FALSE: SfBool = SfBool(sfFalse);
/// The `true` boolean value.
pub const TRUE: SfBool = SfBool(sfTrue);

impl From<bool> for SfBool {
    fn from(src: bool) -> Self {
        if src { TRUE } else { FALSE }
    }
}

impl Into<bool> for SfBool {
    fn into(self) -> bool {
        self == TRUE
    }
}
