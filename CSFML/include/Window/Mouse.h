
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

#ifndef SFML_MOUSE_H
#define SFML_MOUSE_H

// Headers

#include "Config.h"
#include "System/Vector2.h"
#include "Window/Types.h"

typedef enum {
    sfMouseLeft,     ///< The left mouse button
    sfMouseRight,    ///< The right mouse button
    sfMouseMiddle,   ///< The middle (wheel) mouse button
    sfMouseXButton1, ///< The first extra mouse button
    sfMouseXButton2, ///< The second extra mouse button

    sfMouseButtonCount ///< Keep last -- the total number of mouse buttons
} sfMouseButton;

typedef enum {
    sfMouseVerticalWheel,  ///< The vertical mouse wheel
    sfMouseHorizontalWheel ///< The horizontal mouse wheel
} sfMouseWheel;

extern "C" sfBool sfMouse_isButtonPressed(sfMouseButton button);

extern "C" sfVector2i sfMouse_getPosition(const sfWindow *relativeTo);

extern "C" void sfMouse_setPosition(sfVector2i position, const sfWindow *relativeTo);

#endif // SFML_MOUSE_H
