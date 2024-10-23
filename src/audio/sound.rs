use {
    crate::{
        audio::{SoundBuffer, SoundSource, SoundStatus},
        ffi,
        system::{Time, Vector3f},
    },
    std::{marker::PhantomData, ptr::NonNull},
};

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
/// [`Music`]: crate::audio::Music
#[derive(Debug)]
pub struct Sound<'buf> {
    handle: NonNull<ffi::audio::sfSound>,
    buffer: PhantomData<&'buf SoundBuffer>,
}

/// Creation
impl<'buf> Sound<'buf> {
    /// Create a new `Sound`
    #[must_use]
    pub fn new() -> Self {
        let s = unsafe { ffi::audio::sfSound_new() };
        Sound {
            handle: NonNull::new(s).expect("Failed to create Sound"),
            buffer: PhantomData,
        }
    }

    /// Create a new `Sound` with a buffer
    #[must_use]
    pub fn with_buffer(buffer: &'buf SoundBuffer) -> Self {
        let mut s = Sound::new();
        s.set_buffer(buffer);
        s
    }
}

/// Playback
impl Sound<'_> {
    /// Start or resume playing a sound
    ///
    /// This function starts the sound if it was stopped, resumes
    /// it if it was paused, and restarts it from beginning if it
    /// was it already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the sound is played.
    pub fn play(&mut self) {
        unsafe { ffi::audio::sfSound_play(self.handle.as_ptr()) }
    }

    /// Pause a sound
    ///
    /// This function pauses the sound if it was playing,
    /// otherwise (sound already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe { ffi::audio::sfSound_pause(self.handle.as_ptr()) }
    }

    /// Stop playing a sound
    ///
    /// This function stops the sound if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe { ffi::audio::sfSound_stop(self.handle.as_ptr()) }
    }
}

/// Query properties
impl<'buf> Sound<'buf> {
    /// Tell whether or not a sound is in loop mode
    ///
    /// Return true if the sound is looping, false otherwise
    #[must_use]
    pub fn is_looping(&self) -> bool {
        unsafe { ffi::audio::sfSound_getLoop(self.handle.as_ptr()) }
    }

    /// Get the current status of a sound (stopped, paused, playing)
    ///
    /// Return current status
    #[must_use]
    pub fn status(&self) -> SoundStatus {
        unsafe { SoundStatus(ffi::audio::sfSound_getStatus(self.handle.as_ptr())) }
    }

    /// Get the current playing position of a sound
    ///
    /// Return the current playing position
    #[must_use]
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from_raw(ffi::audio::sfSound_getPlayingOffset(self.handle.as_ptr())) }
    }
    /// Get the audio buffer attached to a sound
    ///
    /// Return an option to Sound buffer attached to the sound or None
    #[must_use]
    pub fn buffer(&self) -> Option<&'buf SoundBuffer> {
        unsafe { ffi::audio::sfSound_getBuffer(self.handle.as_ptr()).as_ref() }
    }
}

/// Set properties
impl<'buf> Sound<'buf> {
    /// Sets whether this sound should loop or not.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { ffi::audio::sfSound_setLoop(self.handle.as_ptr(), looping) }
    }

    /// Change the current playing position of a sound
    ///
    /// The playing position can be changed when the sound is
    /// either paused or playing.
    ///
    /// # Arguments
    /// * timeOffset - New playing position
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::audio::sfSound_setPlayingOffset(self.handle.as_ptr(), time_offset.raw()) }
    }

    /// Set the source buffer containing the audio data to play
    ///
    /// # Arguments
    /// * buffer - Sound buffer to attach to the sound
    pub fn set_buffer(&mut self, buffer: &'buf SoundBuffer) {
        unsafe { ffi::audio::sfSound_setBuffer(self.handle.as_ptr(), buffer) }
    }
}

impl Default for Sound<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Sound<'_> {
    fn clone(&self) -> Self {
        let s = unsafe { ffi::audio::sfSound_cpy(self.handle.as_ptr()) };
        Sound {
            handle: NonNull::new(s).expect("Failed to copy Sound"),
            buffer: self.buffer,
        }
    }
}

impl SoundSource for Sound<'_> {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::audio::sfSound_setPitch(self.handle.as_ptr(), pitch) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::audio::sfSound_setVolume(self.handle.as_ptr(), volume) }
    }
    fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        unsafe { ffi::audio::sfSound_setPosition(self.handle.as_ptr(), position.into()) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::audio::sfSound_setRelativeToListener(self.handle.as_ptr(), relative) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::audio::sfSound_setMinDistance(self.handle.as_ptr(), distance) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::audio::sfSound_setAttenuation(self.handle.as_ptr(), attenuation) }
    }
    fn pitch(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getPitch(self.handle.as_ptr()) }
    }
    fn volume(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getVolume(self.handle.as_ptr()) }
    }
    fn position(&self) -> Vector3f {
        unsafe { ffi::audio::sfSound_getPosition(self.handle.as_ptr()) }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::audio::sfSound_isRelativeToListener(self.handle.as_ptr()) }
    }
    fn min_distance(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getMinDistance(self.handle.as_ptr()) }
    }
    fn attenuation(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getAttenuation(self.handle.as_ptr()) }
    }
}

impl Drop for Sound<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::audio::sfSound_del(self.handle.as_ptr());
        }
    }
}
