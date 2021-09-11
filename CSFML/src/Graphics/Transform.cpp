
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

// Headers

#include "Graphics/Transform.h"
#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Transform.hpp>
#include <cstddef>
#include <cstring>

extern "C" sf::Transform sfTransform_fromMatrix(float a00, float a01, float a02,
                                                float a10, float a11, float a12,
                                                float a20, float a21, float a22) {
    return sf::Transform(a00, a01, a02, a10, a11, a12, a20, a21, a22);
}

extern "C" const float *sfTransform_getMatrix(const sf::Transform *transform) {
    return transform->getMatrix();
}

extern "C" sf::Transform sfTransform_getInverse(const sf::Transform *transform) {
    return transform->getInverse();
}

extern "C" sfVector2f sfTransform_transformPoint(const sf::Transform *transform, sfVector2f point) {
    sfVector2f p = {0, 0};

    sf::Vector2f sfmlPoint = transform->transformPoint(point.x, point.y);

    p.x = sfmlPoint.x;
    p.y = sfmlPoint.y;

    return p;
}

extern "C" sfFloatRect sfTransform_transformRect(const sf::Transform *transform, sfFloatRect rectangle) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = transform->transformRect(sf::FloatRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));

    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
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
