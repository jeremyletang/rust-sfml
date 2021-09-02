
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

#ifndef SFML_TRANSFORMABLE_H
#define SFML_TRANSFORMABLE_H

// Headers

#include <SFML/Graphics/Transform.h>
#include <SFML/Graphics/Types.h>
#include <SFML/System/Vector2.h>

extern "C" sfTransformable *sfTransformable_create(void);

extern "C" sfTransformable *sfTransformable_copy(const sfTransformable *transformable);

extern "C" void sfTransformable_destroy(sfTransformable *transformable);

extern "C" void sfTransformable_setPosition(sfTransformable *transformable, sfVector2f position);

extern "C" void sfTransformable_setRotation(sfTransformable *transformable, float angle);

extern "C" void sfTransformable_setScale(sfTransformable *transformable, sfVector2f scale);

extern "C" void sfTransformable_setOrigin(sfTransformable *transformable, sfVector2f origin);

extern "C" sfVector2f sfTransformable_getPosition(const sfTransformable *transformable);

extern "C" float sfTransformable_getRotation(const sfTransformable *transformable);

extern "C" sfVector2f sfTransformable_getScale(const sfTransformable *transformable);

extern "C" sfVector2f sfTransformable_getOrigin(const sfTransformable *transformable);

extern "C" void sfTransformable_move(sfTransformable *transformable, sfVector2f offset);

extern "C" void sfTransformable_rotate(sfTransformable *transformable, float angle);

extern "C" void sfTransformable_scale(sfTransformable *transformable, sfVector2f factors);

extern "C" sfTransform sfTransformable_getTransform(const sfTransformable *transformable);

extern "C" sfTransform sfTransformable_getInverseTransform(const sfTransformable *transformable);

#endif // SFML_TRANSFORMABLE_H
