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

#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Transform.hpp>
#include <cstddef>
#include <cstring>

extern "C" sfVector2f sfTransform_transformPoint(const sf::Transform *transform, sfVector2f point) {
    sf::Vector2f vec2 = transform->transformPoint(point.x, point.y);
    return {vec2.x, vec2.y};
}

extern "C" sfFloatRect sfTransform_transformRect(const sf::Transform *transform, sfFloatRect rectangle) {
    sf::FloatRect rect = transform->transformRect(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" void sfTransform_combine(sf::Transform *transform, const sf::Transform *other) {
    transform->combine(*other);
}

extern "C" void sfTransform_translate(sf::Transform *transform, float x, float y) {
    transform->translate(x, y);
}

extern "C" void sfTransform_rotate(sf::Transform *transform, float angle) {
    transform->rotate(angle);
}

extern "C" void sfTransform_rotateWithCenter(sf::Transform *transform, float angle, float centerX, float centerY) {
    transform->rotate(angle, centerX, centerY);
}

extern "C" void sfTransform_scale(sf::Transform *transform, float scaleX, float scaleY) {
    transform->scale(scaleX, scaleY);
}

extern "C" void sfTransform_scaleWithCenter(sf::Transform *transform, float scaleX, float scaleY, float centerX, float centerY) {
    transform->scale(scaleX, scaleY, centerX, centerY);
}
