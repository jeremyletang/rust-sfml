
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

#ifndef SFML_COLOR_H
#define SFML_COLOR_H

// Headers
#include <SFML/Config.h>

typedef struct
{
    sfUint8 r;
    sfUint8 g;
    sfUint8 b;
    sfUint8 a;
} sfColor;

extern "C" sfColor sfBlack;       ///< Black predefined color
extern "C" sfColor sfWhite;       ///< White predefined color
extern "C" sfColor sfRed;         ///< Red predefined color
extern "C" sfColor sfGreen;       ///< Green predefined color
extern "C" sfColor sfBlue;        ///< Blue predefined color
extern "C" sfColor sfYellow;      ///< Yellow predefined color
extern "C" sfColor sfMagenta;     ///< Magenta predefined color
extern "C" sfColor sfCyan;        ///< Cyan predefined color
extern "C" sfColor sfTransparent; ///< Transparent (black) predefined color

extern "C" sfColor sfColor_fromRGB(sfUint8 red, sfUint8 green, sfUint8 blue);

extern "C" sfColor sfColor_fromRGBA(sfUint8 red, sfUint8 green, sfUint8 blue, sfUint8 alpha);

extern "C" sfColor sfColor_fromInteger(sfUint32 color);

extern "C" sfUint32 sfColor_toInteger(sfColor color);

extern "C" sfColor sfColor_add(sfColor color1, sfColor color2);

extern "C" sfColor sfColor_subtract(sfColor color1, sfColor color2);

extern "C" sfColor sfColor_modulate(sfColor color1, sfColor color2);

#endif // SFML_COLOR_H
