use crate::ffi::system::sfBuffer;
pub use crate::ffi::*;
use {
    super::system::sfInputStreamHelper,
    crate::{
        ffi::window::{Event as sfEvent, sfContextSettings, sfCursor, sfWindow, sfWindowHandle},
        graphics::{Color, Rect, RenderStates as sfRenderStates, Transform as sfTransform},
        window::VideoMode as sfVideoMode,
    },
};

decl_opaque! {
    pub(crate) sfCircleShape;
    pub(crate) sfConvexShape;
    pub(crate) sfShader;
    pub(crate) sfRectangleShape;
    pub(crate) sfCustomShape;
    // Kind of unnecessary, but required by sf_render*_drawShape(...)
    pub(crate) sfShape;
    pub(crate) sfSprite;
    pub(crate) sfText;
}

type sfTexture = crate::graphics::Texture;
type sfView = crate::graphics::View;
type sfFont = crate::graphics::Font;
type sfImage = crate::graphics::Image;
type sfRenderWindow = crate::graphics::RenderWindow;
type sfRenderTexture = crate::graphics::RenderTexture;
type sfVertexBuffer = crate::graphics::VertexBuffer;
pub(crate) type sfState = crate::window::window_enums::State;

/// Enumeration of the blending factors.
///
/// The factors are mapped directly to their OpenGL equivalents, specified by
/// `glBlendFunc()` or `glBlendFuncSeparate()`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendFactor {
    /// (0, 0, 0, 0)
    Zero,
    /// (1, 1, 1, 1)
    One,
    /// (src.r, src.g, src.b, src.a)
    SrcColor,
    /// (1, 1, 1, 1) - (src.r, src.g, src.b, src.a)
    OneMinusSrcColor,
    /// (dst.r, dst.g, dst.b, dst.a)
    DstColor,
    /// (1, 1, 1, 1) - (dst.r, dst.g, dst.b, dst.a)
    OneMinusDstColor,
    /// (src.a, src.a, src.a, src.a)
    SrcAlpha,
    /// (1, 1, 1, 1) - (src.a, src.a, src.a, src.a)
    OneMinusSrcAlpha,
    /// (dst.a, dst.a, dst.a, dst.a)
    DstAlpha,
    /// (1, 1, 1, 1) - (dst.a, dst.a, dst.a, dst.a)
    OneMinusDstAlpha,
}

/// Enumeration of the blending equations.
///
/// The equations are mapped directly to their OpenGL equivalents, specified by
/// `glBlendEquation()` or `glBlendEquationSeparate()`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendEquation {
    /// `pixel = src * src_factor + dst * dst_factor`
    Add,
    /// `pixel = src * src_factor - dst * dst_factor`
    Subtract,
    /// `pixel = dst * dst_factor - src * src_factor`
    ReverseSubtract,
    /// `pixel = min(Dst, Src)`
    Min,
    /// `pixel = max(Dst, Src)`
    Max,
}

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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlendMode {
    /// Source blending factor for the color channels
    pub color_src_factor: BlendFactor,
    /// Destination blending factor for the color channels
    pub color_dst_factor: BlendFactor,
    /// Blending equation for the color channels
    pub color_equation: BlendEquation,
    /// Source blending factor for the alpha channel
    pub alpha_src_factor: BlendFactor,
    /// Destination blending factor for the alpha channel
    pub alpha_dst_factor: BlendFactor,
    /// Blending equation for the alpha channel
    pub alpha_equation: BlendEquation,
}

/// Enumeration of the stencil test comparisons that can be performed
/// The comparisons are mapped directly to their OpenGL equivalents,
/// specified by `glStencilFunc()`.
#[allow(dead_code)]
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum StencilComparison {
    /// The stencil test never passes    
    Never,
    /// The stencil test passes if the new value is less than the value in the stencil buffer    
    Less,
    /// The stencil test passes if the new value is less than or equal to the value in the stencil buffer    
    LessEqual,
    /// The stencil test passes if the new value is greater than the value in the stencil buffer    
    Greater,
    /// The stencil test passes if the new value is greater than or equal to the value in the stencil buffer    
    GreaterEqual,
    /// The stencil test passes if the new value is strictly equal to the value in the stencil buffer    
    Equal,
    /// The stencil test passes if the new value is strictly unequal to the value in the stencil buffer    
    NotEqual,
    /// The stencil test always passes
    #[default]
    Always,
}

