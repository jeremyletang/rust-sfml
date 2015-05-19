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

/// Available factors for blend computations.
///
/// The factors are mapped directly to their OpenGL equivalents, specified by
/// `glBlendFunc()` or `glBlendFuncSeparate()`.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum BlendFactor {
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

/// Available equations for blend computations.
///
/// The equations are mapped directly to their OpenGL equivalents, specified by
/// `glBlendEquation()` or `glBlendEquationSeparate()`.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum BlendEquation {
	/// Pixel = Src * SrcFactor + Dst * DstFactor
	Add = 0,
	/// Pixel = Src * SrcFactor - Dst * DstFactor
	Subtract = 1,
}

/// A combination of factors and equations describing a blend mode.
///
/// A blend mode determines how the colors of an object you draw are mixed with
/// the colors that are already in the buffer. The source factor specifies how
/// the pixel you are drawing contributes to the final color. The destination
/// factor specifies how the pixel already drawn in the buffer contributes to
/// the final color.
///
/// The color channels RGB (red, green, blue; simply referred to as color) and
/// A (alpha; the transparency) can be treated separately. This separation can
/// be useful for specific blend modes, but most often you won't need it and
/// will simply treat the color as a single unit.
///
/// The blend factors and equations correspond to their OpenGL equivalents. In
/// general, the color of the resulting pixel is calculated according to the
/// following formula (`src` is the color of the source pixel, `dst` the color
/// of the destination pixel, the other variables correspond to the public
/// members, with the equations being + or - operators):
/// ```text
/// dst.rgb = colorSrcFactor * src.rgb (colorEquation) colorDstFactor * dst.rgb
/// dst.a   = alphaSrcFactor * src.a   (alphaEquation) alphaDstFactor * dst.a
/// ```
///
/// All factors and colors are represented as floating point numbers between 0
/// and 1. Where necessary, the result is clamped to fit in that range.
///
/// The most common blending modes are available through the `alpha()`,
/// `add()`, `multiply()`, and `none()` methods.
///
///
/// In SFML, a blend mode can be specified every time you draw a `Drawable`
/// object to a render target. It is part of the `RenderStates` compound that is
/// passed to `RenderTarget::draw()`.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct BlendMode {
	/// Source blending factor for the color channels.
	pub color_src_factor: BlendFactor,
	/// Destination blending factor for the color channels.
	pub color_dst_factor: BlendFactor,
	/// Blending equation for the color channels.
	pub color_equation: BlendEquation,
	/// Source blending factor for the alpha channel.
	pub alpha_src_factor: BlendFactor,
	/// Destination blending factor for the alpha channel.
	pub alpha_dst_factor: BlendFactor,
	/// Blending equation for the alpha channel.
	pub alpha_equation: BlendEquation
}

impl BlendMode {
	/// Create a new BlendMode with the given factors and equations, using the
	/// same factors and equations for the color and alpha channels.
	pub fn new(src_factor: BlendFactor, dst_factor: BlendFactor, equation: BlendEquation) -> BlendMode {
		BlendMode {
			color_src_factor: src_factor,
			color_dst_factor: dst_factor,
			color_equation: equation,
			alpha_src_factor: src_factor,
			alpha_dst_factor: dst_factor,
			alpha_equation: equation,
		}
	}

	/// Create a new BlendMode with the given factors and equations.
	pub fn new_all(color_src_factor: BlendFactor, color_dst_factor: BlendFactor, color_equation: BlendEquation,
			   alpha_src_factor: BlendFactor, alpha_dst_factor: BlendFactor, alpha_equation: BlendEquation) -> BlendMode {
		BlendMode {
			color_src_factor: color_src_factor,
			color_dst_factor: color_dst_factor,
			color_equation: color_equation,
			alpha_src_factor: alpha_src_factor,
			alpha_dst_factor: alpha_dst_factor,
			alpha_equation: alpha_equation,
		}
	}

	/// Get the predefined Alpha blend mode.
	pub fn alpha() -> BlendMode {
		BlendMode::new_all(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha, BlendEquation::Add,
					       BlendFactor::One, BlendFactor::OneMinusSrcAlpha, BlendEquation::Add)
	}
	/// Get the predefined Add blend mode.
	pub fn add() -> BlendMode {
		BlendMode::new_all(BlendFactor::SrcAlpha, BlendFactor::One, BlendEquation::Add,
					       BlendFactor::One, BlendFactor::One, BlendEquation::Add)
	}
	/// Get the predefined Multiply blend mode.
	pub fn multiply() -> BlendMode {
		BlendMode::new(BlendFactor::DstColor, BlendFactor::Zero, BlendEquation::Add)
	}
	/// Get the predefined None blend mode.
	pub fn none() -> BlendMode {
		BlendMode::new(BlendFactor::One, BlendFactor::Zero, BlendEquation::Add)
	}
}

impl Default for BlendMode {
	/// Returns `BlendMode::alpha()`.
	fn default() -> BlendMode {
		BlendMode::alpha()
	}
}
