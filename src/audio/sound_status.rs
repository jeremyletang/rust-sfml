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

//! Sound and musics statues

use std::libc::c_int;

use ffi = ffi::audio::sound_status;

/// Enumeration of statuses for sounds and musics
#[deriving(Eq, Ord)]
#[repr(C)]
pub enum Status {
    /// Sound is not playing.
    Stopped =   ffi::SFSTOPPED as c_int,
    /// Sound is paused.
    Paused =    ffi::SFPAUSED as c_int,
    /// Sound is playing.
    Playing =   ffi::SFPLAYING as c_int
}