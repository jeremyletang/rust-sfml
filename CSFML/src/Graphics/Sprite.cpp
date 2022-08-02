
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
#include "Graphics/Rect.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Image.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <cstddef>

extern "C" sf::Sprite *sfSprite_create(void) {
    sf::Sprite *sprite = new sf::Sprite;
    return reinterpret_cast<sf::Sprite *>(sprite);
}

extern "C" sf::Sprite *sfSprite_copy(const sf::Sprite *sprite) {
    const sf::Sprite *src = reinterpret_cast<const sf::Sprite *>(sprite);
    sf::Sprite *newSprite = new sf::Sprite(*src);
    return reinterpret_cast<sf::Sprite *>(newSprite);
}

extern "C" void sfSprite_destroy(sf::Sprite *sprite) {
    delete reinterpret_cast<sf::Sprite *>(sprite);
}

extern "C" void sfSprite_setPosition(sf::Sprite *sprite, sfVector2f position) {
    reinterpret_cast<sf::Sprite *>(sprite)->setPosition(position.x, position.y);
}

extern "C" void sfSprite_setRotation(sf::Sprite *sprite, float angle) {
    reinterpret_cast<sf::Sprite *>(sprite)->setRotation(angle);
}

extern "C" void sfSprite_setScale(sf::Sprite *sprite, sfVector2f scale) {
    reinterpret_cast<sf::Sprite *>(sprite)->setScale(scale.x, scale.y);
}

extern "C" void sfSprite_setOrigin(sf::Sprite *sprite, sfVector2f origin) {
    reinterpret_cast<sf::Sprite *>(sprite)->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfSprite_getPosition(const sf::Sprite *sprite) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = reinterpret_cast<const sf::Sprite *>(sprite)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfSprite_getRotation(const sf::Sprite *sprite) {
    return reinterpret_cast<const sf::Sprite *>(sprite)->getRotation();
}

extern "C" sfVector2f sfSprite_getScale(const sf::Sprite *sprite) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = reinterpret_cast<const sf::Sprite *>(sprite)->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfSprite_getOrigin(const sf::Sprite *sprite) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = reinterpret_cast<const sf::Sprite *>(sprite)->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

extern "C" void sfSprite_move(sf::Sprite *sprite, sfVector2f offset) {
    reinterpret_cast<sf::Sprite *>(sprite)->move(offset.x, offset.y);
}

extern "C" void sfSprite_rotate(sf::Sprite *sprite, float angle) {
    reinterpret_cast<sf::Sprite *>(sprite)->rotate(angle);
}

extern "C" void sfSprite_scale(sf::Sprite *sprite, sfVector2f factors) {
    reinterpret_cast<sf::Sprite *>(sprite)->scale(factors.x, factors.y);
}

extern "C" sf::Transform sfSprite_getTransform(const sf::Sprite *sprite) {
    return reinterpret_cast<const sf::Sprite *>(sprite)->getTransform();
}

extern "C" sf::Transform sfSprite_getInverseTransform(const sf::Sprite *sprite) {
    return reinterpret_cast<const sf::Sprite *>(sprite)->getInverseTransform();
}

extern "C" void sfSprite_setTexture(sf::Sprite *sprite, const sf::Texture *texture, bool resetRect) {
    reinterpret_cast<sf::Sprite *>(sprite)->setTexture(*reinterpret_cast<const sf::Texture *>(texture), resetRect);
}

extern "C" void sfSprite_setTextureRect(sf::Sprite *sprite, sfIntRect rectangle) {
    reinterpret_cast<sf::Sprite *>(sprite)->setTextureRect(sf::IntRect(rectangle.left, rectangle.top, rectangle.width, rectangle.height));
}

extern "C" void sfSprite_setColor(sf::Sprite *sprite, sfColor color) {
    reinterpret_cast<sf::Sprite *>(sprite)->setColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" const sf::Texture *sfSprite_getTexture(const sf::Sprite *sprite) {
    const sf::Sprite *sprite_ = reinterpret_cast<const sf::Sprite *>(sprite);
    return reinterpret_cast<const sf::Texture *>(sprite_->getTexture());
}

extern "C" sfIntRect sfSprite_getTextureRect(const sf::Sprite *sprite) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect sfmlRect = reinterpret_cast<const sf::Sprite *>(sprite)->getTextureRect();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfColor sfSprite_getColor(const sf::Sprite *sprite) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = reinterpret_cast<const sf::Sprite *>(sprite)->getColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfFloatRect sfSprite_getLocalBounds(const sf::Sprite *sprite) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::Sprite *>(sprite)->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfSprite_getGlobalBounds(const sf::Sprite *sprite) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::Sprite *>(sprite)->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