/// Enumeration of the stencil buffer update operations
///
/// The update operations are mapped directly to their OpenGL equivalents,
/// specified by `glStencilOp()`.
#[allow(dead_code)]
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum StencilUpdateOperation {
    /// If the stencil test passes, the value in the stencil buffer is not modified
    #[default]
    Keep,
    /// If the stencil test passes, the value in the stencil buffer is set to zero    
    Zero,
    /// If the stencil test passes, the value in the stencil buffer is set to the new value    
    Replace,
    /// If the stencil test passes, the value in the stencil buffer is incremented and if required clamped    
    Increment,
    /// If the stencil test passes, the value in the stencil buffer is decremented and if required clamped    
    Decrement,
    /// If the stencil test passes, the value in the stencil buffer is bitwise inverted    
    Invert,
}

/// Stencil value type (also used as a mask)
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct StencilValue {
    /// The stored stencil value    
    value: u32,
}

/// Stencil modes for drawing
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StencilMode {
    /// The comparison we're performing the stencil test with    
    comparison: StencilComparison,
    /// The update operation to perform if the stencil test passes     
    update_operation: StencilUpdateOperation,
    /// The reference value we're performing the stencil test with      
    reference: StencilValue,
    /// The mask to apply to both the reference value and the value in the stencil buffer     
    mask: StencilValue,
    /// Whether we should update the color buffer in addition to the stencil buffer     
    only: bool,
}

impl StencilMode {
    pub const DEFAULT: Self = Self {
        comparison: StencilComparison::Always,
        update_operation: StencilUpdateOperation::Keep,
        reference: StencilValue { value: 0 },
        mask: StencilValue { value: u32::MAX },
        only: false,
    };
}

impl Default for StencilMode {
    fn default() -> Self {
        Self::DEFAULT
    }
}
/// Types of texture coordinates that can be used for rendering.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateType {
    /// Texture coordinates in range [0 .. 1].
    sfCoordinateTypeNormalized,
    /// Texture coordinates in range [0 .. size].    
    sfCoordinateTypePixels,
}

/// Types of shaders
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    /// Vertex shader
    Vertex,
    /// Geometry shader
    Geometry,
    /// Fragment (pixel) shader
    Fragment,
}

type sfShaderType = ShaderType;

#[repr(C)]
pub struct sfFontInfo {
    pub family: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct sfGlyph {
    ///< Offset to move horizontically to the next character
    pub advance: f32,
    ///< Bounding rectangle of the glyph, in coordinates relative to the baseline
    pub bounds: sfFloatRect,
    ///< Texture coordinates of the glyph inside the font's image
    pub texture_rect: sfIntRect,
}

pub type sfFloatRect = Rect<f32>;
pub type sfIntRect = Rect<c_int>;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfVertexBufferUsage {
    ///< Constantly changing data
    Stream,
    ///< Occasionally changing data
    Dynamic,
    ///< Rarely changing data
    Static,
}

#[repr(C)]
pub struct sfVertex {
    ///< Position of the vertex
    pub position: sfVector2f,
    ///< Color of the vertex
    pub color: Color,
    ///< Coordinates of the texture's pixel to map to the vertex
    pub tex_coords: sfVector2f,
}

// 2D vectors
pub type sfGlslVec2 = sfVector2f;
pub type sfGlslIvec2 = sfVector2i;

#[repr(C)]
pub struct sfGlslBvec2 {
    pub x: bool,
    pub y: bool,
}

// 3D vectors
pub type sfGlslVec3 = crate::system::Vector3<f32>;

#[repr(C)]
pub struct sfGlslIvec3 {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
}

#[repr(C)]
pub struct sfGlslBvec3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

// 4D vectors
#[repr(C)]
pub struct sfGlslVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
pub struct sfGlslIvec4 {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub w: c_int,
}

#[repr(C)]
pub struct sfGlslBvec4 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}

// matrices
#[repr(C)]
pub struct sfGlslMat3 {
    pub array: [f32; 3 * 3],
}

#[repr(C)]
pub struct sfGlslMat4 {
    pub array: [f32; 4 * 4],
}

type sfCustomShapeGetPointCountCb = Option<unsafe extern "C" fn(user_data: *mut c_void) -> usize>;
type sfCustomShapeGetPointCb =
    Option<unsafe extern "C" fn(idx: usize, user_data: *mut c_void) -> sfVector2f>;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum sfPrimitiveType {
    /// List of individual points
    Points,
    /// List of individual lines
    Lines,
    /// List of connected lines, a point uses the previous point to form a line
    LineStrip,
    /// List of individual triangles
    Triangles,
    /// List of connected triangles, a point uses the two previous points to form a triangle
    TriangleStrip,
    /// List of connected triangles, a point uses the common center and the previous point to form a triangle
    TriangleFan,
}

type sfColor = Color;
type sfStencilValue = StencilValue;

include!("graphics_bindgen.rs");
