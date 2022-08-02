
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

#ifndef SFML_JOYSTICK_H
#define SFML_JOYSTICK_H

// Headers

#include "Config.h"

typedef enum {
    sfJoystickX,    ///< The X axis
    sfJoystickY,    ///< The Y axis
    sfJoystickZ,    ///< The Z axis
    sfJoystickR,    ///< The R axis
    sfJoystickU,    ///< The U axis
    sfJoystickV,    ///< The V axis
    sfJoystickPovX, ///< The X axis of the point-of-view hat
    sfJoystickPovY  ///< The Y axis of the point-of-view hat
} sfJoystickAxis;

extern "C" bool sfJoystick_isConnected(unsigned int joystick);

extern "C" unsigned int sfJoystick_getButtonCount(unsigned int joystick);

extern "C" bool sfJoystick_hasAxis(unsigned int joystick, sfJoystickAxis axis);

extern "C" bool sfJoystick_isButtonPressed(unsigned int joystick, unsigned int button);

extern "C" float sfJoystick_getAxisPosition(unsigned int joystick, sfJoystickAxis axis);

extern "C" void sfJoystick_update(void);

#endif // SFML_JOYSTICK_H
