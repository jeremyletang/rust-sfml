/// Enumeration of statuses for sounds and musics
#[repr(u32)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum SoundStatus {
    /// Sound is not playing.
    Stopped = 0,
    /// Sound is paused.
    Paused = 1,
    /// Sound is playing.
    Playing = 2,
}
