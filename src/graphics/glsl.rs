//! GLSL types.

use crate::{
    graphics::Color,
    system::{Vector2, Vector3},
};
use csfml_graphics_sys as ffi;

/// 2D float vector (`vec2` in GLSL).
pub type Vec2 = Vector2<f32>;
/// 2D int vector (`ivec2` in GLSL).
pub type IVec2 = Vector2<i32>;
/// 2D bool vector (`bvec2` in GLSL).
pub type BVec2 = Vector2<bool>;
/// 3D float vector (`vec3` in GLSL).
pub type Vec3 = Vector3<f32>;
/// 3D int vector (`ivec3` in GLSL).
pub type IVec3 = Vector3<i32>;
/// 3D bool vector (`bvec3` in GLSL).
pub type BVec3 = Vector3<bool>;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
/// GLSL `vec4` type.
pub struct Vec4 {
    /// `x` field.
    pub x: f32,
    /// `y` field.
    pub y: f32,
    /// `z` field.
    pub z: f32,
    /// `w` field.
    pub w: f32,
}

impl Vec4 {
    pub(super) fn raw(&self) -> ffi::sfGlslVec4 {
        ffi::sfGlslVec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl From<Color> for Vec4 {
    fn from(src: Color) -> Self {
        Self {
            x: f32::from(src.0.r) / 255.0,
            y: f32::from(src.0.g) / 255.0,
            z: f32::from(src.0.b) / 255.0,
            w: f32::from(src.0.a) / 255.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// GLSL `ivec4` type.
pub struct IVec4 {
    /// `x` field.
    pub x: i32,
    /// `y` field.
    pub y: i32,
    /// `z` field.
    pub z: i32,
    /// `w` field.
    pub w: i32,
}

impl IVec4 {
    pub(super) fn raw(&self) -> ffi::sfGlslIvec4 {
        ffi::sfGlslIvec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl From<Color> for IVec4 {
    fn from(src: Color) -> Self {
        Self {
            x: src.0.r.into(),
            y: src.0.g.into(),
            z: src.0.b.into(),
            w: src.0.a.into(),
        }
    }
}

impl Into<ffi::sfGlslIvec3> for IVec3 {
    fn into(self) -> ffi::sfGlslIvec3 {
        ffi::sfGlslIvec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Into<ffi::sfGlslBvec2> for BVec2 {
    fn into(self) -> ffi::sfGlslBvec2 {
        use crate::sf_bool_ext::SfBoolExt;
        ffi::sfGlslBvec2 {
            x: SfBoolExt::from_bool(self.x),
            y: SfBoolExt::from_bool(self.y),
        }
    }
}

impl Into<ffi::sfGlslBvec3> for BVec3 {
    fn into(self) -> ffi::sfGlslBvec3 {
        use crate::sf_bool_ext::SfBoolExt;
        ffi::sfGlslBvec3 {
            x: SfBoolExt::from_bool(self.x),
            y: SfBoolExt::from_bool(self.y),
            z: SfBoolExt::from_bool(self.z),
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// GLSL `bvec4` type.
pub struct BVec4 {
    /// `x` field.
    pub x: bool,
    /// `y` field.
    pub y: bool,
    /// `z` field.
    pub z: bool,
    /// `w` field.
    pub w: bool,
}

impl Into<ffi::sfGlslBvec4> for BVec4 {
    fn into(self) -> ffi::sfGlslBvec4 {
        use crate::sf_bool_ext::SfBoolExt;
        ffi::sfGlslBvec4 {
            x: SfBoolExt::from_bool(self.x),
            y: SfBoolExt::from_bool(self.y),
            z: SfBoolExt::from_bool(self.z),
            w: SfBoolExt::from_bool(self.w),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// GLSL `mat3` type.
pub struct Mat3(pub [f32; 9]);

impl From<crate::graphics::Transform> for Mat3 {
    fn from(src: crate::graphics::Transform) -> Self {
        Mat3(src.0.matrix)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// GLSL `mat4` type.
pub struct Mat4(pub [f32; 16]);

impl From<crate::graphics::Transform> for Mat4 {
    fn from(src: crate::graphics::Transform) -> Self {
        let mut mat = [0.0; 16];
        src.get_matrix(&mut mat);
        Mat4(mat)
    }
}
