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
    
    pub use std::libc::c_int;

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

    use std::libc::{c_void, c_uint, c_float, c_char};

    use system::vector3::Vector3f;

    use ffi::system::time::sfTime;
    use ffi::audio::sound_status::sfSoundStatus;
    use ffi::sfml_types::SfBool;

    pub struct sfMusic {
        This :  *c_void,
        This1 : *c_void
    }

    extern "C" {
        pub fn sfMusic_createFromFile(filename : *c_char) -> *sfMusic;
        // sfMusic* sfMusic_createFromMemory(const void* data, size_t sizeInBytes);
        // sfMusic* sfMusic_createFromStream(sfInputStream* stream);
        pub fn sfMusic_destroy(music : *sfMusic) -> ();
        pub fn sfMusic_setLoop(music : *sfMusic, lloop : SfBool) -> ();
        pub fn sfMusic_getLoop(music : *sfMusic) -> SfBool;
        pub fn sfMusic_getDuration(music : *sfMusic) -> sfTime;
        pub fn sfMusic_play(music : *sfMusic) -> ();
        pub fn sfMusic_pause(music : *sfMusic) -> ();
        pub fn sfMusic_stop(music : *sfMusic) -> ();
        pub fn sfMusic_getChannelCount(music : *sfMusic) -> c_uint;
        pub fn sfMusic_getSampleRate(music : *sfMusic) -> c_uint;
        pub fn sfMusic_getStatus(music : *sfMusic) -> sfSoundStatus;
        pub fn sfMusic_getPlayingOffset(music : *sfMusic) -> sfTime;
        pub fn sfMusic_setPitch(music : *sfMusic, pitch : c_float) -> ();
        pub fn sfMusic_setVolume(music : *sfMusic, volume : c_float) -> ();
        pub fn sfMusic_setPosition(music : *sfMusic, position : Vector3f) -> ();
        pub fn sfMusic_setRelativeToListener(music : *sfMusic, relative : SfBool) -> ();
        pub fn sfMusic_setMinDistance(music : *sfMusic, distance : c_float) -> ();
        pub fn sfMusic_setAttenuation(music : *sfMusic, attenuation : c_float) -> ();
        pub fn sfMusic_setPlayingOffset(music : *sfMusic, timeOffset : sfTime) -> ();
        pub fn sfMusic_getPitch(music : *sfMusic) -> c_float;
        pub fn sfMusic_getVolume(music : *sfMusic) -> c_float;
        pub fn sfMusic_getPosition(music : *sfMusic) -> Vector3f;
        pub fn sfMusic_isRelativeToListener(music : *sfMusic) -> SfBool;
        pub fn sfMusic_getMinDistance(music : *sfMusic) -> c_float;
        pub fn sfMusic_getAttenuation(music : *sfMusic) -> c_float;
    }
}

pub mod sound {
    
    use std::libc::{c_float, c_void};

    use system::vector3::Vector3f;

    use ffi::audio::sound_status::sfSoundStatus; 
    use ffi::audio::sound_buffer::sfSoundBuffer;
    use ffi::system::time::sfTime;
    use ffi::sfml_types::SfBool;

    pub struct sfSound {
        This : *c_void,
        This2 : *c_void
    }
    
