
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

#ifndef SFML_TRANSFORM_H
#define SFML_TRANSFORM_H

// Headers

#include "Config.h"
#include "Graphics/Rect.h"
#include "Graphics/Types.h"
#include "System/Vector2.h"

typedef struct
{
    float matrix[9];
} sfTransform;

extern "C" sfTransform sfTransform_fromMatrix(float a00, float a01, float a02,
                                              float a10, float a11, float a12,
                                              float a20, float a21, float a22);

extern "C" void sfTransform_getMatrix(const sfTransform *transform, float *matrix);

extern "C" sfTransform sfTransform_getInverse(const sfTransform *transform);

extern "C" sfVector2f sfTransform_transformPoint(const sfTransform *transform, sfVector2f point);

extern "C" sfFloatRect sfTransform_transformRect(const sfTransform *transform, sfFloatRect rectangle);

extern "C" void sfTransform_combine(sfTransform *transform, const sfTransform *other);

extern "C" void sfTransform_translate(sfTransform *transform, float x, float y);

extern "C" void sfTransform_rotate(sfTransform *transform, float angle);

extern "C" void sfTransform_rotateWithCenter(sfTransform *transform, float angle, float centerX, float centerY);

extern "C" void sfTransform_scale(sfTransform *transform, float scaleX, float scaleY);

extern "C" void sfTransform_scaleWithCenter(sfTransform *transform, float scaleX, float scaleY, float centerX, float centerY);

#endif // SFML_TRANSFORM_H
