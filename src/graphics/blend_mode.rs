//! Available blending modes for drawing

use crate::ffi::graphics as ffi;

/// Blending modes for drawing.
///
/// `BlendMode` is a type that represents a blend mode.
///
/// A blend mode determines how the colors of an object you draw are mixed with the colors that
/// are already in the buffer.
///
/// The type is composed of 6 components
///
/// - Color Source Factor
/// - Color Destination Factor
/// - Color Blend Equation
/// - Alpha Source Factor
/// - Alpha Destination Factor
/// - Alpha Blend Equation
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
/// dst.rgb = colorSrcFactor * src.rgb (colorEquation) colorDstFactor * dst.rgb
/// dst.a   = alphaSrcFactor * src.a   (alphaEquation) alphaDstFactor * dst.a
/// ```
///
/// All factors and colors are represented as floating point numbers between 0 and 1.
/// Where necessary, the result is clamped to fit in that range.
///
/// In SFML, a blend mode can be specified every time you draw a [`Drawable`] object to
/// a render target. It is part of the [`RenderStates`] compound that is passed to
/// [`RenderTarget::draw`].
///
/// [`Drawable`]: crate::graphics::Drawable
/// [`RenderStates`]: crate::graphics::RenderStates
/// [`RenderTarget::draw`]: crate::graphics::RenderTarget::draw
#[derive(Clone, Debug, Copy)]
#[repr(transparent)]
pub struct BlendMode(ffi::sfBlendMode);

impl Default for BlendMode {
    /// Default blending mode is alpha blending.
    fn default() -> Self {
        Self::ALPHA
    }
}

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
    /// Create a new `BlendMode`
    #[must_use]
    pub const fn new(
        col_src: Factor,
        col_dst: Factor,
        col_equ: Equation,
        alpha_src: Factor,
        alpha_dst: Factor,
        alpha_equ: Equation,
    ) -> Self {
        Self(ffi::sfBlendMode {
            colorSrcFactor: col_src as _,
            colorDstFactor: col_dst as _,
            colorEquation: col_equ as _,
            alphaSrcFactor: alpha_src as _,
            alphaDstFactor: alpha_dst as _,
            alphaEquation: alpha_equ as _,
        })
    }
    pub(super) const fn raw(&self) -> ffi::sfBlendMode {
        self.0
    }
    /// "Alpha" blend mode
    pub const ALPHA: Self = Self(ffi::sfBlendMode {
        colorSrcFactor: Factor::SrcAlpha as _,
        colorDstFactor: Factor::OneMinusSrcAlpha as _,
        colorEquation: Equation::Add as _,
        alphaSrcFactor: Factor::One as _,
        alphaDstFactor: Factor::OneMinusSrcAlpha as _,
        alphaEquation: Equation::Add as _,
    });

    /// "Add" blend mode
    pub const ADD: BlendMode = Self(ffi::sfBlendMode {
        colorSrcFactor: Factor::SrcAlpha as _,
        colorDstFactor: Factor::One as _,
        colorEquation: Equation::Add as _,
        alphaSrcFactor: Factor::One as _,
        alphaDstFactor: Factor::One as _,
        alphaEquation: Equation::Add as _,
    });

    /// "Multiply" blend mode
    pub const MULTIPLY: BlendMode = Self(ffi::sfBlendMode {
        colorSrcFactor: Factor::DstColor as _,
        colorDstFactor: Factor::Zero as _,
        colorEquation: Equation::Add as _,
        alphaSrcFactor: Factor::DstColor as _,
        alphaDstFactor: Factor::Zero as _,
        alphaEquation: Equation::Add as _,
    });

    /// "None" blend mode
    pub const NONE: BlendMode = Self(ffi::sfBlendMode {
        colorSrcFactor: Factor::One as _,
        colorDstFactor: Factor::Zero as _,
        colorEquation: Equation::Add as _,
        alphaSrcFactor: Factor::One as _,
        alphaDstFactor: Factor::Zero as _,
        alphaEquation: Equation::Add as _,
    });
}
