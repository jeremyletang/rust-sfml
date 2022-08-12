
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
#include "Graphics/ShapeStruct.h"
#include <SFML/Graphics/Color.hpp>
#include <cstddef>

extern "C" sfShape *sfShape_create(sfShapeGetPointCountCallback getPointCount,
                                   sfShapeGetPointCallback getPoint,
                                   void *userData) {
    return new sfShape(getPointCount, getPoint, userData);
}

extern "C" void sfShape_destroy(sfShape *shape) {
    delete shape;
}

extern "C" void sfShape_setPosition(sfShape *shape, sfVector2f position) {
    shape->setPosition(position.x, position.y);
}

extern "C" void sfShape_setRotation(sfShape *shape, float angle) {
    shape->setRotation(angle);
}

extern "C" void sfShape_setScale(sfShape *shape, sfVector2f scale) {
    shape->setScale(scale.x, scale.y);
}

extern "C" void sfShape_setOrigin(sfShape *shape, sfVector2f origin) {
    shape->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfShape_getPosition(const sfShape *shape) {
    sf::Vector2f vec2 = shape->getPosition();
    return {vec2.x, vec2.y};
}

extern "C" float sfShape_getRotation(const sfShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfShape_getScale(const sfShape *shape) {
    sf::Vector2f vec2 = shape->getScale();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfShape_getOrigin(const sfShape *shape) {
    sf::Vector2f vec2 = shape->getOrigin();
    return {vec2.x, vec2.y};
}

extern "C" void sfShape_move(sfShape *shape, sfVector2f offset) {
    shape->move(offset.x, offset.y);
}

extern "C" void sfShape_rotate(sfShape *shape, float angle) {
    shape->rotate(angle);
}

extern "C" void sfShape_scale(sfShape *shape, sfVector2f factors) {
    shape->scale(factors.x, factors.y);
}

extern "C" sf::Transform const *sfShape_getTransform(const sfShape *shape) {
    return &shape->getTransform();
}

extern "C" sf::Transform const *sfShape_getInverseTransform(const sfShape *shape) {
    return &shape->getInverseTransform();
}

extern "C" void sfShape_setTexture(sfShape *shape, const sf::Texture *texture, bool resetRect) {
    shape->setTexture(texture, resetRect);
}

extern "C" void sfShape_setTextureRect(sfShape *shape, sfIntRect rect) {
    shape->setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfShape_setFillColor(sfShape *shape, sfColor color) {
    shape->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfShape_setOutlineColor(sfShape *shape, sfColor color) {
    shape->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfShape_setOutlineThickness(sfShape *shape, float thickness) {
    shape->setOutlineThickness(thickness);
}

extern "C" const sf::Texture *sfShape_getTexture(const sfShape *shape) {
    return shape->getTexture();
}

extern "C" sfIntRect sfShape_getTextureRect(const sfShape *shape) {
    sf::IntRect rect = shape->getTextureRect();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfColor sfShape_getFillColor(const sfShape *shape) {
    sf::Color color = shape->getFillColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" sfColor sfShape_getOutlineColor(const sfShape *shape) {
    sf::Color color = shape->getOutlineColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" float sfShape_getOutlineThickness(const sfShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfShape_getPointCount(const sfShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfShape_getPoint(const sfShape *shape, size_t index) {
    sf::Vector2f vec2 = shape->getPoint(index);
    return {vec2.x, vec2.y};
}

extern "C" sfFloatRect sfShape_getLocalBounds(const sfShape *shape) {
    sf::FloatRect rect = shape->getLocalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfFloatRect sfShape_getGlobalBounds(const sfShape *shape) {
    sf::FloatRect rect = shape->getGlobalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" void sfShape_update(sfShape *shape) {
    shape->update();
}
