use {
    super::sound_source::SoundSource,
    crate::{
        audio::SoundBuffer,
        ffi,
        system::{Time, Vector3f},
    },
    std::{
        ffi::{c_uint, c_void},
        marker::PhantomData,
        ptr::NonNull,
    },
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

// SAFETY: An `sfSound` isn't tied to a particular thread, so it can be sent between threads safely.
unsafe impl Send for Sound<'_> {}

// SAFETY: An `&Sound` only allows access to methods which read the status of the sound, which is
// fine to do from multiple threads at once. Thus it is safe to pass `&Sound` between threads.
unsafe impl Sync for Sound<'_> {}

/// Creation
impl<'buf> Sound<'buf> {
    /// Create a new `Sound`
    ///
    /// # Panics
    ///
    /// Panics on allocation failure
    #[must_use]
    pub fn new(sound_buffer: &'buf SoundBuffer) -> Self {
        let s = unsafe { ffi::audio::sfSound_new(sound_buffer) };
        Sound {
            handle: NonNull::new(s).expect("Failed to create Sound"),
            buffer: PhantomData,
        }
    }

    /// Create a new `Sound` with a buffer
    #[must_use]
    pub fn with_buffer(buffer: &'buf SoundBuffer) -> Self {
        let mut s = Sound::new(buffer);
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
        unsafe { ffi::audio::sfSound_isLooping(self.handle.as_ptr()) }
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
        unsafe { ffi::audio::sfSound_setLooping(self.handle.as_ptr(), looping) }
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
    fn set_pan(&mut self, pan: f32) {
        unsafe { ffi::audio::sfSound_setPan(self.handle.as_ptr(), pan) }
    }
    fn set_spatialization_enabled(&mut self, enabled: bool) {
        unsafe { ffi::audio::sfSound_setSpatializationEnabled(self.handle.as_ptr(), enabled) }
    }
    fn set_direction<P: Into<Vector3f>>(&mut self, direction: P) {
        unsafe { ffi::audio::sfSound_setDirection(self.handle.as_ptr(), direction.into()) }
    }
    fn set_cone(&mut self, cone: super::sound_source::Cone) {
        unsafe { ffi::audio::sfSound_setCone(self.handle.as_ptr(), cone.into()) }
    }
    fn set_velocity<P: Into<Vector3f>>(&mut self, velocity: P) {
        unsafe { ffi::audio::sfSound_setVelocity(self.handle.as_ptr(), velocity.into()) }
    }
    fn set_doppler_factor(&mut self, factor: f32) {
        unsafe { ffi::audio::sfSound_setDopplerFactor(self.handle.as_ptr(), factor) }
    }
    fn set_directional_attenuation_factor(&mut self, factor: f32) {
        unsafe { ffi::audio::sfSound_setDirectionalAttenuationFactor(self.handle.as_ptr(), factor) }
    }
    fn set_max_distance(&mut self, distance: f32) {
        unsafe { ffi::audio::sfSound_setMaxDistance(self.handle.as_ptr(), distance) }
    }
    fn set_min_gain(&mut self, gain: f32) {
        unsafe { ffi::audio::sfSound_setMinGain(self.handle.as_ptr(), gain) }
    }
    fn set_max_gain(&mut self, gain: f32) {
        unsafe { ffi::audio::sfSound_setMaxGain(self.handle.as_ptr(), gain) }
    }
    fn set_effect_processor(&mut self, effect_processor: super::sound_source::EffectProcessor) {
        let (cb, user_data) = match effect_processor {
            Some(cb) => {
                let boxed = Box::new(cb);
                let trampoline: unsafe extern "C" fn(
                    *const f32,
                    *mut c_uint,
                    *mut f32,
                    *mut c_uint,
                    c_uint,
                    *mut c_void,
                ) = ffi::audio::effect_processor_trampoline;
                (Some(trampoline), Box::into_raw(boxed).cast::<c_void>())
            }
            None => (None, std::ptr::null_mut()),
        };

        unsafe {
            ffi::audio::sfSound_setEffectProcessor(self.handle.as_ptr(), cb, user_data);
        }
    }
    fn pan(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getPan(self.handle.as_ptr()) }
    }
    fn is_spatialization_enabled(&self) -> bool {
        unsafe { ffi::audio::sfSound_isSpatializationEnabled(self.handle.as_ptr()) }
    }
    fn direction(&self) -> Vector3f {
        unsafe { ffi::audio::sfSound_getDirection(self.handle.as_ptr()) }
    }
    fn cone(&self) -> super::sound_source::Cone {
        unsafe { ffi::audio::sfSound_getCone(self.handle.as_ptr()).into() }
    }
    fn velocity(&self) -> Vector3f {
        unsafe { ffi::audio::sfSound_getVelocity(self.handle.as_ptr()) }
    }
    fn doppler_factor(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getDopplerFactor(self.handle.as_ptr()) }
    }
    fn directional_attenuation_factor(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getDirectionalAttenuationFactor(self.handle.as_ptr()) }
    }
    fn get_max_distance(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getMaxDistance(self.handle.as_ptr()) }
    }
    fn get_min_gain(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getMinGain(self.handle.as_ptr()) }
    }
    fn get_max_gain(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getMaxGain(self.handle.as_ptr()) }
    }
    fn status(&self) -> super::sound_source::Status {
        unsafe { ffi::audio::sfSound_getStatus(self.handle.as_ptr()) }
    }
}

impl Drop for Sound<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::audio::sfSound_del(self.handle.as_ptr());
        }
    }
}
