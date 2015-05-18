/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

use libc::{c_uint, c_float, c_char, size_t, c_uchar, c_void};
use system::{Vector3f, Time, InputStream};
use audio::SoundStatus;
use ffi::SfBool;

foreign_type! {
	sfMusic, sfMusic_destroy;
	sfSound, sfSound_destroy;
	sfSoundBuffer, sfSoundBuffer_destroy;
	sfSoundBufferRecorder, sfSoundBufferRecorder_destroy;
	sfSoundStream, sfSoundStream_destroy;
	sfSoundRecorder, sfSoundRecorder_destroy;
}

#[repr(C)]
pub struct sfSoundStreamChunk {
	pub samples: *const i16,
	pub sample_count: c_uint
}

pub type sfSoundStreamGetDataCallback = unsafe extern fn(chunk: *mut sfSoundStreamChunk, user: *mut c_void) -> SfBool;
pub type sfSoundStreamSeekCallback = unsafe extern fn(time: Time, user: *mut c_void);

pub type sfSoundRecorderStartCallback = unsafe extern fn(user: *mut c_void) -> SfBool;
pub type sfSoundRecorderProcessCallback = unsafe extern fn(buffer: *const i16, size: size_t, user: *mut c_void) -> SfBool;
pub type sfSoundRecorderStopCallback = unsafe extern fn(user: *mut c_void);

