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

use libc::{c_float, size_t};
use std::ffi::CString;
use std::marker::PhantomData;

use audio::{SoundStatus, SoundSource, PlayableSound};
use system::{Time, Vector3f, InputStream};

use ffi::{SfBool, Foreign};
use ffi::audio as ffi;

/// Streamed music played from an audio file.
///
/// Musics are sounds that are streamed rather than completely loaded in memory.
/// This is especially useful for compressed music that usually takes hundreds
/// of MB when it is uncompressed: by streaming it instead of loading it
/// entirely, you avoid saturating the memory and have almost no loading delay.
///
/// Apart from that, `Music` has almost the same features as the `SoundBuffer` /
/// `Sound` pair: you can play/pause/stop it, request its parameters (channels,
/// sample rate), change the way it is played (pitch, volume, 3D position, ...),
/// etc.
///
/// As a sound stream, a music is played in its own thread in order not to block
/// the rest of the program. This means that you can leave the music alone after
/// calling `play()` and it will manage itself very well.
///
/// The supported formats are: ogg, wav, flac, aiff, au, raw, paf, svx, nist,
/// voc, ircam, w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
pub struct Music<'a> {
	ptr: Foreign<ffi::sfMusic>,
	phantom: PhantomData<&'a [u8]>
}

impl<'a> Music<'a> {
    /// Create a new music and stream it from a file.
    ///
    /// This function doesn't start playing the music (call
    /// `play()` to do so).
    ///
    /// Returns Some(Music) or None.
    pub fn new_from_file(filename: &str) -> Option<Music<'a>> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfMusic_createFromFile(c_str.as_ptr()))
        }.map(|ptr| Music {
			ptr: ptr,
			phantom: PhantomData
		})
    }

    /// Create a new music and stream it from memory.
    ///
    /// This function doesn't start playing the music (call
    /// `play()` to do so).
	///
	/// Returns Some(Music) or None.
    pub fn new_from_memory(mem: &'a [u8]) -> Option<Music<'a>> {
        unsafe {
			Foreign::new(ffi::sfMusic_createFromMemory(mem.as_ptr(), mem.len() as size_t))
		}.map(|ptr| Music {
			ptr: ptr,
			phantom: PhantomData
		})
    }

	/// Create a new music and stream it from the given source.
	///
	/// Since the music is not loaded completely but rather streamed
	/// continuously, the `stream` must stay alive as long as the music is
	/// playing. See the docs for `InputStream` for more information.
	///
	/// Returns Some(Music) or None.
	pub fn new_from_stream(stream: &'a mut InputStream<'a>) -> Option<Music<'a>> {
		unsafe {
			Foreign::new(ffi::sfMusic_createFromStream(stream))
		}.map(|ptr| Music {
			ptr: ptr,
			phantom: PhantomData
		})
	}

	fn raw(&self) -> &ffi::sfMusic { self.ptr.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfMusic { self.ptr.as_mut() }

    /// Get the total duration of the music.
    pub fn get_duration(&self) -> Time {
        unsafe { ffi::sfMusic_getDuration(self.raw()) }
    }
}

impl<'a> SoundSource for Music<'a> {
    fn get_pitch(&self) -> f32 {
        unsafe { ffi::sfMusic_getPitch(self.raw()) as f32 }
    }
    fn get_volume(&self) -> f32 {
        unsafe { ffi::sfMusic_getVolume(self.raw()) as f32 }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfMusic_isRelativeToListener(self.raw()) }.to_bool()
    }
    fn get_min_distance(&self) -> f32 {
        unsafe { ffi::sfMusic_getMinDistance(self.raw()) as f32 }
    }
    fn get_attenuation(&self) -> f32 {
        unsafe { ffi::sfMusic_getAttenuation(self.raw()) as f32 }
    }
    fn set_position(&mut self, position: &Vector3f) {
        unsafe { ffi::sfMusic_setPosition(self.raw_mut(), *position) }
    }
    fn get_position(&self) -> Vector3f {
        unsafe { ffi::sfMusic_getPosition(self.raw()) }
    }
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::sfMusic_setPitch(self.raw_mut(), pitch as c_float) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::sfMusic_setVolume(self.raw_mut(), volume as c_float) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::sfMusic_setRelativeToListener(self.raw_mut(), SfBool::from_bool(relative)) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::sfMusic_setMinDistance(self.raw_mut(), distance as c_float) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::sfMusic_setAttenuation(self.raw_mut(), attenuation as c_float) }
    }
}

impl<'a> PlayableSound for Music<'a> {
    fn get_status(&self) -> SoundStatus {
        unsafe { ffi::sfMusic_getStatus(self.raw()) }
    }
    fn get_playing_offset(&self) -> Time {
        unsafe { ffi::sfMusic_getPlayingOffset(self.raw()) }
    }
    fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::sfMusic_setPlayingOffset(self.raw_mut(), time_offset) }
    }
    fn set_loop(&mut self, lloop: bool) {
        unsafe { ffi::sfMusic_setLoop(self.raw_mut(), SfBool::from_bool(lloop)) }
    }
    fn get_loop(&self) -> bool {
        unsafe { ffi::sfMusic_getLoop(self.raw()) }.to_bool()
    }
    fn play(&mut self) {
        unsafe { ffi::sfMusic_play(self.raw_mut()) }
    }
    fn pause(&mut self) {
        unsafe { ffi::sfMusic_pause(self.raw_mut()) }
    }
    fn stop(&mut self) {
        unsafe { ffi::sfMusic_stop(self.raw_mut()) }
    }
    fn get_channel_count(&self) -> u32 {
        unsafe { ffi::sfMusic_getChannelCount(self.raw()) as u32 }
    }
    fn get_sample_rate(&self) -> u32 {
        unsafe { ffi::sfMusic_getSampleRate(self.raw()) as u32 }
    }
}
