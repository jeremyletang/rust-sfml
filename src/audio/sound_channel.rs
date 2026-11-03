use crate::{
    cpp::{CppVector, CppVectorItem, CppVectorSliceable},
    ffi::audio as ffi,
};

/// Types of sound channels that can be read/written form sound buffers/files
///
/// In multi-channel audio, each sound channel can be
/// assigned a position. The position of the channel is
/// used to determine where to place a sound when it
/// is spatialized. Assigning an incorrect sound channel
/// will result in multi-channel audio being positioned
/// incorrectly when using spatialization.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[allow(missing_docs)]
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

unsafe impl CppVectorSliceable for SoundChannel {}

unsafe impl CppVectorItem for SoundChannel {
    fn get_data(vec: &crate::cpp::CppVector<Self>) -> *const Self {
        unsafe { ffi::sfSoundChannelVector_getData(vec) }
    }

    fn get_len(vec: &CppVector<Self>) -> usize {
        unsafe { ffi::sfSoundChannelVector_getLength(vec) }
    }

    // Only need to borrow the data. No need to clone, so no need to delete.
    fn del(_: &mut CppVector<Self>) {}
}
