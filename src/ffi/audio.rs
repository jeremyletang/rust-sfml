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
}
