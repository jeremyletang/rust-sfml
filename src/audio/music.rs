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
use std::ffi::CString;

use audio::SoundStatus;
use system::{Time, Vector3f};

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
pub struct Music(Foreign<ffi::sfMusic>);

impl Music {
    /// Create a new music and load it from a file.
    ///
    /// This function doesn't start playing the music (call
    /// `play()` to do so).
    ///
    /// Returns Some(Music) or None.
    pub fn new_from_file(filename: &str) -> Option<Music> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfMusic_createFromFile(c_str.as_ptr()))
        }.map(Music)
    }

	// TODO: requires Music to either reference or take ownership of the buffer.
	/*
    /// Create a new music and load it from memory.
    ///
    /// This function doesn't start playing the music (call
    /// `play()` to do so).
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    ///
    /// Return Some(Music) or None.
    pub fn new_from_memory(mem: &[u8]) -> Option<Music> {
        unsafe {
			Foreign::new(ffi::sfMusic_createFromMemory(mem.as_ptr(), mem.len() as size_t))
		}.map(Music)
    }*/

	fn raw(&self) -> &ffi::sfMusic { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfMusic { self.0.as_mut() }

    /// Set whether or not the music should loop after reaching the end.
    ///
    /// If set, the music will restart from beginning after
    /// reaching the end and so on, until it is stopped or
    /// `set_loop(false)` is called. The default looping state is false.
    pub fn set_loop(&mut self, lloop: bool) {
        unsafe { ffi::sfMusic_setLoop(self.raw_mut(), SfBool::from_bool(lloop)) }
    }

    /// Tell whether or not the music is in loop mode.
    pub fn get_loop(&self) -> bool {
        unsafe { ffi::sfMusic_getLoop(self.raw()) }.to_bool()
    }

    /// Get the total duration of the music.
    pub fn get_duration(&self) -> Time {
        unsafe { ffi::sfMusic_getDuration(self.raw()) }
    }

    /// Start or resume playing the music.
    ///
    /// This function starts the music if it was stopped, resumes
    /// it if it was paused, and restarts it from the beginning if it
    /// was already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the music is played.
    pub fn play(&mut self) {
        unsafe {
            ffi::sfMusic_play(self.raw_mut())
        }
    }

    /// Pause the music.
    ///
    /// This function pauses the music if it was playing,
    /// otherwise (music already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe {
            ffi::sfMusic_pause(self.raw_mut())
        }
    }

    /// Stop playing the music.
    ///
    /// This function stops the music if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe {
            ffi::sfMusic_stop(self.raw_mut())
        }
    }

    /// Return the number of channels of the music.
    ///
    /// 1 channel means a mono sound, 2 means stereo, etc.
    pub fn get_channel_count(&self) -> u32 {
        unsafe {
            ffi::sfMusic_getChannelCount(self.raw()) as u32
        }
    }

    /// Get the sample rate of the music.
    ///
    /// The sample rate is the number of audio samples played per
    /// second. The higher, the better the quality.
    pub fn get_sample_rate(&self) -> u32 {
        unsafe {
            ffi::sfMusic_getSampleRate(self.raw()) as u32
        }
    }

    /// Get the current status of the music (stopped, paused, playing).
    pub fn get_status(&self) -> SoundStatus {
        unsafe { ffi::sfMusic_getStatus(self.raw()) }
    }

    /// Get the current playing position of the music.
    pub fn get_playing_offset(&self) -> Time {
        unsafe { ffi::sfMusic_getPlayingOffset(self.raw()) }
    }

    /// Set the pitch of the music.
    ///
    /// The pitch represents the perceived fundamental frequency
    /// of a sound; thus you can make a music more acute or grave
    /// by changing its pitch. A side effect of changing the pitch
    /// is to modify the playing speed of the music as well.
    /// The default value for the pitch is 1.
    pub fn set_pitch(&mut self, pitch: f32) {
        unsafe {
            ffi::sfMusic_setPitch(self.raw_mut(), pitch as c_float)
        }
    }

    /// Set the volume of the music.
    ///
    /// The volume is a value between 0 (mute) and 100 (full volume).
    /// The default value for the volume is 100.
    pub fn set_volume(&mut self, volume: f32) {
        unsafe {
            ffi::sfMusic_setVolume(self.raw_mut(), volume as c_float)
        }
    }

    /// Make the musics's position relative to the listener or absolute.
    ///
    /// Making a music relative to the listener will ensure that it will always
    /// be played the same way regardless of the position of the listener.
    /// This can be useful for non-spatialized musics, musics that are
    /// produced by the listener, or musics attached to it.
    /// The default value is false (position is absolute).
    pub fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::sfMusic_setRelativeToListener(self.raw_mut(), SfBool::from_bool(relative)) }
    }

    /// Set the minimum distance of the music.
    ///
    /// The "minimum distance" of a music is the maximum
    /// distance at which it is heard at its maximum volume. Further
    /// than the minimum distance, it will start to fade out according
    /// to its attenuation factor. A value of 0 ("inside the head
    /// of the listener") is an invalid value and is forbidden.
    /// The default value of the minimum distance is 1.
    pub fn set_min_distance(&mut self, distance: f32) {
        unsafe {
            ffi::sfMusic_setMinDistance(self.raw_mut(), distance as c_float)
        }
    }

    /// Set the attenuation factor of the music.
    ///
    /// The attenuation is a multiplicative factor which makes
    /// the music more or less loud according to its distance
    /// from the listener. An attenuation of 0 will produce a
    /// non-attenuated music, i.e. its volume will always be the same
    /// whether it is heard from near or from far. On the other hand,
    /// an attenuation value such as 100 will make the music fade out
    /// very quickly as it gets further from the listener.
    /// The default value of the attenuation is 1.
    pub fn set_attenuation(&mut self, attenuation: f32) {
        unsafe {
            ffi::sfMusic_setAttenuation(self.raw_mut(), attenuation as c_float)
        }
    }

    /// Change the current playing position of the music.
    ///
    /// The playing position can be changed when the music is either paused
    /// or playing. Changing the playing position when the stream is stopped
	/// has no effect, since playing the stream would reset its position.
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe {
            ffi::sfMusic_setPlayingOffset(self.raw_mut(), time_offset)
        }
    }

    /// Get the pitch of the music.
    pub fn get_pitch(&self) -> f32 {
        unsafe {
            ffi::sfMusic_getPitch(self.raw()) as f32
        }
    }

    /// Get the volume of the music, in the range [0, 100].
    pub fn get_volume(&self) -> f32 {
        unsafe {
            ffi::sfMusic_getVolume(self.raw()) as f32
        }
    }

    /// Tell whether the music's position is relative to the listener or absolute.
    ///
    /// Returns true if the position is relative, false if it's absolute.
    pub fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfMusic_isRelativeToListener(self.raw()) }.to_bool()
    }

    /// Get the minimum distance of the music.
    pub fn get_min_distance(&self) -> f32 {
        unsafe {
           ffi::sfMusic_getMinDistance(self.raw()) as f32
       }
    }

    /// Get the attenuation factor of the music.
    pub fn get_attenuation(&self) -> f32 {
        unsafe {
            ffi::sfMusic_getAttenuation(self.raw()) as f32
        }
    }

    /// Set the 3D position of the music in the audio scene.
    ///
    /// Only musics with one channel (mono musics) can be
    /// spatialized.
    /// The default position of a music is (0, 0, 0).
    pub fn set_position(&mut self, position: &Vector3f) {
        unsafe {
            ffi::sfMusic_setPosition(self.raw_mut(), *position)
        }
    }

    /// Set the 3D position of the music in the audio scene.
    ///
    /// Only musics with one channel (mono musics) can be
    /// spatialized.
    /// The default position of a music is (0, 0, 0).
    pub fn set_position3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::sfMusic_setPosition(self.raw_mut(), Vector3f::new(x, y, z))
        }
    }

    /// Get the 3D position of the music in the audio scene.
    pub fn get_position(&self) -> Vector3f {
        unsafe {
            ffi::sfMusic_getPosition(self.raw())
        }
    }
}
