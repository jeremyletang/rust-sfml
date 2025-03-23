use crate::{
    cpp::{CppVector, CppVectorItem},
    ffi::audio as ffi,
};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SoundChannel {
    #[default]
    Unspecified,
    Mono,
    FrontLeft,
    FrontRight,
    FrontCenter,
    FrontLeftOfCenter,
    FrontRightOfCenter,
    LowFrequencyEffects,
    BackLeft,
    BackRight,
    BackCenter,
    SideLeft,
    SideRight,
    TopCenter,
    TopFrontLeft,
    TopFrontRight,
    TopFrontCenter,
    TopBackLeft,
    TopBackRight,
    TopBackCenter,
}

unsafe impl CppVectorItem for SoundChannel {
    fn get_data(vec: &crate::cpp::CppVector<Self>) -> *const Self {
        unsafe { ffi::sfSoundChannelVector_getData(vec) }
    }

    fn get_len(vec: &CppVector<Self>) -> usize {
        unsafe { ffi::sfSoundChannelVector_getLength(vec) }
    }

    fn del(vec: &mut CppVector<Self>) {
        unsafe { ffi::sfSoundChannelVector_del(vec) }
    }
}
