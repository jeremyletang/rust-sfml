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

//! Conversion between the raw and wrapped forms of SFML types.

/// A type that has a raw representation that can be acquired through `&self`.
pub trait Raw {
    /// The raw representation of this type.
    type Raw;
    /// Acquires the raw representation of this type through `&self`.
    fn raw(&self) -> Self::Raw;
}

/// A type that has a raw representation that can be acquired through `&mut self`.
pub trait RawMut {
    /// The raw representation of this type.
    type RawMut;
    /// Acquires the raw representation of this type through `&mut self`.
    fn raw_mut(&mut self) -> Self::RawMut;
}

/// A type that can be created from its raw representation.
pub trait FromRaw {
    /// The raw representation of this type.
    type RawFrom;
    /// Creates `Self` from its raw representation.
    ///
    /// Doing this is not always safe, so this function is `unsafe`.
    unsafe fn from_raw(raw: Self::RawFrom) -> Self;
}
