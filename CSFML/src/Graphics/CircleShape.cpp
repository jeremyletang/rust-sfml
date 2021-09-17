
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
#include "Graphics/Texture.h"
#include "System/Vector2.h"
#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/Color.hpp>
#include <cstddef>

extern "C" sf::CircleShape *sfCircleShape_create(void) {
    sf::CircleShape *shape = new sf::CircleShape;

    return shape;
}

extern "C" sf::CircleShape *sfCircleShape_copy(const sf::CircleShape *shape) {

    return new sf::CircleShape(*shape);
}

extern "C" void sfCircleShape_destroy(sf::CircleShape *shape) {
    delete shape;
}

extern "C" void sfCircleShape_setPosition(sf::CircleShape *shape, sfVector2f position) {
    shape->setPosition(position.x, position.y);
}

extern "C" void sfCircleShape_setRotation(sf::CircleShape *shape, float angle) {
    shape->setRotation(angle);
}

extern "C" void sfCircleShape_setScale(sf::CircleShape *shape, sfVector2f scale) {
    shape->setScale(scale.x, scale.y);
}

extern "C" void sfCircleShape_setOrigin(sf::CircleShape *shape, sfVector2f origin) {
    shape->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfCircleShape_getPosition(const sf::CircleShape *shape) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = shape->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfCircleShape_getRotation(const sf::CircleShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfCircleShape_getScale(const sf::CircleShape *shape) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = shape->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfCircleShape_getOrigin(const sf::CircleShape *shape) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = shape->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

extern "C" void sfCircleShape_move(sf::CircleShape *shape, sfVector2f offset) {
    shape->move(offset.x, offset.y);
}

extern "C" void sfCircleShape_rotate(sf::CircleShape *shape, float angle) {
    shape->rotate(angle);
}

extern "C" void sfCircleShape_scale(sf::CircleShape *shape, sfVector2f factors) {
    shape->scale(factors.x, factors.y);
}

extern "C" sf::Transform sfCircleShape_getTransform(const sf::CircleShape *shape) {
    return shape->getTransform();
}

extern "C" sf::Transform sfCircleShape_getInverseTransform(const sf::CircleShape *shape) {
    return shape->getInverseTransform();
}

extern "C" void sfCircleShape_setTexture(sf::CircleShape *shape, const sfTexture *texture, sfBool resetRect) {
    shape->setTexture(reinterpret_cast<const sf::Texture *>(texture), resetRect == sfTrue);
}

extern "C" void sfCircleShape_setTextureRect(sf::CircleShape *shape, sfIntRect rect) {
    shape->setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfCircleShape_setFillColor(sf::CircleShape *shape, sfColor color) {
    shape->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfCircleShape_setOutlineColor(sf::CircleShape *shape, sfColor color) {
    shape->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfCircleShape_setOutlineThickness(sf::CircleShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sfTexture *sfCircleShape_getTexture(const sf::CircleShape *shape) {
    const sf::CircleShape *shape_ = reinterpret_cast<const sf::CircleShape *>(shape);
    return reinterpret_cast<const sfTexture *>(shape_->getTexture());
}

extern "C" sfIntRect sfCircleShape_getTextureRect(const sf::CircleShape *shape) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = shape->getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfColor sfCircleShape_getFillColor(const sf::CircleShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfColor sfCircleShape_getOutlineColor(const sf::CircleShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" float sfCircleShape_getOutlineThickness(const sf::CircleShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfCircleShape_getPointCount(const sf::CircleShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfCircleShape_getPoint(const sf::CircleShape *shape, size_t index) {
    sfVector2f point = {0, 0};

    sf::Vector2f sfmlPoint = shape->getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}

extern "C" void sfCircleShape_setRadius(sf::CircleShape *shape, float radius) {
    shape->setRadius(radius);
}

extern "C" float sfCircleShape_getRadius(const sf::CircleShape *shape) {
    return shape->getRadius();
}

extern "C" void sfCircleShape_setPointCount(sf::CircleShape *shape, size_t count) {
    shape->setPointCount(count);
}

extern "C" sfFloatRect sfCircleShape_getLocalBounds(const sf::CircleShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfCircleShape_getGlobalBounds(const sf::CircleShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
