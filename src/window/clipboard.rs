//! Give access to the system clipboard.
//!
//! `Clipboard` provides an interface for getting and setting the contents of the
//! system clipboard.
//!
//! It is important to note that due to limitations on some operating systems,
//! setting the clipboard contents is only guaranteed to work if there is currently an
//! open window for which events are being handled.

use crate::{
    cpp::FBox,
    ffi::system as ffi,
    system::{SfStr, SfStrConv},
};

/// Get the content of the clipboard as string data.
///
/// If the clipboard does not contain a string, it returns an empty string.
///
/// # Panics
///
/// Panics if the clipboard string allocation fails.
#[must_use]
pub fn get_string() -> String {
    unsafe {
        let sf_string = FBox::new(crate::ffi::window::sfClipboard_getUnicodeString())
            .expect("Failed to get clipboard string");
        SfStr::from_ptr_str(ffi::sfString_getData(&*sf_string)).to_rust_string()
    }
}

/// Set the content of the clipboard as string data.
///
/// This function sets the content of the clipboard as a string.
///
/// # Warning
/// Due to limitations on some operating systems,
/// setting the clipboard contents is only guaranteed to work if there is currently an
/// open window for which events are being handled.
pub fn set_string<S: SfStrConv>(string: S) {
    string.with_as_sfstr(|sfstr| unsafe {
        crate::ffi::window::sfClipboard_setUnicodeString(sfstr.as_ptr());
    });
}

#[cfg_attr(not(feature = "ci-headless"), test)]
#[cfg_attr(feature = "ci-headless", expect(dead_code))]
fn identity_test() {
    set_string("Hello world");
    assert_eq!(&get_string(), "Hello world");
}
