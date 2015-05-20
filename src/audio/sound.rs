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

use libc::c_float;

use audio::{SoundStatus, SoundBuffer, SoundSource, PlayableSound};
use system::{Time, Vector3f};

use ffi::{SfBool, Foreign};
use ffi::audio as ffi;

/// Regular sound that can be played in the audio environment.
///
/// `Sound` is the type to use to play sounds. It provides:
///
/// * Control (play, pause, stop),
/// * Ability to modify output parameters in real-time (pitch, volume, ...),
/// * 3D spatial features (position, attenuation, ...).
///
/// `Sound` is perfect for playing short sounds that can fit in memory and
/// require no latency, like foot steps or gun shots. For longer sounds, like
/// background musics or long speeches, instead see `Music` (which is based on
/// streaming).
///
/// In order to work, a sound must be given a buffer of audio data to play.
/// Audio data (samples) is stored in `SoundBuffer`, and attached to a sound
/// with the `set_buffer()` function.
pub struct Sound<'s> {
    sound: Foreign<ffi::sfSound>,
    buffer: Option<&'s SoundBuffer>
}

impl<'s> Sound<'s> {
    /// Create a new Sound with no buffer specified.
    ///
    /// Return Some(Sound) or None on failure.
    pub fn new() -> Option<Sound<'s>> {
        unsafe {
			Foreign::new(ffi::sfSound_create())
		}.map(|s| Sound {
			sound: s,
			buffer: None
		})
    }

    /// Create a new Sound with the specified buffer.
    ///
    /// Return Some(Sound) or None on failure.
    pub fn new_with_buffer(buffer: &'s SoundBuffer) -> Option<Sound<'s>> {
		Sound::new().map(|mut sound| { sound.set_buffer(buffer); sound })
    }

    /// Create a new sound by copying an existing one.
    ///
    /// Return Some(Sound) or None on failure.
    pub fn clone_opt(&self) -> Option<Sound<'s>> {
        unsafe {
			Foreign::new(ffi::sfSound_copy(self.raw()))
		}.map(|s| Sound {
			sound: s,
			buffer: self.get_buffer()
		})
    }

	fn raw(&self) -> &ffi::sfSound { self.sound.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfSound { self.sound.as_mut() }

    /// Set the source buffer containing the audio data to play.
    pub fn set_buffer(&mut self, buffer: &'s SoundBuffer) {
        self.buffer = Some(buffer);
        unsafe {
            ffi::sfSound_setBuffer(self.raw_mut(), buffer.unwrap())
        }
    }

    /// Get the audio buffer attached to the sound.
    pub fn get_buffer(&self) -> Option<&'s SoundBuffer> {
        self.buffer
    }

    /// Get the total duration of the sound.
    pub fn get_duration(&self) -> Time {
		if let Some(buffer) = self.buffer {
			buffer.get_duration()
		} else {
			Time::with_milliseconds(0)
		}
    }
}

impl<'a> SoundSource for Sound<'a> {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::sfSound_setPitch(self.raw_mut(), pitch as c_float) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::sfSound_setVolume(self.raw_mut(), volume as c_float) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::sfSound_setRelativeToListener(self.raw_mut(), SfBool::from_bool(relative)) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::sfSound_setMinDistance(self.raw_mut(), distance as c_float) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::sfSound_setAttenuation(self.raw_mut(), attenuation as c_float) }
    }
    fn get_position(&self) -> Vector3f {
        unsafe { ffi::sfSound_getPosition(self.raw()) }
    }
    fn set_position(&mut self, position: Vector3f) {
        unsafe { ffi::sfSound_setPosition(self.raw_mut(), position) }
    }
    fn get_pitch(&self) -> f32 {
        unsafe { ffi::sfSound_getPitch(self.raw()) as f32 }
    }
    fn get_volume(&self) -> f32 {
        unsafe { ffi::sfSound_getVolume(self.raw()) as f32 }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfSound_isRelativeToListener(self.raw()) }.to_bool()
    }
    fn get_min_distance(&self) -> f32 {
        unsafe { ffi::sfSound_getMinDistance(self.raw()) as f32 }
    }
    fn get_attenuation(&self) -> f32 {
        unsafe { ffi::sfSound_getAttenuation(self.raw()) as f32 }
    }
}

impl<'a> PlayableSound for Sound<'a> {
    fn set_loop(&mut self, lloop: bool) {
        unsafe { ffi::sfSound_setLoop(self.raw_mut(), SfBool::from_bool(lloop)) }
    }
    fn get_loop(&self) -> bool {
        unsafe { ffi::sfSound_getLoop(self.raw()) }.to_bool()
    }
    fn play(&mut self) {
        unsafe { ffi::sfSound_play(self.raw_mut()) }
    }
    fn pause(&mut self) {
        unsafe { ffi::sfSound_pause(self.raw_mut()) }
    }
    fn stop(&mut self) {
        unsafe { ffi::sfSound_stop(self.raw_mut()) }
    }
    fn get_status(&self) -> SoundStatus {
        unsafe { ffi::sfSound_getStatus(self.raw()) }
    }
    fn get_playing_offset(&self) -> Time {
        unsafe { ffi::sfSound_getPlayingOffset(self.raw()) }
    }
    fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::sfSound_setPlayingOffset(self.raw_mut(), time_offset) }
    }
	fn get_channel_count(&self) -> u32 {
		if let Some(buffer) = self.buffer {
			buffer.get_channel_count()
		} else {
			0
		}
	}
	fn get_sample_rate(&self) -> u32 {
		if let Some(buffer) = self.buffer {
			buffer.get_sample_rate()
		} else {
			0
		}
	}
}

impl<'a> Clone for Sound<'a> {
	fn clone(&self) -> Sound<'a> {
		self.clone_opt().expect("Failed to clone Sound")
	}
}
