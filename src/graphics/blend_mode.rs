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

//! Available blending modes for drawing

#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum BlendFactor {
	Zero = 0,
	One = 1,
	SrcColor = 2,
	OneMinusSrcColor = 3,
	DstColor = 4,
	OneMinusDstColor = 5,
	SrcAlpha = 6,
	OneMinusSrcAlpha = 7,
	DstAlpha = 8,
	OneMinusDstAlpha = 9,
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum BlendEquation {
	Add = 0,
	Subtract = 1,
}

/// Available Blending modes for drawing.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct BlendMode {
	/// The source color factor
	pub color_src_factor: BlendFactor,
	/// The dest color factor
	pub color_dst_factor: BlendFactor,
	/// The color equation
	pub color_equation: BlendEquation,
	/// The source alpha factor
	pub alpha_src_factor: BlendFactor,
	/// The dest alpha factor
	pub alpha_dst_factor: BlendFactor,
	/// The alpha equation
	pub alpha_equation: BlendEquation
}

impl BlendMode {
	/// Create a new BlendMode with the given factors and equations.
	pub fn new(color_src_factor: BlendFactor, color_dst_factor: BlendFactor, color_equation: BlendEquation,
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
		BlendMode::new(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha, BlendEquation::Add,
					   BlendFactor::One, BlendFactor::OneMinusSrcAlpha, BlendEquation::Add)
	}
	/// Get the predefined Add blend mode.
	pub fn add() -> BlendMode {
		BlendMode::new(BlendFactor::SrcAlpha, BlendFactor::One, BlendEquation::Add,
					   BlendFactor::One, BlendFactor::One, BlendEquation::Add)
	}
	/// Get the predefined Multiply blend mode.
	pub fn multiply() -> BlendMode {
		BlendMode::new(BlendFactor::DstColor, BlendFactor::Zero, BlendEquation::Add,
					   BlendFactor::DstColor, BlendFactor::Zero, BlendEquation::Add)
	}
	/// Get the predefined None blend mode.
	pub fn none() -> BlendMode {
		BlendMode::new(BlendFactor::One, BlendFactor::Zero, BlendEquation::Add,
					   BlendFactor::One, BlendFactor::Zero, BlendEquation::Add)
	}
}
