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

#include "System/Vector3.h"
#include <SFML/Audio/Listener.hpp>

extern "C" void sfListener_setGlobalVolume(float volume) {
    sf::Listener::setGlobalVolume(volume);
}

extern "C" float sfListener_getGlobalVolume(void) {
    return sf::Listener::getGlobalVolume();
}

extern "C" void sfListener_setPosition(sfVector3f position) {
    sf::Listener::setPosition(position.x, position.y, position.z);
}

extern "C" sfVector3f sfListener_getPosition() {
    sf::Vector3f pos = sf::Listener::getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" void sfListener_setDirection(sfVector3f direction) {
    sf::Listener::setDirection(direction.x, direction.y, direction.z);
}

extern "C" sfVector3f sfListener_getDirection() {
    sf::Vector3f dir = sf::Listener::getDirection();
    return {dir.x, dir.y, dir.z};
}

extern "C" void sfListener_setUpVector(sfVector3f upVector) {
    sf::Listener::setUpVector(upVector.x, upVector.y, upVector.z);
}

extern "C" sfVector3f sfListener_getUpVector() {
    sf::Vector3f vec = sf::Listener::getUpVector();
    return {vec.x, vec.y, vec.z};
}
