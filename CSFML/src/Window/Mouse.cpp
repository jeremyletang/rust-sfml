
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

#include "Window/Mouse.h"
#include "Window/WindowStruct.h"
#include <SFML/Window/Mouse.hpp>
#include <cstddef>

extern "C" sfBool sfMouse_isButtonPressed(sf::Mouse::Button button) {
    return sf::Mouse::isButtonPressed(button) ? sfTrue : sfFalse;
}

sfVector2i sfMouse_getPosition(const sfWindow *relativeTo) {
    sf::Vector2i sfmlPos;
    if (relativeTo)
        sfmlPos = sf::Mouse::getPosition(relativeTo->This);
    else
        sfmlPos = sf::Mouse::getPosition();

    sfVector2i position = {sfmlPos.x, sfmlPos.y};
    return position;
}

void sfMouse_setPosition(sfVector2i position, const sfWindow *relativeTo) {
    if (relativeTo)
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y), relativeTo->This);
    else
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y));
}
