use crate::decl_opaque;
pub use crate::ffi::*;

decl_opaque! {
    sfStdString;
    sfClock;
}

impl Dispose for sfStdString {
    unsafe fn dispose(&mut self) {
        sfStdString_destroy(self)
    }
}

#[repr(C)]
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct sfStdStringVector {
    _opaque: [u8; 0],
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

impl Dispose for sfStdStringVector {
    unsafe fn dispose(&mut self) {
        sfStdStringVector_destroy(self);
    }
}

#[repr(C)]
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct sfString {
    _opaque: [u8; 0],
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
        let ustr = U32Str::from_slice(data);
        write!(f, "{}", ustr.to_string_lossy())
    }
}

impl sfStdString {
    fn data(&self) -> &[u8] {
        unsafe {
            let len = sfStdString_getLength(self);
            let data = sfStdString_getData(self);
            std::slice::from_raw_parts(data as *const u8, len)
        }
    }
}

impl Display for sfStdString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data();
        let string = String::from_utf8_lossy(data);
        write!(f, "{}", string)
    }
}

extern "C" {
    pub fn sfClipboard_getUnicodeString() -> *const sfUint32;
    pub fn sfClipboard_setUnicodeString(text: *const sfUint32);

    pub fn sfClock_create() -> *mut sfClock;
    pub fn sfClock_copy(clock: *const sfClock) -> *mut sfClock;
    pub fn sfClock_destroy(clock: *mut sfClock);
    pub fn sfClock_getElapsedTime(clock: *const sfClock) -> sfTime;
    pub fn sfClock_restart(clock: *mut sfClock) -> sfTime;

    pub fn sfSleep(duration: sfTime);
    pub fn sfStdString_getLength(s: *const sfStdString) -> usize;
    pub fn sfStdString_getData(s: *const sfStdString) -> *const c_char;

    pub fn sfString_getData(string: *const sfString) -> *const u32;
    pub fn sfString_getLength(string: *const sfString) -> usize;
    pub fn sfStdString_destroy(std_string: *mut sfStdString);
    pub fn sfStdStringVector_getLength(vec: *const sfStdStringVector) -> usize;
    pub fn sfStdStringVector_index(
        vec: *const sfStdStringVector,
        index: usize,
    ) -> *const sfStdString;
    pub fn sfStdStringVector_destroy(vec: *mut sfStdStringVector);
}
