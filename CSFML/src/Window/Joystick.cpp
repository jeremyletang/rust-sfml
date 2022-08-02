
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

#include "Window/Joystick.h"
#include <SFML/Window/Joystick.hpp>
#include <cstddef>

bool sfJoystick_isConnected(unsigned int joystick) {
    return sf::Joystick::isConnected(joystick);
}

unsigned int sfJoystick_getButtonCount(unsigned int joystick) {
    return sf::Joystick::getButtonCount(joystick);
}

bool sfJoystick_hasAxis(unsigned int joystick, sfJoystickAxis axis) {
    return sf::Joystick::hasAxis(joystick, static_cast<sf::Joystick::Axis>(axis));
}

bool sfJoystick_isButtonPressed(unsigned int joystick, unsigned int button) {
    return sf::Joystick::isButtonPressed(joystick, button);
}

float sfJoystick_getAxisPosition(unsigned int joystick, sfJoystickAxis axis) {
    return sf::Joystick::getAxisPosition(joystick, static_cast<sf::Joystick::Axis>(axis));
}

extern "C" sf::Joystick::Identification *sfJoystick_getIdentification(unsigned int joystick) {
    sf::Joystick::Identification ident = sf::Joystick::getIdentification(joystick);
    sf::Joystick::Identification *copy = new sf::Joystick::Identification;
    copy->name = ident.name;
    copy->vendorId = ident.vendorId;
    copy->productId = ident.productId;
    return copy;
}

extern "C" void sfJoystickIdentification_destroy(sf::Joystick::Identification *ident) {
    delete ident;
}

extern "C" unsigned int sfJoystickIdentification_getVendorId(const sf::Joystick::Identification *ident) {
    return ident->vendorId;
}

extern "C" unsigned int sfJoystickIdentification_getProductId(const sf::Joystick::Identification *ident) {
    return ident->productId;
}

extern "C" const sf::String *sfJoystickIdentification_getName(const sf::Joystick::Identification *ident) {
    return &ident->name;
}

void sfJoystick_update(void) {
    sf::Joystick::update();
}
