use crate::{ffi, system::Time};

#[derive(Default, Debug, Clone, Copy)]
/// Structure defining a time range
///
/// # Examples
///
/// ```
/// use sfml::audio::TimeSpan;
/// use sfml::system::Time;
///
/// let span = TimeSpan {
///     offset: Time::seconds(1.),
///     length: Time::milliseconds(500),
/// };
/// assert_eq!(span.offset.as_milliseconds(), 1000);
/// assert_eq!(span.length.as_milliseconds(), 500);
/// ```
pub struct TimeSpan {
    /// The beginning offset of the time range.
    pub offset: Time,
    /// The length of the time range.
    pub length: Time,
}

impl TimeSpan {
    pub(crate) fn from_raw(raw: ffi::audio::sfTimeSpan) -> Self {
        Self {
            offset: Time::from_raw(raw.offset),
            length: Time::from_raw(raw.length),
        }
    }
    pub(crate) fn into_raw(self) -> ffi::audio::sfTimeSpan {
        ffi::audio::sfTimeSpan {
            offset: self.offset.raw(),
            length: self.length.raw(),
        }
    }
}
