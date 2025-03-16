pub use crate::ffi::*;
use crate::{
    audio::{SoundChannel, sound_source},
    system::Angle,
};
use {
    super::system::sfInputStreamHelper,
    crate::audio::listener,
    crate::cpp::{CppString as sfStdString, CppStringVector as sfStdStringVector},
};

decl_opaque! {
    pub(crate) sfSoundBufferRecorder;
    pub(crate) sfCustomSoundRecorder;
    pub(crate) sfMusic;
    pub(crate) sfSound;
    pub(crate) sfCustomSoundStream;
}

pub type sfSoundBuffer = crate::audio::SoundBuffer;
pub(crate) type sfSoundChannelVector = crate::cpp::CppVector<SoundChannel>;

#[repr(C)]
pub struct sfSoundStreamChunk {
    pub samples: *const i16,
    pub sample_count: usize,
}

#[repr(C)]
pub struct sfTimeSpan {
    /// The beginning offset of the time range
    pub offset: i64,
    /// The length of the time range
    pub length: i64,
}

pub(crate) type sfEffectProcessor = Option<
    unsafe extern "C" fn(
        input_frames: *const f32,
        input_frame_count: *mut c_uint,
        output_frames: *mut f32,
        output_frame_count: *mut c_uint,
        frame_channel_count: c_uint,
        user_data: *mut c_void,
    ),
>;

pub(crate) unsafe extern "C" fn effect_processor_trampoline(
    input_frames: *const f32,
    input_frame_count: *mut c_uint,
    output_frames: *mut f32,
    output_frame_count: *mut c_uint,
    frame_channel_count: c_uint,
    user_data: *mut c_void,
) {
    unsafe {
        let closure = &mut *user_data.cast::<sound_source::EffectProcessor>();

        let in_frames = *input_frame_count as usize;
        let out_frames = *output_frame_count as usize;
        let channels = frame_channel_count as usize;

        let input = std::slice::from_raw_parts(input_frames, in_frames * channels);
        let output = std::slice::from_raw_parts_mut(output_frames, out_frames * channels);

        if let Some(closure) = closure.as_mut() {
            closure(
                input,
                output,
                &mut *input_frame_count,
                &mut *output_frame_count,
                frame_channel_count,
            );
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) struct sfListenerCone {
    /// Inner angle, in degrees
    pub inner_angle: f32,
    /// Outer angle, in degrees
    pub outer_angle: f32,
    /// Outer gain
    pub outer_gain: f32,
}

impl From<listener::Cone> for sfListenerCone {
    #[inline]
    fn from(cone: listener::Cone) -> Self {
        sfListenerCone {
            inner_angle: cone.inner_angle.as_degrees(),
            outer_angle: cone.outer_angle.as_degrees(),
            outer_gain: cone.outer_gain,
        }
    }
}

impl From<sfListenerCone> for listener::Cone {
    fn from(cone: sfListenerCone) -> Self {
        listener::Cone {
            inner_angle: Angle::degrees(cone.inner_angle),
            outer_angle: Angle::degrees(cone.outer_angle),
            outer_gain: cone.outer_gain,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(crate) struct sfSoundSourceCone {
    /// Inner angle, in degrees
    pub inner_angle: f32,
    /// Outer angle, in degrees
    pub outer_angle: f32,
    /// Outer gain
    pub outer_gain: f32,
}

impl From<sound_source::Cone> for sfSoundSourceCone {
    fn from(cone: sound_source::Cone) -> Self {
        sfSoundSourceCone {
            inner_angle: cone.inner_angle.as_degrees(),
            outer_angle: cone.outer_angle.as_degrees(),
            outer_gain: cone.outer_gain,
        }
    }
}

impl From<sfSoundSourceCone> for sound_source::Cone {
    fn from(cone: sfSoundSourceCone) -> Self {
        sound_source::Cone {
            inner_angle: Angle::degrees(cone.inner_angle),
            outer_angle: Angle::degrees(cone.outer_angle),
            outer_gain: cone.outer_gain,
        }
    }
}

type sfSoundChannel = SoundChannel;
type sfSoundStatus = sound_source::Status;
type sfCustomSoundRecorderStartCb = Option<unsafe extern "C" fn(user_data: *mut c_void) -> bool>;
type sfCustomSoundRecorderProcessCb =
    Option<unsafe extern "C" fn(samples: *const i16, len: usize, user_data: *mut c_void) -> bool>;
type sfCustomSoundRecorderStopCb = Option<unsafe extern "C" fn(user_data: *mut c_void)>;

type sfCustomSoundStreamGetDataCb =
    Option<unsafe extern "C" fn(chunk: *mut sfSoundStreamChunk, user_data: *mut c_void) -> bool>;
type sfCustomSoundStreamSeekCb = Option<unsafe extern "C" fn(pos: i64, user_data: *mut c_void)>;

include!("audio_bindgen.rs");
