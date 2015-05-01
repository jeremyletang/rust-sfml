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

use libc::{c_uint, c_float, c_char, size_t, c_uchar};
use system::{Vector3f, Time};
use audio::Status;
use ffi::SfBool;

foreign_type! {
	sfMusic, sfMusic_destroy;
	sfSound, sfSound_destroy;
	sfSoundBuffer, sfSoundBuffer_destroy;
	sfSoundBufferRecorder, sfSoundBufferRecorder_destroy;
}

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
	// sfMusic*mut  sfMusic_createFromStream(sfInputStream*mut  stream);
	pub fn sfMusic_destroy(music: *mut sfMusic) -> ();
	pub fn sfMusic_setLoop(music: *mut sfMusic, lloop: SfBool) -> ();
	pub fn sfMusic_getLoop(music: *const sfMusic) -> SfBool;
	pub fn sfMusic_getDuration(music: *const sfMusic) -> Time;
	pub fn sfMusic_play(music: *mut sfMusic) -> ();
	pub fn sfMusic_pause(music: *mut sfMusic) -> ();
	pub fn sfMusic_stop(music: *mut sfMusic) -> ();
	pub fn sfMusic_getChannelCount(music: *const sfMusic) -> c_uint;
	pub fn sfMusic_getSampleRate(music: *const sfMusic) -> c_uint;
	pub fn sfMusic_getStatus(music: *const sfMusic) -> Status;
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
	pub fn sfSound_getBuffer(sound: *const sfSound) -> *const sfSoundBuffer;
	pub fn sfSound_setLoop(sound: *mut sfSound, lloop: SfBool) -> ();
	pub fn sfSound_getLoop(sound: *const sfSound) -> SfBool;
	pub fn sfSound_getStatus(sound: *const sfSound) -> Status;
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
	pub fn sfSoundBuffer_copy(soundBuffer: *const sfSoundBuffer) -> *mut sfSoundBuffer;
	pub fn sfSoundBuffer_destroy(soundBuffer: *mut sfSoundBuffer) -> ();
	pub fn sfSoundBuffer_saveToFile(soundBuffer: *const sfSoundBuffer, filename: *const c_char) -> SfBool;
	//pub fn sfSoundBuffer_getSamples(soundBuffer: *const sfSoundBuffer) -> *const i16;
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
	pub fn sfSoundRecorder_isAvailable() -> SfBool;
	pub fn sfSoundRecorder_setProcessingInterval(soundBufferRecorder: *mut sfSoundBufferRecorder, interval: Time) -> ();
	pub fn sfSoundRecorder_getAvailableDevices(count: *mut size_t) -> *const *const c_char;
	pub fn sfSoundRecorder_getDefaultDevice() -> *const c_char;
	pub fn sfSoundRecorder_setDevice(soundBufferRecorder: *mut sfSoundBufferRecorder, name: *const c_char) -> SfBool;
	pub fn sfSoundRecorder_getDevice(soundBufferRecorder: *const sfSoundBufferRecorder) -> *const c_char;
}
