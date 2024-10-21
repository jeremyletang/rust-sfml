pub use crate::ffi::*;

decl_opaque! {
    pub sfStdString;
    pub(crate) sfStdStringVector;
    pub(crate) sfString;
    pub(crate) sfInputStreamHelper;
}

use crate::system::Clock as sfClock;

pub type sfTime = i64;

impl Drop for sfStdString {
    fn drop(&mut self) {
        unsafe { sfStdString_del(self) }
    }
}

impl<'a> IntoIterator for &'a sfStdStringVector {
    type IntoIter = sfStdStringVectorIter<'a>;
    type Item = &'a sfStdString;
    fn into_iter(self) -> Self::IntoIter {
        sfStdStringVectorIter {
            vec: self,
            len: unsafe { sfStdStringVector_getLength(self) },
            cursor: 0,
        }
    }
}

#[derive(Debug)]
pub struct sfStdStringVectorIter<'a> {
    vec: &'a sfStdStringVector,
    len: usize,
    cursor: usize,
}

impl<'a> Iterator for sfStdStringVectorIter<'a> {
    type Item = &'a sfStdString;
    fn next(&mut self) -> Option<&'a sfStdString> {
        if self.cursor >= self.len {
            return None;
        }
        unsafe {
            let item = sfStdStringVector_index(self.vec, self.cursor);
            self.cursor += 1;
            Some(&*item)
        }
    }
}

impl sfStdString {
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.data())
    }
}

impl PartialEq for sfStdString {
    fn eq(&self, other: &Self) -> bool {
        self.data() == other.data()
    }
}

impl PartialEq<sfStdString> for str {
    fn eq(&self, other: &sfStdString) -> bool {
        self.as_bytes() == other.data()
    }
}

impl Drop for sfStdStringVector {
    fn drop(&mut self) {
        unsafe {
            sfStdStringVector_del(self);
        }
    }
}

impl sfString {
    fn data(&self) -> &[u32] {
        unsafe {
            let len = sfString_getLength(self);
            let data = sfString_getData(self);
            std::slice::from_raw_parts(data, len)
        }
    }
}

impl Display for sfString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data();
        let ustr = widestring::U32Str::from_slice(data);
        write!(f, "{}", ustr.to_string_lossy())
    }
}

impl sfStdString {
    fn data(&self) -> &[u8] {
        unsafe {
            let len = sfStdString_getLength(self);
            let data = sfStdString_getData(self);
            std::slice::from_raw_parts(data.cast(), len)
        }
    }
}

impl Display for sfStdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data();
        let string = String::from_utf8_lossy(data);
        write!(f, "{string}")
    }
}

type sfInputStreamHelperReadCb =
    Option<unsafe extern "C" fn(data: *mut c_void, size: i64, userData: *mut c_void) -> i64>;
type sfInputStreamHelperSeekCb =
    Option<unsafe extern "C" fn(pos: i64, user_data: *mut c_void) -> i64>;
type sfInputStreamHelperTellCb = Option<unsafe extern "C" fn(userData: *mut c_void) -> i64>;
type sfInputStreamHelperGetSizeCb = Option<unsafe extern "C" fn(user_data: *mut c_void) -> i64>;

include!("system_bindgen.rs");
