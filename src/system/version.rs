use crate::ffi::system as ffi;

/// Version of the SFML library
///
/// This is the version of the SFML library that is actually loaded at runtime,
/// as opposed to the version the crate was built against.
///
/// # Examples
///
/// ```
/// # use sfml::system::version;
/// let version = version();
/// assert!(version.major() >= 3);
/// assert!(version.minor() >= 0);
/// assert!(version.patch() >= 0);
/// assert!(version.as_str().starts_with(&format!("{}.", version.major())));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version {
    major: i8,
    minor: i8,
    patch: i8,
    is_release: bool,
    string: &'static str,
}

impl Version {
    /// Get the major version number
    #[must_use]
    pub fn major(&self) -> i8 {
        self.major
    }

    /// Get the minor version number
    #[must_use]
    pub fn minor(&self) -> i8 {
        self.minor
    }

    /// Get the patch version number
    #[must_use]
    pub fn patch(&self) -> i8 {
        self.patch
    }

    /// Check whether this is a release version, as opposed to a development version
    #[must_use]
    pub fn is_release(&self) -> bool {
        self.is_release
    }

    /// Get the string representation of the version, e.g. `3.1.0`
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.string
    }
}

/// Get the runtime version of the SFML library
///
/// # Usage Example
///
/// ```
/// # use sfml::system::version;
/// let version = version();
/// println!("SFML {} (major {})", version.as_str(), version.major());
/// ```
#[must_use]
pub fn version() -> Version {
    unsafe {
        let version = ffi::sfVersion_get();
        let data = ffi::sfVersion_getString(version);
        let len = ffi::sfVersion_getStringLength(version);
        let string =
            std::str::from_utf8(std::slice::from_raw_parts(data.cast(), len)).unwrap_or_default();
        Version {
            major: ffi::sfVersion_getMajor(version),
            minor: ffi::sfVersion_getMinor(version),
            patch: ffi::sfVersion_getPatch(version),
            is_release: ffi::sfVersion_getIsRelease(version),
            string,
        }
    }
}

#[test]
fn test_version() {
    let version = version();
    assert_eq!(
        version.as_str(),
        format!(
            "{}.{}.{}{}",
            version.major(),
            version.minor(),
            version.patch(),
            if version.is_release() { "" } else { "-dev" }
        )
    );
}
