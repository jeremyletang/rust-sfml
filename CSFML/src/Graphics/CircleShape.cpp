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

#include "Graphics/Color.h"
#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/Color.hpp>
#include <cstddef>

extern "C" sf::CircleShape *sfCircleShape_create(void) {
    return new sf::CircleShape;
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
    sf::Vector2f vec2 = shape->getPosition();
    return {vec2.x, vec2.y};
}

extern "C" float sfCircleShape_getRotation(const sf::CircleShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfCircleShape_getScale(const sf::CircleShape *shape) {
    sf::Vector2f vec2 = shape->getScale();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfCircleShape_getOrigin(const sf::CircleShape *shape) {
    sf::Vector2f vec2 = shape->getOrigin();
    return {vec2.x, vec2.y};
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

extern "C" sf::Transform const *sfCircleShape_getTransform(const sf::CircleShape *shape) {
    return &shape->getTransform();
}

extern "C" sf::Transform const *sfCircleShape_getInverseTransform(const sf::CircleShape *shape) {
    return &shape->getInverseTransform();
}

extern "C" void sfCircleShape_setTexture(sf::CircleShape *shape, const sf::Texture *texture, bool resetRect) {
    shape->setTexture(texture, resetRect);
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

extern "C" const sf::Texture *sfCircleShape_getTexture(const sf::CircleShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfCircleShape_getTextureRect(const sf::CircleShape *shape) {
    sf::IntRect rect = shape->getTextureRect();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfColor sfCircleShape_getFillColor(const sf::CircleShape *shape) {
    sf::Color color = shape->getFillColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" sfColor sfCircleShape_getOutlineColor(const sf::CircleShape *shape) {
    sf::Color color = shape->getOutlineColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" float sfCircleShape_getOutlineThickness(const sf::CircleShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfCircleShape_getPointCount(const sf::CircleShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfCircleShape_getPoint(const sf::CircleShape *shape, size_t index) {
    sf::Vector2f vec2 = shape->getPoint(index);
    return {vec2.x, vec2.y};
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
    sf::FloatRect rect = shape->getLocalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfFloatRect sfCircleShape_getGlobalBounds(const sf::CircleShape *shape) {
    sf::FloatRect rect = shape->getGlobalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}
