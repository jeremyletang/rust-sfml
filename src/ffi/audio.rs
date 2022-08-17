use crate::ffi::system::{sfStdString, sfStdStringVector};
pub use crate::ffi::*;

use super::system::sfInputStream;

decl_opaque! {
    sfSoundBuffer;
    sfSoundBufferRecorder;
    sfSoundRecorder;
    sfMusic;
    sfSound;
    sfSoundStream;
}

#[repr(C)]
pub struct sfSoundStreamChunk {
    pub samples: *mut i16,
    pub sample_count: c_uint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum sfSoundStatus {
    /// Sound is not playing
    Stopped,
    /// Sound is paused
    Paused,
    /// Sound is playing
    Playing,
}

type sfMusicStatus = sfSoundStatus;
type sfSoundStreamStatus = sfSoundStatus;

#[repr(C)]
pub struct sfTimeSpan {
    /// The beginning offset of the time range
    pub offset: i64,
    /// The length of the time range
    pub length: i64,
}

type sfSoundRecorderStartCallback = Option<unsafe extern "C" fn(user_data: *mut c_void) -> bool>;
type sfSoundRecorderProcessCallback =
    Option<unsafe extern "C" fn(samples: *const i16, len: usize, user_data: *mut c_void) -> bool>;
type sfSoundRecorderStopCallback = Option<unsafe extern "C" fn(user_data: *mut c_void)>;

type sfSoundStreamGetDataCallback =
    Option<unsafe extern "C" fn(chunk: *mut sfSoundStreamChunk, user_data: *mut c_void) -> bool>;
type sfSoundStreamSeekCallback = Option<unsafe extern "C" fn(pos: i64, user_data: *mut c_void)>;

include!("audio_bindgen.rs");