    extern "C" {
        pub fn sfSound_create() -> *sfSound;
        pub fn sfSound_copy(sound : *sfSound) -> *sfSound;
        pub fn sfSound_destroy(sound : *sfSound) -> ();
        pub fn sfSound_play(sound : *sfSound) -> ();
        pub fn sfSound_pause(sound : *sfSound) -> ();
        pub fn sfSound_stop(sound : *sfSound) -> ();
        pub fn sfSound_setBuffer(sound : *sfSound, buffer : *sfSoundBuffer) -> (); // todo
        pub fn sfSound_getBuffer(sound : *sfSound) -> *sfSoundBuffer; // todo
        pub fn sfSound_setLoop(sound : *sfSound, lloop : SfBool) -> ();
        pub fn sfSound_getLoop(sound : *sfSound) -> SfBool;
        pub fn sfSound_getStatus(sound : *sfSound) -> sfSoundStatus;
        pub fn sfSound_setPitch(sound : *sfSound, pitch : c_float) -> ();
        pub fn sfSound_setVolume(sound : *sfSound, volume : c_float) -> ();
        pub fn sfSound_setPosition(sound : *sfSound, position : Vector3f) -> ();
        pub fn sfSound_setRelativeToListener(sound : *sfSound, relative : SfBool) -> ();
        pub fn sfSound_setMinDistance(sound : *sfSound, distance : c_float) -> ();
        pub fn sfSound_setAttenuation(sound : *sfSound, attenuation : c_float) -> ();
        pub fn sfSound_setPlayingOffset(sound : *sfSound, timeOffset : sfTime) -> ();
        pub fn sfSound_getPitch(sound : *sfSound) -> c_float;
        pub fn sfSound_getVolume(sound : *sfSound) -> c_float;
        pub fn sfSound_getPosition(sound : *sfSound) -> Vector3f;
        pub fn sfSound_isRelativeToListener(sound : *sfSound) -> SfBool;
        pub fn sfSound_getMinDistance(sound : *sfSound) -> c_float;
        pub fn sfSound_getAttenuation(sound : *sfSound) -> c_float;
        pub fn sfSound_getPlayingOffset(sound : *sfSound) -> sfTime;
    }
}

pub mod sound_buffer {
    
    use std::libc::{size_t, c_void, c_uint, c_char};
    
    use ffi::system::time::sfTime;
    use ffi::sfml_types::SfBool;

    pub struct sfSoundBuffer {
        This : *c_void
    }

    extern "C" {
        pub fn sfSoundBuffer_createFromFile(filename : *c_char) -> *sfSoundBuffer;
        pub fn sfSoundBuffer_copy(soundBuffer : *sfSoundBuffer) -> *sfSoundBuffer;
        pub fn sfSoundBuffer_destroy(soundBuffer : *sfSoundBuffer) -> ();
        pub fn sfSoundBuffer_saveToFile(soundBuffer : *sfSoundBuffer, filename : *c_char) -> SfBool;
       // fn sfSoundBuffer_getSamples(soundBuffer : *sfSoundBuffer) -> *i16;
        pub fn sfSoundBuffer_getSampleCount(soundBuffer : *sfSoundBuffer) -> size_t;
        pub fn sfSoundBuffer_getChannelCount(soundBuffer : *sfSoundBuffer) -> c_uint;
        pub fn sfSoundBuffer_getDuration(soundBuffer : *sfSoundBuffer) -> sfTime;
        pub fn sfSoundBuffer_getSampleRate(soundBuffer : *sfSoundBuffer) -> c_uint;
    }
}

pub mod sound_buffer_recorder {

    use std::libc::{c_uint, c_void};
    
    use ffi::audio::sound_buffer::sfSoundBuffer;
    use ffi::sfml_types::SfBool;
    
    pub struct sfSoundBufferRecorder {
        This : *c_void
    }

    extern "C" {
        pub fn sfSoundBufferRecorder_create() -> *sfSoundBufferRecorder;
        pub fn sfSoundBufferRecorder_destroy(soundBufferRecorder : *sfSoundBufferRecorder) -> ();
        pub fn sfSoundBufferRecorder_start(soundBufferRecorder : *sfSoundBufferRecorder, sampleRate : c_uint) -> ();
        pub fn sfSoundBufferRecorder_stop(soundBufferRecorder : *sfSoundBufferRecorder) -> ();
        pub fn sfSoundBufferRecorder_getSampleRate(soundBufferRecorder : *sfSoundBufferRecorder) -> c_uint;
        pub fn sfSoundBufferRecorder_getBuffer(soundBufferRecorder : *sfSoundBufferRecorder) -> *sfSoundBuffer;
        pub fn sfSoundRecorder_isAvailable() -> SfBool;
    }
}

pub mod sound_status {

    use std::libc::c_int;
    
    pub type sfSoundStatus = c_int;
    pub static SFSTOPPED:   sfSoundStatus = 0;
    pub static SFPAUSED:    sfSoundStatus = 1;
    pub static SFPLAYING:   sfSoundStatus = 2;
}
