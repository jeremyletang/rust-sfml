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

//! Upcast trait.

/// A type which is upcastable to some parent type.
pub trait Upcast<T> {
	/// Upcast to the parent type.
	fn upcast(&self) -> &T;
	/// Upcast to the parent type, mutably.
	fn upcast_mut(&mut self) -> &mut T;
}

impl<T> Upcast<T> for T {
    fn upcast(&self) -> &T { self }
    fn upcast_mut(&mut self) -> &mut T { self }
}

macro_rules! upcast {
    ($child:ty, $field:ident => $parent:ty) => (
        impl Upcast<$parent> for $child {
            fn upcast(&self) -> &$parent { &self.$field }
            fn upcast_mut(&mut self) -> &mut $parent { &mut self.$field }
        }
    );
    ($child:ty => $parent:ty => $grandparent:ty) => (
        impl Upcast<$grandparent> for $child {
            fn upcast(&self) -> &$grandparent { Upcast::<$parent>::upcast(self).upcast() }
            fn upcast_mut(&mut self) -> &mut $grandparent { Upcast::<$parent>::upcast_mut(self).upcast_mut() }
        }
    )
}
