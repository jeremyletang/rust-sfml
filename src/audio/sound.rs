// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

use libc::c_float;
use std::mem;

use audio::{SoundStatus, SoundBufferRef, SoundSource};
use system::Time;
use system::Vector3f;
use raw_conv::{Raw, FromRaw};

use csfml_system_sys::{sfBool, sfVector3f};
use csfml_audio_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Regular sound that can be played in the audio environment.
///
/// `Sound` is the type to use to play sounds.
///
/// It provides:
///
/// - Control (play, pause, stop)
/// - Ability to modify output parameters in real-time (pitch, volume, ...)
/// - 3D spatial features (position, attenuation, ...).
///
/// `Sound` is perfect for playing short sounds that can fit in memory and require no latency,
/// like foot steps or gun shots. For longer sounds, like background musics or long speeches,
/// rather see `Music` (which is based on streaming).
///
/// In order to work, a sound must be given a buffer of audio data to play.
/// Audio data (samples) is stored in `SoundBuffer`, and attached to a sound with the
/// `set_buffer()` function. The buffer object attached to a sound must remain alive as long as
/// the sound uses it. Note that multiple sounds can use the same sound buffer at the same time.
///
/// # Usage example
///
/// ```no_run
/// use sfml::audio::{Sound, SoundBuffer};
///
/// let buffer = SoundBuffer::from_file("sound.wav").unwrap();
/// let mut sound = Sound::with_buffer(&buffer);
/// sound.play();
/// ```
pub struct Sound<'s> {
    sound: *mut ffi::sfSound,
    buffer: Option<&'s SoundBufferRef>,
}

impl<'s> Sound<'s> {
    /// Create a new Sound
    pub fn new() -> Sound<'s> {
        let s = unsafe { ffi::sfSound_create() };
        if s.is_null() {
            panic!("sfSound_create returned null.")
        } else {
            Sound {
                sound: s,
                buffer: None,
            }
        }
    }

    /// Create a new Sound with a buffer
    pub fn with_buffer(buffer: &SoundBufferRef) -> Sound {
        let s = unsafe { ffi::sfSound_create() };
        if s.is_null() {
            panic!("sfSound_create returned null.")
        } else {
            unsafe {
                ffi::sfSound_setBuffer(s, buffer as *const _ as _);
            }
            Sound {
                sound: s,
                buffer: Some(buffer),
            }
        }
    }

    /// Sets whether this sound should loop or not.
    pub fn set_loop(&mut self, loop_: bool) {
        unsafe { ffi::sfSound_setLoop(self.sound, sfBool::from_bool(loop_)) }
    }

    /// Tell whether or not a sound is in loop mode
    ///
    /// Return true if the sound is looping, false otherwise
    pub fn get_loop(&self) -> bool {
        unsafe { ffi::sfSound_getLoop(self.sound) }.to_bool()
    }

    /// Start or resume playing a sound
    ///
    /// This function starts the sound if it was stopped, resumes
    /// it if it was paused, and restarts it from beginning if it
    /// was it already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the sound is played.
    pub fn play(&mut self) {
        unsafe { ffi::sfSound_play(self.sound) }
    }

    /// Pause a sound
    ///
    /// This function pauses the sound if it was playing,
    /// otherwise (sound already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe { ffi::sfSound_pause(self.sound) }
    }

    /// Stop playing a sound
    ///
    /// This function stops the sound if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe { ffi::sfSound_stop(self.sound) }
    }

    /// Get the current status of a sound (stopped, paused, playing)
    ///
    /// Return current status
    pub fn get_status(&self) -> SoundStatus {
        unsafe { mem::transmute(ffi::sfSound_getStatus(self.sound)) }
    }

    /// Get the current playing position of a sound
    ///
    /// Return the current playing position
    pub fn get_playing_offset(&self) -> Time {
        Time::from_raw(unsafe { ffi::sfSound_getPlayingOffset(self.sound) })
    }

    /// Change the current playing position of a sound
    ///
    /// The playing position can be changed when the sound is
    /// either paused or playing.
    ///
    /// # Arguments
    /// * timeOffset - New playing position
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::sfSound_setPlayingOffset(self.sound, time_offset.raw()) }
    }

    /// Set the source buffer containing the audio data to play
    ///
    /// It is important to note that the sound buffer is not copied,
    /// thus the sfSoundBuffer object must remain alive as long
    /// as it is attached to the sound.
    ///
    /// # Arguments
    /// * buffer - Sound buffer to attach to the sound
    pub fn set_buffer(&mut self, buffer: &'s SoundBufferRef) {
        self.buffer = Some(buffer);
        unsafe { ffi::sfSound_setBuffer(self.sound, buffer as *const _ as _) }
    }

    /// Get the audio buffer attached to a sound
    ///
    /// Return an option to Sound buffer attached to the sound or None
    pub fn get_buffer(&self) -> Option<&SoundBufferRef> {
        self.buffer
    }
}

impl<'a> Default for Sound<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> Clone for Sound<'s> {
    fn clone(&self) -> Self {
        let s = unsafe { ffi::sfSound_copy(self.sound) };
        if s.is_null() {
            panic!("Sound is null");
        } else {
            Sound {
                sound: s,
                buffer: self.buffer,
            }
        }
    }
}

