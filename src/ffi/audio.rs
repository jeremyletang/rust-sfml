/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

pub mod listener {
    pub use libc::c_int;

    pub use system::vector3;

    extern "C" {
        pub fn sfListener_setGlobalVolume(volume : f32) -> ();
        pub fn sfListener_getGlobalVolume() -> f32;
        pub fn sfListener_setPosition(position : vector3::Vector3f) -> ();
        pub fn sfListener_getPosition() -> vector3::Vector3f;
        pub fn sfListener_setDirection(orientation : vector3::Vector3f) -> ();
        pub fn sfListener_getDirection() -> vector3::Vector3f;
    }
}

pub mod music {
    use libc::{c_void, c_uint, c_float, c_char};

    use system::vector3::Vector3f;

    use ffi::system::time::sfTime;
    use ffi::audio::sound_status::sfSoundStatus;
    use ffi::sfml_types::SfBool;

    pub struct sfMusic {
        this :  *mut c_void,
        this1 : *mut c_void
    }

    extern "C" {
        pub fn sfMusic_createFromFile(filename : *mut c_char) -> *mut sfMusic;
        // sfMusic*mut  sfMusic_createFromMemory(const void*mut  data, size_t sizeInBytes);
        // sfMusic*mut  sfMusic_createFromStream(sfInputStream*mut  stream);
        pub fn sfMusic_destroy(music : *mut sfMusic) -> ();
        pub fn sfMusic_setLoop(music : *mut sfMusic, lloop : SfBool) -> ();
        pub fn sfMusic_getLoop(music : *mut sfMusic) -> SfBool;
        pub fn sfMusic_getDuration(music : *mut sfMusic) -> sfTime;
        pub fn sfMusic_play(music : *mut sfMusic) -> ();
        pub fn sfMusic_pause(music : *mut sfMusic) -> ();
        pub fn sfMusic_stop(music : *mut sfMusic) -> ();
        pub fn sfMusic_getChannelCount(music : *mut sfMusic) -> c_uint;
        pub fn sfMusic_getSampleRate(music : *mut sfMusic) -> c_uint;
        pub fn sfMusic_getStatus(music : *mut sfMusic) -> sfSoundStatus;
        pub fn sfMusic_getPlayingOffset(music : *mut sfMusic) -> sfTime;
        pub fn sfMusic_setPitch(music : *mut sfMusic, pitch : c_float) -> ();
        pub fn sfMusic_setVolume(music : *mut sfMusic, volume : c_float) -> ();
        pub fn sfMusic_setPosition(music : *mut sfMusic, position : Vector3f) -> ();
        pub fn sfMusic_setRelativeToListener(music : *mut sfMusic, relative : SfBool) -> ();
        pub fn sfMusic_setMinDistance(music : *mut sfMusic, distance : c_float) -> ();
        pub fn sfMusic_setAttenuation(music : *mut sfMusic, attenuation : c_float) -> ();
        pub fn sfMusic_setPlayingOffset(music : *mut sfMusic, timeOffset : sfTime) -> ();
        pub fn sfMusic_getPitch(music : *mut sfMusic) -> c_float;
        pub fn sfMusic_getVolume(music : *mut sfMusic) -> c_float;
        pub fn sfMusic_getPosition(music : *mut sfMusic) -> Vector3f;
        pub fn sfMusic_isRelativeToListener(music : *mut sfMusic) -> SfBool;
        pub fn sfMusic_getMinDistance(music : *mut sfMusic) -> c_float;
        pub fn sfMusic_getAttenuation(music : *mut sfMusic) -> c_float;
    }
}

pub mod sound {

    use libc::{c_float, c_void};

    use system::vector3::Vector3f;

    use ffi::audio::sound_status::sfSoundStatus;
    use ffi::audio::sound_buffer::sfSoundBuffer;
    use ffi::system::time::sfTime;
    use ffi::sfml_types::SfBool;

    pub struct sfSound {
        this : *mut c_void,
        this2 : *mut c_void
    }

