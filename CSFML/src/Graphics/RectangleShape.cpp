
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

#include <SFML/Graphics/Color.hpp>
#include "Graphics/ConvertTransform.hpp"
#include "Graphics/RectangleShape.h"
#include "Graphics/RectangleShapeStruct.h"
#include <cstddef>

sfRectangleShape *sfRectangleShape_create(void) {
    return new sfRectangleShape;
}

sfRectangleShape *sfRectangleShape_copy(const sfRectangleShape *shape) {

    return new sfRectangleShape(*shape);
}

void sfRectangleShape_destroy(sfRectangleShape *shape) {
    delete shape;
}

void sfRectangleShape_setPosition(sfRectangleShape *shape, sfVector2f position) {
    shape->This.setPosition(position.x, position.y);
}

void sfRectangleShape_setRotation(sfRectangleShape *shape, float angle) {
    shape->This.setRotation(angle);
}

void sfRectangleShape_setScale(sfRectangleShape *shape, sfVector2f scale) {
    shape->This.setScale(scale.x, scale.y);
}

void sfRectangleShape_setOrigin(sfRectangleShape *shape, sfVector2f origin) {
    shape->This.setOrigin(origin.x, origin.y);
}

sfVector2f sfRectangleShape_getPosition(const sfRectangleShape *shape) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = shape->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

float sfRectangleShape_getRotation(const sfRectangleShape *shape) {
    return shape->This.getRotation();
}

sfVector2f sfRectangleShape_getScale(const sfRectangleShape *shape) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = shape->This.getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

sfVector2f sfRectangleShape_getOrigin(const sfRectangleShape *shape) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = shape->This.getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

void sfRectangleShape_move(sfRectangleShape *shape, sfVector2f offset) {
    shape->This.move(offset.x, offset.y);
}

void sfRectangleShape_rotate(sfRectangleShape *shape, float angle) {
    shape->This.rotate(angle);
}

void sfRectangleShape_scale(sfRectangleShape *shape, sfVector2f factors) {
    shape->This.scale(factors.x, factors.y);
}

sfTransform sfRectangleShape_getTransform(const sfRectangleShape *shape) {

    shape->Transform = convertTransform(shape->This.getTransform());
    return shape->Transform;
}

sfTransform sfRectangleShape_getInverseTransform(const sfRectangleShape *shape) {

    shape->InverseTransform = convertTransform(shape->This.getInverseTransform());
    return shape->InverseTransform;
}

void sfRectangleShape_setTexture(sfRectangleShape *shape, const sfTexture *texture, sfBool resetRect) {
    shape->This.setTexture(reinterpret_cast<const sf::Texture*>(texture), resetRect == sfTrue);
    shape->Texture = texture;
}

void sfRectangleShape_setTextureRect(sfRectangleShape *shape, sfIntRect rect) {
    shape->This.setTextureRect(sf::IntRect(rect.left, rect.top, rect.width, rect.height));
}

void sfRectangleShape_setFillColor(sfRectangleShape *shape, sfColor color) {
    shape->This.setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

void sfRectangleShape_setOutlineColor(sfRectangleShape *shape, sfColor color) {
    shape->This.setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

void sfRectangleShape_setOutlineThickness(sfRectangleShape *shape, float thickness) {
    shape->This.setOutlineThickness(thickness);
}

const sfTexture *sfRectangleShape_getTexture(const sfRectangleShape *shape) {

    return shape->Texture;
}

sfIntRect sfRectangleShape_getTextureRect(const sfRectangleShape *shape) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = shape->This.getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

sfColor sfRectangleShape_getFillColor(const sfRectangleShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->This.getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

sfColor sfRectangleShape_getOutlineColor(const sfRectangleShape *shape) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = shape->This.getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

float sfRectangleShape_getOutlineThickness(const sfRectangleShape *shape) {
    return shape->This.getOutlineThickness();
}

size_t sfRectangleShape_getPointCount(const sfRectangleShape *shape) {
    return shape->This.getPointCount();
}

sfVector2f sfRectangleShape_getPoint(const sfRectangleShape *shape, size_t index) {
    sfVector2f point = {0, 0};

    sf::Vector2f sfmlPoint = shape->This.getPoint(index);
    point.x = sfmlPoint.x;
    point.y = sfmlPoint.y;

    return point;
}

void sfRectangleShape_setSize(sfRectangleShape *shape, sfVector2f size) {
    shape->This.setSize(sf::Vector2f(size.x, size.y));
}

sfVector2f sfRectangleShape_getSize(const sfRectangleShape *shape) {
    sfVector2f size = {0, 0};

    sf::Vector2f sfmlSize = shape->This.getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

sfFloatRect sfRectangleShape_getLocalBounds(const sfRectangleShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->This.getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

sfFloatRect sfRectangleShape_getGlobalBounds(const sfRectangleShape *shape) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = shape->This.getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
