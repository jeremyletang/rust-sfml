use {
    super::{sound_channel::SoundChannel, sound_source::SoundSource},
    crate::{
        cpp::CppVector,
        ffi::audio::{sfCustomSoundStream, sfCustomSoundStream_new, sfCustomSoundStream_play, sfCustomSoundStream_pause, sfCustomSoundStream_stop, sfCustomSoundStream_getPlayingOffset, sfCustomSoundStream_setPlayingOffset, sfCustomSoundStream_getChannelCount, sfCustomSoundStream_getSampleRate, sfCustomSoundStream_getChannelMap, sfCustomSoundStream_isLooping, sfCustomSoundStream_setLooping, sfCustomSoundStream_setPitch, sfCustomSoundStream_setVolume, sfCustomSoundStream_setPosition, sfCustomSoundStream_setRelativeToListener, sfCustomSoundStream_setMinDistance, sfCustomSoundStream_setAttenuation, sfCustomSoundStream_getPitch, sfCustomSoundStream_getVolume, sfCustomSoundStream_getPosition, sfCustomSoundStream_isRelativeToListener, sfCustomSoundStream_getMinDistance, sfCustomSoundStream_getAttenuation, sfCustomSoundStream_setPan, sfCustomSoundStream_setSpatializationEnabled, sfCustomSoundStream_setDirection, sfCustomSoundStream_setCone, sfCustomSoundStream_setVelocity, sfCustomSoundStream_setDopplerFactor, sfCustomSoundStream_setDirectionalAttenuationFactor, sfCustomSoundStream_setMaxDistance, sfCustomSoundStream_setMinGain, sfCustomSoundStream_setMaxGain, effect_processor_trampoline, sfCustomSoundStream_setEffectProcessor, sfCustomSoundStream_getPan, sfCustomSoundStream_isSpatializationEnabled, sfCustomSoundStream_getDirection, sfCustomSoundStream_getCone, sfCustomSoundStream_getVelocity, sfCustomSoundStream_getDopplerFactor, sfCustomSoundStream_getDirectionalAttenuationFactor, sfCustomSoundStream_getMaxDistance, sfCustomSoundStream_getMinGain, sfCustomSoundStream_getMaxGain, sfCustomSoundStream_getStatus, sfCustomSoundStream_del},
        system::{Time, Vector3f},
    },
    std::{ffi::c_uint, marker::PhantomData, os::raw::c_void, panic, ptr::NonNull},
};

/// Trait for streamed audio sources.
///
/// The implementor must be able to be sent to another thread, so it must
/// implement [`Send`].
pub trait SoundStream: Send {
    /// Request a new chunk of audio samples from the stream source.
    ///
    /// Returns `(chunk, keep_playing)`, where `chunk` is the chunk of audio samples,
    /// and `keep_playing` tells the streaming loop whether to keep playing or to stop.
    fn get_data(&mut self) -> (&[i16], bool);
    /// Change the current playing position in the stream source.
    fn seek(&mut self, offset: Time);
    /// Return the number of channels of the stream.
    fn channel_count(&self) -> u32;
    /// Get the stream sample rate of the stream.
    fn sample_rate(&self) -> u32;
    /// This is used to map a sample in the sample stream to a
    /// position during spatialization.
    ///
    /// Return Map of position in sample frame to sound channel}
    fn get_channel_map(&self) -> Vec<SoundChannel>;
}

/// Player for custom streamed audio sources. See [`SoundStream`].
#[derive(Debug)]
pub struct SoundStreamPlayer<'a, S: SoundStream + 'a> {
    handle: NonNull<sfCustomSoundStream>,
    /// We need to hold a raw pointer instead of a reference, since
    /// we send it to another thread, violating `&mut` rules.
    ///
    /// Not sure if `NonNull` can be used to be honest. Not gonna risk it.
    stream: *mut S,
    _borrow: PhantomData<&'a mut S>,
}

unsafe extern "C" fn get_data_callback<S: SoundStream>(
    chunk: *mut crate::ffi::audio::sfSoundStreamChunk,
    user_data: *mut c_void,
) -> bool {
    let stream: *mut S = user_data.cast();
    unsafe {
        let (data, keep_playing) =
            if let Ok(ret) = panic::catch_unwind(panic::AssertUnwindSafe(|| (*stream).get_data())) { ret } else {
                eprintln!("sound_stream: Stopping playback beacuse `get_data` panicked.");
                (&[][..], false)
            };
        (*chunk).samples = data.as_ptr();
        (*chunk).sample_count = data.len();
        keep_playing
    }
}

unsafe extern "C" fn seek_callback<S: SoundStream>(
    offset: crate::ffi::system::sfTime,
    user_data: *mut c_void,
) {
    let stream: *mut S = user_data.cast();
    let result = unsafe {
        panic::catch_unwind(panic::AssertUnwindSafe(|| {
            (*stream).seek(Time::from_raw(offset));
        }))
    };
    if result.is_err() {
        eprintln!("sound_stream: Failed to seek because `seek` panicked.");
    }
}

