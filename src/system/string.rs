use {
    crate::ffi::system as ffi,
    core::fmt,
    std::error::Error,
    widestring::{error::Utf32Error, U32CStr, U32CString},
};

/// A borrowed string type that's compatible with `sf::String`.
///
/// It uses UTF-32 encoding, which is compatible with `sf::String`'s internal representation.
#[derive(Debug)]
#[repr(transparent)]
pub struct SfStr(U32CStr);

impl SfStr {
    pub(crate) unsafe fn from_ptr_str<'a>(p: *const u32) -> &'a Self {
        unsafe {
            let ptr: *const U32CStr = U32CStr::from_ptr_str(p);
            &*(ptr as *const Self)
        }
    }
    /// Convert to a UTF-8 `String` from the Rust standard library.
    ///
    /// # Panics
    ///
    /// Panics if the string is not valid UTF-32.
    #[must_use]
    #[expect(clippy::unwrap_used)]
    pub fn to_rust_string(&self) -> String {
        self.0.to_string().unwrap()
    }

    /// Convert to a UTF-8 `String` from the Rust standard library.
    ///
    /// Returns a `Result` and errors if the string is not valid UTF-32
    pub fn try_to_rust_string(&self) -> Result<String, SfStrConvError> {
        match self.0.to_string() {
            Ok(string) => Ok(string),
            Err(utf32error) => Err(SfStrConvError::from_utf32error(utf32error)),
        }
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

impl SfStrConv for &SfStr {
    fn with_as_sfstr<F, R>(self, fun: F) -> R
    where
        F: FnOnce(&SfStr) -> R,
    {
        fun(self)
    }
}

impl SfStrConv for &str {
    fn with_as_sfstr<F, R>(self, fun: F) -> R
    where
        F: FnOnce(&SfStr) -> R,
    {
        #[expect(clippy::unwrap_used)]
        let uc_string = U32CString::from_str(self).unwrap();
        let uc_str_ptr: *const U32CStr = uc_string.as_ucstr();
        let sf_str: &SfStr = unsafe { &*(uc_str_ptr as *const SfStr) };
        fun(sf_str)
    }
}

impl SfStrConv for &String {
    fn with_as_sfstr<F, R>(self, fun: F) -> R
    where
        F: FnOnce(&SfStr) -> R,
    {
        let str: &str = self;
        str.with_as_sfstr(fun)
    }
}

/// Errors which can occur when attempting to translate from UTF-32 to UTF-8
#[derive(Debug)]
pub struct SfStrConvError(Utf32Error);

impl SfStrConvError {
    fn from_utf32error(value: Utf32Error) -> Self {
        Self(value)
    }

    /// Returns the index in the string where the conversion error occured
    pub fn index(&self) -> usize {
        self.0.index()
    }

    /// Returns the underlying vector of values which generated the error in the first place.
    ///
    /// If the sequence that generated the error was a reference to a slice instead of a [`Vec`],
    /// this will return [`None`].
    #[must_use]
    pub fn into_vec(self) -> Option<Vec<u32>> {
        self.0.into_vec()
    }
}

impl Error for SfStrConvError {}

impl fmt::Display for SfStrConvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

decl_opaque! {
    /// Opaque handle to a C++ `sf::String`
    pub SfString;
}

impl SfString {
    fn data(&self) -> &[u32] {
        unsafe {
            let len = ffi::sfString_getLength(self);
            let data = ffi::sfString_getData(self);
            std::slice::from_raw_parts(data, len)
        }
    }
}

impl fmt::Display for SfString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data();
        let ustr = widestring::U32Str::from_slice(data);
        write!(f, "{}", ustr.to_string_lossy())
    }
}