    extern "C" {
        pub fn sfSound_create() -> *mut sfSound;
        pub fn sfSound_copy(sound : *mut sfSound) -> *mut sfSound;
        pub fn sfSound_destroy(sound : *mut sfSound) -> ();
        pub fn sfSound_play(sound : *mut sfSound) -> ();
        pub fn sfSound_pause(sound : *mut sfSound) -> ();
        pub fn sfSound_stop(sound : *mut sfSound) -> ();
        pub fn sfSound_setBuffer(sound : *mut sfSound, buffer : *mut sfSoundBuffer) -> (); // todo
        pub fn sfSound_getBuffer(sound : *mut sfSound) -> *mut sfSoundBuffer; // todo
        pub fn sfSound_setLoop(sound : *mut sfSound, lloop : SfBool) -> ();
        pub fn sfSound_getLoop(sound : *mut sfSound) -> SfBool;
        pub fn sfSound_getStatus(sound : *mut sfSound) -> sfSoundStatus;
        pub fn sfSound_setPitch(sound : *mut sfSound, pitch : c_float) -> ();
        pub fn sfSound_setVolume(sound : *mut sfSound, volume : c_float) -> ();
        pub fn sfSound_setPosition(sound : *mut sfSound, position : Vector3f) -> ();
        pub fn sfSound_setRelativeToListener(sound : *mut sfSound, relative : SfBool) -> ();
        pub fn sfSound_setMinDistance(sound : *mut sfSound, distance : c_float) -> ();
        pub fn sfSound_setAttenuation(sound : *mut sfSound, attenuation : c_float) -> ();
        pub fn sfSound_setPlayingOffset(sound : *mut sfSound, timeOffset : sfTime) -> ();
        pub fn sfSound_getPitch(sound : *mut sfSound) -> c_float;
        pub fn sfSound_getVolume(sound : *mut sfSound) -> c_float;
        pub fn sfSound_getPosition(sound : *mut sfSound) -> Vector3f;
        pub fn sfSound_isRelativeToListener(sound : *mut sfSound) -> SfBool;
        pub fn sfSound_getMinDistance(sound : *mut sfSound) -> c_float;
        pub fn sfSound_getAttenuation(sound : *mut sfSound) -> c_float;
        pub fn sfSound_getPlayingOffset(sound : *mut sfSound) -> sfTime;
    }
}

pub mod sound_buffer {
    use libc::{size_t, c_void, c_uint, c_char};

    use ffi::system::time::sfTime;
    use ffi::sfml_types::SfBool;

    pub struct sfSoundBuffer {
        this : *mut c_void
    }

    extern "C" {
        pub fn sfSoundBuffer_createFromFile(filename : *mut c_char) -> *mut sfSoundBuffer;
        pub fn sfSoundBuffer_copy(soundBuffer : *mut sfSoundBuffer) -> *mut sfSoundBuffer;
        pub fn sfSoundBuffer_destroy(soundBuffer : *mut sfSoundBuffer) -> ();
        pub fn sfSoundBuffer_saveToFile(soundBuffer : *mut sfSoundBuffer, filename : *mut c_char) -> SfBool;
       // fn sfSoundBuffer_getSamples(soundBuffer : *mut sfSoundBuffer) -> *mut i16;
        pub fn sfSoundBuffer_getSampleCount(soundBuffer : *mut sfSoundBuffer) -> size_t;
        pub fn sfSoundBuffer_getChannelCount(soundBuffer : *mut sfSoundBuffer) -> c_uint;
        pub fn sfSoundBuffer_getDuration(soundBuffer : *mut sfSoundBuffer) -> sfTime;
        pub fn sfSoundBuffer_getSampleRate(soundBuffer : *mut sfSoundBuffer) -> c_uint;
    }
}

pub mod sound_buffer_recorder {
    use libc::{c_uint, c_void};

    use ffi::audio::sound_buffer::sfSoundBuffer;
    use ffi::sfml_types::SfBool;

    pub struct sfSoundBufferRecorder {
        this : *mut c_void
    }

    extern "C" {
        pub fn sfSoundBufferRecorder_create() -> *mut sfSoundBufferRecorder;
        pub fn sfSoundBufferRecorder_destroy(soundBufferRecorder : *mut sfSoundBufferRecorder) -> ();
        pub fn sfSoundBufferRecorder_start(soundBufferRecorder : *mut sfSoundBufferRecorder, sampleRate : c_uint) -> ();
        pub fn sfSoundBufferRecorder_stop(soundBufferRecorder : *mut sfSoundBufferRecorder) -> ();
        pub fn sfSoundBufferRecorder_getSampleRate(soundBufferRecorder : *mut sfSoundBufferRecorder) -> c_uint;
        pub fn sfSoundBufferRecorder_getBuffer(soundBufferRecorder : *mut sfSoundBufferRecorder) -> *mut sfSoundBuffer;
        pub fn sfSoundRecorder_isAvailable() -> SfBool;
    }
}

pub mod sound_status {
    use libc::c_int;

    pub type sfSoundStatus = c_int;
    pub static SFSTOPPED:   sfSoundStatus = 0;
    pub static SFPAUSED:    sfSoundStatus = 1;
    pub static SFPLAYING:   sfSoundStatus = 2;
}
