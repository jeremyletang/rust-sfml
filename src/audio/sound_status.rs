use crate::ffi;
/// Enumeration of statuses for sounds and musics
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct SoundStatus(pub(super) ffi::audio::sfSoundStatus);

impl SoundStatus {
    /// Sound is not playing.
    pub const STOPPED: Self = Self(ffi::audio::sfSoundStatus::Stopped);
    /// Sound is paused.
    pub const PAUSED: Self = Self(ffi::audio::sfSoundStatus::Paused);
    /// Sound is playing.
    pub const PLAYING: Self = Self(ffi::audio::sfSoundStatus::Playing);
}
