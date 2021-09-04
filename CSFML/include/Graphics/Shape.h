
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

#ifndef SFML_SHAPE_H
#define SFML_SHAPE_H

// Headers

#include "Graphics/Color.h"

#include "Graphics/Rect.h"
#include "Graphics/Transform.h"
#include "Graphics/Types.h"
#include "System/Vector2.h"
#include <stddef.h>

typedef size_t (*sfShapeGetPointCountCallback)(void *);        ///< Type of the callback used to get the number of points in a shape
typedef sfVector2f (*sfShapeGetPointCallback)(size_t, void *); ///< Type of the callback used to get a point of a shape

extern "C" sfShape *sfShape_create(sfShapeGetPointCountCallback getPointCount,
                                   sfShapeGetPointCallback getPoint,
                                   void *userData);

extern "C" void sfShape_destroy(sfShape *shape);

extern "C" void sfShape_setPosition(sfShape *shape, sfVector2f position);

extern "C" void sfShape_setRotation(sfShape *shape, float angle);

extern "C" void sfShape_setScale(sfShape *shape, sfVector2f scale);

extern "C" void sfShape_setOrigin(sfShape *shape, sfVector2f origin);

extern "C" sfVector2f sfShape_getPosition(const sfShape *shape);

extern "C" float sfShape_getRotation(const sfShape *shape);

extern "C" sfVector2f sfShape_getScale(const sfShape *shape);

extern "C" sfVector2f sfShape_getOrigin(const sfShape *shape);

extern "C" void sfShape_move(sfShape *shape, sfVector2f offset);

extern "C" void sfShape_rotate(sfShape *shape, float angle);

extern "C" void sfShape_scale(sfShape *shape, sfVector2f factors);

extern "C" sfTransform sfShape_getTransform(const sfShape *shape);

extern "C" sfTransform sfShape_getInverseTransform(const sfShape *shape);

extern "C" void sfShape_setTexture(sfShape *shape, const sfTexture *texture, sfBool resetRect);

extern "C" void sfShape_setTextureRect(sfShape *shape, sfIntRect rect);

extern "C" void sfShape_setFillColor(sfShape *shape, sfColor color);

extern "C" void sfShape_setOutlineColor(sfShape *shape, sfColor color);

extern "C" void sfShape_setOutlineThickness(sfShape *shape, float thickness);

extern "C" const sfTexture *sfShape_getTexture(const sfShape *shape);

extern "C" sfIntRect sfShape_getTextureRect(const sfShape *shape);

extern "C" sfColor sfShape_getFillColor(const sfShape *shape);

extern "C" sfColor sfShape_getOutlineColor(const sfShape *shape);

extern "C" float sfShape_getOutlineThickness(const sfShape *shape);

extern "C" size_t sfShape_getPointCount(const sfShape *shape);

extern "C" sfVector2f sfShape_getPoint(const sfShape *shape, size_t index);

extern "C" sfFloatRect sfShape_getLocalBounds(const sfShape *shape);

extern "C" sfFloatRect sfShape_getGlobalBounds(const sfShape *shape);

extern "C" void sfShape_update(sfShape *shape);

#endif // SFML_SHAPE_H
