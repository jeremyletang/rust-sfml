
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
#include <SFML/Graphics/Text.hpp>
#include "Graphics/ConvertTransform.hpp"
#include "Graphics/Font.h"
#include "Graphics/FontStruct.h"
#include "Graphics/Text.h"
#include <cstddef>

sfText *sfText_create(void) {
    sf::Text *text = new sf::Text;

    return reinterpret_cast<sfText*>(text);
}

sfText *sfText_copy(const sfText *text) {
    const sf::Text *src = reinterpret_cast<const sf::Text *>(text);
    sf::Text *newText = new sf::Text(*src);
    return reinterpret_cast<sfText*>(newText);
}

void sfText_destroy(sfText *text) {
    delete reinterpret_cast<sf::Text*>(text);
}

void sfText_setPosition(sfText *text, sfVector2f position) {
    reinterpret_cast<sf::Text*>(text)->setPosition(position.x, position.y);
}

void sfText_setRotation(sfText *text, float angle) {
    reinterpret_cast<sf::Text*>(text)->setRotation(angle);
}

void sfText_setScale(sfText *text, sfVector2f scale) {
    reinterpret_cast<sf::Text*>(text)->setScale(scale.x, scale.y);
}

void sfText_setOrigin(sfText *text, sfVector2f origin) {
    reinterpret_cast<sf::Text*>(text)->setOrigin(origin.x, origin.y);
}

sfVector2f sfText_getPosition(const sfText *text) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = reinterpret_cast<const sf::Text*>(text)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

float sfText_getRotation(const sfText *text) {
    return reinterpret_cast<const sf::Text*>(text)->getRotation();
}

sfVector2f sfText_getScale(const sfText *text) {
    sfVector2f scale = {0, 0};

    sf::Vector2f sfmlScale = reinterpret_cast<const sf::Text*>(text)->getScale();
    scale.x = sfmlScale.x;
    scale.y = sfmlScale.y;

    return scale;
}

sfVector2f sfText_getOrigin(const sfText *text) {
    sfVector2f origin = {0, 0};

    sf::Vector2f sfmlOrigin = reinterpret_cast<const sf::Text*>(text)->getOrigin();
    origin.x = sfmlOrigin.x;
    origin.y = sfmlOrigin.y;

    return origin;
}

void sfText_move(sfText *text, sfVector2f offset) {
    reinterpret_cast<sf::Text*>(text)->move(offset.x, offset.y);
}

void sfText_rotate(sfText *text, float angle) {
    reinterpret_cast<sf::Text*>(text)->rotate(angle);
}

void sfText_scale(sfText *text, sfVector2f factors) {
    reinterpret_cast<sf::Text*>(text)->scale(factors.x, factors.y);
}

sfTransform sfText_getTransform(const sfText *text) {

    sfTransform transform = convertTransform(reinterpret_cast<const sf::Text*>(text)->getTransform());
    return transform;
}

sfTransform sfText_getInverseTransform(const sfText *text) {

    sfTransform transform = convertTransform(reinterpret_cast<const sf::Text*>(text)->getInverseTransform());
    return transform;
}

void sfText_setUnicodeString(sfText *text, const sfUint32 *string) {
    sf::String UTF32Text = string;
    reinterpret_cast<sf::Text*>(text)->setString(UTF32Text);
}

void sfText_setFont(sfText *text, const sfFont *font) {

    reinterpret_cast<sf::Text*>(text)->setFont(font->This);
}

void sfText_setCharacterSize(sfText *text, unsigned int size) {
    reinterpret_cast<sf::Text*>(text)->setCharacterSize(size);
}

void sfText_setLineSpacing(sfText *text, float spacingFactor) {
    reinterpret_cast<sf::Text*>(text)->setLineSpacing(spacingFactor);
}

void sfText_setLetterSpacing(sfText *text, float spacingFactor) {
    reinterpret_cast<sf::Text*>(text)->setLetterSpacing(spacingFactor);
}

void sfText_setStyle(sfText *text, sfUint32 style) {
    reinterpret_cast<sf::Text*>(text)->setStyle(style);
}

void sfText_setColor(sfText *text, sfColor color) {
    sfText_setFillColor(text, color);
}

void sfText_setFillColor(sfText *text, sfColor color) {
    reinterpret_cast<sf::Text*>(text)->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

void sfText_setOutlineColor(sfText *text, sfColor color) {
    reinterpret_cast<sf::Text*>(text)->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

void sfText_setOutlineThickness(sfText *text, float thickness) {
    reinterpret_cast<sf::Text*>(text)->setOutlineThickness(thickness);
}

const sfUint32 *sfText_getUnicodeString(const sfText *text) {

    return reinterpret_cast<const sf::Text*>(text)->getString().getData();
}

const sfFont *sfText_getFont(const sfText *text) {
    //const sf::Font * font = reinterpret_cast<const sf::Text*>(text)->getFont();
    // TODO: Get rid of FontStruct
    abort();
}

unsigned int sfText_getCharacterSize(const sfText *text) {
    return reinterpret_cast<const sf::Text*>(text)->getCharacterSize();
}

float sfText_getLetterSpacing(const sfText *text) {
    return reinterpret_cast<const sf::Text*>(text)->getLetterSpacing();
}

float sfText_getLineSpacing(const sfText *text) {
    return reinterpret_cast<const sf::Text*>(text)->getLineSpacing();
}

sfUint32 sfText_getStyle(const sfText *text) {
    return reinterpret_cast<const sf::Text*>(text)->getStyle();
}

sfColor sfText_getColor(const sfText *text) {
    return sfText_getFillColor(text);
}

sfColor sfText_getFillColor(const sfText *text) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = reinterpret_cast<const sf::Text*>(text)->getFillColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

sfColor sfText_getOutlineColor(const sfText *text) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = reinterpret_cast<const sf::Text*>(text)->getOutlineColor();
    color.r = sfmlColor.r;
    color.g = sfmlColor.g;
    color.b = sfmlColor.b;
    color.a = sfmlColor.a;

    return color;
}

float sfText_getOutlineThickness(const sfText *text) {
    return reinterpret_cast<const sf::Text*>(text)->getOutlineThickness();
}

sfVector2f sfText_findCharacterPos(const sfText *text, size_t index) {
    sfVector2f position = {0, 0};

    sf::Vector2f sfmlPos = reinterpret_cast<const sf::Text*>(text)->findCharacterPos(index);
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

sfFloatRect sfText_getLocalBounds(const sfText *text) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::Text*>(text)->getLocalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}

sfFloatRect sfText_getGlobalBounds(const sfText *text) {
    sfFloatRect rect = {0, 0, 0, 0};

    sf::FloatRect sfmlRect = reinterpret_cast<const sf::Text*>(text)->getGlobalBounds();
    rect.left = sfmlRect.left;
    rect.top = sfmlRect.top;
    rect.width = sfmlRect.width;
    rect.height = sfmlRect.height;

    return rect;
}