impl<'a, S: SoundStream> SoundStreamPlayer<'a, S> {
    /// Create a new `SoundStreamPlayer` with the specified [`SoundStream`].
    ///
    /// # Panics
    ///
    /// Panics if for some reason a `SoundStreamPlayer` can't be created.
    pub fn new(sound_stream: &'a mut S) -> Self {
        let channel_count = sound_stream.channel_count();
        let sample_rate = sound_stream.sample_rate();
        let channel_map = sound_stream.get_channel_map();
        let sound_stream: *mut S = sound_stream;
        Self {
            handle: unsafe {
                NonNull::new(sfCustomSoundStream_new(
                    Some(get_data_callback::<S>),
                    Some(seek_callback::<S>),
                    channel_count,
                    sample_rate,
                    channel_map.as_ptr().cast(),
                    channel_map.len(),
                    sound_stream.cast(),
                ))
                .expect("Failed to create SoundStreamPlayer")
            },
            stream: sound_stream,
            _borrow: PhantomData,
        }
    }
    /// Start or resume playing the audio stream.
    pub fn play(&mut self) {
        unsafe {
            sfCustomSoundStream_play(self.handle.as_ptr());
        }
    }
    /// Pause the audio stream.
    ///
    /// This function pauses the stream if it was playing,
    /// otherwise (stream already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe {
            sfCustomSoundStream_pause(self.handle.as_ptr());
        }
    }
    /// Stop playing, lending out the underlying [`SoundStream`].
    ///
    /// This function stops the stream if it was playing or paused, and does nothing if it was
    /// already stopped. It also resets the playing position (unlike [`pause`]).
    ///
    /// [`pause`]: SoundStreamPlayer::pause
    ///
    /// It lends out the underlying `SoundStream`, allowing it to be manipulated.
    pub fn stop(&mut self) -> &mut S {
        unsafe {
            sfCustomSoundStream_stop(self.handle.as_ptr());
            &mut *self.stream
        }
    }
    /// Get the current playing position, from the beginning of the stream
    #[must_use]
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from_raw(sfCustomSoundStream_getPlayingOffset(self.handle.as_ptr())) }
    }
    /// Change the current playing position of the stream.
    ///
    /// The playing position can be changed when the stream is either paused or playing.
    /// Changing the playing position when the stream is stopped has no effect,
    /// since playing the stream would reset its position.
    pub fn set_playing_offset(&mut self, offset: Time) {
        unsafe { sfCustomSoundStream_setPlayingOffset(self.handle.as_ptr(), offset.raw()) }
    }
    /// Return the number of channels of the stream.
    ///
    /// 1 channel means a mono sound, 2 means stereo, etc.
    #[must_use]
    pub fn channel_count(&self) -> u32 {
        unsafe { sfCustomSoundStream_getChannelCount(self.handle.as_ptr()) }
    }
    /// Get the stream sample rate of the stream.
    ///
    /// The sample rate is the number of audio samples played per second.
    /// The higher, the better the quality.
    #[must_use]
    pub fn sample_rate(&self) -> u32 {
        unsafe { sfCustomSoundStream_getSampleRate(self.handle.as_ptr()) }
    }
    /// This is used to map a sample in the sample stream to a
    /// position during spatialization.
    ///
    /// Return Map of position in sample frame to sound channel
    #[must_use]
    pub fn channel_map(&self) -> &'static CppVector<SoundChannel> {
        unsafe { &*sfCustomSoundStream_getChannelMap(self.handle.as_ptr()) }
    }
    /// Tell whether or not the stream is in loop mode.
    #[must_use]
    pub fn is_looping(&self) -> bool {
        unsafe { sfCustomSoundStream_isLooping(self.handle.as_ptr()) }
    }
    /// Set whether or not the stream should loop after reaching the end.
    ///
    /// If set, the stream will restart from beginning after reaching the end and so on,
    /// until it is stopped or `set_looping(false)` is called.
    /// The default looping state for streams is false.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { sfCustomSoundStream_setLooping(self.handle.as_ptr(), looping) }
    }
}

