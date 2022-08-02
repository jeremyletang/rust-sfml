use crate::{ffi, system::Time};

#[derive(Default, Debug, Clone, Copy)]
/// Structure defining a time range
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
