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

//! Play Music
//!
//! Streamed music played from an audio file.
//! Musics are sounds that are streamed rather than completely loaded in memory.

use libc::{c_float, size_t};
use std::mem;
use std::ffi::CString;
use std::io::{Read, Seek};

use audio::{SoundStatus, SoundSource};
use system::Time;
use system::Vector3f;
use inputstream::InputStream;
use raw_conv::{Raw, FromRaw};

use csfml_system_sys::{sfBool, sfVector3f};
use csfml_audio_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Play Music
///
/// Streamed music played from an audio file.
/// Musics are sounds that are streamed rather than completely loaded in memory.
pub struct Music {
    music: *mut ffi::sfMusic
}

impl Music {
    /// Create a new music and load it from a file
    ///
    /// This function doesn't start playing the music (call
    /// sfMusic_play to do so).
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * filename - Path of the music file to open
    ///
    /// Return Some(Music) or None
    pub fn from_file(filename: &str) -> Option<Music> {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        let music_tmp: *mut ffi::sfMusic = unsafe {
            ffi::sfMusic_createFromFile(c_str.as_ptr())
        };
        if music_tmp.is_null() {
            None
        } else {
            Some(Music{
                    music: music_tmp
                })
        }
    }

    /// Create a new music and load it from a stream (a struct implementing Read and Seek)
    ///
    /// This function doesn't start playing the music (call
    /// sfMusic_play to do so).
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * stream - Your struct, implementing Read and Seek
    ///
    /// Return Some(Music) or None
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> Option<Music> {
        let mut input_stream = InputStream::new(stream);
        let music_tmp: *mut ffi::sfMusic = unsafe {
            ffi::sfMusic_createFromStream(&mut input_stream.0)
        };
        if music_tmp.is_null() {
            None
        } else {
            Some(Music{
                    music: music_tmp
                })
        }
    }

    /// Create a new music and load it from memory
    ///
    /// This function doesn't start playing the music (call
    /// sfMusic_play to do so).
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    ///
    /// Return Some(Music) or None
    pub fn from_memory(mem: &[u8]) -> Option<Music> {
        let music_tmp = unsafe { ffi::sfMusic_createFromMemory(mem.as_ptr() as *const _, mem.len() as size_t) };
        if music_tmp.is_null() {
            None
        } else {
            Some(Music{
                    music: music_tmp
                })
        }
    }

    /// Set whether or not a music should loop after reaching the end
    ///
    /// If set, the music will restart from beginning after
    /// reaching the end and so on, until it is stopped or
    /// sfMusic_setLoop(music, SFFALSE) is called.
    /// The default looping state for musics is false.
    ///
    /// # Arguments
    /// * loop - SFTRUE to play in loop, SFFALSE to play once
    pub fn set_loop(&mut self, loop_: bool) {
        unsafe { ffi::sfMusic_setLoop(self.music, sfBool::from_bool(loop_)) }
    }

    /// Tell whether or not a music is in loop mode
    ///
    /// Return true if the music is looping, false otherwise
    pub fn get_loop(&self) -> bool {
        unsafe { ffi::sfMusic_getLoop(self.music) }.to_bool()
    }

    /// Get the total duration of a music
    ///
    /// Return Music duration
    pub fn get_duration(&self) -> Time {
        Time::from_raw( unsafe { ffi::sfMusic_getDuration(self.music) })
    }

    /// Start or resume playing a music
    ///
    /// This function starts the music if it was stopped, resumes
    /// it if it was paused, and restarts it from beginning if it
    /// was it already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the music is played.
    pub fn play(&mut self) {
        unsafe {
            ffi::sfMusic_play(self.music)
        }
    }

    /// Pause a music
    ///
    /// This function pauses the music if it was playing,
    /// otherwise (music already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe {
            ffi::sfMusic_pause(self.music)
        }
    }

    /// Stop playing a music
    ///
    /// This function stops the music if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe {
            ffi::sfMusic_stop(self.music)
        }
    }

    /// Return the number of channels of a music
    ///
    /// 1 channel means a mono sound, 2 means stereo, etc.
    ///
    /// Return the number of channels
    pub fn get_channel_count(&self) -> u32 {
        unsafe {
            ffi::sfMusic_getChannelCount(self.music) as u32
        }
    }

    /// Get the sample rate of a music
    ///
    /// The sample rate is the number of audio samples played per
    /// second. The higher, the better the quality.
    ///
    /// Return the sample rate, in number of samples per second
    pub fn get_sample_rate(&self) -> u32 {
        unsafe {
            ffi::sfMusic_getSampleRate(self.music) as u32
        }
    }

    /// Get the current status of a music (stopped, paused, playing)
    ///
    /// Return current status
    pub fn get_status(&self) -> SoundStatus {
        unsafe { mem::transmute(ffi::sfMusic_getStatus(self.music))}
    }

    /// Get the current playing position of a music
    ///
    /// Return the current playing position
    pub fn get_playing_offset(&self) -> Time {
        Time::from_raw(unsafe { ffi::sfMusic_getPlayingOffset(self.music) })
    }

    /// Change the current playing position of a music
    ///
    /// The playing position can be changed when the music is
    /// either paused or playing.
    ///
    /// # Arguments
    /// * timeOffset - New playing position
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe {
            ffi::sfMusic_setPlayingOffset(self.music, time_offset.raw())
        }
    }
}

