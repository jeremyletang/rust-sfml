
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

#include <SFML/Window/Joystick.h>
#include <SFML/Window/Joystick.hpp>
#include <cstddef>



sfBool sfJoystick_isConnected(unsigned int joystick)
{
    return sf::Joystick::isConnected(joystick) ? sfTrue : sfFalse;
}



unsigned int sfJoystick_getButtonCount(unsigned int joystick)
{
    return sf::Joystick::getButtonCount(joystick);
}



sfBool sfJoystick_hasAxis(unsigned int joystick, sfJoystickAxis axis)
{
    return sf::Joystick::hasAxis(joystick, static_cast<sf::Joystick::Axis>(axis)) ? sfTrue : sfFalse;
}



sfBool sfJoystick_isButtonPressed(unsigned int joystick, unsigned int button)
{
    return sf::Joystick::isButtonPressed(joystick, button) ? sfTrue : sfFalse;
}



float sfJoystick_getAxisPosition(unsigned int joystick, sfJoystickAxis axis)
{
    return sf::Joystick::getAxisPosition(joystick, static_cast<sf::Joystick::Axis>(axis));
}


sfJoystickIdentification sfJoystick_getIdentification(unsigned int joystick)
{
    static std::string name;

    sf::Joystick::Identification identification = sf::Joystick::getIdentification(joystick);
    name = identification.name;

    sfJoystickIdentification result;

    result.name = name.c_str();
    result.productId = identification.productId;
    result.vendorId = identification.vendorId;

    return result;
}


void sfJoystick_update(void)
{
    sf::Joystick::update();
}
