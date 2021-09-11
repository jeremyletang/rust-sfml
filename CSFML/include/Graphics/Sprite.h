
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

#ifndef SFML_SPRITE_H
#define SFML_SPRITE_H

// Headers

#include "Graphics/Color.h"

#include "Graphics/Rect.h"
#include "Graphics/Transform.h"
#include "Graphics/Types.h"
#include "System/Vector2.h"

extern "C" sfSprite *sfSprite_create(void);

extern "C" sfSprite *sfSprite_copy(const sfSprite *sprite);

extern "C" void sfSprite_destroy(sfSprite *sprite);

extern "C" void sfSprite_setPosition(sfSprite *sprite, sfVector2f position);

extern "C" void sfSprite_setRotation(sfSprite *sprite, float angle);

extern "C" void sfSprite_setScale(sfSprite *sprite, sfVector2f scale);

extern "C" void sfSprite_setOrigin(sfSprite *sprite, sfVector2f origin);

extern "C" sfVector2f sfSprite_getPosition(const sfSprite *sprite);

extern "C" float sfSprite_getRotation(const sfSprite *sprite);

extern "C" sfVector2f sfSprite_getScale(const sfSprite *sprite);

extern "C" sfVector2f sfSprite_getOrigin(const sfSprite *sprite);

extern "C" void sfSprite_move(sfSprite *sprite, sfVector2f offset);

extern "C" void sfSprite_rotate(sfSprite *sprite, float angle);

extern "C" void sfSprite_scale(sfSprite *sprite, sfVector2f factors);

extern "C" void sfSprite_setTexture(sfSprite *sprite, const sfTexture *texture, sfBool resetRect);

extern "C" void sfSprite_setTextureRect(sfSprite *sprite, sfIntRect rectangle);

extern "C" void sfSprite_setColor(sfSprite *sprite, sfColor color);

extern "C" const sfTexture *sfSprite_getTexture(const sfSprite *sprite);

extern "C" sfIntRect sfSprite_getTextureRect(const sfSprite *sprite);

extern "C" sfColor sfSprite_getColor(const sfSprite *sprite);

extern "C" sfFloatRect sfSprite_getLocalBounds(const sfSprite *sprite);

extern "C" sfFloatRect sfSprite_getGlobalBounds(const sfSprite *sprite);

#endif // SFML_SPRITE_H