impl<'s> SoundSource for Sound<'s> {
    /// Set the pitch of a sound
    ///
    /// The pitch represents the perceived fundamental frequency
    /// of a sound; thus you can make a sound more acute or grave
    /// by changing its pitch. A side effect of changing the pitch
    /// is to modify the playing speed of the sound as well.
    /// The default value for the pitch is 1.
    ///
    /// # Arguments
    /// * pitch - new pitch to apply to the sound
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::sfSound_setPitch(self.sound, pitch as c_float) }
    }

    /// Set the volume of a sound
    ///
    /// he volume is a value between 0 (mute) and 100 (full volume).
    /// The default value for the volume is 100.
    ///
    /// # Arguments
    /// * volume - Volume of the sound
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::sfSound_setVolume(self.sound, volume as c_float) }
    }

    /// Set the 3D position of a sound in the audio scene
    ///
    /// Only sounds with one channel (mono sounds) can be
    /// spatialized.
    /// The default position of a sound is (0, 0, 0).
    ///
    /// # Arguments
    /// * position - Position of the sound in the scene
    fn set_position(&mut self, position: &Vector3f) {
        unsafe { ffi::sfSound_setPosition(self.sound, position.raw()) }
    }

    /// Set the 3D position of a sound in the audio scene
    ///
    /// Only sounds with one channel (mono sounds) can be
    /// spatialized.
    /// The default position of a sound is (0, 0, 0).
    ///
    /// # Arguments
    /// * x - X coordinate of the position of the sound in the scene
    /// * y - Y coordinate of the position of the sound in the scene
    /// * z - Z coordinate of the position of the sound in the scene
    fn set_position3f(&mut self, x: f32, y: f32, z: f32) {
        unsafe { ffi::sfSound_setPosition(self.sound, sfVector3f { x: x, y: y, z: z }) }
    }

    /// Make a sounds's position relative to the listener or absolute
    ///
    /// Making a sound relative to the listener will ensure that it will always
    /// be played the same way regardless the position of the listener.
    /// This can be useful for non-spatialized sounds, sounds that are
    /// produced by the listener, or sounds attached to it.
    /// The default value is false (position is absolute).
    ///
    /// # Arguments
    /// * relative - true to set the position relative, false to set it absolute
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::sfSound_setRelativeToListener(self.sound, sfBool::from_bool(relative)) }
    }

    /// Set the minimum distance of a sound
    ///
    /// The "minimum distance" of a sound is the maximum
    /// distance at which it is heard at its maximum volume. Further
    /// than the minimum distance, it will start to fade out according
    /// to its attenuation factor. A value of 0 ("inside the head
    /// of the listener") is an invalid value and is forbidden.
    /// The default value of the minimum distance is 1.
    ///
    /// # Arguments
    /// * distance - New minimum distance of the sound
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::sfSound_setMinDistance(self.sound, distance as c_float) }
    }

    ///  Set the attenuation factor of a sound
    ///
    /// The attenuation is a multiplicative factor which makes
    /// the sound more or less loud according to its distance
    /// from the listener. An attenuation of 0 will produce a
    /// non-attenuated sound, i.e. its volume will always be the same
    /// whether it is heard from near or from far. On the other hand,
    /// an attenuation value such as 100 will make the sound fade out
    /// very quickly as it gets further from the listener.
    /// The default value of the attenuation is 1.
    ///
    /// # Arguments
    /// * attenuation - New attenuation factor of the sound
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::sfSound_setAttenuation(self.sound, attenuation as c_float) }
    }

    /// Get the pitch of a sound
    ///
    /// Return the pitch of the sound
    fn get_pitch(&self) -> f32 {
        unsafe { ffi::sfSound_getPitch(self.sound) as f32 }
    }

    /// Get the volume of a sound
    ///
    /// Return the volume of the sound, in the range [0, 100]
    fn get_volume(&self) -> f32 {
        unsafe { ffi::sfSound_getVolume(self.sound) as f32 }
    }

    /// Get the 3D position of a sound in the audio scene
    ///
    /// Return the position of the sound in the world
    fn get_position(&self) -> Vector3f {
        unsafe { Vector3f::from_raw(ffi::sfSound_getPosition(self.sound)) }
    }

    /// Tell whether a sound's position is relative to the listener or is absolute
    ///
    /// Return true if the position is relative, false if it's absolute
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfSound_isRelativeToListener(self.sound).to_bool() }
    }

    /// Get the minimum distance of a sound
    ///
    /// Return the minimum distance of the sound
    fn get_min_distance(&self) -> f32 {
        unsafe { ffi::sfSound_getMinDistance(self.sound) as f32 }
    }

    /// Get the attenuation factor of a sound
    ///
    /// Return the attenuation factor of the sound
    fn get_attenuation(&self) -> f32 {
        unsafe { ffi::sfSound_getAttenuation(self.sound) as f32 }
    }
}

impl<'s> Drop for Sound<'s> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfSound_destroy(self.sound);
        }
    }
}
