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

use system::raw_conv::{FromRaw, Raw};
use csfml_graphics_sys::sfPrimitiveType;

/// Types of primitives that a `VertexArray` can render.
///
/// Points and lines have no area, therefore their thickness will always be 1 pixel,
/// regardless the current transform and view.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    /// List of individual points.
    Points = 0,
    /// List of individual lines.
    Lines = 1,
    /// List of connected lines, a point uses the previous point to form a line.
    LineStrip = 2,
    /// List of individual triangles.
    Triangles = 3,
    /// List of connected triangles, a point uses the two previous points to form a triangle.
    TriangleStrip = 4,
    /// List of connected triangles, a point uses the common center
    /// and the previous point to form a triangle.
    TriangleFan = 5,
    /// List of individual quads (deprecated, don't work with OpenGL ES)
    Quads = 6,
}

impl Raw for PrimitiveType {
    type Raw = sfPrimitiveType;

    fn raw(&self) -> sfPrimitiveType {
        unsafe { ::std::mem::transmute(*self) }
    }
}

impl FromRaw for PrimitiveType {
    type RawFrom = sfPrimitiveType;

    unsafe fn from_raw(raw: sfPrimitiveType) -> Self {
        ::std::mem::transmute(raw)
    }
}
