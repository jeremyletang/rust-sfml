//! Available blending modes for drawing

use graphics::csfml_graphics_sys as ffi;

/// Blending modes for drawing.
///
/// `BlendMode` is a type that represents a blend mode.
///
/// A blend mode determines how the colors of an object you draw are mixed with the colors that
/// are already in the buffer.
///
/// The type is composed of 6 components, each of which has its own public field:
///
/// - Color Source Factor (`color_src_factor`)
/// - Color Destination Factor (`color_dst_factor`)
/// - Color Blend Equation (`color_equation`)
/// - Alpha Source Factor (`alpha_src_factor`)
/// - Alpha Destination Factor (`alpha_dst_factor`)
/// - Alpha Blend Equation (`alpha_equation`)
///
/// The source factor specifies how the pixel you are drawing contributes to the final color.
/// The destination factor specifies how the pixel already drawn in the buffer contributes to
/// the final color.
///
/// The color channels RGB (red, green, blue; simply referred to as color) and A
/// (alpha; the transparency) can be treated separately. This separation can be useful for
/// specific blend modes, but most often you won't need it and will simply treat the color as
/// a single unit.
///
/// The blend factors and equations correspond to their OpenGL equivalents.
/// In general, the color of the resulting pixel is calculated according to the following
/// formula (src is the color of the source pixel, dst the color of the destination pixel,
/// the other variables correspond to the public members, with the equations
/// being + or - operators):
///
/// ```ignore
/// dst.rgb = color_src_factor * src.rgb (color_equation) color_dst_factor * dst.rgb
/// dst.a   = alpha_src_factor * src.a   (alpha_equation) alpha_dst_factor * dst.a
/// ```
///
/// All factors and colors are represented as floating point numbers between 0 and 1.
/// Where necessary, the result is clamped to fit in that range.
///
/// In SFML, a blend mode can be specified every time you draw a `Drawable` object to
/// a render target. It is part of the `RenderStates` compound that is passed to
/// `RenderTarget::draw()`.
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
#[repr(C)]
pub struct BlendMode {
    /// Source blending factor for the color channels.
    pub color_src_factor: Factor,
    /// Destination blending factor for the color channels.
    pub color_dst_factor: Factor,
    /// Blending equation for the color channels.
    pub color_equation: Equation,
    /// Source blending factor for the alpha channel.
    pub alpha_src_factor: Factor,
    /// Destination blending factor for the alpha channel.
    pub alpha_dst_factor: Factor,
    /// Blending equation for the alpha channel.
    pub alpha_equation: Equation,
}

impl Default for BlendMode {
    /// Default blending mode is alpha blending.
    fn default() -> Self {
        ALPHA
    }
}

/// "Alpha" blend mode
pub const ALPHA: BlendMode = BlendMode {
    color_src_factor: Factor::SrcAlpha,
    color_dst_factor: Factor::OneMinusSrcAlpha,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::OneMinusSrcAlpha,
    alpha_equation: Equation::Add,
};

/// "Add" blend mode
pub const ADD: BlendMode = BlendMode {
    color_src_factor: Factor::SrcAlpha,
    color_dst_factor: Factor::One,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::One,
    alpha_equation: Equation::Add,
};

/// "Multiply" blend mode
pub const MULTIPLY: BlendMode = BlendMode {
    color_src_factor: Factor::DstColor,
    color_dst_factor: Factor::Zero,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::DstColor,
    alpha_dst_factor: Factor::Zero,
    alpha_equation: Equation::Add,
};

/// "None" blend mode
pub const NONE: BlendMode = BlendMode {
    color_src_factor: Factor::One,
    color_dst_factor: Factor::Zero,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::Zero,
    alpha_equation: Equation::Add,
};

/// Enumeration of the blending factors.
///
/// The factors are mapped directly to their OpenGL equivalents, specified by
/// `glBlendFunc()` or `glBlendFuncSeparate()`.
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
#[repr(u32)]
pub enum Factor {
    /// (0, 0, 0, 0)
    Zero = 0,
    /// (1, 1, 1, 1)
    One = 1,
    /// (src.r, src.g, src.b, src.a)
    SrcColor = 2,
    /// (1, 1, 1, 1) - (src.r, src.g, src.b, src.a)
    OneMinusSrcColor = 3,
    /// (dst.r, dst.g, dst.b, dst.a)
    DstColor = 4,
    /// (1, 1, 1, 1) - (dst.r, dst.g, dst.b, dst.a)
    OneMinusDstColor = 5,
    /// (src.a, src.a, src.a, src.a)
    SrcAlpha = 6,
    /// (1, 1, 1, 1) - (src.a, src.a, src.a, src.a)
    OneMinusSrcAlpha = 7,
    /// (dst.a, dst.a, dst.a, dst.a)
    DstAlpha = 8,
    /// (1, 1, 1, 1) - (dst.a, dst.a, dst.a, dst.a)
    OneMinusDstAlpha = 9,
}

/// Enumeration of the blending equations.
///
/// The equations are mapped directly to their OpenGL equivalents, specified by
/// `glBlendEquation()` or `glBlendEquationSeparate()`.
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
#[repr(u32)]
pub enum Equation {
    /// Pixel = Src * SrcFactor + Dst * DstFactor.
    Add = 0,
    /// Pixel = Src * SrcFactor - Dst * DstFactor.
    Subtract = 1,
    /// Pixel = Dst * DstFactor - Src * SrcFactor.
    ReverseSubtract = 2,
}

impl BlendMode {
    /// Create a new BlendMode
    pub fn new(col_src: Factor,
               col_dst: Factor,
               col_equ: Equation,
               alpha_src: Factor,
               alpha_dst: Factor,
               alpha_equ: Equation)
               -> Self {
        BlendMode {
            color_src_factor: col_src,
            color_dst_factor: col_dst,
            color_equation: col_equ,
            alpha_src_factor: alpha_src,
            alpha_dst_factor: alpha_dst,
            alpha_equation: alpha_equ,
        }
    }
    pub(super) fn raw(&self) -> ffi::sfBlendMode {
        unsafe { ::std::mem::transmute(*self) }
    }
}
