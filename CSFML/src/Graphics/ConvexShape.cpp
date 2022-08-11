
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

#include "Graphics/Color.h"
#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <cstddef>

extern "C" sf::ConvexShape *sfConvexShape_create(void) {
    return new sf::ConvexShape;
}

extern "C" sf::ConvexShape *sfConvexShape_copy(const sf::ConvexShape *shape) {
    return new sf::ConvexShape(*shape);
}

extern "C" void sfConvexShape_destroy(sf::ConvexShape *shape) {
    delete shape;
}

extern "C" void sfConvexShape_setPosition(sf::ConvexShape *shape, sfVector2f position) {
    shape->setPosition(position.x, position.y);
}

extern "C" void sfConvexShape_setRotation(sf::ConvexShape *shape, float angle) {
    shape->setRotation(angle);
}

extern "C" void sfConvexShape_setScale(sf::ConvexShape *shape, sfVector2f scale) {
    shape->setScale(scale.x, scale.y);
}

extern "C" void sfConvexShape_setOrigin(sf::ConvexShape *shape, sfVector2f origin) {
    shape->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfConvexShape_getPosition(const sf::ConvexShape *shape) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = shape->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfConvexShape_getRotation(const sf::ConvexShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfConvexShape_getScale(const sf::ConvexShape *shape) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = shape->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfConvexShape_getOrigin(const sf::ConvexShape *shape) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = shape->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

extern "C" void sfConvexShape_move(sf::ConvexShape *shape, sfVector2f offset) {
    shape->move(offset.x, offset.y);
}

extern "C" void sfConvexShape_rotate(sf::ConvexShape *shape, float angle) {
    shape->rotate(angle);
}

extern "C" void sfConvexShape_scale(sf::ConvexShape *shape, sfVector2f factors) {
    shape->scale(factors.x, factors.y);
}

extern "C" sf::Transform const *sfConvexShape_getTransform(const sf::ConvexShape *shape) {
    return &shape->getTransform();
}

extern "C" sf::Transform const *sfConvexShape_getInverseTransform(const sf::ConvexShape *shape) {
    return &shape->getInverseTransform();
}

extern "C" void sfConvexShape_setTexture(sf::ConvexShape *shape, const sf::Texture *texture, bool resetRect) {
    shape->setTexture(texture, resetRect);
}

extern "C" void sfConvexShape_setTextureRect(sf::ConvexShape *shape, sfIntRect rect) {
    shape->setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfConvexShape_setFillColor(sf::ConvexShape *shape, sfColor color) {
    shape->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfConvexShape_setOutlineColor(sf::ConvexShape *shape, sfColor color) {
    shape->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfConvexShape_setOutlineThickness(sf::ConvexShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfConvexShape_getTexture(const sf::ConvexShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfConvexShape_getTextureRect(const sf::ConvexShape *shape) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = shape->getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfColor sfConvexShape_getFillColor(const sf::ConvexShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfColor sfConvexShape_getOutlineColor(const sf::ConvexShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" float sfConvexShape_getOutlineThickness(const sf::ConvexShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfConvexShape_getPointCount(const sf::ConvexShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfConvexShape_getPoint(const sf::ConvexShape *shape, size_t index) {
    sfVector2f point = {0, 0};

    sf::Vector2f sfmlPoint = shape->getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}

extern "C" void sfConvexShape_setPointCount(sf::ConvexShape *shape, size_t count) {
    shape->setPointCount(count);
}

extern "C" void sfConvexShape_setPoint(sf::ConvexShape *shape, size_t index, sfVector2f point) {
    shape->setPoint(index, sf::Vector2f(point.x, point.y));
}

extern "C" sfFloatRect sfConvexShape_getLocalBounds(const sf::ConvexShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfConvexShape_getGlobalBounds(const sf::ConvexShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
