
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
#include "Graphics/Font.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Text.hpp>
#include <cstddef>

extern "C" sfText *sfText_create(void) {
    sf::Text *text = new sf::Text;

    return reinterpret_cast<sfText *>(text);
}

extern "C" sfText *sfText_copy(const sfText *text) {
    const sf::Text *src = reinterpret_cast<const sf::Text *>(text);
    sf::Text *newText = new sf::Text(*src);
    return reinterpret_cast<sfText *>(newText);
}

extern "C" void sfText_destroy(sfText *text) {
    delete reinterpret_cast<sf::Text *>(text);
}

extern "C" void sfText_setPosition(sfText *text, sfVector2f position) {
    reinterpret_cast<sf::Text *>(text)->setPosition(position.x, position.y);
}

extern "C" void sfText_setRotation(sfText *text, float angle) {
    reinterpret_cast<sf::Text *>(text)->setRotation(angle);
}

extern "C" void sfText_setScale(sfText *text, sfVector2f scale) {
    reinterpret_cast<sf::Text *>(text)->setScale(scale.x, scale.y);
}

extern "C" void sfText_setOrigin(sfText *text, sfVector2f origin) {
    reinterpret_cast<sf::Text *>(text)->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfText_getPosition(const sfText *text) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = reinterpret_cast<const sf::Text *>(text)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" float sfText_getRotation(const sfText *text) {
    return reinterpret_cast<const sf::Text *>(text)->getRotation();
}

extern "C" sfVector2f sfText_getScale(const sfText *text) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = reinterpret_cast<const sf::Text *>(text)->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

extern "C" sfVector2f sfText_getOrigin(const sfText *text) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = reinterpret_cast<const sf::Text *>(text)->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

extern "C" void sfText_move(sfText *text, sfVector2f offset) {
    reinterpret_cast<sf::Text *>(text)->move(offset.x, offset.y);
}

extern "C" void sfText_rotate(sfText *text, float angle) {
    reinterpret_cast<sf::Text *>(text)->rotate(angle);
}

extern "C" void sfText_scale(sfText *text, sfVector2f factors) {
    reinterpret_cast<sf::Text *>(text)->scale(factors.x, factors.y);
}

extern "C" sf::Transform sfText_getTransform(const sfText *text) {
    return reinterpret_cast<const sf::Text *>(text)->getTransform();
}

extern "C" sf::Transform sfText_getInverseTransform(const sfText *text) {
    return reinterpret_cast<const sf::Text *>(text)->getInverseTransform();
}

extern "C" void sfText_setUnicodeString(sfText *text, const sfUint32 *string) {
    sf::String UTF32Text = string;
    reinterpret_cast<sf::Text *>(text)->setString(UTF32Text);
}

extern "C" void sfText_setFont(sfText *text, const sfFont *font) {

    reinterpret_cast<sf::Text *>(text)->setFont(*reinterpret_cast<const sf::Font *>(font));
}

extern "C" void sfText_setCharacterSize(sfText *text, unsigned int size) {
    reinterpret_cast<sf::Text *>(text)->setCharacterSize(size);
}

extern "C" void sfText_setLineSpacing(sfText *text, float spacingFactor) {
    reinterpret_cast<sf::Text *>(text)->setLineSpacing(spacingFactor);
}

extern "C" void sfText_setLetterSpacing(sfText *text, float spacingFactor) {
    reinterpret_cast<sf::Text *>(text)->setLetterSpacing(spacingFactor);
}

extern "C" void sfText_setStyle(sfText *text, sfUint32 style) {
    reinterpret_cast<sf::Text *>(text)->setStyle(style);
}

extern "C" void sfText_setFillColor(sfText *text, sfColor color) {
    reinterpret_cast<sf::Text *>(text)->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfText_setOutlineColor(sfText *text, sfColor color) {
    reinterpret_cast<sf::Text *>(text)->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfText_setOutlineThickness(sfText *text, float thickness) {
    reinterpret_cast<sf::Text *>(text)->setOutlineThickness(thickness);
}

extern "C" const sfUint32 *sfText_getUnicodeString(const sfText *text) {

    return reinterpret_cast<const sf::Text *>(text)->getString().getData();
}

extern "C" const sfFont *sfText_getFont(const sfText *text) {
    const sf::Font *font = reinterpret_cast<const sf::Text *>(text)->getFont();
    return reinterpret_cast<const sfFont *>(font);
}

extern "C" unsigned int sfText_getCharacterSize(const sfText *text) {
    return reinterpret_cast<const sf::Text *>(text)->getCharacterSize();
}

extern "C" float sfText_getLetterSpacing(const sfText *text) {
    return reinterpret_cast<const sf::Text *>(text)->getLetterSpacing();
}

extern "C" float sfText_getLineSpacing(const sfText *text) {
    return reinterpret_cast<const sf::Text *>(text)->getLineSpacing();
}

extern "C" sfUint32 sfText_getStyle(const sfText *text) {
    return reinterpret_cast<const sf::Text *>(text)->getStyle();
}

extern "C" sfColor sfText_getFillColor(const sfText *text) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = reinterpret_cast<const sf::Text *>(text)->getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" sfColor sfText_getOutlineColor(const sfText *text) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = reinterpret_cast<const sf::Text *>(text)->getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

extern "C" float sfText_getOutlineThickness(const sfText *text) {
    return reinterpret_cast<const sf::Text *>(text)->getOutlineThickness();
}

extern "C" sfVector2f sfText_findCharacterPos(const sfText *text, size_t index) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = reinterpret_cast<const sf::Text *>(text)->findCharacterPos(index);
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" sfFloatRect sfText_getLocalBounds(const sfText *text) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::Text *>(text)->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

extern "C" sfFloatRect sfText_getGlobalBounds(const sfText *text) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::Text *>(text)->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
