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

use audio::{SoundStatus, SoundBuffer};
use system::{Time, Vector3f};

use ffi::{SfBool, Foreign};
use ffi::audio as ffi;

//pub mod rc;

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

    /// Set whether or not the sound should loop after reaching the end.
    ///
    /// If set, the sound will restart from beginning after
    /// reaching the end and so on, until it is stopped or
    /// `set_loop(false)` is called. The default looping state is false.
    pub fn set_loop(&mut self, lloop: bool) -> () {
        unsafe { ffi::sfSound_setLoop(self.raw_mut(), SfBool::from_bool(lloop)) }
    }

    /// Tell whether or not the sound is in loop mode.
    pub fn get_loop(&self) -> bool {
        unsafe { ffi::sfSound_getLoop(self.raw()) }.to_bool()
    }

    /// Start or resume playing the sound.
    ///
    /// This function starts the sound if it was stopped, resumes
    /// it if it was paused, and restarts it from the beginning if it
    /// was already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the sound is played.
    pub fn play(&mut self) -> () {
        unsafe {ffi::sfSound_play(self.raw_mut())}
    }

    /// Pause the sound.
    ///
    /// This function pauses the sound if it was playing,
    /// otherwise (sound already paused or stopped) it has no effect.
    pub fn pause(&mut self) -> () {
        unsafe {ffi::sfSound_pause(self.raw_mut())}
    }

    /// Stop playing the sound.
    ///
    /// This function stops the sound if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) -> () {
        unsafe {ffi::sfSound_stop(self.raw_mut())}
    }

    /// Get the current status of the sound (stopped, paused, playing).
    pub fn get_status(&self) -> SoundStatus {
        unsafe { ffi::sfSound_getStatus(self.raw()) }
    }

    /// Get the current playing position of the sound.
    pub fn get_playing_offset(&self) -> Time {
        unsafe {ffi::sfSound_getPlayingOffset(self.raw())}
    }

    /// Set the pitch of the sound.
    ///
    /// The pitch represents the perceived fundamental frequency
    /// of a sound; thus you can make a sound more acute or grave
    /// by changing its pitch. A side effect of changing the pitch
    /// is to modify the playing speed of the sound as well.
    /// The default value for the pitch is 1.
    pub fn set_pitch(&mut self, pitch: f32) -> () {
        unsafe {ffi::sfSound_setPitch(self.raw_mut(), pitch as c_float)}
    }

    /// Set the volume of the sound.
    ///
    /// The volume is a value between 0 (mute) and 100 (full volume).
    /// The default value for the volume is 100.
    pub fn set_volume(&mut self, volume: f32) -> () {
        unsafe {ffi::sfSound_setVolume(self.raw_mut(), volume as c_float)}
    }

    /// Make the sounds's position relative to the listener or absolute.
    ///
    /// Making a sound relative to the listener will ensure that it will always
    /// be played the same way regardless of the position of the listener.
    /// This can be useful for non-spatialized sounds, sounds that are
    /// produced by the listener, or sounds attached to it.
    /// The default value is false (position is absolute).
    pub fn set_relative_to_listener(&mut self, relative: bool) -> () {
        unsafe { ffi::sfSound_setRelativeToListener(self.raw_mut(), SfBool::from_bool(relative)) }
    }

    /// Set the minimum distance of the sound.
    ///
    /// The "minimum distance" of a sound is the maximum
    /// distance at which it is heard at its maximum volume. Further
    /// than the minimum distance, it will start to fade out according
    /// to its attenuation factor. A value of 0 ("inside the head
    /// of the listener") is an invalid value and is forbidden.
    /// The default value of the minimum distance is 1.
    pub fn set_min_distance(&mut self, distance: f32) -> () {
        unsafe {ffi::sfSound_setMinDistance(self.raw_mut(), distance as c_float)}
    }

    /// Set the attenuation factor of the sound.
    ///
    /// The attenuation is a multiplicative factor which makes
    /// the sound more or less loud according to its distance
    /// from the listener. An attenuation of 0 will produce a
    /// non-attenuated sound, i.e. its volume will always be the same
    /// whether it is heard from near or from far. On the other hand,
    /// an attenuation value such as 100 will make the sound fade out
    /// very quickly as it gets further from the listener.
    /// The default value of the attenuation is 1.
    pub fn set_attenuation(&mut self, attenuation: f32) -> () {
        unsafe {ffi::sfSound_setAttenuation(self.raw_mut(), attenuation as c_float)}
    }

    /// Change the current playing position of the sound.
    ///
    /// The playing position can be changed when the sound is either paused
    /// or playing. Changing the playing position when the stream is stopped
	/// has no effect, since playing the stream would reset its position.
    pub fn set_playing_offset(&mut self, time_offset: Time) -> () {
        unsafe {
            ffi::sfSound_setPlayingOffset(self.raw_mut(), time_offset)
        }
    }

    /// Get the pitch of the sound.
    pub fn get_pitch(&self) -> f32 {
        unsafe {
            ffi::sfSound_getPitch(self.raw()) as f32
        }
    }

    /// Get the volume of the sound, in the range [0, 100].
    pub fn get_volume(&self) -> f32 {
        unsafe {
            ffi::sfSound_getVolume(self.raw()) as f32
        }
    }

    /// Tell whether the sound's position is relative to the listener or absolute.
    ///
    /// Returns true if the position is relative, false if it's absolute.
    pub fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfSound_isRelativeToListener(self.raw()) }.to_bool()
    }

    /// Get the minimum distance of the sound.
    pub fn get_min_distance(&self) -> f32 {
        unsafe {
           ffi::sfSound_getMinDistance(self.raw()) as f32
        }
    }

    /// Get the attenuation factor of the sound.
    pub fn get_attenuation(&self) -> f32 {
        unsafe {
            ffi::sfSound_getAttenuation(self.raw()) as f32
        }
    }

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

    /// Get the 3D position of the sound in the audio scene.
    pub fn get_position(&self) -> Vector3f {
        unsafe {
            ffi::sfSound_getPosition(self.raw())
        }
    }

    /// Set the 3D position of the sound in the audio scene.
    ///
    /// Only sounds with one channel (mono sounds) can be
    /// spatialized.
    /// The default position of a sound is (0, 0, 0).
    pub fn set_position(&mut self, position: &Vector3f) {
        unsafe {
            ffi::sfSound_setPosition(self.raw_mut(), *position)
        }
    }

    /// Set the 3D position of the sound in the audio scene.
    ///
    /// Only sounds with one channel (mono sounds) can be
    /// spatialized.
    /// The default position of a sound is (0, 0, 0).
    pub fn set_position3f(&mut self, x: f32, y: f32, z: f32) -> () {
		self.set_position(&Vector3f::new(x, y, z))
    }
}

impl<'a> Clone for Sound<'a> {
	fn clone(&self) -> Sound<'a> {
		self.clone_opt().expect("Failed to clone Sound")
	}
}
