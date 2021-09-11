//! Available blending modes for drawing

use crate::ffi;
pub use ffi::{BlendEquation as Equation, BlendFactor as Factor, BlendMode};

impl Default for BlendMode {
    /// Default blending mode is alpha blending.
    fn default() -> Self {
        Self::ALPHA
    }
}

impl BlendMode {
    /// "Alpha" blend mode
    pub const ALPHA: Self = BlendMode {
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
}