#[cfg_attr(any(target_os="macos", target_os="linux", target_os="windows"), link(name="csfml-audio"))]
extern "C" {
	pub fn sfListener_setGlobalVolume(volume: f32) -> ();
	pub fn sfListener_getGlobalVolume() -> f32;
	pub fn sfListener_setPosition(position: Vector3f) -> ();
	pub fn sfListener_getPosition() -> Vector3f;
	pub fn sfListener_setDirection(orientation: Vector3f) -> ();
	pub fn sfListener_getDirection() -> Vector3f;
	pub fn sfListener_setUpVector(upVector: Vector3f) -> ();
	pub fn sfListener_getUpVector() -> Vector3f;

	pub fn sfMusic_createFromFile(filename: *const c_char) -> *mut sfMusic;
	pub fn sfMusic_createFromMemory(data: *const c_uchar, sizeInBytes: size_t) -> *mut sfMusic;
	//  vv sfMusic_createFromStream(stream: *mut InputStream) -> *mut sfMusic;
	pub fn sfMusic_destroy(music: *mut sfMusic) -> ();
	pub fn sfMusic_setLoop(music: *mut sfMusic, lloop: SfBool) -> ();
	pub fn sfMusic_getLoop(music: *const sfMusic) -> SfBool;
	pub fn sfMusic_getDuration(music: *const sfMusic) -> Time;
	pub fn sfMusic_play(music: *mut sfMusic) -> ();
	pub fn sfMusic_pause(music: *mut sfMusic) -> ();
	pub fn sfMusic_stop(music: *mut sfMusic) -> ();
	pub fn sfMusic_getChannelCount(music: *const sfMusic) -> c_uint;
	pub fn sfMusic_getSampleRate(music: *const sfMusic) -> c_uint;
	pub fn sfMusic_getStatus(music: *const sfMusic) -> SoundStatus;
	pub fn sfMusic_getPlayingOffset(music: *const sfMusic) -> Time;
	pub fn sfMusic_setPitch(music: *mut sfMusic, pitch: c_float) -> ();
	pub fn sfMusic_setVolume(music: *mut sfMusic, volume: c_float) -> ();
	pub fn sfMusic_setPosition(music: *mut sfMusic, position: Vector3f) -> ();
	pub fn sfMusic_setRelativeToListener(music: *mut sfMusic, relative: SfBool) -> ();
	pub fn sfMusic_setMinDistance(music: *mut sfMusic, distance: c_float) -> ();
	pub fn sfMusic_setAttenuation(music: *mut sfMusic, attenuation: c_float) -> ();
	pub fn sfMusic_setPlayingOffset(music: *mut sfMusic, timeOffset: Time) -> ();
	pub fn sfMusic_getPitch(music: *const sfMusic) -> c_float;
	pub fn sfMusic_getVolume(music: *const sfMusic) -> c_float;
	pub fn sfMusic_getPosition(music: *const sfMusic) -> Vector3f;
	pub fn sfMusic_isRelativeToListener(music: *const sfMusic) -> SfBool;
	pub fn sfMusic_getMinDistance(music: *const sfMusic) -> c_float;
	pub fn sfMusic_getAttenuation(music: *const sfMusic) -> c_float;

	pub fn sfSound_create() -> *mut sfSound;
	pub fn sfSound_copy(sound: *const sfSound) -> *mut sfSound;
	pub fn sfSound_destroy(sound: *mut sfSound) -> ();
	pub fn sfSound_play(sound: *mut sfSound) -> ();
	pub fn sfSound_pause(sound: *mut sfSound) -> ();
	pub fn sfSound_stop(sound: *mut sfSound) -> ();
	pub fn sfSound_setBuffer(sound: *mut sfSound, buffer: *const sfSoundBuffer) -> ();
	//pub fn sfSound_getBuffer(sound: *const sfSound) -> *const sfSoundBuffer;
	pub fn sfSound_setLoop(sound: *mut sfSound, lloop: SfBool) -> ();
	pub fn sfSound_getLoop(sound: *const sfSound) -> SfBool;
	pub fn sfSound_getStatus(sound: *const sfSound) -> SoundStatus;
	pub fn sfSound_setPitch(sound: *mut sfSound, pitch: c_float) -> ();
	pub fn sfSound_setVolume(sound: *mut sfSound, volume: c_float) -> ();
	pub fn sfSound_setPosition(sound: *mut sfSound, position: Vector3f) -> ();
	pub fn sfSound_setRelativeToListener(sound: *mut sfSound, relative: SfBool) -> ();
	pub fn sfSound_setMinDistance(sound: *mut sfSound, distance: c_float) -> ();
	pub fn sfSound_setAttenuation(sound: *mut sfSound, attenuation: c_float) -> ();
	pub fn sfSound_setPlayingOffset(sound: *mut sfSound, timeOffset: Time) -> ();
	pub fn sfSound_getPitch(sound: *const sfSound) -> c_float;
	pub fn sfSound_getVolume(sound: *const sfSound) -> c_float;
	pub fn sfSound_getPosition(sound: *const sfSound) -> Vector3f;
	pub fn sfSound_isRelativeToListener(sound: *const sfSound) -> SfBool;
	pub fn sfSound_getMinDistance(sound: *const sfSound) -> c_float;
	pub fn sfSound_getAttenuation(sound: *const sfSound) -> c_float;
	pub fn sfSound_getPlayingOffset(sound: *const sfSound) -> Time;

	pub fn sfSoundBuffer_createFromFile(filename: *const c_char) -> *mut sfSoundBuffer;
	pub fn sfSoundBuffer_createFromMemory(data: *const c_uchar, sizeInBytes: size_t) -> *mut sfSoundBuffer;
	//  vv sfSoundBuffer_createFromStream(stream: *mut InputStream) -> *mut sfSoundBuffer;
	pub fn sfSoundBuffer_createFromSamples(samples: *const i16, sampleCount: size_t, channelCount: c_uint, sampleRate: c_uint) -> *mut sfSoundBuffer;
	pub fn sfSoundBuffer_copy(soundBuffer: *const sfSoundBuffer) -> *mut sfSoundBuffer;
	pub fn sfSoundBuffer_destroy(soundBuffer: *mut sfSoundBuffer) -> ();
	pub fn sfSoundBuffer_saveToFile(soundBuffer: *const sfSoundBuffer, filename: *const c_char) -> SfBool;
	pub fn sfSoundBuffer_getSamples(soundBuffer: *const sfSoundBuffer) -> *const i16;
	pub fn sfSoundBuffer_getSampleCount(soundBuffer: *const sfSoundBuffer) -> size_t;
	pub fn sfSoundBuffer_getChannelCount(soundBuffer: *const sfSoundBuffer) -> c_uint;
	pub fn sfSoundBuffer_getDuration(soundBuffer: *const sfSoundBuffer) -> Time;
	pub fn sfSoundBuffer_getSampleRate(soundBuffer: *const sfSoundBuffer) -> c_uint;

	pub fn sfSoundBufferRecorder_create() -> *mut sfSoundBufferRecorder;
	pub fn sfSoundBufferRecorder_destroy(soundBufferRecorder: *mut sfSoundBufferRecorder) -> ();
	pub fn sfSoundBufferRecorder_start(soundBufferRecorder: *mut sfSoundBufferRecorder, sampleRate: c_uint) -> SfBool;
	pub fn sfSoundBufferRecorder_stop(soundBufferRecorder: *mut sfSoundBufferRecorder) -> ();
	pub fn sfSoundBufferRecorder_getSampleRate(soundBufferRecorder: *const sfSoundBufferRecorder) -> c_uint;
	pub fn sfSoundBufferRecorder_getBuffer(soundBufferRecorder: *const sfSoundBufferRecorder) -> *const sfSoundBuffer;

	pub fn sfSoundRecorder_create(onStart: sfSoundRecorderStartCallback, onProcess: sfSoundRecorderProcessCallback, onStop: sfSoundRecorderStopCallback, user: *mut c_void) -> *mut sfSoundRecorder;
	pub fn sfSoundRecorder_destroy(soundBufferRecorder: *mut sfSoundRecorder) -> ();
	pub fn sfSoundRecorder_start(soundBufferRecorder: *mut sfSoundRecorder, sampleRate: c_uint) -> SfBool;
	pub fn sfSoundRecorder_stop(soundBufferRecorder: *mut sfSoundRecorder) -> ();
	pub fn sfSoundRecorder_getSampleRate(soundBufferRecorder: *const sfSoundRecorder) -> c_uint;

	pub fn sfSoundRecorder_isAvailable() -> SfBool;
	pub fn sfSoundRecorder_setProcessingInterval(soundBufferRecorder: *mut sfSoundRecorder, interval: Time) -> ();
	pub fn sfSoundRecorder_getAvailableDevices(count: *mut size_t) -> *const *const c_char;
	pub fn sfSoundRecorder_getDefaultDevice() -> *const c_char;
	pub fn sfSoundRecorder_setDevice(soundBufferRecorder: *mut sfSoundRecorder, name: *const c_char) -> SfBool;
	pub fn sfSoundRecorder_getDevice(soundBufferRecorder: *const sfSoundRecorder) -> *const c_char;

	pub fn sfSoundStream_create(onGetData: sfSoundStreamGetDataCallback, onSeek: sfSoundStreamSeekCallback, channelCount: c_uint, sampleRate: c_uint, userData: *mut c_void) -> *mut sfSoundStream;
	pub fn sfSoundStream_destroy(music: *mut sfSoundStream) -> ();
	pub fn sfSoundStream_play(music: *mut sfSoundStream) -> ();
	pub fn sfSoundStream_pause(music: *mut sfSoundStream) -> ();
	pub fn sfSoundStream_stop(music: *mut sfSoundStream) -> ();
	pub fn sfSoundStream_getStatus(music: *const sfSoundStream) -> SoundStatus;
	pub fn sfSoundStream_getChannelCount(music: *const sfSoundStream) -> c_uint;
	pub fn sfSoundStream_getSampleRate(music: *const sfSoundStream) -> c_uint;
	pub fn sfSoundStream_setPitch(music: *mut sfSoundStream, pitch: c_float) -> ();
	pub fn sfSoundStream_setVolume(music: *mut sfSoundStream, volume: c_float) -> ();
	pub fn sfSoundStream_setPosition(music: *mut sfSoundStream, position: Vector3f) -> ();
	pub fn sfSoundStream_setRelativeToListener(music: *mut sfSoundStream, relative: SfBool) -> ();
	pub fn sfSoundStream_setMinDistance(music: *mut sfSoundStream, distance: c_float) -> ();
	pub fn sfSoundStream_setAttenuation(music: *mut sfSoundStream, attenuation: c_float) -> ();
	pub fn sfSoundStream_setPlayingOffset(music: *mut sfSoundStream, timeOffset: Time) -> ();
	pub fn sfSoundStream_setLoop(music: *mut sfSoundStream, lloop: SfBool) -> ();
	pub fn sfSoundStream_getPitch(music: *const sfSoundStream) -> c_float;
	pub fn sfSoundStream_getVolume(music: *const sfSoundStream) -> c_float;
	pub fn sfSoundStream_getPosition(music: *const sfSoundStream) -> Vector3f;
	pub fn sfSoundStream_isRelativeToListener(music: *const sfSoundStream) -> SfBool;
	pub fn sfSoundStream_getMinDistance(music: *const sfSoundStream) -> c_float;
	pub fn sfSoundStream_getAttenuation(music: *const sfSoundStream) -> c_float;
	pub fn sfSoundStream_getLoop(music: *const sfSoundStream) -> SfBool;
	pub fn sfSoundStream_getPlayingOffset(music: *const sfSoundStream) -> Time;
}

// InputStream isn't properly #[repr(C)] due to containing a PhantomData.
#[allow(improper_ctypes)]
extern "C" {
	pub fn sfMusic_createFromStream(stream: *mut InputStream) -> *mut sfMusic;
	pub fn sfSoundBuffer_createFromStream(stream: *mut InputStream) -> *mut sfSoundBuffer;
}
