
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
#include "Graphics/Types.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <cstddef>

extern "C" sfRectangleShape *sfRectangleShape_create(void) {
    return reinterpret_cast<sfRectangleShape *>(new sf::RectangleShape);
}

extern "C" sfRectangleShape *sfRectangleShape_copy(const sfRectangleShape *shape) {
    const sf::RectangleShape *src = reinterpret_cast<const sf::RectangleShape *>(shape);
    sf::RectangleShape *newShape = new sf::RectangleShape(*src);
    return reinterpret_cast<sfRectangleShape *>(newShape);
}

extern "C" void sfRectangleShape_destroy(sfRectangleShape *shape) {
    delete reinterpret_cast<sf::RectangleShape *>(shape);
}

extern "C" void sfRectangleShape_setPosition(sfRectangleShape *shape, sfVector2f position) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setPosition(position.x, position.y);
}

extern "C" void sfRectangleShape_setRotation(sfRectangleShape *shape, float angle) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setRotation(angle);
}

extern "C" void sfRectangleShape_setScale(sfRectangleShape *shape, sfVector2f scale) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setScale(scale.x, scale.y);
}

extern "C" void sfRectangleShape_setOrigin(sfRectangleShape *shape, sfVector2f origin) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfRectangleShape_getPosition(const sfRectangleShape *shape) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = reinterpret_cast<const sf::RectangleShape *>(shape)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfRectangleShape_getRotation(const sfRectangleShape *shape) {
    return reinterpret_cast<const sf::RectangleShape *>(shape)->getRotation();
}

extern "C" sfVector2f sfRectangleShape_getScale(const sfRectangleShape *shape) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = reinterpret_cast<const sf::RectangleShape *>(shape)->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfRectangleShape_getOrigin(const sfRectangleShape *shape) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = reinterpret_cast<const sf::RectangleShape *>(shape)->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

extern "C" void sfRectangleShape_move(sfRectangleShape *shape, sfVector2f offset) {
    reinterpret_cast<sf::RectangleShape *>(shape)->move(offset.x, offset.y);
}

extern "C" void sfRectangleShape_rotate(sfRectangleShape *shape, float angle) {
    reinterpret_cast<sf::RectangleShape *>(shape)->rotate(angle);
}

extern "C" void sfRectangleShape_scale(sfRectangleShape *shape, sfVector2f factors) {
    reinterpret_cast<sf::RectangleShape *>(shape)->scale(factors.x, factors.y);
}

extern "C" sf::Transform sfRectangleShape_getTransform(const sfRectangleShape *shape) {
    return reinterpret_cast<const sf::RectangleShape *>(shape)->getTransform();
}

extern "C" sf::Transform sfRectangleShape_getInverseTransform(const sfRectangleShape *shape) {
    return reinterpret_cast<const sf::RectangleShape *>(shape)->getInverseTransform();
}

extern "C" void sfRectangleShape_setTexture(sfRectangleShape *shape, const sfTexture *texture, sfBool resetRect) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setTexture(reinterpret_cast<const sf::Texture *>(texture), resetRect == sfTrue);
}

extern "C" void sfRectangleShape_setTextureRect(sfRectangleShape *shape, sfIntRect rect) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

extern "C" void sfRectangleShape_setFillColor(sfRectangleShape *shape, sfColor color) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfRectangleShape_setOutlineColor(sfRectangleShape *shape, sfColor color) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfRectangleShape_setOutlineThickness(sfRectangleShape *shape, float thickness) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setOutlineThickness(thickness);
}

extern "C" const sfTexture *sfRectangleShape_getTexture(const sfRectangleShape *shape) {
    const sf::Texture *texture = reinterpret_cast<const sf::RectangleShape *>(shape)->getTexture();
    return reinterpret_cast<const sfTexture *>(texture);
}

extern "C" sfIntRect sfRectangleShape_getTextureRect(const sfRectangleShape *shape) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = reinterpret_cast<const sf::RectangleShape *>(shape)->getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfColor sfRectangleShape_getFillColor(const sfRectangleShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = reinterpret_cast<const sf::RectangleShape *>(shape)->getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfColor sfRectangleShape_getOutlineColor(const sfRectangleShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = reinterpret_cast<const sf::RectangleShape *>(shape)->getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" float sfRectangleShape_getOutlineThickness(const sfRectangleShape *shape) {
    return reinterpret_cast<const sf::RectangleShape *>(shape)->getOutlineThickness();
}

extern "C" size_t sfRectangleShape_getPointCount(const sfRectangleShape *shape) {
    return reinterpret_cast<const sf::RectangleShape *>(shape)->getPointCount();
}

extern "C" sfVector2f sfRectangleShape_getPoint(const sfRectangleShape *shape, size_t index) {
    sfVector2f point = {0, 0};

    sf::Vector2f sfmlPoint = reinterpret_cast<const sf::RectangleShape *>(shape)->getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}

extern "C" void sfRectangleShape_setSize(sfRectangleShape *shape, sfVector2f size) {
    reinterpret_cast<sf::RectangleShape *>(shape)->setSize(sf::Vector2f(size.x, size.y));
}

extern "C" sfVector2f sfRectangleShape_getSize(const sfRectangleShape *shape) {
    sfVector2f size = {0, 0};

    sf::Vector2f sfmlSize = reinterpret_cast<const sf::RectangleShape *>(shape)->getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" sfFloatRect sfRectangleShape_getLocalBounds(const sfRectangleShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::RectangleShape *>(shape)->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfRectangleShape_getGlobalBounds(const sfRectangleShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::RectangleShape *>(shape)->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
