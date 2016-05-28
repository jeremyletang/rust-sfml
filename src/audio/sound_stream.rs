use csfml_audio_sys::*;
use csfml_system_sys::*;
use ext::sf_bool_ext::SfBoolExt;
use raw_conv::FromRaw;
use system::Time;
use std::panic;

/// Trait for streamed audio sources.
pub trait SoundStream {
    /// Request a new chunk of audio samples from the stream source.
    ///
    /// Returns (`chunk`, `keep_playing`), where `chunk` is the chunk of audio samples,
    /// and `keep_playing` tells the streaming loop whether to keep playing or to stop.
    fn get_data(&mut self) -> (&mut [i16], bool);
    /// Change the current playing position in the stream source.
    fn seek(&mut self, offset: Time);
}

/// Player for custom streamed audio sources. See `SoundStream`.
pub struct SoundStreamPlayer<S: SoundStream> {
    _sound_stream: Box<S>,
    sf_sound_stream: *mut sfSoundStream,
}

unsafe extern "C" fn get_data_callback<S: SoundStream>(chunk: *mut sfSoundStreamChunk,
                                                       stream: *mut S)
                                                       -> sfBool {
    let (data, keep_playing) = match panic::catch_unwind(panic::AssertUnwindSafe(|| {
        (*stream).get_data()
    })) {
        Ok(ret) => ret,
        Err(_) => {
            use std::io::Write;
            let _ = writeln!(::std::io::stderr(),
                             "sound_stream: Stopping playback beacuse get_data_callback panicked.");
            (&mut [][..], false)
        }
    };
    (*chunk).samples = data.as_mut_ptr();
    (*chunk).sampleCount = data.len() as u32;
    sfBool::from_bool(keep_playing)
}

unsafe extern "C" fn seek_callback<S: SoundStream>(offset: sfTime, stream: *mut S) {
    (*stream).seek(Time::from_raw(offset));
}

impl<S: SoundStream> SoundStreamPlayer<S> {
    /// Create a new `SoundStreamPlayer` with the specified `SoundStream`.
    pub fn new(sound_stream: S, channels: u32, sample_rate: u32) -> Self {
        let mut boxed_sound_stream = Box::new(sound_stream);
        let sound_stream_ptr: *mut SoundStream = &mut *boxed_sound_stream;
        let get_data_callback =
            get_data_callback::<S> as unsafe extern "C" fn(*mut sfSoundStreamChunk, *mut S) -> sfBool;
        let seek_callback = seek_callback::<S> as unsafe extern "C" fn(sfTime, *mut S);
        SoundStreamPlayer {
            _sound_stream: boxed_sound_stream,
            sf_sound_stream: unsafe {
                sfSoundStream_create(Some(::std::mem::transmute(get_data_callback)),
                                     Some(::std::mem::transmute(seek_callback)),
                                     channels, // channel count
                                     sample_rate, // sample rate
                                     sound_stream_ptr as *mut _)
            },
        }
    }
    /// Start or resume playing the audio stream.
    pub fn play(&mut self) {
        unsafe {
            sfSoundStream_play(self.sf_sound_stream);
        }
    }
}

impl<S: SoundStream> Drop for SoundStreamPlayer<S> {
    fn drop(&mut self) {
        unsafe {
            sfSoundStream_destroy(self.sf_sound_stream);
        }
    }
}
