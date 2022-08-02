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
    pub fn sfMusic_createFromStream(stream: *mut crate::ffi::system::sfInputStream)
        -> *mut sfMusic;
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
    // Sound
    pub fn sfSound_create() -> *mut sfSound;
    pub fn sfSound_copy(sound: *const sfSound) -> *mut sfSound;
    pub fn sfSound_destroy(sound: *mut sfSound);
    pub fn sfSound_play(sound: *mut sfSound);
    pub fn sfSound_pause(sound: *mut sfSound);
    pub fn sfSound_stop(sound: *mut sfSound);
    pub fn sfSound_setBuffer(sound: *mut sfSound, buffer: *const sfSoundBuffer);
    pub fn sfSound_getBuffer(sound: *const sfSound) -> *const sfSoundBuffer;
    pub fn sfSound_setLoop(sound: *mut sfSound, loop_: bool);
    pub fn sfSound_getLoop(sound: *const sfSound) -> bool;
    pub fn sfSound_getStatus(sound: *const sfSound) -> sfSoundStatus;
    pub fn sfSound_setPitch(sound: *mut sfSound, pitch: f32);
    pub fn sfSound_setVolume(sound: *mut sfSound, volume: f32);
    pub fn sfSound_setPosition(sound: *mut sfSound, position: sfVector3f);
    pub fn sfSound_setRelativeToListener(sound: *mut sfSound, relative: bool);
    pub fn sfSound_setMinDistance(sound: *mut sfSound, distance: f32);
    pub fn sfSound_setAttenuation(sound: *mut sfSound, attenuation: f32);
    pub fn sfSound_setPlayingOffset(sound: *mut sfSound, timeOffset: i64);
    pub fn sfSound_getPitch(sound: *const sfSound) -> f32;
    pub fn sfSound_getVolume(sound: *const sfSound) -> f32;
    pub fn sfSound_getPosition(sound: *const sfSound) -> sfVector3f;
    pub fn sfSound_isRelativeToListener(sound: *const sfSound) -> bool;
    pub fn sfSound_getMinDistance(sound: *const sfSound) -> f32;
    pub fn sfSound_getAttenuation(sound: *const sfSound) -> f32;
    pub fn sfSound_getPlayingOffset(sound: *const sfSound) -> i64;
    // SoundBuffer
    pub fn sfSoundBuffer_createFromFile(filename: *const c_char) -> *mut sfSoundBuffer;
    pub fn sfSoundBuffer_createFromMemory(
        data: *const c_void,
        sizeInBytes: usize,
    ) -> *mut sfSoundBuffer;
    pub fn sfSoundBuffer_createFromStream(
        stream: *mut crate::ffi::system::sfInputStream,
    ) -> *mut sfSoundBuffer;
    pub fn sfSoundBuffer_createFromSamples(
        samples: *const i16,
        sampleCount: u64,
        channelCount: c_uint,
        sampleRate: c_uint,
    ) -> *mut sfSoundBuffer;
    pub fn sfSoundBuffer_copy(soundBuffer: *const sfSoundBuffer) -> *mut sfSoundBuffer;
    pub fn sfSoundBuffer_destroy(soundBuffer: *mut sfSoundBuffer);
    pub fn sfSoundBuffer_saveToFile(
        soundBuffer: *const sfSoundBuffer,
        filename: *const c_char,
    ) -> bool;
    pub fn sfSoundBuffer_getSamples(soundBuffer: *const sfSoundBuffer) -> *const i16;
    pub fn sfSoundBuffer_getSampleCount(soundBuffer: *const sfSoundBuffer) -> u64;
    pub fn sfSoundBuffer_getSampleRate(soundBuffer: *const sfSoundBuffer) -> c_uint;
    pub fn sfSoundBuffer_getChannelCount(soundBuffer: *const sfSoundBuffer) -> c_uint;
    pub fn sfSoundBuffer_getDuration(soundBuffer: *const sfSoundBuffer) -> i64;
}

#[repr(C)]
pub struct sfTimeSpan {
    /// The beginning offset of the time range
    pub offset: i64,
    /// The length of the time range
    pub length: i64,
}
