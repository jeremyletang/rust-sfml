pub use crate::ffi::*;
use crate::{
    ffi::window::{sfContextSettings, sfCursor, sfWindow, sfWindowHandle, Event},
    graphics::{RenderStates, Transform},
};

decl_opaque! {
    sfCircleShape;
    sfConvexShape;
    sfFont;
    sfImage;
    sfShader;
    sfRectangleShape;
    sfRenderTexture;
    sfRenderWindow;
    sfShape;
    sfSprite;
    sfText;
    sfTexture;
    sfTransformable;
    sfVertexBuffer;
    sfView;
}

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
    /// Pixel = Src * SrcFactor + Dst * DstFactor.
    Add,
    /// Pixel = Src * SrcFactor - Dst * DstFactor.
    Subtract,
    /// Pixel = Dst * DstFactor - Src * SrcFactor.
    ReverseSubtract,
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

extern "C" {
    // Transform
    pub fn sfTransform_transformPoint(transform: *const Transform, point: sfVector2f)
        -> sfVector2f;
    pub fn sfTransform_transformRect(
        transform: *const Transform,
        rectangle: sfFloatRect,
    ) -> sfFloatRect;
    pub fn sfTransform_combine(transform: *mut Transform, other: *const Transform);
    pub fn sfTransform_translate(transform: *mut Transform, x: f32, y: f32);
    pub fn sfTransform_rotate(transform: *mut Transform, angle: f32);
    pub fn sfTransform_rotateWithCenter(
        transform: *mut Transform,
        angle: f32,
        centerX: f32,
        centerY: f32,
    );
    pub fn sfTransform_scale(transform: *mut Transform, scaleX: f32, scaleY: f32);
    pub fn sfTransform_scaleWithCenter(
        transform: *mut Transform,
        scaleX: f32,
        scaleY: f32,
        centerX: f32,
        centerY: f32,
    );
    // RenderTexture
    pub fn sfRenderTexture_drawSprite(
        rt: *mut sfRenderTexture,
        object: *const sfSprite,
        states: *const RenderStates,
    );
    pub fn sfRenderTexture_drawText(
        rt: *mut sfRenderTexture,
        object: *const sfText,
        states: *const RenderStates,
    );
    pub fn sfRenderTexture_drawShape(
        rt: *mut sfRenderTexture,
        object: *const sfShape,
        states: *const RenderStates,
    );
    pub fn sfRenderTexture_drawCircleShape(
        rt: *mut sfRenderTexture,
        object: *const sfCircleShape,
        states: *const RenderStates,
    );
    pub fn sfRenderTexture_drawConvexShape(
        rt: *mut sfRenderTexture,
        object: *const sfConvexShape,
        states: *const RenderStates,
    );
    pub fn sfRenderTexture_drawRectangleShape(
        rt: *mut sfRenderTexture,
        object: *const sfRectangleShape,
        states: *const RenderStates,
    );
    pub fn sfRenderTexture_drawVertexBuffer(
        rt: *mut sfRenderTexture,
        object: *const sfVertexBuffer,
        states: *const RenderStates,
    );
    pub fn sfRenderTexture_drawPrimitives(
        rt: *mut sfRenderTexture,
        vertices: *const sfVertex,
        vertex_count: usize,
        type_: sfPrimitiveType,
        states: *const RenderStates,
    );
    pub fn sfRenderTexture_createWithSettings(
        width: c_uint,
        height: c_uint,
        settings: *const sfContextSettings,
    ) -> *mut sfRenderTexture;
    pub fn sfRenderTexture_destroy(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_getSize(renderTexture: *const sfRenderTexture) -> sfVector2u;
    pub fn sfRenderTexture_setActive(renderTexture: *mut sfRenderTexture, active: bool) -> bool;
    pub fn sfRenderTexture_display(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_clear(renderTexture: *mut sfRenderTexture, color: sfColor);
    pub fn sfRenderTexture_setView(renderTexture: *mut sfRenderTexture, view: *const sfView);
    pub fn sfRenderTexture_getView(renderTexture: *const sfRenderTexture) -> *const sfView;
    pub fn sfRenderTexture_getDefaultView(renderTexture: *const sfRenderTexture) -> *const sfView;
    pub fn sfRenderTexture_getViewport(
        renderTexture: *const sfRenderTexture,
        view: *const sfView,
    ) -> sfIntRect;
    pub fn sfRenderTexture_mapPixelToCoords(
        renderTexture: *const sfRenderTexture,
        point: sfVector2i,
        view: *const sfView,
    ) -> sfVector2f;
    pub fn sfRenderTexture_mapCoordsToPixel(
        renderTexture: *const sfRenderTexture,
        point: sfVector2f,
        view: *const sfView,
    ) -> sfVector2i;
    pub fn sfRenderTexture_pushGLStates(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_popGLStates(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_resetGLStates(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_getTexture(renderTexture: *const sfRenderTexture) -> *const sfTexture;
    pub fn sfRenderTexture_getMaximumAntialiasingLevel() -> c_uint;
    pub fn sfRenderTexture_setSmooth(renderTexture: *mut sfRenderTexture, smooth: bool);
    pub fn sfRenderTexture_isSmooth(renderTexture: *const sfRenderTexture) -> bool;
    pub fn sfRenderTexture_setRepeated(renderTexture: *mut sfRenderTexture, repeated: bool);
    pub fn sfRenderTexture_isRepeated(renderTexture: *const sfRenderTexture) -> bool;
    pub fn sfRenderTexture_generateMipmap(renderTexture: *mut sfRenderTexture) -> bool;
    // sfCircleShape
    pub fn sfCircleShape_create() -> *mut sfCircleShape;
    pub fn sfCircleShape_copy(shape: *const sfCircleShape) -> *mut sfCircleShape;
    pub fn sfCircleShape_destroy(shape: *mut sfCircleShape);
    pub fn sfCircleShape_setPosition(shape: *mut sfCircleShape, position: sfVector2f);
    pub fn sfCircleShape_setRotation(shape: *mut sfCircleShape, angle: f32);
    pub fn sfCircleShape_setScale(shape: *mut sfCircleShape, scale: sfVector2f);
    pub fn sfCircleShape_setOrigin(shape: *mut sfCircleShape, origin: sfVector2f);
    pub fn sfCircleShape_getPosition(shape: *const sfCircleShape) -> sfVector2f;
    pub fn sfCircleShape_getRotation(shape: *const sfCircleShape) -> f32;
    pub fn sfCircleShape_getScale(shape: *const sfCircleShape) -> sfVector2f;
    pub fn sfCircleShape_getOrigin(shape: *const sfCircleShape) -> sfVector2f;
    pub fn sfCircleShape_move(shape: *mut sfCircleShape, offset: sfVector2f);
    pub fn sfCircleShape_rotate(shape: *mut sfCircleShape, angle: f32);
    pub fn sfCircleShape_scale(shape: *mut sfCircleShape, factors: sfVector2f);
    pub fn sfCircleShape_getTransform(shape: *const sfCircleShape) -> *const Transform;
    pub fn sfCircleShape_getInverseTransform(shape: *const sfCircleShape) -> *const Transform;
    pub fn sfCircleShape_setTexture(
        shape: *mut sfCircleShape,
        texture: *const sfTexture,
        resetRect: bool,
    );
    pub fn sfCircleShape_setTextureRect(shape: *mut sfCircleShape, rect: sfIntRect);
    pub fn sfCircleShape_setFillColor(shape: *mut sfCircleShape, color: sfColor);
    pub fn sfCircleShape_setOutlineColor(shape: *mut sfCircleShape, color: sfColor);
    pub fn sfCircleShape_setOutlineThickness(shape: *mut sfCircleShape, thickness: f32);
    pub fn sfCircleShape_getTexture(shape: *const sfCircleShape) -> *const sfTexture;
    pub fn sfCircleShape_getTextureRect(shape: *const sfCircleShape) -> sfIntRect;
    pub fn sfCircleShape_getFillColor(shape: *const sfCircleShape) -> sfColor;
    pub fn sfCircleShape_getOutlineColor(shape: *const sfCircleShape) -> sfColor;
    pub fn sfCircleShape_getOutlineThickness(shape: *const sfCircleShape) -> f32;
    pub fn sfCircleShape_getPointCount(shape: *const sfCircleShape) -> usize;
    pub fn sfCircleShape_getPoint(shape: *const sfCircleShape, index: usize) -> sfVector2f;
    pub fn sfCircleShape_setRadius(shape: *mut sfCircleShape, radius: f32);
    pub fn sfCircleShape_getRadius(shape: *const sfCircleShape) -> f32;
    pub fn sfCircleShape_setPointCount(shape: *mut sfCircleShape, count: usize);
    pub fn sfCircleShape_getLocalBounds(shape: *const sfCircleShape) -> sfFloatRect;
    pub fn sfCircleShape_getGlobalBounds(shape: *const sfCircleShape) -> sfFloatRect;

    // RenderWindow
    pub fn sfRenderWindow_createUnicode(
        mode: crate::ffi::window::sfVideoMode,
        title: *const u32,
        style: u32,
        settings: *const sfContextSettings,
    ) -> *mut sfRenderWindow;
    pub fn sfRenderWindow_createFromHandle(
        handle: sfWindowHandle,
        settings: *const sfContextSettings,
    ) -> *mut sfRenderWindow;
    pub fn sfRenderWindow_destroy(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_close(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_isOpen(renderWindow: *const sfRenderWindow) -> bool;
    pub fn sfRenderWindow_getSettings(
        renderWindow: *const sfRenderWindow,
    ) -> *const sfContextSettings;
    pub(crate) fn sfRenderWindow_pollEvent(
        renderWindow: *mut sfRenderWindow,
        event: *mut Event,
    ) -> bool;
    pub(crate) fn sfRenderWindow_waitEvent(
        renderWindow: *mut sfRenderWindow,
        event: *mut Event,
    ) -> bool;
    pub fn sfRenderWindow_getPosition(renderWindow: *const sfRenderWindow) -> sfVector2i;
    pub fn sfRenderWindow_setPosition(renderWindow: *mut sfRenderWindow, position: sfVector2i);
    pub fn sfRenderWindow_getSize(renderWindow: *const sfRenderWindow) -> sfVector2u;
    pub fn sfRenderWindow_setSize(renderWindow: *mut sfRenderWindow, size: sfVector2u);
    pub fn sfRenderWindow_setUnicodeTitle(renderWindow: *mut sfRenderWindow, title: *const u32);
    pub fn sfRenderWindow_setIcon(
        renderWindow: *mut sfRenderWindow,
        width: c_uint,
        height: c_uint,
        pixels: *const u8,
    );
    pub fn sfRenderWindow_setVisible(renderWindow: *mut sfRenderWindow, visible: bool);
    pub fn sfRenderWindow_setVerticalSyncEnabled(renderWindow: *mut sfRenderWindow, enabled: bool);
    pub fn sfRenderWindow_setMouseCursorVisible(renderWindow: *mut sfRenderWindow, visible: bool);
    pub fn sfRenderWindow_setMouseCursorGrabbed(renderWindow: *mut sfRenderWindow, grabbed: bool);
    pub fn sfRenderWindow_setMouseCursor(window: *mut sfRenderWindow, cursor: *const sfCursor);
    pub fn sfRenderWindow_setKeyRepeatEnabled(renderWindow: *mut sfRenderWindow, enabled: bool);
    pub fn sfRenderWindow_setActive(renderWindow: *mut sfRenderWindow, active: bool) -> bool;
    pub fn sfRenderWindow_requestFocus(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_hasFocus(renderWindow: *const sfRenderWindow) -> bool;
    pub fn sfRenderWindow_display(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_setFramerateLimit(renderWindow: *mut sfRenderWindow, limit: c_uint);
    pub fn sfRenderWindow_setJoystickThreshold(renderWindow: *mut sfRenderWindow, threshold: f32);
    pub fn sfRenderWindow_getSystemHandle(renderWindow: *const sfRenderWindow) -> sfWindowHandle;
    pub fn sfRenderWindow_clear(renderWindow: *mut sfRenderWindow, color: sfColor);
    pub fn sfRenderWindow_setView(renderWindow: *mut sfRenderWindow, view: *const sfView);
    pub fn sfRenderWindow_getView(renderWindow: *const sfRenderWindow) -> *const sfView;
    pub fn sfRenderWindow_getDefaultView(renderWindow: *const sfRenderWindow) -> *const sfView;
    pub fn sfRenderWindow_getViewport(
        renderWindow: *const sfRenderWindow,
        view: *const sfView,
    ) -> sfIntRect;
    pub fn sfRenderWindow_mapPixelToCoords(
        renderWindow: *const sfRenderWindow,
        point: sfVector2i,
        targetView: *const sfView,
    ) -> sfVector2f;
    pub fn sfRenderWindow_mapCoordsToPixel(
        renderWindow: *const sfRenderWindow,
        point: sfVector2f,
        targetView: *const sfView,
    ) -> sfVector2i;
    pub fn sfRenderWindow_drawSprite(
        renderWindow: *mut sfRenderWindow,
        object: *const sfSprite,
        states: *const RenderStates,
    );
    pub fn sfRenderWindow_drawText(
        renderWindow: *mut sfRenderWindow,
        object: *const sfText,
        states: *const RenderStates,
    );
    pub fn sfRenderWindow_drawShape(
        renderWindow: *mut sfRenderWindow,
        object: *const sfShape,
        states: *const RenderStates,
    );
    pub fn sfRenderWindow_drawCircleShape(
        renderWindow: *mut sfRenderWindow,
        object: *const sfCircleShape,
        states: *const RenderStates,
    );
    pub fn sfRenderWindow_drawConvexShape(
        renderWindow: *mut sfRenderWindow,
        object: *const sfConvexShape,
        states: *const RenderStates,
    );
    pub fn sfRenderWindow_drawRectangleShape(
        renderWindow: *mut sfRenderWindow,
        object: *const sfRectangleShape,
        states: *const RenderStates,
    );
    pub fn sfRenderWindow_drawVertexBuffer(
        renderWindow: *mut sfRenderWindow,
        object: *const sfVertexBuffer,
        states: *const RenderStates,
    );
    pub fn sfRenderWindow_drawPrimitives(
        renderWindow: *mut sfRenderWindow,
        vertices: *const sfVertex,
        vertexCount: usize,
        type_: sfPrimitiveType,
        states: *const RenderStates,
    );
    pub fn sfRenderWindow_pushGLStates(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_popGLStates(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_resetGLStates(renderWindow: *mut sfRenderWindow);
    pub fn sfMouse_getPositionRenderWindow(relativeTo: *const sfRenderWindow) -> sfVector2i;
    pub fn sfMouse_setPositionRenderWindow(position: sfVector2i, relativeTo: *const sfRenderWindow);
    pub fn sfTouch_getPositionRenderWindow(
        finger: c_uint,
        relativeTo: *const sfRenderWindow,
    ) -> sfVector2i;
    // ConvexShape
    pub fn sfConvexShape_create() -> *mut sfConvexShape;
    pub fn sfConvexShape_copy(shape: *const sfConvexShape) -> *mut sfConvexShape;
    pub fn sfConvexShape_destroy(shape: *mut sfConvexShape);
    pub fn sfConvexShape_setPosition(shape: *mut sfConvexShape, position: sfVector2f);
    pub fn sfConvexShape_setRotation(shape: *mut sfConvexShape, angle: f32);
    pub fn sfConvexShape_setScale(shape: *mut sfConvexShape, scale: sfVector2f);
    pub fn sfConvexShape_setOrigin(shape: *mut sfConvexShape, origin: sfVector2f);
    pub fn sfConvexShape_getPosition(shape: *const sfConvexShape) -> sfVector2f;
    pub fn sfConvexShape_getRotation(shape: *const sfConvexShape) -> f32;
    pub fn sfConvexShape_getScale(shape: *const sfConvexShape) -> sfVector2f;
    pub fn sfConvexShape_getOrigin(shape: *const sfConvexShape) -> sfVector2f;
    pub fn sfConvexShape_move(shape: *mut sfConvexShape, offset: sfVector2f);
    pub fn sfConvexShape_rotate(shape: *mut sfConvexShape, angle: f32);
    pub fn sfConvexShape_scale(shape: *mut sfConvexShape, factors: sfVector2f);
    pub fn sfConvexShape_getTransform(shape: *const sfConvexShape) -> *const Transform;
    pub fn sfConvexShape_getInverseTransform(shape: *const sfConvexShape) -> *const Transform;
    pub fn sfConvexShape_setTexture(
        shape: *mut sfConvexShape,
        texture: *const sfTexture,
        resetRect: bool,
    );
    pub fn sfConvexShape_setTextureRect(shape: *mut sfConvexShape, rect: sfIntRect);
    pub fn sfConvexShape_setFillColor(shape: *mut sfConvexShape, color: sfColor);
    pub fn sfConvexShape_setOutlineColor(shape: *mut sfConvexShape, color: sfColor);
    pub fn sfConvexShape_setOutlineThickness(shape: *mut sfConvexShape, thickness: f32);
    pub fn sfConvexShape_getTexture(shape: *const sfConvexShape) -> *const sfTexture;
    pub fn sfConvexShape_getTextureRect(shape: *const sfConvexShape) -> sfIntRect;
    pub fn sfConvexShape_getFillColor(shape: *const sfConvexShape) -> sfColor;
    pub fn sfConvexShape_getOutlineColor(shape: *const sfConvexShape) -> sfColor;
    pub fn sfConvexShape_getOutlineThickness(shape: *const sfConvexShape) -> f32;
    pub fn sfConvexShape_getPointCount(shape: *const sfConvexShape) -> usize;
    pub fn sfConvexShape_getPoint(shape: *const sfConvexShape, index: usize) -> sfVector2f;
    pub fn sfConvexShape_setPointCount(shape: *mut sfConvexShape, count: usize);
    pub fn sfConvexShape_setPoint(shape: *mut sfConvexShape, index: usize, point: sfVector2f);
    pub fn sfConvexShape_getLocalBounds(shape: *const sfConvexShape) -> sfFloatRect;
    pub fn sfConvexShape_getGlobalBounds(shape: *const sfConvexShape) -> sfFloatRect;

    // SfImage
    pub fn sfImage_create(width: c_uint, height: c_uint) -> *mut sfImage;
    pub fn sfImage_createFromColor(width: c_uint, height: c_uint, color: sfColor) -> *mut sfImage;
    pub fn sfImage_createFromPixels(width: c_uint, height: c_uint, data: *const u8)
        -> *mut sfImage;
    pub fn sfImage_createFromFile(filename: *const c_char) -> *mut sfImage;
    pub fn sfImage_createFromMemory(data: *const c_void, sizeInBytes: usize) -> *mut sfImage;
    pub fn sfImage_createFromStream(stream: *mut crate::ffi::system::sfInputStream)
        -> *mut sfImage;
    pub fn sfImage_copy(image: *const sfImage) -> *mut sfImage;
    pub fn sfImage_destroy(image: *mut sfImage);
    pub fn sfImage_saveToFile(image: *const sfImage, filename: *const c_char) -> bool;
    pub fn sfImage_createMaskFromColor(image: *mut sfImage, colorKey: sfColor, alpha: u8);
    pub fn sfImage_copyImage(
        image: *mut sfImage,
        source: *const sfImage,
        destX: c_uint,
        destY: c_uint,
        sourceRect: sfIntRect,
        applyAlpha: bool,
    );
    pub fn sfImage_setPixel(image: *mut sfImage, x: c_uint, y: c_uint, color: sfColor);
    pub fn sfImage_getPixel(image: *const sfImage, x: c_uint, y: c_uint) -> sfColor;
    pub fn sfImage_getPixelsPtr(image: *const sfImage) -> *const u8;
    pub fn sfImage_getSize(image: *const sfImage) -> sfVector2u;
    pub fn sfImage_flipHorizontally(image: *mut sfImage);
    pub fn sfImage_flipVertically(image: *mut sfImage);
    // SfRectangleShape
    pub fn sfRectangleShape_create() -> *mut sfRectangleShape;
    pub fn sfRectangleShape_copy(shape: *const sfRectangleShape) -> *mut sfRectangleShape;
    pub fn sfRectangleShape_destroy(shape: *mut sfRectangleShape);
    pub fn sfRectangleShape_setPosition(shape: *mut sfRectangleShape, position: sfVector2f);
    pub fn sfRectangleShape_setRotation(shape: *mut sfRectangleShape, angle: f32);
    pub fn sfRectangleShape_setScale(shape: *mut sfRectangleShape, scale: sfVector2f);
    pub fn sfRectangleShape_setOrigin(shape: *mut sfRectangleShape, origin: sfVector2f);
    pub fn sfRectangleShape_getPosition(shape: *const sfRectangleShape) -> sfVector2f;
    pub fn sfRectangleShape_getRotation(shape: *const sfRectangleShape) -> f32;
    pub fn sfRectangleShape_getScale(shape: *const sfRectangleShape) -> sfVector2f;
    pub fn sfRectangleShape_getOrigin(shape: *const sfRectangleShape) -> sfVector2f;
    pub fn sfRectangleShape_move(shape: *mut sfRectangleShape, offset: sfVector2f);
    pub fn sfRectangleShape_rotate(shape: *mut sfRectangleShape, angle: f32);
    pub fn sfRectangleShape_scale(shape: *mut sfRectangleShape, factors: sfVector2f);
    pub fn sfRectangleShape_getTransform(shape: *const sfRectangleShape) -> *const Transform;
    pub fn sfRectangleShape_getInverseTransform(shape: *const sfRectangleShape)
        -> *const Transform;
    pub fn sfRectangleShape_setTexture(
        shape: *mut sfRectangleShape,
        texture: *const sfTexture,
        resetRect: bool,
    );
    pub fn sfRectangleShape_setTextureRect(shape: *mut sfRectangleShape, rect: sfIntRect);
    pub fn sfRectangleShape_setFillColor(shape: *mut sfRectangleShape, color: sfColor);
    pub fn sfRectangleShape_setOutlineColor(shape: *mut sfRectangleShape, color: sfColor);
    pub fn sfRectangleShape_setOutlineThickness(shape: *mut sfRectangleShape, thickness: f32);
    pub fn sfRectangleShape_getTexture(shape: *const sfRectangleShape) -> *const sfTexture;
    pub fn sfRectangleShape_getTextureRect(shape: *const sfRectangleShape) -> sfIntRect;
    pub fn sfRectangleShape_getFillColor(shape: *const sfRectangleShape) -> sfColor;
    pub fn sfRectangleShape_getOutlineColor(shape: *const sfRectangleShape) -> sfColor;
    pub fn sfRectangleShape_getOutlineThickness(shape: *const sfRectangleShape) -> f32;
    pub fn sfRectangleShape_getPointCount(shape: *const sfRectangleShape) -> usize;
    pub fn sfRectangleShape_getPoint(shape: *const sfRectangleShape, index: usize) -> sfVector2f;
    pub fn sfRectangleShape_setSize(shape: *mut sfRectangleShape, size: sfVector2f);
    pub fn sfRectangleShape_getSize(shape: *const sfRectangleShape) -> sfVector2f;
    pub fn sfRectangleShape_getLocalBounds(shape: *const sfRectangleShape) -> sfFloatRect;
    pub fn sfRectangleShape_getGlobalBounds(shape: *const sfRectangleShape) -> sfFloatRect;

    // SfShader
    pub fn sfShader_defaultConstruct() -> *mut sfShader;
    pub fn sfShader_loadFromMemory_1(
        shader: *mut sfShader,
        content: *const c_char,
        type_: ShaderType,
    ) -> bool;
    pub fn sfShader_loadFromFile_1(
        shader: *mut sfShader,
        filename: *const c_char,
        type_: ShaderType,
    ) -> bool;
    pub fn sfShader_loadFromStream_1(
        shader: *mut sfShader,
        stream: *mut crate::ffi::system::sfInputStream,
        type_: ShaderType,
    ) -> bool;
    pub fn sfShader_loadFromMemory_vert_frag(
        shader: *mut sfShader,
        vert: *const c_char,
        frag: *const c_char,
    ) -> bool;
    pub fn sfShader_loadFromFile_vert_frag(
        shader: *mut sfShader,
        vert: *const c_char,
        frag: *const c_char,
    ) -> bool;
    pub fn sfShader_loadFromStream_vert_frag(
        shader: *mut sfShader,
        vert: *mut crate::ffi::system::sfInputStream,
        frag: *mut crate::ffi::system::sfInputStream,
    ) -> bool;
    pub fn sfShader_loadFromMemory_all(
        shader: *mut sfShader,
        vert: *const c_char,
        geom: *const c_char,
        frag: *const c_char,
    ) -> bool;
    pub fn sfShader_loadFromFile_all(
        shader: *mut sfShader,
        vert: *const c_char,
        geom: *const c_char,
        frag: *const c_char,
    ) -> bool;
    pub fn sfShader_loadFromStream_all(
        shader: *mut sfShader,
        vert: *mut crate::ffi::system::sfInputStream,
        geom: *mut crate::ffi::system::sfInputStream,
        frag: *mut crate::ffi::system::sfInputStream,
    ) -> bool;
    pub fn sfShader_destroy(shader: *mut sfShader);
    pub fn sfShader_setFloatUniform(shader: *mut sfShader, name: *const c_char, x: f32);
    pub fn sfShader_setVec2Uniform(shader: *mut sfShader, name: *const c_char, vector: sfGlslVec2);
    pub fn sfShader_setVec3Uniform(shader: *mut sfShader, name: *const c_char, vector: sfGlslVec3);
    pub fn sfShader_setVec4Uniform(shader: *mut sfShader, name: *const c_char, vector: sfGlslVec4);
    pub fn sfShader_setIntUniform(shader: *mut sfShader, name: *const c_char, x: c_int);
    pub fn sfShader_setIvec2Uniform(
        shader: *mut sfShader,
        name: *const c_char,
        vector: sfGlslIvec2,
    );
    pub fn sfShader_setIvec3Uniform(
        shader: *mut sfShader,
        name: *const c_char,
        vector: sfGlslIvec3,
    );
    pub fn sfShader_setIvec4Uniform(
        shader: *mut sfShader,
        name: *const c_char,
        vector: sfGlslIvec4,
    );
    pub fn sfShader_setBoolUniform(shader: *mut sfShader, name: *const c_char, x: bool);
    pub fn sfShader_setBvec2Uniform(
        shader: *mut sfShader,
        name: *const c_char,
        vector: sfGlslBvec2,
    );
    pub fn sfShader_setBvec3Uniform(
        shader: *mut sfShader,
        name: *const c_char,
        vector: sfGlslBvec3,
    );
    pub fn sfShader_setBvec4Uniform(
        shader: *mut sfShader,
        name: *const c_char,
        vector: sfGlslBvec4,
    );
    pub fn sfShader_setMat3Uniform(
        shader: *mut sfShader,
        name: *const c_char,
        matrix: *const sfGlslMat3,
    );
    pub fn sfShader_setMat4Uniform(
        shader: *mut sfShader,
        name: *const c_char,
        matrix: *const sfGlslMat4,
    );
    pub fn sfShader_setTextureUniform(
        shader: *mut sfShader,
        name: *const c_char,
        texture: *const sfTexture,
    );
    pub fn sfShader_setCurrentTextureUniform(shader: *mut sfShader, name: *const c_char);
    pub fn sfShader_setFloatUniformArray(
        shader: *mut sfShader,
        name: *const c_char,
        scalarArray: *const f32,
        length: usize,
    );
    pub fn sfShader_setVec2UniformArray(
        shader: *mut sfShader,
        name: *const c_char,
        vectorArray: *const sfGlslVec2,
        length: usize,
    );
    pub fn sfShader_setVec3UniformArray(
        shader: *mut sfShader,
        name: *const c_char,
        vectorArray: *const sfGlslVec3,
        length: usize,
    );
    pub fn sfShader_setVec4UniformArray(
        shader: *mut sfShader,
        name: *const c_char,
        vectorArray: *const sfGlslVec4,
        length: usize,
    );
    pub fn sfShader_setMat3UniformArray(
        shader: *mut sfShader,
        name: *const c_char,
        matrixArray: *const sfGlslMat3,
        length: usize,
    );
    pub fn sfShader_setMat4UniformArray(
        shader: *mut sfShader,
        name: *const c_char,
        matrixArray: *const sfGlslMat4,
        length: usize,
    );
    pub fn sfShader_getNativeHandle(shader: *const sfShader) -> c_uint;
    pub fn sfShader_bind(shader: *const sfShader);
    pub fn sfShader_isAvailable() -> bool;
    pub fn sfShader_isGeometryAvailable() -> bool;
    // SfSprite
    pub fn sfSprite_create() -> *mut sfSprite;
    pub fn sfSprite_copy(sprite: *const sfSprite) -> *mut sfSprite;
    pub fn sfSprite_destroy(sprite: *mut sfSprite);
    pub fn sfSprite_setPosition(sprite: *mut sfSprite, position: sfVector2f);
    pub fn sfSprite_setRotation(sprite: *mut sfSprite, angle: f32);
    pub fn sfSprite_setScale(sprite: *mut sfSprite, scale: sfVector2f);
    pub fn sfSprite_setOrigin(sprite: *mut sfSprite, origin: sfVector2f);
    pub fn sfSprite_getPosition(sprite: *const sfSprite) -> sfVector2f;
    pub fn sfSprite_getRotation(sprite: *const sfSprite) -> f32;
    pub fn sfSprite_getScale(sprite: *const sfSprite) -> sfVector2f;
    pub fn sfSprite_getOrigin(sprite: *const sfSprite) -> sfVector2f;
    pub fn sfSprite_move(sprite: *mut sfSprite, offset: sfVector2f);
    pub fn sfSprite_rotate(sprite: *mut sfSprite, angle: f32);
    pub fn sfSprite_scale(sprite: *mut sfSprite, factors: sfVector2f);
    pub fn sfSprite_getTransform(sprite: *const sfSprite) -> *const Transform;
    pub fn sfSprite_getInverseTransform(sprite: *const sfSprite) -> *const Transform;
    pub fn sfSprite_setTexture(sprite: *mut sfSprite, texture: *const sfTexture, resetRect: bool);
    pub fn sfSprite_setTextureRect(sprite: *mut sfSprite, rectangle: sfIntRect);
    pub fn sfSprite_setColor(sprite: *mut sfSprite, color: sfColor);
    pub fn sfSprite_getTexture(sprite: *const sfSprite) -> *const sfTexture;
    pub fn sfSprite_getTextureRect(sprite: *const sfSprite) -> sfIntRect;
    pub fn sfSprite_getColor(sprite: *const sfSprite) -> sfColor;
    pub fn sfSprite_getLocalBounds(sprite: *const sfSprite) -> sfFloatRect;
    pub fn sfSprite_getGlobalBounds(sprite: *const sfSprite) -> sfFloatRect;
    // SfText
    pub fn sfText_create() -> *mut sfText;
    pub fn sfText_copy(text: *const sfText) -> *mut sfText;
    pub fn sfText_destroy(text: *mut sfText);
    pub fn sfText_setPosition(text: *mut sfText, position: sfVector2f);
    pub fn sfText_setRotation(text: *mut sfText, angle: f32);
    pub fn sfText_setScale(text: *mut sfText, scale: sfVector2f);
    pub fn sfText_setOrigin(text: *mut sfText, origin: sfVector2f);
    pub fn sfText_getPosition(text: *const sfText) -> sfVector2f;
    pub fn sfText_getRotation(text: *const sfText) -> f32;
    pub fn sfText_getScale(text: *const sfText) -> sfVector2f;
    pub fn sfText_getOrigin(text: *const sfText) -> sfVector2f;
    pub fn sfText_move(text: *mut sfText, offset: sfVector2f);
    pub fn sfText_rotate(text: *mut sfText, angle: f32);
    pub fn sfText_scale(text: *mut sfText, factors: sfVector2f);
    pub fn sfText_getTransform(text: *const sfText) -> *const Transform;
    pub fn sfText_getInverseTransform(text: *const sfText) -> *const Transform;
    pub fn sfText_setUnicodeString(text: *mut sfText, string: *const u32);
    pub fn sfText_setFont(text: *mut sfText, font: *const sfFont);
    pub fn sfText_setCharacterSize(text: *mut sfText, size: c_uint);
    pub fn sfText_setLineSpacing(text: *mut sfText, spacingFactor: f32);
    pub fn sfText_setLetterSpacing(text: *mut sfText, spacingFactor: f32);
    pub fn sfText_setStyle(text: *mut sfText, style: u32);
    pub fn sfText_setFillColor(text: *mut sfText, color: sfColor);
    pub fn sfText_setOutlineColor(text: *mut sfText, color: sfColor);
    pub fn sfText_setOutlineThickness(text: *mut sfText, thickness: f32);
    pub fn sfText_getUnicodeString(text: *const sfText) -> *const u32;
    pub fn sfText_getFont(text: *const sfText) -> *const sfFont;
    pub fn sfText_getCharacterSize(text: *const sfText) -> c_uint;
    pub fn sfText_getLetterSpacing(text: *const sfText) -> f32;
    pub fn sfText_getLineSpacing(text: *const sfText) -> f32;
    pub fn sfText_getStyle(text: *const sfText) -> u32;
    pub fn sfText_getFillColor(text: *const sfText) -> sfColor;
    pub fn sfText_getOutlineColor(text: *const sfText) -> sfColor;
    pub fn sfText_getOutlineThickness(text: *const sfText) -> f32;
    pub fn sfText_findCharacterPos(text: *const sfText, index: usize) -> sfVector2f;
    pub fn sfText_getLocalBounds(text: *const sfText) -> sfFloatRect;
    pub fn sfText_getGlobalBounds(text: *const sfText) -> sfFloatRect;
    // SfTexture
    pub fn sfTexture_new() -> *mut sfTexture;
    pub fn sfTexture_create(tex: *mut sfTexture, width: c_uint, height: c_uint) -> bool;
    pub fn sfTexture_loadFromFile(
        tex: *mut sfTexture,
        filename: *const c_char,
        area: sfIntRect,
    ) -> bool;
    pub fn sfTexture_loadFromMemory(
        tex: *mut sfTexture,
        data: *const c_void,
        sizeInBytes: usize,
        area: sfIntRect,
    ) -> bool;
    pub fn sfTexture_loadFromStream(
        tex: *mut sfTexture,
        stream: *mut crate::ffi::system::sfInputStream,
        area: sfIntRect,
    ) -> bool;
    pub fn sfTexture_loadFromImage(
        tex: *mut sfTexture,
        image: *const sfImage,
        area: sfIntRect,
    ) -> bool;
    pub fn sfTexture_copy(texture: *const sfTexture) -> *mut sfTexture;
    pub fn sfTexture_destroy(texture: *mut sfTexture);
    pub fn sfTexture_getSize(texture: *const sfTexture) -> sfVector2u;
    pub fn sfTexture_copyToImage(texture: *const sfTexture) -> *mut sfImage;
    pub fn sfTexture_updateFromPixels(
        texture: *mut sfTexture,
        pixels: *const u8,
        width: c_uint,
        height: c_uint,
        x: c_uint,
        y: c_uint,
    );
    pub fn sfTexture_updateFromTexture(
        destination: *mut sfTexture,
        texture: *const sfTexture,
        x: c_uint,
        y: c_uint,
    );
    pub fn sfTexture_updateFromImage(
        texture: *mut sfTexture,
        image: *const sfImage,
        x: c_uint,
        y: c_uint,
    );
    pub fn sfTexture_updateFromWindow(
        texture: *mut sfTexture,
        window: *const sfWindow,
        x: c_uint,
        y: c_uint,
    );
    pub fn sfTexture_updateFromRenderWindow(
        texture: *mut sfTexture,
        renderWindow: *const sfRenderWindow,
        x: c_uint,
        y: c_uint,
    );
    pub fn sfTexture_setSmooth(texture: *mut sfTexture, smooth: bool);
    pub fn sfTexture_isSmooth(texture: *const sfTexture) -> bool;
    pub fn sfTexture_setSrgb(texture: *mut sfTexture, sRgb: bool);
    pub fn sfTexture_isSrgb(texture: *const sfTexture) -> bool;
    pub fn sfTexture_setRepeated(texture: *mut sfTexture, repeated: bool);
    pub fn sfTexture_isRepeated(texture: *const sfTexture) -> bool;
    pub fn sfTexture_generateMipmap(texture: *mut sfTexture) -> bool;
    pub fn sfTexture_swap(left: *mut sfTexture, right: *mut sfTexture);
    pub fn sfTexture_getNativeHandle(texture: *const sfTexture) -> c_uint;
    pub fn sfTexture_bind(texture: *const sfTexture);
    pub fn sfTexture_getMaximumSize() -> c_uint;
    // SfView
    pub fn sfView_create() -> *mut sfView;
    pub fn sfView_createFromRect(rectangle: sfFloatRect) -> *mut sfView;
    pub fn sfView_copy(view: *const sfView) -> *mut sfView;
    pub fn sfView_destroy(view: *mut sfView);
    pub fn sfView_setCenter(view: *mut sfView, center: sfVector2f);
    pub fn sfView_setSize(view: *mut sfView, size: sfVector2f);
    pub fn sfView_setRotation(view: *mut sfView, angle: f32);
    pub fn sfView_setViewport(view: *mut sfView, viewport: sfFloatRect);
    pub fn sfView_reset(view: *mut sfView, rectangle: sfFloatRect);
    pub fn sfView_getCenter(view: *const sfView) -> sfVector2f;
    pub fn sfView_getSize(view: *const sfView) -> sfVector2f;
    pub fn sfView_getRotation(view: *const sfView) -> f32;
    pub fn sfView_getViewport(view: *const sfView) -> sfFloatRect;
    pub fn sfView_move(view: *mut sfView, offset: sfVector2f);
    pub fn sfView_rotate(view: *mut sfView, angle: f32);
    pub fn sfView_zoom(view: *mut sfView, factor: f32);
    // SfFont
    pub fn sfFont_createFromFile(filename: *const c_char) -> *mut sfFont;
    pub fn sfFont_createFromMemory(data: *const c_void, sizeInBytes: usize) -> *mut sfFont;
    pub fn sfFont_createFromStream(stream: *mut crate::ffi::system::sfInputStream) -> *mut sfFont;
    pub fn sfFont_copy(font: *const sfFont) -> *mut sfFont;
    pub fn sfFont_destroy(font: *mut sfFont);
    pub fn sfFont_getGlyph(
        font: *const sfFont,
        codePoint: u32,
        characterSize: c_uint,
        bold: bool,
        outlineThickness: f32,
    ) -> sfGlyph;
    pub fn sfFont_getKerning(
        font: *const sfFont,
        first: u32,
        second: u32,
        characterSize: c_uint,
    ) -> f32;
    pub fn sfFont_getLineSpacing(font: *const sfFont, characterSize: c_uint) -> f32;
    pub fn sfFont_getUnderlinePosition(font: *const sfFont, characterSize: c_uint) -> f32;
    pub fn sfFont_getUnderlineThickness(font: *const sfFont, characterSize: c_uint) -> f32;
    pub fn sfFont_getTexture(font: *const sfFont, characterSize: c_uint) -> *const sfTexture;
    pub fn sfFont_getInfo(font: *const sfFont) -> sfFontInfo;
    // Shape
    pub fn sfShape_create(
        getPointCount: sfShapeGetPointCountCallback,
        getPoint: sfShapeGetPointCallback,
        userData: *mut c_void,
    ) -> *mut sfShape;
    pub fn sfShape_destroy(shape: *mut sfShape);
    pub fn sfShape_setPosition(shape: *mut sfShape, position: sfVector2f);
    pub fn sfShape_setRotation(shape: *mut sfShape, angle: f32);
    pub fn sfShape_setScale(shape: *mut sfShape, scale: sfVector2f);
    pub fn sfShape_setOrigin(shape: *mut sfShape, origin: sfVector2f);
    pub fn sfShape_getPosition(shape: *const sfShape) -> sfVector2f;
    pub fn sfShape_getRotation(shape: *const sfShape) -> f32;
    pub fn sfShape_getScale(shape: *const sfShape) -> sfVector2f;
    pub fn sfShape_getOrigin(shape: *const sfShape) -> sfVector2f;
    pub fn sfShape_move(shape: *mut sfShape, offset: sfVector2f);
    pub fn sfShape_rotate(shape: *mut sfShape, angle: f32);
    pub fn sfShape_scale(shape: *mut sfShape, factors: sfVector2f);
    pub fn sfShape_getTransform(shape: *const sfShape) -> *const Transform;
    pub fn sfShape_getInverseTransform(shape: *const sfShape) -> *const Transform;
    pub fn sfShape_setTexture(shape: *mut sfShape, texture: *const sfTexture, resetRect: bool);
    pub fn sfShape_setTextureRect(shape: *mut sfShape, rect: sfIntRect);
    pub fn sfShape_setFillColor(shape: *mut sfShape, color: sfColor);
    pub fn sfShape_setOutlineColor(shape: *mut sfShape, color: sfColor);
    pub fn sfShape_setOutlineThickness(shape: *mut sfShape, thickness: f32);
    pub fn sfShape_getTexture(shape: *const sfShape) -> *const sfTexture;
    pub fn sfShape_getTextureRect(shape: *const sfShape) -> sfIntRect;
    pub fn sfShape_getFillColor(shape: *const sfShape) -> sfColor;
    pub fn sfShape_getOutlineColor(shape: *const sfShape) -> sfColor;
    pub fn sfShape_getOutlineThickness(shape: *const sfShape) -> f32;
    pub fn sfShape_getPointCount(shape: *const sfShape) -> usize;
    pub fn sfShape_getPoint(shape: *const sfShape, index: usize) -> sfVector2f;
    pub fn sfShape_getLocalBounds(shape: *const sfShape) -> sfFloatRect;
    pub fn sfShape_getGlobalBounds(shape: *const sfShape) -> sfFloatRect;
    pub fn sfShape_update(shape: *mut sfShape);

    // VertexBuffer
    pub fn sfVertexBuffer_create(
        vertexCount: c_uint,
        type_: sfPrimitiveType,
        usage: sfVertexBufferUsage,
    ) -> *mut sfVertexBuffer;
    pub fn sfVertexBuffer_copy(vertexBuffer: *const sfVertexBuffer) -> *mut sfVertexBuffer;
    pub fn sfVertexBuffer_destroy(vertexBuffer: *mut sfVertexBuffer);
    pub fn sfVertexBuffer_getVertexCount(vertexBuffer: *const sfVertexBuffer) -> c_uint;
    pub fn sfVertexBuffer_update(
        vertexBuffer: *mut sfVertexBuffer,
        vertices: *const sfVertex,
        vertexCount: c_uint,
        offset: c_uint,
    ) -> bool;
    pub fn sfVertexBuffer_updateFromVertexBuffer(
        vertexBuffer: *mut sfVertexBuffer,
        other: *const sfVertexBuffer,
    ) -> bool;
    pub fn sfVertexBuffer_swap(left: *mut sfVertexBuffer, right: *mut sfVertexBuffer);
    pub fn sfVertexBuffer_getNativeHandle(vertexBuffer: *mut sfVertexBuffer) -> c_uint;
    pub fn sfVertexBuffer_setPrimitiveType(
        vertexBuffer: *mut sfVertexBuffer,
        type_: sfPrimitiveType,
    );
    pub fn sfVertexBuffer_getPrimitiveType(vertexBuffer: *const sfVertexBuffer) -> sfPrimitiveType;
    pub fn sfVertexBuffer_setUsage(vertexBuffer: *mut sfVertexBuffer, usage: sfVertexBufferUsage);
    pub fn sfVertexBuffer_getUsage(vertexBuffer: *const sfVertexBuffer) -> sfVertexBufferUsage;
    pub fn sfVertexBuffer_bind(vertexBuffer: *const sfVertexBuffer);
    pub fn sfVertexBuffer_isAvailable() -> bool;
}

#[repr(C)]
pub struct sfFontInfo {
    pub family: *const c_char,
}
