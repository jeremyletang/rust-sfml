//! GLSL types.

use crate::{
    ffi,
    graphics::Color,
    system::{Vector2, Vector3},
};

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

impl From<IVec3> for ffi::sfGlslIvec3 {
    fn from(src: IVec3) -> Self {
        Self {
            x: src.x,
            y: src.y,
            z: src.z,
        }
    }
}

impl From<BVec2> for ffi::sfGlslBvec2 {
    fn from(src: BVec2) -> Self {
        use crate::sf_bool_ext::SfBoolExt;
        Self {
            x: SfBoolExt::from_bool(src.x),
            y: SfBoolExt::from_bool(src.y),
        }
    }
}

impl From<BVec3> for ffi::sfGlslBvec3 {
    fn from(src: BVec3) -> Self {
        use crate::sf_bool_ext::SfBoolExt;
        Self {
            x: SfBoolExt::from_bool(src.x),
            y: SfBoolExt::from_bool(src.y),
            z: SfBoolExt::from_bool(src.z),
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

impl From<BVec4> for ffi::sfGlslBvec4 {
    fn from(src: BVec4) -> Self {
        use crate::sf_bool_ext::SfBoolExt;
        Self {
            x: SfBoolExt::from_bool(src.x),
            y: SfBoolExt::from_bool(src.y),
            z: SfBoolExt::from_bool(src.z),
            w: SfBoolExt::from_bool(src.w),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// GLSL `mat3` type.
pub struct Mat3(pub [f32; 9]);

impl From<crate::graphics::Transform> for Mat3 {
    fn from(src: crate::graphics::Transform) -> Self {
        let src = src.matrix;
        let mut dest = [0.0; 9];
        dest[0] = src[0];
        dest[1] = src[1];
        dest[2] = src[3];
        dest[3] = src[4];
        dest[4] = src[5];
        dest[5] = src[7];
        dest[6] = src[12];
        dest[7] = src[13];
        dest[8] = src[15];
        Mat3(dest)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// GLSL `mat4` type.
pub struct Mat4(pub [f32; 16]);

impl From<crate::graphics::Transform> for Mat4 {
    fn from(src: crate::graphics::Transform) -> Self {
        Mat4(*src.get_matrix())
    }
}
