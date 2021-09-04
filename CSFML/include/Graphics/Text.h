
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

#ifndef SFML_TEXT_H
#define SFML_TEXT_H

// Headers

#include "Graphics/Color.h"

#include "Graphics/Rect.h"
#include "Graphics/Transform.h"
#include "Graphics/Types.h"
#include "System/Vector2.h"
#include <stddef.h>

extern "C" sfText *sfText_create(void);

extern "C" sfText *sfText_copy(const sfText *text);

extern "C" void sfText_destroy(sfText *text);

extern "C" void sfText_setPosition(sfText *text, sfVector2f position);

extern "C" void sfText_setRotation(sfText *text, float angle);

extern "C" void sfText_setScale(sfText *text, sfVector2f scale);

extern "C" void sfText_setOrigin(sfText *text, sfVector2f origin);

extern "C" sfVector2f sfText_getPosition(const sfText *text);

extern "C" float sfText_getRotation(const sfText *text);

extern "C" sfVector2f sfText_getScale(const sfText *text);

extern "C" sfVector2f sfText_getOrigin(const sfText *text);

extern "C" void sfText_move(sfText *text, sfVector2f offset);

extern "C" void sfText_rotate(sfText *text, float angle);

extern "C" void sfText_scale(sfText *text, sfVector2f factors);

extern "C" sfTransform sfText_getTransform(const sfText *text);

extern "C" sfTransform sfText_getInverseTransform(const sfText *text);

extern "C" void sfText_setUnicodeString(sfText *text, const sfUint32 *string);

extern "C" void sfText_setFont(sfText *text, const sfFont *font);

extern "C" void sfText_setCharacterSize(sfText *text, unsigned int size);

extern "C" void sfText_setLineSpacing(sfText *text, float spacingFactor);

extern "C" void sfText_setLetterSpacing(sfText *text, float spacingFactor);

extern "C" void sfText_setStyle(sfText *text, sfUint32 style);

extern "C" void sfText_setFillColor(sfText *text, sfColor color);

extern "C" void sfText_setOutlineColor(sfText *text, sfColor color);

extern "C" void sfText_setOutlineThickness(sfText *text, float thickness);

extern "C" const sfUint32 *sfText_getUnicodeString(const sfText *text);

extern "C" const sfFont *sfText_getFont(const sfText *text);

extern "C" unsigned int sfText_getCharacterSize(const sfText *text);

extern "C" float sfText_getLetterSpacing(const sfText *text);

extern "C" float sfText_getLineSpacing(const sfText *text);

extern "C" sfUint32 sfText_getStyle(const sfText *text);

extern "C" sfColor sfText_getFillColor(const sfText *text);

extern "C" sfColor sfText_getOutlineColor(const sfText *text);

extern "C" float sfText_getOutlineThickness(const sfText *text);

extern "C" sfVector2f sfText_findCharacterPos(const sfText *text, size_t index);

extern "C" sfFloatRect sfText_getLocalBounds(const sfText *text);

extern "C" sfFloatRect sfText_getGlobalBounds(const sfText *text);

#endif // SFML_TEXT_H
