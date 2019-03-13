use widestring::{U32CStr, U32CString};

/// An owned string type that's compatible with `sf::String`.
///
/// It uses UTF-32 encoding, which is compatible with `sf::String`'s internal representation.
#[derive(Debug)]
pub struct SfString(U32CString);

impl SfString {
    pub(crate) fn as_ptr(&self) -> *const u32 {
        self.0.as_ptr()
    }
}

impl<'a> From<&'a str> for SfString {
    /// Panics if the string data contains nul bytes.
    fn from(src: &'a str) -> Self {
        Self(U32CString::from_str(src).unwrap())
    }
}

impl<'a> From<&'a String> for SfString {
    fn from(src: &'a String) -> Self {
        src[..].into()
    }
}

/// A borrowed string type that's compatible with `sf::String`.
///
/// It uses UTF-32 encoding, which is compatible with `sf::String`'s internal representation.
#[derive(Debug)]
#[repr(transparent)]
pub struct SfStr(U32CStr);

impl SfStr {
    pub(crate) unsafe fn from_ptr_str<'a>(p: *const u32) -> &'a Self {
        std::mem::transmute(U32CStr::from_ptr_str(p))
    }
    /// Convert to a UTF-8 `String` from the Rust standard library.
    ///
    /// Panics if the string is not valid UTF-32.
    pub fn to_rust_string(&self) -> String {
        self.0.to_string().unwrap()
    }
}
