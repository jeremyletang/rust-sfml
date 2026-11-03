use super::CppVectorItem;

decl_opaque! {
/// Opaque handle to a C++ `std::string`
pub CppString;
}

impl CppString {
    /// Attempt to get `&str` out of this `CppString`, as long as it's valid UTF-8
    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.data())
    }

    /// Convert to a Rust `String`, replacing invalid UTF-8 sequences.
    pub(crate) fn to_rust_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.data()).into_owned()
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

impl PartialEq<str> for CppString {
    fn eq(&self, other: &str) -> bool {
        self.data() == other.as_bytes()
    }
}

impl PartialEq<&str> for CppString {
    fn eq(&self, other: &&str) -> bool {
        self.data() == other.as_bytes()
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

unsafe impl CppVectorItem for CppString {
    fn get_data(vec: &super::CppVector<Self>) -> *const Self {
        unsafe { crate::ffi::system::sfStdStringVector_getData(vec) }
    }

    fn get_len(vec: &super::CppVector<Self>) -> usize {
        unsafe { crate::ffi::system::sfStdStringVector_getLength(vec) }
    }

    // A C++ std::string is not zero-sized, so the element address has to come from C++
    fn get_at(vec: &super::CppVector<Self>, index: usize) -> *const Self {
        unsafe { crate::ffi::system::sfStdStringVector_getAt(vec, index) }
    }

    fn del(vec: &mut super::CppVector<Self>) {
        unsafe {
            crate::ffi::system::sfStdStringVector_del(vec);
        }
    }
}
