use csfml_audio_sys as ffi;
/// Enumeration of statuses for sounds and musics
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct SoundStatus(pub(super) ffi::sfSoundStatus);

impl SoundStatus {
    /// Sound is not playing.
    pub const STOPPED: Self = Self(ffi::sfSoundStatus_sfStopped);
    /// Sound is paused.
    pub const PAUSED: Self = Self(ffi::sfSoundStatus_sfPaused);
    /// Sound is playing.
    pub const PLAYING: Self = Self(ffi::sfSoundStatus_sfPlaying);
}
