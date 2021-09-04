
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

#ifndef SFML_CIRCLESHAPE_H
#define SFML_CIRCLESHAPE_H

// Headers

#include "Graphics/Color.h"

#include "Graphics/Rect.h"
#include "Graphics/Transform.h"
#include "Graphics/Types.h"
#include "System/Vector2.h"
#include <stddef.h>

extern "C" sfCircleShape *sfCircleShape_create(void);

extern "C" sfCircleShape *sfCircleShape_copy(const sfCircleShape *shape);

extern "C" void sfCircleShape_destroy(sfCircleShape *shape);

extern "C" void sfCircleShape_setPosition(sfCircleShape *shape, sfVector2f position);

extern "C" void sfCircleShape_setRotation(sfCircleShape *shape, float angle);

extern "C" void sfCircleShape_setScale(sfCircleShape *shape, sfVector2f scale);

extern "C" void sfCircleShape_setOrigin(sfCircleShape *shape, sfVector2f origin);

extern "C" sfVector2f sfCircleShape_getPosition(const sfCircleShape *shape);

extern "C" float sfCircleShape_getRotation(const sfCircleShape *shape);

extern "C" sfVector2f sfCircleShape_getScale(const sfCircleShape *shape);

extern "C" sfVector2f sfCircleShape_getOrigin(const sfCircleShape *shape);

extern "C" void sfCircleShape_move(sfCircleShape *shape, sfVector2f offset);

extern "C" void sfCircleShape_rotate(sfCircleShape *shape, float angle);

extern "C" void sfCircleShape_scale(sfCircleShape *shape, sfVector2f factors);

extern "C" sfTransform sfCircleShape_getTransform(const sfCircleShape *shape);

extern "C" sfTransform sfCircleShape_getInverseTransform(const sfCircleShape *shape);

extern "C" void sfCircleShape_setTexture(sfCircleShape *shape, const sfTexture *texture, sfBool resetRect);

extern "C" void sfCircleShape_setTextureRect(sfCircleShape *shape, sfIntRect rect);

extern "C" void sfCircleShape_setFillColor(sfCircleShape *shape, sfColor color);

extern "C" void sfCircleShape_setOutlineColor(sfCircleShape *shape, sfColor color);

extern "C" void sfCircleShape_setOutlineThickness(sfCircleShape *shape, float thickness);

extern "C" const sfTexture *sfCircleShape_getTexture(const sfCircleShape *shape);

extern "C" sfIntRect sfCircleShape_getTextureRect(const sfCircleShape *shape);

extern "C" sfColor sfCircleShape_getFillColor(const sfCircleShape *shape);

extern "C" sfColor sfCircleShape_getOutlineColor(const sfCircleShape *shape);

extern "C" float sfCircleShape_getOutlineThickness(const sfCircleShape *shape);

extern "C" size_t sfCircleShape_getPointCount(const sfCircleShape *shape);

extern "C" sfVector2f sfCircleShape_getPoint(const sfCircleShape *shape, size_t index);

extern "C" void sfCircleShape_setRadius(sfCircleShape *shape, float radius);

extern "C" float sfCircleShape_getRadius(const sfCircleShape *shape);

extern "C" void sfCircleShape_setPointCount(sfCircleShape *shape, size_t count);

extern "C" sfFloatRect sfCircleShape_getLocalBounds(const sfCircleShape *shape);

extern "C" sfFloatRect sfCircleShape_getGlobalBounds(const sfCircleShape *shape);

#endif // SFML_CIRCLESHAPE_H
