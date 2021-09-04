
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

#ifndef SFML_CURSOR_H
#define SFML_CURSOR_H

// Headers

#include "Config.h"
#include "System/Vector2.h"
#include "Window/Types.h"

typedef enum {
    sfCursorArrow,                  ///< Arrow cursor (default)
    sfCursorArrowWait,              ///< Busy arrow cursor
    sfCursorWait,                   ///< Busy cursor
    sfCursorText,                   ///< I-beam, cursor when hovering over a field allowing text entry
    sfCursorHand,                   ///< Pointing hand cursor
    sfCursorSizeHorizontal,         ///< Horizontal double arrow cursor
    sfCursorSizeVertical,           ///< Vertical double arrow cursor
    sfCursorSizeTopLeftBottomRight, ///< Double arrow cursor going from top-left to bottom-right
    sfCursorSizeBottomLeftTopRight, ///< Double arrow cursor going from bottom-left to top-right
    sfCursorSizeAll,                ///< Combination of SizeHorizontal and SizeVertical
    sfCursorCross,                  ///< Crosshair cursor
    sfCursorHelp,                   ///< Help cursor
    sfCursorNotAllowed              ///< Action not allowed cursor
} sfCursorType;

extern "C" sfCursor *sfCursor_createFromPixels(const sfUint8 *pixels, sfVector2u size, sfVector2u hotspot);

extern "C" sfCursor *sfCursor_createFromSystem(sfCursorType type);

extern "C" void sfCursor_destroy(sfCursor *cursor);

#endif // SFML_CURSOR_H