impl SoundSource for Music {
    /// Set the pitch of a music
    ///
    /// The pitch represents the perceived fundamental frequency
    /// of a sound; thus you can make a music more acute or grave
    /// by changing its pitch. A side effect of changing the pitch
    /// is to modify the playing speed of the music as well.
    /// The default value for the pitch is 1.
    ///
    /// # Arguments
    /// * pitch - new pitch to apply to the music
    fn set_pitch(&mut self, pitch: f32) {
        unsafe {
            ffi::sfMusic_setPitch(self.music, pitch as c_float)
        }
    }

    /// Set the volume of a music
    ///
    /// he volume is a value between 0 (mute) and 100 (full volume).
    /// The default value for the volume is 100.
    ///
    /// # Arguments
    /// * volume - Volume of the music
    fn set_volume(&mut self, volume: f32) {
        unsafe {
            ffi::sfMusic_setVolume(self.music, volume as c_float)
        }
    }

    /// Set the 3D position of a music in the audio scene
    ///
    /// Only musics with one channel (mono musics) can be
    /// spatialized.
    /// The default position of a music is (0, 0, 0).
    ///
    /// # Arguments
    /// * position - Position of the music in the scene
    fn set_position(&mut self, position: &Vector3f) {
        unsafe {
            ffi::sfMusic_setPosition(self.music, position.raw())
        }
    }

    /// Set the 3D position of a music in the audio scene
    ///
    /// Only musics with one channel (mono musics) can be
    /// spatialized.
    /// The default position of a music is (0, 0, 0).
    ///
    /// # Arguments
    /// * x - X coordinate of the position of the sound in the scene
    /// * y - Y coordinate of the position of the sound in the scene
    /// * z - Z coordinate of the position of the sound in the scene
    fn set_position3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::sfMusic_setPosition(self.music, sfVector3f{x: x, y: y, z: z})
        }
    }

    /// Make a musics's position relative to the listener or absolute
    ///
    /// Making a music relative to the listener will ensure that it will always
    /// be played the same way regardless the position of the listener.
    /// This can be useful for non-spatialized musics, musics that are
    /// produced by the listener, or musics attached to it.
    /// The default value is false (position is absolute).
    ///
    /// # Arguments
    /// * relative - true to set the position relative, false to set it absolute
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe {
            ffi::sfMusic_setRelativeToListener(self.music, sfBool::from_bool(relative))
        }
    }

    /// Set the minimum distance of a music
    ///
    /// The "minimum distance" of a music is the maximum
    /// distance at which it is heard at its maximum volume. Further
    /// than the minimum distance, it will start to fade out according
    /// to its attenuation factor. A value of 0 ("inside the head
    /// of the listener") is an invalid value and is forbidden.
    /// The default value of the minimum distance is 1.
    ///
    /// # Arguments
    /// * distance - New minimum distance of the music
    fn set_min_distance(&mut self, distance: f32) {
        unsafe {
            ffi::sfMusic_setMinDistance(self.music, distance as c_float)
        }
    }

    ///  Set the attenuation factor of a music
    ///
    /// The attenuation is a multiplicative factor which makes
    /// the music more or less loud according to its distance
    /// from the listener. An attenuation of 0 will produce a
    /// non-attenuated music, i.e. its volume will always be the same
    /// whether it is heard from near or from far. On the other hand,
    /// an attenuation value such as 100 will make the music fade out
    /// very quickly as it gets further from the listener.
    /// The default value of the attenuation is 1.
    ///
    /// # Arguments
    /// * attenuation - New attenuation factor of the music
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe {
            ffi::sfMusic_setAttenuation(self.music, attenuation as c_float)
        }
    }

    /// Get the pitch of a music
    ///
    /// Return the pitch of the music
    fn get_pitch(&self) -> f32 {
        unsafe {
            ffi::sfMusic_getPitch(self.music) as f32
        }
    }

    /// Get the volume of a music
    ///
    /// Return the volume of the music, in the range [0, 100]
    fn get_volume(&self) -> f32 {
        unsafe {
            ffi::sfMusic_getVolume(self.music) as f32
        }
    }

    /// Get the 3D position of a music in the audio scene
    ///
    /// Return the position of the music in the world
    fn get_position(&self) -> Vector3f {
        unsafe {
            Vector3f::from_raw(ffi::sfMusic_getPosition(self.music))
        }
    }

    /// Tell whether a music's position is relative to the listener or is absolute
    ///
    /// Return true if the position is relative, false if it's absolute
    fn is_relative_to_listener(&self) -> bool {
        unsafe {
            ffi::sfMusic_isRelativeToListener(self.music).to_bool()
        }
    }

    /// Get the minimum distance of a music
    ///
    /// Return the minimum distance of the music
    fn get_min_distance(&self) -> f32 {
        unsafe {
           ffi::sfMusic_getMinDistance(self.music) as f32
        }
    }

    /// Get the attenuation factor of a music
    ///
    /// Return the attenuation factor of the music
    fn get_attenuation(&self) -> f32 {
        unsafe {
            ffi::sfMusic_getAttenuation(self.music) as f32
        }
    }
}

impl Drop for Music {
    /// Destructor for class Music. Destroy all the ressource.
    fn drop(&mut self) {
        unsafe {
            ffi::sfMusic_destroy(self.music);
        }
    }
}
