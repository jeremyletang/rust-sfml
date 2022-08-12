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
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Text.hpp>
#include <cstddef>

extern "C" sf::Text *sfText_create(void) {
    return new sf::Text;
}

extern "C" sf::Text *sfText_copy(const sf::Text *text) {
    return new sf::Text(*text);
}

extern "C" void sfText_destroy(sf::Text *text) {
    delete text;
}

extern "C" void sfText_setPosition(sf::Text *text, sfVector2f position) {
    text->setPosition(position.x, position.y);
}

extern "C" void sfText_setRotation(sf::Text *text, float angle) {
    text->setRotation(angle);
}

extern "C" void sfText_setScale(sf::Text *text, sfVector2f scale) {
    text->setScale(scale.x, scale.y);
}

extern "C" void sfText_setOrigin(sf::Text *text, sfVector2f origin) {
    text->setOrigin(origin.x, origin.y);
}

extern "C" sfVector2f sfText_getPosition(const sf::Text *text) {
    sf::Vector2f vec2 = text->getPosition();
    return {vec2.x, vec2.y};
}

extern "C" float sfText_getRotation(const sf::Text *text) {
    return text->getRotation();
}

extern "C" sfVector2f sfText_getScale(const sf::Text *text) {
    sf::Vector2f vec2 = text->getScale();
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfText_getOrigin(const sf::Text *text) {
    sf::Vector2f vec2 = text->getOrigin();
    return {vec2.x, vec2.y};
}

extern "C" void sfText_move(sf::Text *text, sfVector2f offset) {
    text->move(offset.x, offset.y);
}

extern "C" void sfText_rotate(sf::Text *text, float angle) {
    text->rotate(angle);
}

extern "C" void sfText_scale(sf::Text *text, sfVector2f factors) {
    text->scale(factors.x, factors.y);
}

extern "C" sf::Transform const *sfText_getTransform(const sf::Text *text) {
    return &text->getTransform();
}

extern "C" sf::Transform const *sfText_getInverseTransform(const sf::Text *text) {
    return &text->getInverseTransform();
}

extern "C" void sfText_setUnicodeString(sf::Text *text, const uint32_t *string) {
    text->setString(sf::String(string));
}

extern "C" void sfText_setFont(sf::Text *text, const sf::Font *font) {
    text->setFont(*font);
}

extern "C" void sfText_setCharacterSize(sf::Text *text, unsigned int size) {
    text->setCharacterSize(size);
}

extern "C" void sfText_setLineSpacing(sf::Text *text, float spacingFactor) {
    text->setLineSpacing(spacingFactor);
}

extern "C" void sfText_setLetterSpacing(sf::Text *text, float spacingFactor) {
    text->setLetterSpacing(spacingFactor);
}

extern "C" void sfText_setStyle(sf::Text *text, uint32_t style) {
    text->setStyle(style);
}

extern "C" void sfText_setFillColor(sf::Text *text, sfColor color) {
    text->setFillColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfText_setOutlineColor(sf::Text *text, sfColor color) {
    text->setOutlineColor(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfText_setOutlineThickness(sf::Text *text, float thickness) {
    text->setOutlineThickness(thickness);
}

extern "C" const uint32_t *sfText_getUnicodeString(const sf::Text *text) {
    return text->getString().getData();
}

extern "C" const sf::Font *sfText_getFont(const sf::Text *text) {
    return text->getFont();
}

extern "C" unsigned int sfText_getCharacterSize(const sf::Text *text) {
    return text->getCharacterSize();
}

extern "C" float sfText_getLetterSpacing(const sf::Text *text) {
    return text->getLetterSpacing();
}

extern "C" float sfText_getLineSpacing(const sf::Text *text) {
    return text->getLineSpacing();
}

extern "C" uint32_t sfText_getStyle(const sf::Text *text) {
    return text->getStyle();
}

extern "C" sfColor sfText_getFillColor(const sf::Text *text) {
    sf::Color color = text->getFillColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" sfColor sfText_getOutlineColor(const sf::Text *text) {
    sf::Color color = text->getOutlineColor();
    return {color.r, color.g, color.b, color.a};
}

extern "C" float sfText_getOutlineThickness(const sf::Text *text) {
    return text->getOutlineThickness();
}

extern "C" sfVector2f sfText_findCharacterPos(const sf::Text *text, size_t index) {
    sf::Vector2f vec2 = text->findCharacterPos(index);
    return {vec2.x, vec2.y};
}

extern "C" sfFloatRect sfText_getLocalBounds(const sf::Text *text) {
    sf::FloatRect rect = text->getLocalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfFloatRect sfText_getGlobalBounds(const sf::Text *text) {
    sf::FloatRect rect = text->getGlobalBounds();
    return {rect.left, rect.top, rect.width, rect.height};
}
