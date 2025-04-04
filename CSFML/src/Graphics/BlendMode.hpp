#pragma once
////////////////////////////////////////////////////////////
/// \brief Enumeration of the blending factors
///
////////////////////////////////////////////////////////////
typedef enum {
    sfBlendFactorZero,             ///< (0, 0, 0, 0)
    sfBlendFactorOne,              ///< (1, 1, 1, 1)
    sfBlendFactorSrcColor,         ///< (src.r, src.g, src.b, src.a)
    sfBlendFactorOneMinusSrcColor, ///< (1, 1, 1, 1) - (src.r, src.g, src.b, src.a)
    sfBlendFactorDstColor,         ///< (dst.r, dst.g, dst.b, dst.a)
    sfBlendFactorOneMinusDstColor, ///< (1, 1, 1, 1) - (dst.r, dst.g, dst.b, dst.a)
    sfBlendFactorSrcAlpha,         ///< (src.a, src.a, src.a, src.a)
    sfBlendFactorOneMinusSrcAlpha, ///< (1, 1, 1, 1) - (src.a, src.a, src.a, src.a)
    sfBlendFactorDstAlpha,         ///< (dst.a, dst.a, dst.a, dst.a)
    sfBlendFactorOneMinusDstAlpha  ///< (1, 1, 1, 1) - (dst.a, dst.a, dst.a, dst.a)
} sfBlendFactor;

////////////////////////////////////////////////////////////
/// \brief Enumeration of the blending equations
///
////////////////////////////////////////////////////////////
typedef enum {
    sfBlendEquationAdd,             ///< Pixel = Src * SrcFactor + Dst * DstFactor
    sfBlendEquationSubtract,        ///< Pixel = Src * SrcFactor - Dst * DstFactor
    sfBlendEquationReverseSubtract, ///< Pixel = Dst * DstFactor - Src * SrcFactor
    sfBlendEquationMin,             ///< Pixel = min(Dst, Src)
    sfBlendEquationMax              ///< Pixel = max(Dst, Src)
} sfBlendEquation;

////////////////////////////////////////////////////////////
/// \brief Blending mode for drawing
///
////////////////////////////////////////////////////////////
typedef struct
{
    sfBlendFactor colorSrcFactor;  ///< Source blending factor for the color channels
    sfBlendFactor colorDstFactor;  ///< Destination blending factor for the color channels
    sfBlendEquation colorEquation; ///< Blending equation for the color channels
    sfBlendFactor alphaSrcFactor;  ///< Source blending factor for the alpha channel
    sfBlendFactor alphaDstFactor;  ///< Destination blending factor for the alpha channel
    sfBlendEquation alphaEquation; ///< Blending equation for the alpha channel
} sfBlendMode;
