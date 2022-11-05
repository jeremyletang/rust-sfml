use widestring::{U32CStr, U32CString};

/// A borrowed string type that's compatible with `sf::String`.
///
/// It uses UTF-32 encoding, which is compatible with `sf::String`'s internal representation.
#[derive(Debug)]
#[repr(transparent)]
pub struct SfStr(U32CStr);

impl SfStr {
    pub(crate) unsafe fn from_ptr_str<'a>(p: *const u32) -> &'a Self {
        let ptr: *const U32CStr = U32CStr::from_ptr_str(p);
        &*(ptr as *const Self)
    }
    /// Convert to a UTF-8 `String` from the Rust standard library.
    ///
    /// Panics if the string is not valid UTF-32.
    #[must_use]
    pub fn to_rust_string(&self) -> String {
        self.0.to_string().unwrap()
    }
    pub(crate) fn as_ptr(&self) -> *const u32 {
        self.0.as_ptr()
    }
}

/// Trait for types that can be converted into `SfStr`
pub trait SfStrConv {
    #[doc(hidden)]
    fn with_as_sfstr<F, R>(self, fun: F) -> R
    where
        F: FnOnce(&SfStr) -> R;
}

impl<'a> SfStrConv for &'a SfStr {
    fn with_as_sfstr<F, R>(self, fun: F) -> R
    where
        F: FnOnce(&SfStr) -> R,
    {
        fun(self)
    }
}

impl<'a> SfStrConv for &'a str {
    fn with_as_sfstr<F, R>(self, fun: F) -> R
    where
        F: FnOnce(&SfStr) -> R,
    {
        let uc_string = U32CString::from_str(self).unwrap();
        let uc_str_ptr: *const U32CStr = uc_string.as_ucstr();
        let sf_str: &SfStr = unsafe { &*(uc_str_ptr as *const SfStr) };
        fun(sf_str)
    }
}

impl<'a> SfStrConv for &'a String {
    fn with_as_sfstr<F, R>(self, fun: F) -> R
    where
        F: FnOnce(&SfStr) -> R,
    {
        let str: &str = self;
        str.with_as_sfstr(fun)
    }
}
