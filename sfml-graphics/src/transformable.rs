// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! base transformable trait

use Transform;
use sfml::system::Vector2f;

#[allow(missing_docs)]
pub trait Transformable {
    fn set_position(&mut self, position: &Vector2f);
    fn set_position2f(&mut self, x: f32, y: f32);
    fn set_rotation(&mut self, angle: f32);
    fn set_scale(&mut self, scale: &Vector2f);
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32);
    fn set_origin(&mut self, origin: &Vector2f);
    fn set_origin2f(&mut self, x: f32, y: f32);
    fn get_position(&self) -> Vector2f;
    fn get_rotation(&self) -> f32;
    fn get_scale(&self) -> Vector2f;
    fn get_origin(&self) -> Vector2f;
    fn move_(&mut self, offset: &Vector2f);
    fn move2f(&mut self, offset_x: f32, offset_y: f32);
    fn rotate(&mut self, angle: f32);
    fn scale(&mut self, factors: &Vector2f);
    fn scale2f(&mut self, factor_x: f32, factor_y: f32);
    fn get_transform(&self) -> Transform;
    fn get_inverse_transform(&self) -> Transform;
}
