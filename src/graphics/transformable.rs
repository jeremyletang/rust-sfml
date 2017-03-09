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

use graphics::Transform;
use system::Vector2f;

/// Decomposed transform defined by a position, a rotation and a scale.
pub trait Transformable {
    /// Sets the position of the object.
    ///
    /// This function completely overwrites the previous position.
    /// See the move function to apply an offset based on the previous position instead.
    /// The default position of a transformable object is (0, 0).
    fn set_position(&mut self, position: &Vector2f);
    /// Sets the position of the object.
    ///
    /// This function completely overwrites the previous position.
    /// See the move function to apply an offset based on the previous position instead.
    /// The default position of a transformable object is (0, 0).
    fn set_position2f(&mut self, x: f32, y: f32);
    /// Set the orientation of the object.
    ///
    /// This function completely overwrites the previous rotation.
    /// See the rotate function to add an angle based on the previous rotation instead.
    /// The default rotation of a transformable object is 0.
    fn set_rotation(&mut self, angle: f32);
    /// Sets the scale factors of the object.
    ///
    /// This function completely overwrites the previous scale.
    /// See the scale function to add a factor based on the previous scale instead.
    /// The default scale of a transformable object is (1, 1).
    fn set_scale(&mut self, scale: &Vector2f);
    /// Sets the scale factors of the object.
    ///
    /// This function completely overwrites the previous scale.
    /// See the scale function to add a factor based on the previous scale instead.
    /// The default scale of a transformable object is (1, 1).
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32); 
    /// Sets the local origin of the object.
    ///
    /// The origin of an object defines the center point for all transformations
    /// (position, scale, rotation).
    /// The coordinates of this point must be relative to the top-left corner of the object,
    /// and ignore all transformations (position, scale, rotation).
    /// The default origin of a transformable object is (0, 0).
    fn set_origin(&mut self, origin: &Vector2f);
    /// Sets the local origin of the object.
    ///
    /// The origin of an object defines the center point for all transformations
    /// (position, scale, rotation).
    /// The coordinates of this point must be relative to the top-left corner of the object,
    /// and ignore all transformations (position, scale, rotation).
    /// The default origin of a transformable object is (0, 0).
    fn set_origin2f(&mut self, x: f32, y: f32);
    /// Gets the position of the object.
    fn get_position(&self) -> Vector2f;
    /// Gets the rotation of the object.
    ///
    /// The rotation is always in the range [0, 360].
    fn get_rotation(&self) -> f32;
    /// Gets the current scale of the object.
    fn get_scale(&self) -> Vector2f;
    /// Gets the local origin of the object.
    fn get_origin(&self) -> Vector2f;
    /// Moves the object by a given offset.
    ///
    /// This function adds to the current position of the object,
    /// unlike `set_position` which overwrites it.
    fn move_(&mut self, offset: &Vector2f);
    /// Moves the object by a given offset.
    ///
    /// This function adds to the current position of the object,
    /// unlike `set_position` which overwrites it.
    fn move2f(&mut self, offset_x: f32, offset_y: f32);
    /// Rotates the object.
    ///
    /// This function adds to the current rotation of the object, unlike `set_rotation`
    /// which overwrites it.
    fn rotate(&mut self, angle: f32);
    /// Scales the object.
    ///
    /// This function multiplies the current scale of the object, unlike `set_scale`
    /// which overwrites it.
    fn scale(&mut self, factors: &Vector2f);
    /// Scales the object.
    ///
    /// This function multiplies the current scale of the object, unlike `set_scale`
    /// which overwrites it.
    fn scale2f(&mut self, factor_x: f32, factor_y: f32);
    /// Gets the combined transform of the object.
    fn get_transform(&self) -> Transform;
    /// Gets the inverse combined transform of the object.
    fn get_inverse_transform(&self) -> Transform;
}
