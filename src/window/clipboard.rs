//! Give access to the system clipboard.
//!
//! `Clipboard` provides an interface for getting and setting the contents of the
//! system clipboard.
//!
//! It is important to note that due to limitations on some operating systems,
//! setting the clipboard contents is only guaranteed to work if there is currently an
//! open window for which events are being handled.
//!
//! Usage example:
//!
//! ```no_run
//! # use sfml::window::*;
//! // get the clipboard content as a string
//! let mut string = clipboard::get_string();
//! # let mut window: Window = unimplemented!();
//! // or use it in the event loop
//! while let Some(event) = window.poll_event()
//! {
//!     match event {
//!         Event::Closed => window.close(),
//!         Event::KeyPressed{ctrl, code, ..} => {
//!             // Using Ctrl + V to paste a string into SFML
//!             if ctrl && code == Key::V {
//!                 string = clipboard::get_string();
//!             }
//!             // Using Ctrl + C to copy a string out of SFML
//!             if(ctrl && code == Key::C) {
//!                 clipboard::set_string("Hello World!");
//!             }
//!         }
//!         _ => {}
//!     }
//! }
//! ```

use crate::{
    ffi::window as ffi,
    system::{SfStr, SfStrConv},
};

/// Get the content of the clipboard as string data.
///
/// This function returns the content of the clipboard as an SFML string.
/// If the clipboard does not contain string it returns an empty string.
#[must_use]
pub fn get_string() -> &'static SfStr {
    unsafe {
        let raw = ffi::sfClipboard_getUnicodeString();
        SfStr::from_ptr_str(raw)
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
        ffi::sfClipboard_setUnicodeString(sfstr.as_ptr());
    })
}
