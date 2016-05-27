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

/// A type that has a raw representation that can be acquired through `&self`.
pub trait Raw {
    type Raw;
    fn raw(&self) -> Self::Raw;
}

/// A type that has a raw representation that can be acquired through `&mut self`.
pub trait RawMut {
    type Raw;
    fn raw_mut(&mut self) -> Self::Raw;
}

/// A type that can be created from its raw representation.
pub trait FromRaw: Raw {
    fn from_raw(raw: Self::Raw) -> Self;
}

impl Raw for ::sfml_types::Vector3f {
    type Raw = ::csfml_system_sys::sfVector3f;

    fn raw(&self) -> Self::Raw {
        ::csfml_system_sys::sfVector3f {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl FromRaw for ::sfml_types::Vector3f {
    fn from_raw(raw: Self::Raw) -> Self {
        ::sfml_types::Vector3f {
            x: raw.x,
            y: raw.y,
            z: raw.z,
        }
    }
}

impl Raw for ::sfml_types::Vector2i {
    type Raw = ::csfml_system_sys::sfVector2i;

    fn raw(&self) -> Self::Raw {
        ::csfml_system_sys::sfVector2i {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromRaw for ::sfml_types::Vector2i {
    fn from_raw(raw: Self::Raw) -> Self {
        ::sfml_types::Vector2i {
            x: raw.x,
            y: raw.y,
        }
    }
}

impl Raw for ::sfml_types::Vector2u {
    type Raw = ::csfml_system_sys::sfVector2u;

    fn raw(&self) -> Self::Raw {
        ::csfml_system_sys::sfVector2u {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromRaw for ::sfml_types::Vector2u {
    fn from_raw(raw: Self::Raw) -> Self {
        ::sfml_types::Vector2u {
            x: raw.x,
            y: raw.y,
        }
    }
}

impl Raw for ::sfml_types::Vector2f {
    type Raw = ::csfml_system_sys::sfVector2f;

    fn raw(&self) -> Self::Raw {
        ::csfml_system_sys::sfVector2f {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromRaw for ::sfml_types::Vector2f {
    fn from_raw(raw: Self::Raw) -> Self {
        ::sfml_types::Vector2f {
            x: raw.x,
            y: raw.y,
        }
    }
}

impl Raw for ::sfml_types::IntRect {
    type Raw = ::csfml_graphics_sys::sfIntRect;

    fn raw(&self) -> Self::Raw {
        ::csfml_graphics_sys::sfIntRect {
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
}

impl FromRaw for ::sfml_types::IntRect {
    fn from_raw(raw: Self::Raw) -> Self {
        ::sfml_types::IntRect {
            top: raw.top,
            left: raw.left,
            width: raw.width,
            height: raw.height,
        }
    }
}

impl Raw for ::sfml_types::FloatRect {
    type Raw = ::csfml_graphics_sys::sfFloatRect;

    fn raw(&self) -> Self::Raw {
        ::csfml_graphics_sys::sfFloatRect {
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
}

impl FromRaw for ::sfml_types::FloatRect {
    fn from_raw(raw: Self::Raw) -> Self {
        ::sfml_types::FloatRect {
            top: raw.top,
            left: raw.left,
            width: raw.width,
            height: raw.height,
        }
    }
}
