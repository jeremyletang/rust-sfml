
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

#ifndef SFML_IMAGE_H
#define SFML_IMAGE_H

// Headers

#include <SFML/Graphics/Color.h>

#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/Types.h>
#include <SFML/System/InputStream.h>
#include <SFML/System/Vector2.h>
#include <stddef.h>

extern "C" sfImage *sfImage_create(unsigned int width, unsigned int height);

extern "C" sfImage *sfImage_createFromColor(unsigned int width, unsigned int height, sfColor color);

extern "C" sfImage *sfImage_createFromPixels(unsigned int width, unsigned int height, const sfUint8 *pixels);

extern "C" sfImage *sfImage_createFromFile(const char *filename);

extern "C" sfImage *sfImage_createFromMemory(const void *data, size_t size);

extern "C" sfImage *sfImage_createFromStream(sfInputStream *stream);

extern "C" sfImage *sfImage_copy(const sfImage *image);

extern "C" void sfImage_destroy(sfImage *image);

extern "C" sfBool sfImage_saveToFile(const sfImage *image, const char *filename);

extern "C" sfVector2u sfImage_getSize(const sfImage *image);

extern "C" void sfImage_createMaskFromColor(sfImage *image, sfColor color, sfUint8 alpha);

extern "C" void sfImage_copyImage(sfImage *image, const sfImage *source, unsigned int destX, unsigned int destY, sfIntRect sourceRect, sfBool applyAlpha);

extern "C" void sfImage_setPixel(sfImage *image, unsigned int x, unsigned int y, sfColor color);

extern "C" sfColor sfImage_getPixel(const sfImage *image, unsigned int x, unsigned int y);

extern "C" const sfUint8 *sfImage_getPixelsPtr(const sfImage *image);

extern "C" void sfImage_flipHorizontally(sfImage *image);

extern "C" void sfImage_flipVertically(sfImage *image);

#endif // SFML_IMAGE_H
