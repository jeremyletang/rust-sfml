use {
    crate::{
        audio::{SoundSource, SoundStatus},
        ffi::audio::*,
        system::{Time, Vector3f},
    },
    std::{marker::PhantomData, os::raw::c_void, panic, ptr::NonNull},
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
            match panic::catch_unwind(panic::AssertUnwindSafe(|| (*stream).get_data())) {
                Ok(ret) => ret,
                Err(_) => {
                    eprintln!("sound_stream: Stopping playback beacuse `get_data` panicked.");
                    (&[][..], false)
                }
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
            (*stream).seek(Time::from_raw(offset))
        }))
    };
    if result.is_err() {
        eprintln!("sound_stream: Failed to seek because `seek` panicked.");
    }
}

impl<'a, S: SoundStream> SoundStreamPlayer<'a, S> {
    /// Create a new `SoundStreamPlayer` with the specified [`SoundStream`].
    pub fn new(sound_stream: &'a mut S) -> Self {
        let channel_count = sound_stream.channel_count();
        let sample_rate = sound_stream.sample_rate();
        let sound_stream: *mut S = sound_stream;
        Self {
            handle: unsafe {
                NonNull::new(sfCustomSoundStream_new(
                    Some(get_data_callback::<S>),
                    Some(seek_callback::<S>),
                    channel_count,
                    sample_rate,
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
    /// Get the current status of the stream (stopped, paused, playing)
    #[must_use]
    pub fn status(&self) -> SoundStatus {
        unsafe { SoundStatus(sfCustomSoundStream_getStatus(self.handle.as_ptr())) }
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
    /// Tell whether or not the stream is in loop mode.
    #[must_use]
    pub fn is_looping(&self) -> bool {
        unsafe { sfCustomSoundStream_getLoop(self.handle.as_ptr()) }
    }
    /// Set whether or not the stream should loop after reaching the end.
    ///
    /// If set, the stream will restart from beginning after reaching the end and so on,
    /// until it is stopped or `set_looping(false)` is called.
    /// The default looping state for streams is false.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { sfCustomSoundStream_setLoop(self.handle.as_ptr(), looping) }
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
