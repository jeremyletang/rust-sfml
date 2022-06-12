use crate::{
    audio::{SoundBuffer, SoundSource, SoundStatus},
    ffi::{self as ffi, sfBool},
    sf_bool_ext::SfBoolExt,
    system::{Time, Vector3f},
};
use std::{marker::PhantomData, ptr::NonNull};

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
/// rather see [`Music`] (which is based on streaming).
///
/// In order to work, a sound must be given a buffer of audio data to play.
/// Audio data (samples) is stored in [`SoundBuffer`], and attached to a sound with the
/// [`set_buffer`] function. The buffer object attached to a sound must remain alive as long as
/// the sound uses it. Note that multiple sounds can use the same sound buffer at the same time.
///
/// [`set_buffer`]: Sound::set_buffer
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
///
/// [`Music`]: crate::audio::Music
#[derive(Debug)]
pub struct Sound<'s> {
    sound: NonNull<ffi::sfSound>,
    buffer: PhantomData<&'s SoundBuffer>,
}

impl<'s> Sound<'s> {
    /// Create a new `Sound`
    #[must_use]
    pub fn new() -> Sound<'s> {
        let s = unsafe { ffi::sfSound_create() };
        Sound {
            sound: NonNull::new(s).expect("Failed to create Sound"),
            buffer: PhantomData,
        }
    }

    /// Create a new `Sound` with a buffer
    #[must_use]
    pub fn with_buffer(buffer: &SoundBuffer) -> Sound {
        let mut s = Sound::new();
        s.set_buffer(buffer);
        s
    }

    /// Sets whether this sound should loop or not.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { ffi::sfSound_setLoop(self.sound.as_ptr(), sfBool::from_bool(looping)) }
    }

    /// Tell whether or not a sound is in loop mode
    ///
    /// Return true if the sound is looping, false otherwise
    #[must_use]
    pub fn is_looping(&self) -> bool {
        unsafe { ffi::sfSound_getLoop(self.sound.as_ptr()) }.into_bool()
    }

    /// Start or resume playing a sound
    ///
    /// This function starts the sound if it was stopped, resumes
    /// it if it was paused, and restarts it from beginning if it
    /// was it already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the sound is played.
    pub fn play(&mut self) {
        unsafe { ffi::sfSound_play(self.sound.as_ptr()) }
    }

    /// Pause a sound
    ///
    /// This function pauses the sound if it was playing,
    /// otherwise (sound already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe { ffi::sfSound_pause(self.sound.as_ptr()) }
    }

    /// Stop playing a sound
    ///
    /// This function stops the sound if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe { ffi::sfSound_stop(self.sound.as_ptr()) }
    }

    /// Get the current status of a sound (stopped, paused, playing)
    ///
    /// Return current status
    #[must_use]
    pub fn status(&self) -> SoundStatus {
        unsafe { SoundStatus(ffi::sfSound_getStatus(self.sound.as_ptr())) }
    }

    /// Get the current playing position of a sound
    ///
    /// Return the current playing position
    #[must_use]
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfSound_getPlayingOffset(self.sound.as_ptr())) }
    }

    /// Change the current playing position of a sound
    ///
    /// The playing position can be changed when the sound is
    /// either paused or playing.
    ///
    /// # Arguments
    /// * timeOffset - New playing position
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::sfSound_setPlayingOffset(self.sound.as_ptr(), time_offset.raw()) }
    }

    /// Set the source buffer containing the audio data to play
    ///
    /// # Arguments
    /// * buffer - Sound buffer to attach to the sound
    pub fn set_buffer(&mut self, buffer: &'s SoundBuffer) {
        let ptr: *const SoundBuffer = buffer;
        unsafe { ffi::sfSound_setBuffer(self.sound.as_ptr(), ptr as _) }
    }

    /// Get the audio buffer attached to a sound
    ///
    /// Return an option to Sound buffer attached to the sound or None
    #[must_use]
    pub fn buffer(&self) -> Option<&SoundBuffer> {
        unsafe {
            let ptr = ffi::sfSound_getBuffer(self.sound.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *const SoundBuffer))
            }
        }
    }
}

impl<'a> Default for Sound<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> Clone for Sound<'s> {
    fn clone(&self) -> Self {
        let s = unsafe { ffi::sfSound_copy(self.sound.as_ptr()) };
        Sound {
            sound: NonNull::new(s).expect("Failed to copy Sound"),
            buffer: self.buffer,
        }
    }
}

impl<'s> SoundSource for Sound<'s> {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::sfSound_setPitch(self.sound.as_ptr(), pitch) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::sfSound_setVolume(self.sound.as_ptr(), volume) }
    }
    fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        unsafe { ffi::sfSound_setPosition(self.sound.as_ptr(), position.into().raw()) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe {
            ffi::sfSound_setRelativeToListener(self.sound.as_ptr(), sfBool::from_bool(relative))
        }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::sfSound_setMinDistance(self.sound.as_ptr(), distance) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::sfSound_setAttenuation(self.sound.as_ptr(), attenuation) }
    }
    fn pitch(&self) -> f32 {
        unsafe { ffi::sfSound_getPitch(self.sound.as_ptr()) }
    }
    fn volume(&self) -> f32 {
        unsafe { ffi::sfSound_getVolume(self.sound.as_ptr()) }
    }
    fn position(&self) -> Vector3f {
        unsafe { Vector3f::from_raw(ffi::sfSound_getPosition(self.sound.as_ptr())) }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfSound_isRelativeToListener(self.sound.as_ptr()).into_bool() }
    }
    fn min_distance(&self) -> f32 {
        unsafe { ffi::sfSound_getMinDistance(self.sound.as_ptr()) }
    }
    fn attenuation(&self) -> f32 {
        unsafe { ffi::sfSound_getAttenuation(self.sound.as_ptr()) }
    }
}

impl<'s> Drop for Sound<'s> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfSound_destroy(self.sound.as_ptr());
        }
    }
}
