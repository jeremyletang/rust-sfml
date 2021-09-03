
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2018 Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

#ifndef SFML_BLENDMODE_H
#define SFML_BLENDMODE_H

// Headers

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

typedef enum {
    sfBlendEquationAdd,            ///< Pixel = Src * SrcFactor + Dst * DstFactor
    sfBlendEquationSubtract,       ///< Pixel = Src * SrcFactor - Dst * DstFactor
    sfBlendEquationReverseSubtract ///< Pixel = Dst * DstFactor - Src * SrcFactor
} sfBlendEquation;

typedef struct
{
    sfBlendFactor colorSrcFactor;  ///< Source blending factor for the color channels
    sfBlendFactor colorDstFactor;  ///< Destination blending factor for the color channels
    sfBlendEquation colorEquation; ///< Blending equation for the color channels
    sfBlendFactor alphaSrcFactor;  ///< Source blending factor for the alpha channel
    sfBlendFactor alphaDstFactor;  ///< Destination blending factor for the alpha channel
    sfBlendEquation alphaEquation; ///< Blending equation for the alpha channel
} sfBlendMode;

#endif // SFML_BLENDMODE_H
