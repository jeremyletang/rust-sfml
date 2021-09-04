
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
#include <SFML/Graphics/Image.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include "Graphics/Sprite.h"
#include <cstddef>

sfSprite *sfSprite_create(void) {
    sf::Sprite *sprite = new sf::Sprite;
    return reinterpret_cast<sfSprite*>(sprite);
}

sfSprite *sfSprite_copy(const sfSprite *sprite) {
    const sf::Sprite * src = reinterpret_cast<const sf::Sprite*>(sprite);
    sf::Sprite * newSprite = new sf::Sprite(*src);
    return reinterpret_cast<sfSprite*>(newSprite);
}

void sfSprite_destroy(sfSprite *sprite) {
    delete reinterpret_cast<sf::Sprite*>(sprite);
}

void sfSprite_setPosition(sfSprite *sprite, sfVector2f position) {
    reinterpret_cast<sf::Sprite*>(sprite)->setPosition(position.x, position.y);
}

void sfSprite_setRotation(sfSprite *sprite, float angle) {
    reinterpret_cast<sf::Sprite*>(sprite)->setRotation(angle);
}

void sfSprite_setScale(sfSprite *sprite, sfVector2f scale) {
    reinterpret_cast<sf::Sprite*>(sprite)->setScale(scale.x, scale.y);
}

void sfSprite_setOrigin(sfSprite *sprite, sfVector2f origin) {
    reinterpret_cast<sf::Sprite*>(sprite)->setOrigin(origin.x, origin.y);
}

sfVector2f sfSprite_getPosition(const sfSprite *sprite) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = reinterpret_cast<const sf::Sprite*>(sprite)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

float sfSprite_getRotation(const sfSprite *sprite) {
    return reinterpret_cast<const sf::Sprite*>(sprite)->getRotation();
}

sfVector2f sfSprite_getScale(const sfSprite *sprite) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = reinterpret_cast<const sf::Sprite*>(sprite)->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

sfVector2f sfSprite_getOrigin(const sfSprite *sprite) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = reinterpret_cast<const sf::Sprite*>(sprite)->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

void sfSprite_move(sfSprite *sprite, sfVector2f offset) {
    reinterpret_cast<sf::Sprite*>(sprite)->move(offset.x, offset.y);
}

void sfSprite_rotate(sfSprite *sprite, float angle) {
    reinterpret_cast<sf::Sprite*>(sprite)->rotate(angle);
}

void sfSprite_scale(sfSprite *sprite, sfVector2f factors) {
    reinterpret_cast<sf::Sprite*>(sprite)->scale(factors.x, factors.y);
}

sfTransform sfSprite_getTransform(const sfSprite *sprite) {

    sfTransform transform = convertTransform(reinterpret_cast<const sf::Sprite*>(sprite)->getTransform());
    return transform;
}

sfTransform sfSprite_getInverseTransform(const sfSprite *sprite) {

    sfTransform transform = convertTransform(reinterpret_cast<const sf::Sprite*>(sprite)->getInverseTransform());
    return transform;
}

void sfSprite_setTexture(sfSprite *sprite, const sfTexture *texture, sfBool resetRect) {
    reinterpret_cast<sf::Sprite*>(sprite)->setTexture(*reinterpret_cast<const sf::Texture*>(texture), resetRect == sfTrue);
}

void sfSprite_setTextureRect(sfSprite *sprite, sfIntRect rectangle) {
    reinterpret_cast<sf::Sprite*>(sprite)->setTextureRect(sf::IntRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
}

void sfSprite_setColor(sfSprite *sprite, sfColor color) {
    reinterpret_cast<sf::Sprite*>(sprite)->setColor(sf::Color(color.r, color.g, color.b, color.a));
}

const sfTexture *sfSprite_getTexture(const sfSprite *sprite) {
    const sf::Sprite * sprite_ = reinterpret_cast<const sf::Sprite *>(sprite);
    return reinterpret_cast<const sfTexture*>(sprite_->getTexture());
}

sfIntRect sfSprite_getTextureRect(const sfSprite *sprite) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = reinterpret_cast<const sf::Sprite*>(sprite)->getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

sfColor sfSprite_getColor(const sfSprite *sprite) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = reinterpret_cast<const sf::Sprite*>(sprite)->getColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

sfFloatRect sfSprite_getLocalBounds(const sfSprite *sprite) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::Sprite*>(sprite)->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

sfFloatRect sfSprite_getGlobalBounds(const sfSprite *sprite) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::Sprite*>(sprite)->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
