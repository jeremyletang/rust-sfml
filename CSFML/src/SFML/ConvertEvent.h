
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

#ifndef SFML_CONVERTEVENT_H
#define SFML_CONVERTEVENT_H

// Headers

#include <SFML/Window/Event.h>
#include <SFML/Window/Event.hpp>

// Define a function to convert a sf::Event to a sfEvent

inline void convertEvent(const sf::Event &SFMLEvent, sfEvent *event) {
    // Convert its type
    event->type = static_cast<sfEventType>(SFMLEvent.type);

    // Fill its fields
    switch (event->type) {
    case sfEvtResized:
        event->size.width = SFMLEvent.size.width;
        event->size.height = SFMLEvent.size.height;
        break;

    case sfEvtTextEntered:
        event->text.unicode = SFMLEvent.text.unicode;
        break;

    case sfEvtKeyReleased:
    case sfEvtKeyPressed:
        event->key.code = static_cast<sfKeyCode>(SFMLEvent.key.code);
        event->key.alt = SFMLEvent.key.alt ? sfTrue : sfFalse;
        event->key.control = SFMLEvent.key.control ? sfTrue : sfFalse;
        event->key.shift = SFMLEvent.key.shift ? sfTrue : sfFalse;
        event->key.system = SFMLEvent.key.system ? sfTrue : sfFalse;
        break;

    case sfEvtMouseWheelMoved:
        event->mouseWheel.delta = SFMLEvent.mouseWheel.delta;
        event->mouseWheel.x = SFMLEvent.mouseWheel.x;
        event->mouseWheel.y = SFMLEvent.mouseWheel.y;
        break;

    case sfEvtMouseWheelScrolled:
        event->mouseWheelScroll.wheel = static_cast<sfMouseWheel>(SFMLEvent.mouseWheelScroll.wheel);
        event->mouseWheelScroll.delta = SFMLEvent.mouseWheelScroll.delta;
        event->mouseWheelScroll.x = SFMLEvent.mouseWheelScroll.x;
        event->mouseWheelScroll.y = SFMLEvent.mouseWheelScroll.y;
        break;

    case sfEvtMouseButtonPressed:
    case sfEvtMouseButtonReleased:
        event->mouseButton.button = static_cast<sfMouseButton>(SFMLEvent.mouseButton.button);
        event->mouseButton.x = SFMLEvent.mouseButton.x;
        event->mouseButton.y = SFMLEvent.mouseButton.y;
        break;

    case sfEvtMouseMoved:
        event->mouseMove.x = SFMLEvent.mouseMove.x;
        event->mouseMove.y = SFMLEvent.mouseMove.y;
        break;

    case sfEvtJoystickButtonPressed:
    case sfEvtJoystickButtonReleased:
        event->joystickButton.joystickId = SFMLEvent.joystickButton.joystickId;
        event->joystickButton.button = SFMLEvent.joystickButton.button;
        break;

    case sfEvtJoystickMoved:
        event->joystickMove.joystickId = SFMLEvent.joystickMove.joystickId;
        event->joystickMove.axis = static_cast<sfJoystickAxis>(SFMLEvent.joystickMove.axis);
        event->joystickMove.position = SFMLEvent.joystickMove.position;
        break;

    case sfEvtJoystickConnected:
    case sfEvtJoystickDisconnected:
        event->joystickConnect.joystickId = SFMLEvent.joystickConnect.joystickId;
        break;

    case sfEvtTouchBegan:
    case sfEvtTouchMoved:
    case sfEvtTouchEnded:
        event->touch.finger = SFMLEvent.touch.finger;
        event->touch.x = SFMLEvent.touch.x;
        event->touch.y = SFMLEvent.touch.y;
        break;

    case sfEvtSensorChanged:
        event->sensor.sensorType = static_cast<sfSensorType>(SFMLEvent.sensor.type);
        event->sensor.x = SFMLEvent.sensor.x;
        event->sensor.y = SFMLEvent.sensor.y;
        event->sensor.z = SFMLEvent.sensor.z;
        break;

    default:
        break;
    }
}

#endif // SFML_CONVERTEVENT_H
