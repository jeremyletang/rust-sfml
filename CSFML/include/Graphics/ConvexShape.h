
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

#ifndef SFML_CONVEXSHAPE_H
#define SFML_CONVEXSHAPE_H

// Headers

#include "Graphics/Color.h"

#include "Graphics/Rect.h"
#include "Graphics/Transform.h"
#include "Graphics/Types.h"
#include "System/Vector2.h"
#include <stddef.h>

extern "C" sfConvexShape *sfConvexShape_create(void);

extern "C" sfConvexShape *sfConvexShape_copy(const sfConvexShape *shape);

extern "C" void sfConvexShape_destroy(sfConvexShape *shape);

extern "C" void sfConvexShape_setPosition(sfConvexShape *shape, sfVector2f position);

extern "C" void sfConvexShape_setRotation(sfConvexShape *shape, float angle);

extern "C" void sfConvexShape_setScale(sfConvexShape *shape, sfVector2f scale);

extern "C" void sfConvexShape_setOrigin(sfConvexShape *shape, sfVector2f origin);

extern "C" sfVector2f sfConvexShape_getPosition(const sfConvexShape *shape);

extern "C" float sfConvexShape_getRotation(const sfConvexShape *shape);

extern "C" sfVector2f sfConvexShape_getScale(const sfConvexShape *shape);

extern "C" sfVector2f sfConvexShape_getOrigin(const sfConvexShape *shape);

extern "C" void sfConvexShape_move(sfConvexShape *shape, sfVector2f offset);

extern "C" void sfConvexShape_rotate(sfConvexShape *shape, float angle);

extern "C" void sfConvexShape_scale(sfConvexShape *shape, sfVector2f factors);

extern "C" sfTransform sfConvexShape_getTransform(const sfConvexShape *shape);

extern "C" sfTransform sfConvexShape_getInverseTransform(const sfConvexShape *shape);

extern "C" void sfConvexShape_setTexture(sfConvexShape *shape, const sfTexture *texture, sfBool resetRect);

extern "C" void sfConvexShape_setTextureRect(sfConvexShape *shape, sfIntRect rect);

extern "C" void sfConvexShape_setFillColor(sfConvexShape *shape, sfColor color);

extern "C" void sfConvexShape_setOutlineColor(sfConvexShape *shape, sfColor color);

extern "C" void sfConvexShape_setOutlineThickness(sfConvexShape *shape, float thickness);

extern "C" const sfTexture *sfConvexShape_getTexture(const sfConvexShape *shape);

extern "C" sfIntRect sfConvexShape_getTextureRect(const sfConvexShape *shape);

extern "C" sfColor sfConvexShape_getFillColor(const sfConvexShape *shape);

extern "C" sfColor sfConvexShape_getOutlineColor(const sfConvexShape *shape);

extern "C" float sfConvexShape_getOutlineThickness(const sfConvexShape *shape);

extern "C" size_t sfConvexShape_getPointCount(const sfConvexShape *shape);

extern "C" sfVector2f sfConvexShape_getPoint(const sfConvexShape *shape, size_t index);

extern "C" void sfConvexShape_setPointCount(sfConvexShape *shape, size_t count);

extern "C" void sfConvexShape_setPoint(sfConvexShape *shape, size_t index, sfVector2f point);

extern "C" sfFloatRect sfConvexShape_getLocalBounds(const sfConvexShape *shape);

extern "C" sfFloatRect sfConvexShape_getGlobalBounds(const sfConvexShape *shape);

#endif // SFML_CONVEXSHAPE_H
