use crate::ffi::window::{sfContextSettings, sfWindowHandle, Event};
pub use crate::ffi::*;

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

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct sfRenderStates {
    pub blendMode: BlendMode,
    pub transform: sfTransform,
    pub texture: *const sfTexture,
    pub shader: *const sfShader,
}

extern "C" {
    pub fn sfTexture_new() -> *mut sfTexture;
    pub fn sfTexture_create(tex: *mut sfTexture, width: c_uint, height: c_uint) -> sfBool;
    pub fn sfTexture_loadFromFile(
        tex: *mut sfTexture,
        filename: *const c_char,
        area: sfIntRect,
    ) -> sfBool;
    pub fn sfTexture_loadFromMemory(
        tex: *mut sfTexture,
        data: *const c_void,
        size: usize,
        area: sfIntRect,
    ) -> sfBool;
    pub fn sfTexture_loadFromStream(
        tex: *mut sfTexture,
        stream: *mut sfInputStream,
        area: sfIntRect,
    ) -> sfBool;
    pub fn sfTexture_loadFromImage(
        tex: *mut sfTexture,
        image: *const sfImage,
        area: sfIntRect,
    ) -> sfBool;
    pub fn sfCircleShape_getTransform(shape: *const sfCircleShape) -> sfTransform;
    pub fn sfCircleShape_getInverseTransform(shape: *const sfCircleShape) -> sfTransform;
    pub fn sfShape_getTransform(shape: *const sfShape) -> sfTransform;
    pub fn sfShape_getInverseTransform(shape: *const sfShape) -> sfTransform;
    pub fn sfRectangleShape_getTransform(shape: *const sfRectangleShape) -> sfTransform;
    pub fn sfRectangleShape_getInverseTransform(shape: *const sfRectangleShape) -> sfTransform;
    pub fn sfConvexShape_getTransform(shape: *const sfConvexShape) -> sfTransform;
    pub fn sfConvexShape_getInverseTransform(shape: *const sfConvexShape) -> sfTransform;
    pub fn sfSprite_getTransform(shape: *const sfSprite) -> sfTransform;
    pub fn sfSprite_getInverseTransform(shape: *const sfSprite) -> sfTransform;
    pub fn sfText_getTransform(shape: *const sfText) -> sfTransform;
    pub fn sfText_getInverseTransform(shape: *const sfText) -> sfTransform;
    pub fn sfTransform_fromMatrix(
        a00: f32,
        a01: f32,
        a02: f32,
        a10: f32,
        a11: f32,
        a12: f32,
        a20: f32,
        a21: f32,
        a22: f32,
    ) -> sfTransform;
    pub fn sfTransform_getMatrix(tf: *const sfTransform) -> *const f32;
    pub fn sfTransform_getInverse(tf: *const sfTransform) -> sfTransform;
    pub fn sfTransform_transformPoint(tf: *const sfTransform, point: sfVector2f) -> sfVector2f;
    pub fn sfTransform_transformRect(tf: *const sfTransform, rect: sfFloatRect) -> sfFloatRect;
    pub fn sfTransform_combine(tf: *mut sfTransform, other: *const sfTransform);
    pub fn sfTransform_translate(tf: *mut sfTransform, x: f32, y: f32);
    pub fn sfTransform_rotate(tf: *mut sfTransform, angle: f32);
    pub fn sfTransform_rotateWithCenter(tf: *mut sfTransform, angle: f32, cx: f32, cy: f32);
    pub fn sfTransform_scale(tf: *mut sfTransform, x: f32, y: f32);
    pub fn sfTransform_scaleWithCenter(tf: *mut sfTransform, x: f32, y: f32, cx: f32, cy: f32);
    pub fn sfRenderTexture_drawSprite(
        rt: *mut sfRenderTexture,
        object: *const sfSprite,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawText(
        rt: *mut sfRenderTexture,
        object: *const sfText,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawShape(
        rt: *mut sfRenderTexture,
        object: *const sfShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawCircleShape(
        rt: *mut sfRenderTexture,
        object: *const sfCircleShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawConvexShape(
        rt: *mut sfRenderTexture,
        object: *const sfConvexShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawRectangleShape(
        rt: *mut sfRenderTexture,
        object: *const sfRectangleShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawVertexBuffer(
        rt: *mut sfRenderTexture,
        object: *const sfVertexBuffer,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawPrimitives(
        rt: *mut sfRenderTexture,
        vertices: *const sfVertex,
        vertex_count: usize,
        type_: sfPrimitiveType,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_createWithSettings(
        width: c_uint,
        height: c_uint,
        settings: *const sfContextSettings,
    ) -> *mut sfRenderTexture;
    pub fn sfRenderTexture_destroy(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_getSize(renderTexture: *const sfRenderTexture) -> sfVector2u;
    pub fn sfRenderTexture_setActive(renderTexture: *mut sfRenderTexture, active: sfBool)
        -> sfBool;
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
    pub fn sfRenderTexture_setSmooth(renderTexture: *mut sfRenderTexture, smooth: sfBool);
    pub fn sfRenderTexture_isSmooth(renderTexture: *const sfRenderTexture) -> sfBool;
    pub fn sfRenderTexture_setRepeated(renderTexture: *mut sfRenderTexture, repeated: sfBool);
    pub fn sfRenderTexture_isRepeated(renderTexture: *const sfRenderTexture) -> sfBool;
    pub fn sfRenderTexture_generateMipmap(renderTexture: *mut sfRenderTexture) -> sfBool;
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
    pub fn sfCircleShape_setTexture(
        shape: *mut sfCircleShape,
        texture: *const sfTexture,
        resetRect: sfBool,
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

    pub fn sfShader_defaultConstruct() -> *mut sfShader;
    pub fn sfShader_loadFromFile_1(
        shader: *mut sfShader,
        path: *const c_char,
        type_: ShaderType,
    ) -> sfBool;
    pub fn sfShader_loadFromMemory_1(
        shader: *mut sfShader,
        path: *const c_char,
        type_: ShaderType,
    ) -> sfBool;
    pub fn sfShader_loadFromStream_1(
        shader: *mut sfShader,
        stream: *mut sfInputStream,
        type_: ShaderType,
    ) -> sfBool;
    pub fn sfShader_loadFromFile_vert_frag(
        shader: *mut sfShader,
        vert: *const c_char,
        frag: *const c_char,
    ) -> sfBool;
    pub fn sfShader_loadFromMemory_vert_frag(
        shader: *mut sfShader,
        vert: *const c_char,
        frag: *const c_char,
    ) -> sfBool;
    pub fn sfShader_loadFromStream_vert_frag(
        shader: *mut sfShader,
        vert: *mut sfInputStream,
        frag: *mut sfInputStream,
    ) -> sfBool;
    pub fn sfShader_loadFromFile_all(
        shader: *mut sfShader,
        vert: *const c_char,
        geom: *const c_char,
        frag: *const c_char,
    ) -> sfBool;
    pub fn sfShader_loadFromMemory_all(
        shader: *mut sfShader,
        vert: *const c_char,
        geom: *const c_char,
        frag: *const c_char,
    ) -> sfBool;
    pub fn sfShader_loadFromStream_all(
        shader: *mut sfShader,
        vert: *mut sfInputStream,
        geom: *mut sfInputStream,
        frag: *mut sfInputStream,
    ) -> sfBool;
    // RenderWindow
    pub fn sfRenderWindow_createUnicode(
        mode: sfVideoMode,
        title: *const sfUint32,
        style: sfUint32,
        settings: *const sfContextSettings,
    ) -> *mut sfRenderWindow;
    pub fn sfRenderWindow_createFromHandle(
        handle: sfWindowHandle,
        settings: *const sfContextSettings,
    ) -> *mut sfRenderWindow;
    pub fn sfRenderWindow_destroy(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_close(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_isOpen(renderWindow: *const sfRenderWindow) -> sfBool;
    pub fn sfRenderWindow_getSettings(
        renderWindow: *const sfRenderWindow,
    ) -> *const sfContextSettings;
    pub(crate) fn sfRenderWindow_pollEvent(
        renderWindow: *mut sfRenderWindow,
        event: *mut Event,
    ) -> sfBool;
    pub(crate) fn sfRenderWindow_waitEvent(
        renderWindow: *mut sfRenderWindow,
        event: *mut Event,
    ) -> sfBool;
    pub fn sfRenderWindow_getPosition(renderWindow: *const sfRenderWindow) -> sfVector2i;
    pub fn sfRenderWindow_setPosition(renderWindow: *mut sfRenderWindow, position: sfVector2i);
    pub fn sfRenderWindow_getSize(renderWindow: *const sfRenderWindow) -> sfVector2u;
    pub fn sfRenderWindow_setSize(renderWindow: *mut sfRenderWindow, size: sfVector2u);
    pub fn sfRenderWindow_setUnicodeTitle(
        renderWindow: *mut sfRenderWindow,
        title: *const sfUint32,
    );
    pub fn sfRenderWindow_setIcon(
        renderWindow: *mut sfRenderWindow,
        width: c_uint,
        height: c_uint,
        pixels: *const sfUint8,
    );
    pub fn sfRenderWindow_setVisible(renderWindow: *mut sfRenderWindow, visible: sfBool);
    pub fn sfRenderWindow_setVerticalSyncEnabled(
        renderWindow: *mut sfRenderWindow,
        enabled: sfBool,
    );
    pub fn sfRenderWindow_setMouseCursorVisible(renderWindow: *mut sfRenderWindow, visible: sfBool);
    pub fn sfRenderWindow_setMouseCursorGrabbed(renderWindow: *mut sfRenderWindow, grabbed: sfBool);
    pub fn sfRenderWindow_setMouseCursor(window: *mut sfRenderWindow, cursor: *const sfCursor);
    pub fn sfRenderWindow_setKeyRepeatEnabled(renderWindow: *mut sfRenderWindow, enabled: sfBool);
    pub fn sfRenderWindow_setActive(renderWindow: *mut sfRenderWindow, active: sfBool) -> sfBool;
    pub fn sfRenderWindow_requestFocus(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_hasFocus(renderWindow: *const sfRenderWindow) -> sfBool;
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
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawText(
        renderWindow: *mut sfRenderWindow,
        object: *const sfText,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawShape(
        renderWindow: *mut sfRenderWindow,
        object: *const sfShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawCircleShape(
        renderWindow: *mut sfRenderWindow,
        object: *const sfCircleShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawConvexShape(
        renderWindow: *mut sfRenderWindow,
        object: *const sfConvexShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawRectangleShape(
        renderWindow: *mut sfRenderWindow,
        object: *const sfRectangleShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawVertexBuffer(
        renderWindow: *mut sfRenderWindow,
        object: *const sfVertexBuffer,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawPrimitives(
        renderWindow: *mut sfRenderWindow,
        vertices: *const sfVertex,
        vertexCount: usize,
        type_: sfPrimitiveType,
        states: *const sfRenderStates,
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

}
