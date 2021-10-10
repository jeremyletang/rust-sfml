
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

#include "Config.h"
#include "Graphics/Color.h"
#include "Graphics/ConvexShapeStruct.h"
#include "Graphics/Rect.h"
#include "Graphics/Types.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Color.hpp>
#include <cstddef>

extern "C" sfConvexShape *sfConvexShape_create(void) {
    return new sfConvexShape;
}

extern "C" sfConvexShape *sfConvexShape_copy(const sfConvexShape *shape) {

    return new sfConvexShape(*shape);
}

extern "C" void sfConvexShape_destroy(sfConvexShape *shape) {
    delete shape;
}

extern "C" void sfConvexShape_setPosition(sfConvexShape *shape, sfVector2f position) {
    shape->This.setPosition(position.x, position.y);
}

extern "C" void sfConvexShape_setRotation(sfConvexShape *shape, float angle) {
    shape->This.setRotation(angle);
}

extern "C" void sfConvexShape_setScale(sfConvexShape *shape, sfVector2f scale) {
    shape->This.setScale(scale.x, scale.y);
}

extern "C" void sfConvexShape_setOrigin(sfConvexShape *shape, sfVector2f origin) {
    shape->This.setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfConvexShape_getPosition(const sfConvexShape *shape) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = shape->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfConvexShape_getRotation(const sfConvexShape *shape) {
    return shape->This.getRotation();
}

extern "C" sfVector2f sfConvexShape_getScale(const sfConvexShape *shape) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = shape->This.getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfConvexShape_getOrigin(const sfConvexShape *shape) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = shape->This.getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

extern "C" void sfConvexShape_move(sfConvexShape *shape, sfVector2f offset) {
    shape->This.move(offset.x, offset.y);
}

extern "C" void sfConvexShape_rotate(sfConvexShape *shape, float angle) {
    shape->This.rotate(angle);
}

extern "C" void sfConvexShape_scale(sfConvexShape *shape, sfVector2f factors) {
    shape->This.scale(factors.x, factors.y);
}

extern "C" sf::Transform sfConvexShape_getTransform(const sfConvexShape *shape) {
    return shape->This.getTransform();
}

extern "C" sf::Transform sfConvexShape_getInverseTransform(const sfConvexShape *shape) {
    return shape->This.getInverseTransform();
}

extern "C" void sfConvexShape_setTexture(sfConvexShape *shape, const sfTexture *texture, sfBool resetRect) {
    shape->This.setTexture(reinterpret_cast<const sf::Texture *>(texture), resetRect == sfTrue);
}

extern "C" void sfConvexShape_setTextureRect(sfConvexShape *shape, sfIntRect rect) {
    shape->This.setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfConvexShape_setFillColor(sfConvexShape *shape, sfColor color) {
    shape->This.setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfConvexShape_setOutlineColor(sfConvexShape *shape, sfColor color) {
    shape->This.setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfConvexShape_setOutlineThickness(sfConvexShape *shape, float thickness) {
    shape->This.setOutlineThickness(thickness);
}

extern "C" const sfTexture *sfConvexShape_getTexture(const sfConvexShape *shape) {
    const sf::ConvexShape *shape_ = reinterpret_cast<const sf::ConvexShape *>(shape);
    return reinterpret_cast<const sfTexture *>(shape_->getTexture());
}

extern "C" sfIntRect sfConvexShape_getTextureRect(const sfConvexShape *shape) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = shape->This.getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfColor sfConvexShape_getFillColor(const sfConvexShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->This.getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfColor sfConvexShape_getOutlineColor(const sfConvexShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->This.getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" float sfConvexShape_getOutlineThickness(const sfConvexShape *shape) {
    return shape->This.getOutlineThickness();
}

extern "C" size_t sfConvexShape_getPointCount(const sfConvexShape *shape) {
    return shape->This.getPointCount();
}

extern "C" sfVector2f sfConvexShape_getPoint(const sfConvexShape *shape, size_t index) {
    sfVector2f point = {0, 0};

    sf::Vector2f sfmlPoint = shape->This.getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}

extern "C" void sfConvexShape_setPointCount(sfConvexShape *shape, size_t count) {
    shape->This.setPointCount(count);
}

extern "C" void sfConvexShape_setPoint(sfConvexShape *shape, size_t index, sfVector2f point) {
    shape->This.setPoint(index, sf::Vector2f(point.x, point.y));
}

extern "C" sfFloatRect sfConvexShape_getLocalBounds(const sfConvexShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->This.getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfConvexShape_getGlobalBounds(const sfConvexShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->This.getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
