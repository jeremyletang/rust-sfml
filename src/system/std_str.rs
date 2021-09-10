use std::str::Utf8Error;

/// Reference to a C++ `std::string` returned by SFML
#[derive(Debug)]
pub struct StdStr<'a>(pub(crate) &'a [u8]);

impl<'a> StdStr<'a> {
    /// Try to get a `&str` out of this `std::string`.
    ///
    /// Fails on invalid UTF-8.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.0)
    }
}
