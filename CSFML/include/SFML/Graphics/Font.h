
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

#ifndef SFML_FONT_H
#define SFML_FONT_H

// Headers

#include <SFML/Graphics/FontInfo.h>
#include <SFML/Graphics/Glyph.h>
#include <SFML/Graphics/Types.h>
#include <SFML/System/InputStream.h>
#include <stddef.h>

extern "C" sfFont *sfFont_createFromFile(const char *filename);

extern "C" sfFont *sfFont_createFromMemory(const void *data, size_t sizeInBytes);

extern "C" sfFont *sfFont_createFromStream(sfInputStream *stream);

extern "C" sfFont *sfFont_copy(const sfFont *font);

extern "C" void sfFont_destroy(sfFont *font);

extern "C" sfGlyph sfFont_getGlyph(const sfFont *font, sfUint32 codePoint, unsigned int characterSize, sfBool bold, float outlineThickness);

extern "C" float sfFont_getKerning(const sfFont *font, sfUint32 first, sfUint32 second, unsigned int characterSize);

extern "C" float sfFont_getLineSpacing(const sfFont *font, unsigned int characterSize);

extern "C" float sfFont_getUnderlinePosition(const sfFont *font, unsigned int characterSize);

extern "C" float sfFont_getUnderlineThickness(const sfFont *font, unsigned int characterSize);

extern "C" const sfTexture *sfFont_getTexture(sfFont *font, unsigned int characterSize);

extern "C" sfFontInfo sfFont_getInfo(const sfFont *font);

#endif // SFML_FONT_H
