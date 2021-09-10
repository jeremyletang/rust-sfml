
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

#ifndef SFML_RECTANGLESHAPE_H
#define SFML_RECTANGLESHAPE_H

// Headers

#include "Graphics/Color.h"

#include "Graphics/Rect.h"
#include "Graphics/Transform.h"
#include "Graphics/Types.h"
#include "System/Vector2.h"
#include <stddef.h>

extern "C" sfRectangleShape *sfRectangleShape_create(void);

extern "C" sfRectangleShape *sfRectangleShape_copy(const sfRectangleShape *shape);

extern "C" void sfRectangleShape_destroy(sfRectangleShape *shape);

extern "C" void sfRectangleShape_setPosition(sfRectangleShape *shape, sfVector2f position);

extern "C" void sfRectangleShape_setRotation(sfRectangleShape *shape, float angle);

extern "C" void sfRectangleShape_setScale(sfRectangleShape *shape, sfVector2f scale);

extern "C" void sfRectangleShape_setOrigin(sfRectangleShape *shape, sfVector2f origin);

extern "C" sfVector2f sfRectangleShape_getPosition(const sfRectangleShape *shape);

extern "C" float sfRectangleShape_getRotation(const sfRectangleShape *shape);

extern "C" sfVector2f sfRectangleShape_getScale(const sfRectangleShape *shape);

extern "C" sfVector2f sfRectangleShape_getOrigin(const sfRectangleShape *shape);

extern "C" void sfRectangleShape_move(sfRectangleShape *shape, sfVector2f offset);

extern "C" void sfRectangleShape_rotate(sfRectangleShape *shape, float angle);

extern "C" void sfRectangleShape_scale(sfRectangleShape *shape, sfVector2f factors);

extern "C" void sfRectangleShape_setTexture(sfRectangleShape *shape, const sfTexture *texture, sfBool resetRect);

extern "C" void sfRectangleShape_setTextureRect(sfRectangleShape *shape, sfIntRect rect);

extern "C" void sfRectangleShape_setFillColor(sfRectangleShape *shape, sfColor color);

extern "C" void sfRectangleShape_setOutlineColor(sfRectangleShape *shape, sfColor color);

extern "C" void sfRectangleShape_setOutlineThickness(sfRectangleShape *shape, float thickness);

extern "C" const sfTexture *sfRectangleShape_getTexture(const sfRectangleShape *shape);

extern "C" sfIntRect sfRectangleShape_getTextureRect(const sfRectangleShape *shape);

extern "C" sfColor sfRectangleShape_getFillColor(const sfRectangleShape *shape);

extern "C" sfColor sfRectangleShape_getOutlineColor(const sfRectangleShape *shape);

extern "C" float sfRectangleShape_getOutlineThickness(const sfRectangleShape *shape);

extern "C" size_t sfRectangleShape_getPointCount(const sfRectangleShape *shape);

extern "C" sfVector2f sfRectangleShape_getPoint(const sfRectangleShape *shape, size_t index);

extern "C" void sfRectangleShape_setSize(sfRectangleShape *shape, sfVector2f size);

extern "C" sfVector2f sfRectangleShape_getSize(const sfRectangleShape *shape);

extern "C" sfFloatRect sfRectangleShape_getLocalBounds(const sfRectangleShape *shape);

extern "C" sfFloatRect sfRectangleShape_getGlobalBounds(const sfRectangleShape *shape);

#endif // SFML_RECTANGLESHAPE_H
