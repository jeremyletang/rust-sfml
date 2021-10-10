
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

#include "Graphics/Color.h"
#include "Graphics/Rect.h"
#include "System/InputStreamStruct.h"
#include "System/Vector2.h"
#include <SFML/Graphics/Image.hpp>
#include <cstddef>

extern "C" sf::Image *sfImage_create(unsigned int width, unsigned int height) {
    sf::Image *image = new sf::Image;
    image->create(width, height);

    return image;
}

extern "C" sf::Image *sfImage_createFromColor(unsigned int width, unsigned int height, sfColor color) {
    sf::Image *image = new sf::Image;
    image->create(width, height, sf::Color(color.r, color.g, color.b, color.a));

    return image;
}

extern "C" sf::Image *sfImage_createFromPixels(unsigned int width, unsigned int height, const sfUint8 *data) {
    sf::Image *image = new sf::Image;
    image->create(width, height, data);

    return image;
}

extern "C" sf::Image *sfImage_createFromFile(const char *filename) {
    sf::Image *image = new sf::Image;

    if (!image->loadFromFile(filename)) {
        delete image;
        image = NULL;
    }

    return image;
}

extern "C" sf::Image *sfImage_createFromMemory(const void *data, size_t sizeInBytes) {
    sf::Image *image = new sf::Image;

    if (!image->loadFromMemory(data, sizeInBytes)) {
        delete image;
        image = NULL;
    }

    return image;
}

extern "C" sf::Image *sfImage_createFromStream(sfInputStream *stream) {

    sf::Image *image = new sf::Image;

    if (!image->loadFromStream(*stream)) {
        delete image;
        image = NULL;
    }

    return image;
}

extern "C" sf::Image *sfImage_copy(const sf::Image *image) {

    return new sf::Image(*image);
}

extern "C" void sfImage_destroy(sf::Image *image) {
    delete image;
}

extern "C" sfBool sfImage_saveToFile(const sf::Image *image, const char *filename) {
    return image->saveToFile(filename);
}

extern "C" void sfImage_createMaskFromColor(sf::Image *image, sfColor colorKey, sfUint8 alpha) {
    image->createMaskFromColor(sf::Color(colorKey.r, colorKey.g, colorKey.b, colorKey.a), alpha);
}

extern "C" void sfImage_copyImage(sf::Image *image, const sf::Image *source, unsigned int destX, unsigned int destY, sfIntRect sourceRect, sfBool applyAlpha) {

    sf::IntRect sfmlRect(sourceRect.left, sourceRect.top, sourceRect.width, sourceRect.height);
    image->copy(*source, destX, destY, sfmlRect, applyAlpha == sfTrue);
}

extern "C" void sfImage_setPixel(sf::Image *image, unsigned int x, unsigned int y, sfColor color) {
    image->setPixel(x, y, sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" sfColor sfImage_getPixel(const sf::Image *image, unsigned int x, unsigned int y) {
    sf::Color sfmlColor = image->getPixel(x, y);

    return sfColor{sfmlColor.r, sfmlColor.g, sfmlColor.b, sfmlColor.a};
}

extern "C" const sfUint8 *sfImage_getPixelsPtr(const sf::Image *image) {
    return image->getPixelsPtr();
}

extern "C" sfVector2u sfImage_getSize(const sf::Image *image) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = image->getSize();

    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" void sfImage_flipHorizontally(sf::Image *image) {
    image->flipHorizontally();
}

extern "C" void sfImage_flipVertically(sf::Image *image) {
    image->flipVertically();
}
