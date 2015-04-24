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

use graphics::Transform;
use system::vector2::Vector2f;
use std::cell::Cell;
use std::default::Default;

/// The data used to implement Transformable.
#[derive(Clone, Debug)]
pub struct TransformableData {
	origin: Vector2f,
	position: Vector2f,
	rotation: f32,
	scale: Vector2f,
	transform: Cell<Option<Transform>>,
	inverse: Cell<Option<Transform>>
}

impl TransformableData {
	/// Create a new, default TransformableData.
	pub fn new() -> TransformableData {
		TransformableData {
			origin: Vector2f::new(0., 0.),
			position: Vector2f::new(0., 0.),
			rotation: 0.,
			scale: Vector2f::new(0., 0.),
			transform: Cell::new(None),
			inverse: Cell::new(None)
		}
	}
}

impl Default for TransformableData {
	fn default() -> TransformableData {
		TransformableData::new()
	}
}

fn mark_dirty(data: &mut TransformableData) {
	data.transform.set(None);
	data.inverse.set(None);
}

trait Custom<T> {
	/// Inner access.
	fn _data(&self) -> &T;
	/// Mutable inner access.
	fn _data_mut(&mut self) -> &mut T;
}

impl Custom<TransformableData> for TransformableData {
	fn _data(&self) -> &TransformableData { self }
	fn _data_mut(&mut self) -> &mut TransformableData { self }
}

/// Trait that can be applied to transformable objects.
///
/// User types may implement this trait by embedding a `TransformableData` item
/// and implementing the `_data` and `_data_mut` methods to return it.
pub trait Transformable {
	/// Get the position of the transformable
	fn get_position(&self) -> Vector2f;
	
    /// Get the orientation of the transformable
    ///
    /// The rotation is always in the range [0, 360].
	fn get_rotation(&self) -> f32;
	
    /// Get the current scale factors of the transformable
	fn get_scale(&self) -> Vector2f;
	
	/// Get the local origin of the transformable
	fn get_origin(&self) -> Vector2f;

    /// Set the position of a transformable
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a transformable Transformable object is (0, 0).
	fn set_position(&mut self, position: Vector2f);

	/// Set the orientation of a transformable
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a transformable Transformable object is 0.
	fn set_rotation(&mut self, angle: f32);
	
    /// Set the scale factors of a transformable
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a transformable Transformable object is (1, 1).
	fn set_scale(&mut self, scale: Vector2f);

    /// Set the local origin of a transformable
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a transformable Transformable object is (0, 0).
	fn set_origin(&mut self, origin: Vector2f);

    /// Move a transformable by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike set_position which overwrites it.
	fn move_(&mut self, offset: Vector2f);

    /// Rotate a transformable
    ///
    /// This function adds to the current rotation of the object,
    /// unlike set_rotation which overwrites it.
	fn rotate(&mut self, angle: f32);

    /// Scale a transformable
    ///
    /// This function multiplies the current scale of the object,
    /// unlike set_scale which overwrites it.
	fn scale(&mut self, factors: Vector2f);

    /// Get the combined transform of the transformable, including the position,
	/// rotation, scale, and origin.
	fn get_transform(&self) -> Transform;
	
	/// Get the inverse of the combined transform of the transformable.
	fn get_inverse_transform(&self) -> Transform;
}

impl Transformable for Custom<TransformableData> {
	/// Get the position of the transformable
	fn get_position(&self) -> Vector2f {
		self._data().position
	}
	
    /// Get the orientation of the transformable
    ///
    /// The rotation is always in the range [0, 360].
	fn get_rotation(&self) -> f32 {
		self._data().rotation
	}
	
    /// Get the current scale factors of the transformable
	fn get_scale(&self) -> Vector2f {
		self._data().scale
	}
	
	/// Get the local origin of the transformable
	fn get_origin(&self) -> Vector2f {
		self._data().origin
	}

    /// Set the position of a transformable
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a transformable Transformable object is (0, 0).
	fn set_position(&mut self, position: Vector2f) {
		self._data_mut().position = position;
		mark_dirty(self._data_mut());
	}

	/// Set the orientation of a transformable
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a transformable Transformable object is 0.
	fn set_rotation(&mut self, angle: f32) {
		let angle = angle % 360.;
		self._data_mut().rotation = if angle < 0. {
			angle + 360.
		} else {
			angle
		};
		mark_dirty(self._data_mut());
	}
	
    /// Set the scale factors of a transformable
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a transformable Transformable object is (1, 1).
	fn set_scale(&mut self, scale: Vector2f) {
		self._data_mut().scale = scale;
		mark_dirty(self._data_mut());
	}

    /// Set the local origin of a transformable
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a transformable Transformable object is (0, 0).
	fn set_origin(&mut self, origin: Vector2f) {
		self._data_mut().origin = origin;
		mark_dirty(self._data_mut());
	}

    /// Move a transformable by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike set_position which overwrites it.
	fn move_(&mut self, offset: Vector2f) {
		self._data_mut().position = self._data().position + offset;
		mark_dirty(self._data_mut());
	}

    /// Rotate a transformable
    ///
    /// This function adds to the current rotation of the object,
    /// unlike set_rotation which overwrites it.
	fn rotate(&mut self, angle: f32) {
		let angle = self.get_rotation() + angle;
		self.set_rotation(angle);
		mark_dirty(self._data_mut());
	}

    /// Scale a transformable
    ///
    /// This function multiplies the current scale of the object,
    /// unlike set_scale which overwrites it.
	fn scale(&mut self, factors: Vector2f) {
		let scale = self._data().scale;
		self._data_mut().scale = Vector2f {
			x: scale.x * factors.x,
			y: scale.y * factors.y
		};
		mark_dirty(self._data_mut());
	}

    /// Get the combined transform of the transformable, including the position,
	/// rotation, scale, and origin.
	fn get_transform(&self) -> Transform {
		if let Some(t) = self._data().transform.get() {
			t
		} else {
			// [SFML sourced]
			let angle  = -self._data().rotation * 3.141592654 / 180.;
			let cosine = angle.cos();
			let sine   = angle.sin();
			let scale  = self._data().scale;
			let sxc    = scale.x * cosine;
			let syc    = scale.y * cosine;
			let sxs    = scale.x * sine;
			let sys    = scale.y * sine;
			let origin = self._data().origin;
			let pos    = self._data().position;
			let tx     = -origin.x * sxc - origin.y * sys + pos.x;
			let ty     =  origin.x * sxs - origin.y * syc + pos.y;

			let result = Transform::new(sxc, sys, tx,
									    -sxs, syc, ty,
									    0., 0., 1.);
			self._data().transform.set(Some(result));
			result
		}
	}
	
	/// Get the inverse of the combined transform of the transformable.
	fn get_inverse_transform(&self) -> Transform {
		if let Some(t) = self._data().inverse.get() {
			t
		} else {
			let result = self.get_transform().get_inverse();
			self._data().inverse.set(Some(result));
			result
		}
	}
}
