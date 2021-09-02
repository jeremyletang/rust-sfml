
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

#ifndef SFML_TEXTURE_H
#define SFML_TEXTURE_H

// Headers

#include <SFML/Graphics/Rect.h>
#include <SFML/Graphics/Types.h>
#include <SFML/System/InputStream.h>
#include <SFML/System/Vector2.h>
#include <SFML/Window/Types.h>
#include <stddef.h>

extern "C" sfTexture *sfTexture_create(unsigned int width, unsigned int height);

extern "C" sfTexture *sfTexture_createFromFile(const char *filename, const sfIntRect *area);

extern "C" sfTexture *sfTexture_createFromMemory(const void *data, size_t sizeInBytes, const sfIntRect *area);

extern "C" sfTexture *sfTexture_createFromStream(sfInputStream *stream, const sfIntRect *area);

extern "C" sfTexture *sfTexture_createFromImage(const sfImage *image, const sfIntRect *area);

extern "C" sfTexture *sfTexture_copy(const sfTexture *texture);

extern "C" void sfTexture_destroy(sfTexture *texture);

extern "C" sfVector2u sfTexture_getSize(const sfTexture *texture);

extern "C" sfImage *sfTexture_copyToImage(const sfTexture *texture);

extern "C" void sfTexture_updateFromPixels(sfTexture *texture, const sfUint8 *pixels, unsigned int width, unsigned int height, unsigned int x, unsigned int y);

extern "C" void sfTexture_updateFromTexture(sfTexture *destination, const sfTexture *source, unsigned int x, unsigned int y);

extern "C" void sfTexture_updateFromImage(sfTexture *texture, const sfImage *image, unsigned int x, unsigned int y);

extern "C" void sfTexture_updateFromWindow(sfTexture *texture, const sfWindow *window, unsigned int x, unsigned int y);

extern "C" void sfTexture_updateFromRenderWindow(sfTexture *texture, const sfRenderWindow *renderWindow, unsigned int x, unsigned int y);

extern "C" void sfTexture_setSmooth(sfTexture *texture, sfBool smooth);

extern "C" sfBool sfTexture_isSmooth(const sfTexture *texture);

extern "C" void sfTexture_setSrgb(sfTexture *texture, sfBool sRgb);

extern "C" sfBool sfTexture_isSrgb(const sfTexture *texture);

extern "C" void sfTexture_setRepeated(sfTexture *texture, sfBool repeated);

extern "C" sfBool sfTexture_isRepeated(const sfTexture *texture);

extern "C" sfBool sfTexture_generateMipmap(sfTexture *texture);

extern "C" void sfTexture_swap(sfTexture *left, sfTexture *right);

extern "C" unsigned int sfTexture_getNativeHandle(const sfTexture *texture);

extern "C" void sfTexture_bind(const sfTexture *texture);

extern "C" unsigned int sfTexture_getMaximumSize();

#endif // SFML_TEXTURE_H
