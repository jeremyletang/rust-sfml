use crate::ffi::system::{sfStdString, sfStdStringVector};
pub use crate::ffi::*;

extern "C" {
    pub fn sfSoundRecorder_getDevice(rec: *const sfSoundRecorder) -> *const sfStdString;
    pub fn sfSoundRecorder_getDefaultDevice() -> *mut sfStdString;
    pub fn sfSoundRecorder_getAvailableDevices() -> *mut sfStdStringVector;
    pub fn sfSoundBufferRecorder_create() -> *mut sfSoundBufferRecorder;
    pub fn sfSoundBufferRecorder_destroy(bufRec: *mut sfSoundBufferRecorder);
    pub fn sfSoundBufferRecorder_start(
        bufRec: *mut sfSoundBufferRecorder,
        sampRate: c_uint,
    ) -> bool;
    pub fn sfSoundBufferRecorder_stop(bufRec: *mut sfSoundBufferRecorder);
    pub fn sfSoundBufferRecorder_getSampleRate(bufRec: *const sfSoundBufferRecorder) -> c_uint;
    pub fn sfSoundBufferRecorder_getBuffer(
        bufRec: *const sfSoundBufferRecorder,
    ) -> *const sfSoundBuffer;
    pub fn sfSoundBufferRecorder_setDevice(
        bufRec: *mut sfSoundBufferRecorder,
        name: *const c_char,
    ) -> bool;
    pub fn sfSoundBufferRecorder_getDevice(
        bufRec: *const sfSoundBufferRecorder,
    ) -> *const sfStdString;
    // Listener
    pub fn sfListener_setGlobalVolume(volume: f32);
    pub fn sfListener_getGlobalVolume() -> f32;
    pub fn sfListener_setPosition(position: sfVector3f);
    pub fn sfListener_getPosition() -> sfVector3f;
    pub fn sfListener_setDirection(direction: sfVector3f);
    pub fn sfListener_getDirection() -> sfVector3f;
    pub fn sfListener_setUpVector(upVector: sfVector3f);
    pub fn sfListener_getUpVector() -> sfVector3f;
    // Music
    pub fn sfMusic_createFromFile(filename: *const c_char) -> *mut sfMusic;
    pub fn sfMusic_createFromMemory(data: *const c_void, sizeInBytes: usize) -> *mut sfMusic;
    pub fn sfMusic_createFromStream(stream: *mut sfInputStream) -> *mut sfMusic;
    pub fn sfMusic_destroy(music: *mut sfMusic);
    pub fn sfMusic_setLoop(music: *mut sfMusic, loop_: bool);
    pub fn sfMusic_getLoop(music: *const sfMusic) -> bool;
    pub fn sfMusic_getDuration(music: *const sfMusic) -> i64;
    pub fn sfMusic_getLoopPoints(music: *const sfMusic) -> sfTimeSpan;
    pub fn sfMusic_setLoopPoints(music: *mut sfMusic, timePoints: sfTimeSpan);
    pub fn sfMusic_play(music: *mut sfMusic);
    pub fn sfMusic_pause(music: *mut sfMusic);
    pub fn sfMusic_stop(music: *mut sfMusic);
    pub fn sfMusic_getChannelCount(music: *const sfMusic) -> c_uint;
    pub fn sfMusic_getSampleRate(music: *const sfMusic) -> c_uint;
    pub fn sfMusic_getStatus(music: *const sfMusic) -> sfSoundStatus;
    pub fn sfMusic_getPlayingOffset(music: *const sfMusic) -> i64;
    pub fn sfMusic_setPitch(music: *mut sfMusic, pitch: f32);
    pub fn sfMusic_setVolume(music: *mut sfMusic, volume: f32);
    pub fn sfMusic_setPosition(music: *mut sfMusic, position: sfVector3f);
    pub fn sfMusic_setRelativeToListener(music: *mut sfMusic, relative: bool);
    pub fn sfMusic_setMinDistance(music: *mut sfMusic, distance: f32);
    pub fn sfMusic_setAttenuation(music: *mut sfMusic, attenuation: f32);
    pub fn sfMusic_setPlayingOffset(music: *mut sfMusic, timeOffset: i64);
    pub fn sfMusic_getPitch(music: *const sfMusic) -> f32;
    pub fn sfMusic_getVolume(music: *const sfMusic) -> f32;
    pub fn sfMusic_getPosition(music: *const sfMusic) -> sfVector3f;
    pub fn sfMusic_isRelativeToListener(music: *const sfMusic) -> bool;
    pub fn sfMusic_getMinDistance(music: *const sfMusic) -> f32;
    pub fn sfMusic_getAttenuation(music: *const sfMusic) -> f32;
}

#[repr(C)]
pub struct sfTimeSpan {
    /// The beginning offset of the time range
    pub offset: i64,
    /// The length of the time range
    pub length: i64,
}
