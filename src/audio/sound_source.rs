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

//! Base trait for sounds source (music and sounds)
#![allow(missing_docs)]

use system::Vector3f;

pub trait SoundSource {
    fn set_pitch(&mut self, pitch: f32);
    fn set_volume(&mut self, volume: f32);
    fn set_position3f(&mut self, x: f32, y: f32, z: f32);
    fn set_position(&mut self, position: &Vector3f);
    fn set_relative_to_listener(&mut self, relative: bool);
    fn set_min_distance(&mut self, distance: f32);
    fn set_attenuation (&mut self, attenuation: f32);
    fn get_pitch(&self) -> f32;
    fn get_volume(&self) -> f32;
    fn get_position(&self) -> Vector3f;
    fn is_relative_to_listener(&self) -> bool;
    fn get_min_distance (&self) -> f32;
    fn get_attenuation(&self) -> f32;
}