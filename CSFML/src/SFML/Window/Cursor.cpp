
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

#include <SFML/Window/Cursor.h>
#include <SFML/Window/CursorStruct.h>
#include <cstddef>

sfCursor *sfCursor_createFromPixels(const sfUint8 *pixels, sfVector2u size, sfVector2u hotspot) {
    sfCursor *cursor = new sfCursor;

    if (!cursor->This.loadFromPixels(pixels, sf::Vector2u(size.x, size.y), sf::Vector2u(hotspot.x, hotspot.y))) {
        delete cursor;
        cursor = NULL;
    }

    return cursor;
}

sfCursor *sfCursor_createFromSystem(sfCursorType type) {
    sfCursor *cursor = new sfCursor;

    if (!cursor->This.loadFromSystem(static_cast<sf::Cursor::Type>(type))) {
        delete cursor;
        cursor = NULL;
    }

    return cursor;
}

void sfCursor_destroy(sfCursor *cursor) {
    delete cursor;
}
