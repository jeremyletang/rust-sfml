
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
#include <SFML/Graphics/RectangleShape.hpp>
#include <cstddef>

extern "C" sf::RectangleShape *sfRectangleShape_create(void) {
    return reinterpret_cast<sf::RectangleShape *>(new sf::RectangleShape);
}

extern "C" sf::RectangleShape *sfRectangleShape_copy(const sf::RectangleShape *shape) {
    const sf::RectangleShape *src = shape;
    sf::RectangleShape *newShape = new sf::RectangleShape(*src);
    return newShape;
}

extern "C" void sfRectangleShape_destroy(sf::RectangleShape *shape) {
    delete shape;
}

extern "C" void sfRectangleShape_setPosition(sf::RectangleShape *shape, sfVector2f position) {
    shape->setPosition(position.x, position.y);
}

extern "C" void sfRectangleShape_setRotation(sf::RectangleShape *shape, float angle) {
    shape->setRotation(angle);
}

extern "C" void sfRectangleShape_setScale(sf::RectangleShape *shape, sfVector2f scale) {
    shape->setScale(scale.x, scale.y);
}

extern "C" void sfRectangleShape_setOrigin(sf::RectangleShape *shape, sfVector2f origin) {
    shape->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfRectangleShape_getPosition(const sf::RectangleShape *shape) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = shape->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfRectangleShape_getRotation(const sf::RectangleShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfRectangleShape_getScale(const sf::RectangleShape *shape) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = shape->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfRectangleShape_getOrigin(const sf::RectangleShape *shape) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = shape->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

extern "C" void sfRectangleShape_move(sf::RectangleShape *shape, sfVector2f offset) {
    shape->move(offset.x, offset.y);
}

extern "C" void sfRectangleShape_rotate(sf::RectangleShape *shape, float angle) {
    shape->rotate(angle);
}

extern "C" void sfRectangleShape_scale(sf::RectangleShape *shape, sfVector2f factors) {
    shape->scale(factors.x, factors.y);
}

extern "C" sf::Transform sfRectangleShape_getTransform(const sf::RectangleShape *shape) {
    return shape->getTransform();
}

extern "C" sf::Transform sfRectangleShape_getInverseTransform(const sf::RectangleShape *shape) {
    return shape->getInverseTransform();
}

extern "C" void sfRectangleShape_setTexture(sf::RectangleShape *shape, const sf::Texture *texture, sfBool resetRect) {
    shape->setTexture(texture, resetRect == sfTrue);
}

extern "C" void sfRectangleShape_setTextureRect(sf::RectangleShape *shape, sfIntRect rect) {
    shape->setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfRectangleShape_setFillColor(sf::RectangleShape *shape, sfColor color) {
    shape->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfRectangleShape_setOutlineColor(sf::RectangleShape *shape, sfColor color) {
    shape->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfRectangleShape_setOutlineThickness(sf::RectangleShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfRectangleShape_getTexture(const sf::RectangleShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfRectangleShape_getTextureRect(const sf::RectangleShape *shape) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = shape->getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfColor sfRectangleShape_getFillColor(const sf::RectangleShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfColor sfRectangleShape_getOutlineColor(const sf::RectangleShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" float sfRectangleShape_getOutlineThickness(const sf::RectangleShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfRectangleShape_getPointCount(const sf::RectangleShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfRectangleShape_getPoint(const sf::RectangleShape *shape, size_t index) {
    sfVector2f point = {0, 0};

    sf::Vector2f sfmlPoint = shape->getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}

extern "C" void sfRectangleShape_setSize(sf::RectangleShape *shape, sfVector2f size) {
    shape->setSize(sf::Vector2f(size.x, size.y));
}

extern "C" sfVector2f sfRectangleShape_getSize(const sf::RectangleShape *shape) {
    sfVector2f size = {0, 0};

    sf::Vector2f sfmlSize = shape->getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" sfFloatRect sfRectangleShape_getLocalBounds(const sf::RectangleShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfRectangleShape_getGlobalBounds(const sf::RectangleShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
