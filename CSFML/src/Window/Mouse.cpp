
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

#include "System/Vector2.h"
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Window/Mouse.hpp>
#include <SFML/Window/Window.hpp>

extern "C" bool sfMouse_isButtonPressed(sf::Mouse::Button button) {
    return sf::Mouse::isButtonPressed(button);
}

extern "C" sfVector2i sfMouse_getPosition() {
    sf::Vector2i pos = sf::Mouse::getPosition();
    return {pos.x, pos.y};
}

extern "C" sfVector2i sfMouse_getPositionRelativeTo(const sf::Window *relativeTo) {
    sf::Vector2i pos = sf::Mouse::getPosition(*relativeTo);
    return {pos.x, pos.y};
}

extern "C" sfVector2i sfMouse_getPositionRenderWindow(const sf::RenderWindow *relativeTo) {
    sf::Vector2i vec2 = sf::Mouse::getPosition(*relativeTo);
    return {vec2.x, vec2.y};
}

extern "C" void sfMouse_setPosition(sfVector2i position) {
    sf::Mouse::setPosition(sf::Vector2i(position.x, position.y));
}

extern "C" void sfMouse_setPositionRelativeTo(sfVector2i pos, const sf::Window *relativeTo) {
    sf::Mouse::setPosition(sf::Vector2i(pos.x, pos.y), *relativeTo);
}

extern "C" void sfMouse_setPositionRenderWindow(sfVector2i pos, const sf::RenderWindow *relativeTo) {
    sf::Mouse::setPosition(sf::Vector2i(pos.x, pos.y), *relativeTo);
}