impl<S: SoundStream> SoundSource for SoundStreamPlayer<'_, S> {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { sfCustomSoundStream_setPitch(self.handle.as_ptr(), pitch) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { sfCustomSoundStream_setVolume(self.handle.as_ptr(), volume) }
    }
    fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        unsafe { sfCustomSoundStream_setPosition(self.handle.as_ptr(), position.into()) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { sfCustomSoundStream_setRelativeToListener(self.handle.as_ptr(), relative) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { sfCustomSoundStream_setMinDistance(self.handle.as_ptr(), distance) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { sfCustomSoundStream_setAttenuation(self.handle.as_ptr(), attenuation) }
    }
    fn pitch(&self) -> f32 {
        unsafe { sfCustomSoundStream_getPitch(self.handle.as_ptr()) }
    }
    fn volume(&self) -> f32 {
        unsafe { sfCustomSoundStream_getVolume(self.handle.as_ptr()) }
    }
    fn position(&self) -> Vector3f {
        unsafe { sfCustomSoundStream_getPosition(self.handle.as_ptr()) }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { sfCustomSoundStream_isRelativeToListener(self.handle.as_ptr()) }
    }
    fn min_distance(&self) -> f32 {
        unsafe { sfCustomSoundStream_getMinDistance(self.handle.as_ptr()) }
    }
    fn attenuation(&self) -> f32 {
        unsafe { sfCustomSoundStream_getAttenuation(self.handle.as_ptr()) }
    }
    fn set_pan(&mut self, pan: f32) {
        unsafe { sfCustomSoundStream_setPan(self.handle.as_ptr(), pan) }
    }
    fn set_spatialization_enabled(&mut self, enabled: bool) {
        unsafe { sfCustomSoundStream_setSpatializationEnabled(self.handle.as_ptr(), enabled) }
    }
    fn set_direction<P: Into<Vector3f>>(&mut self, direction: P) {
        unsafe { sfCustomSoundStream_setDirection(self.handle.as_ptr(), direction.into()) }
    }
    fn set_cone(&mut self, cone: super::sound_source::Cone) {
        unsafe { sfCustomSoundStream_setCone(self.handle.as_ptr(), cone.into()) }
    }
    fn set_velocity<P: Into<Vector3f>>(&mut self, velocity: P) {
        unsafe { sfCustomSoundStream_setVelocity(self.handle.as_ptr(), velocity.into()) }
    }
    fn set_doppler_factor(&mut self, factor: f32) {
        unsafe { sfCustomSoundStream_setDopplerFactor(self.handle.as_ptr(), factor) }
    }
    fn set_directional_attenuation_factor(&mut self, factor: f32) {
        unsafe { sfCustomSoundStream_setDirectionalAttenuationFactor(self.handle.as_ptr(), factor) }
    }
    fn set_max_distance(&mut self, distance: f32) {
        unsafe { sfCustomSoundStream_setMaxDistance(self.handle.as_ptr(), distance) }
    }
    fn set_min_gain(&mut self, gain: f32) {
        unsafe { sfCustomSoundStream_setMinGain(self.handle.as_ptr(), gain) }
    }
    fn set_max_gain(&mut self, gain: f32) {
        unsafe { sfCustomSoundStream_setMaxGain(self.handle.as_ptr(), gain) }
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
                ) = effect_processor_trampoline;
                (Some(trampoline), Box::into_raw(boxed).cast::<c_void>())
            }
            None => (None, std::ptr::null_mut()),
        };

        unsafe {
            sfCustomSoundStream_setEffectProcessor(self.handle.as_ptr(), cb, user_data);
        }
    }
    fn pan(&self) -> f32 {
        unsafe { sfCustomSoundStream_getPan(self.handle.as_ptr()) }
    }
    fn is_spatialization_enabled(&self) -> bool {
        unsafe { sfCustomSoundStream_isSpatializationEnabled(self.handle.as_ptr()) }
    }
    fn direction(&self) -> Vector3f {
        unsafe { sfCustomSoundStream_getDirection(self.handle.as_ptr()) }
    }
    fn cone(&self) -> super::sound_source::Cone {
        unsafe { sfCustomSoundStream_getCone(self.handle.as_ptr()).into() }
    }
    fn velocity(&self) -> Vector3f {
        unsafe { sfCustomSoundStream_getVelocity(self.handle.as_ptr()) }
    }
    fn doppler_factor(&self) -> f32 {
        unsafe { sfCustomSoundStream_getDopplerFactor(self.handle.as_ptr()) }
    }
    fn directional_attenuation_factor(&self) -> f32 {
        unsafe { sfCustomSoundStream_getDirectionalAttenuationFactor(self.handle.as_ptr()) }
    }
    fn get_max_distance(&self) -> f32 {
        unsafe { sfCustomSoundStream_getMaxDistance(self.handle.as_ptr()) }
    }
    fn get_min_gain(&self) -> f32 {
        unsafe { sfCustomSoundStream_getMinGain(self.handle.as_ptr()) }
    }
    fn get_max_gain(&self) -> f32 {
        unsafe { sfCustomSoundStream_getMaxGain(self.handle.as_ptr()) }
    }
    fn status(&self) -> super::sound_source::Status {
        unsafe { sfCustomSoundStream_getStatus(self.handle.as_ptr()) }
    }
}

impl<S: SoundStream> Drop for SoundStreamPlayer<'_, S> {
    fn drop(&mut self) {
        unsafe {
            // It seems there can be problems (e.g. "pure virtual method called") if the
            // stream is not stopped before it's destroyed. So let's make sure it's stopped.
            sfCustomSoundStream_stop(self.handle.as_ptr());
            sfCustomSoundStream_del(self.handle.as_ptr());
        }
    }
}
