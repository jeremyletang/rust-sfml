
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
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = shape->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfShape_getRotation(const sfShape *shape) {
    return shape->getRotation();
}

extern "C" sfVector2f sfShape_getScale(const sfShape *shape) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = shape->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfShape_getOrigin(const sfShape *shape) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = shape->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
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
    shape->setTexture(reinterpret_cast<const sf::Texture *>(texture), resetRect);
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
    const sf::Shape *shape_ = reinterpret_cast<const sf::Shape *>(shape);
    return reinterpret_cast<const sf::Texture *>(shape_->getTexture());
}

extern "C" sfIntRect sfShape_getTextureRect(const sfShape *shape) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = shape->getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfColor sfShape_getFillColor(const sfShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfColor sfShape_getOutlineColor(const sfShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" float sfShape_getOutlineThickness(const sfShape *shape) {
    return shape->getOutlineThickness();
}

extern "C" size_t sfShape_getPointCount(const sfShape *shape) {
    return shape->getPointCount();
}

extern "C" sfVector2f sfShape_getPoint(const sfShape *shape, size_t index) {
    sfVector2f point = {0, 0};

    sf::Vector2f sfmlPoint = shape->getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}

extern "C" sfFloatRect sfShape_getLocalBounds(const sfShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfShape_getGlobalBounds(const sfShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" void sfShape_update(sfShape *shape) {
    shape->update();
}
