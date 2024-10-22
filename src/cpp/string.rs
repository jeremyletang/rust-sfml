decl_opaque! {
/// Opaque handle to a C++ `std::string`
pub CppString;
/// Opaque handle to a C++ `std::vector<std::string>>`
pub CppStringVector;
}

impl CppString {
    /// Attempt to get `&str` out of this `CppString`, as long as it's valid UTF-8
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.data())
    }
}

impl PartialEq for CppString {
    fn eq(&self, other: &Self) -> bool {
        self.data() == other.data()
    }
}

impl PartialEq<CppString> for str {
    fn eq(&self, other: &CppString) -> bool {
        self.as_bytes() == other.data()
    }
}

impl CppString {
    fn data(&self) -> &[u8] {
        unsafe {
            let len = crate::ffi::system::sfStdString_getLength(self);
            let data = crate::ffi::system::sfStdString_getData(self);
            std::slice::from_raw_parts(data.cast(), len)
        }
    }
}

impl std::fmt::Display for CppString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data();
        let string = String::from_utf8_lossy(data);
        write!(f, "{string}")
    }
}

impl Drop for CppString {
    fn drop(&mut self) {
        unsafe { crate::ffi::system::sfStdString_del(self) }
    }
}

impl std::ops::Deref for CppStringVector {
    type Target = [CppString];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                crate::ffi::system::sfStdStringVector_getData(self),
                crate::ffi::system::sfStdStringVector_getLength(self),
            )
        }
    }
}

impl Drop for CppStringVector {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::system::sfStdStringVector_del(self);
        }
    }
}
