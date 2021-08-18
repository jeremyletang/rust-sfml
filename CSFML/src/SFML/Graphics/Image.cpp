
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

#include <SFML/CallbackStream.h>
#include <SFML/Graphics/Image.h>
#include <SFML/Graphics/ImageStruct.h>
#include <cstddef>

sfImage *sfImage_create(unsigned int width, unsigned int height) {
    sfImage *image = new sfImage;
    image->This.create(width, height);

    return image;
}

sfImage *sfImage_createFromColor(unsigned int width, unsigned int height, sfColor color) {
    sfImage *image = new sfImage;
    image->This.create(width, height, sf::Color(color.r, color.g, color.b, color.a));

    return image;
}

sfImage *sfImage_createFromPixels(unsigned int width, unsigned int height, const sfUint8 *data) {
    sfImage *image = new sfImage;
    image->This.create(width, height, data);

    return image;
}

sfImage *sfImage_createFromFile(const char *filename) {
    sfImage *image = new sfImage;

    if (!image->This.loadFromFile(filename)) {
        delete image;
        image = NULL;
    }

    return image;
}

sfImage *sfImage_createFromMemory(const void *data, size_t sizeInBytes) {
    sfImage *image = new sfImage;

    if (!image->This.loadFromMemory(data, sizeInBytes)) {
        delete image;
        image = NULL;
    }

    return image;
}

sfImage *sfImage_createFromStream(sfInputStream *stream) {

    sfImage *image = new sfImage;

    CallbackStream sfmlStream(stream);
    if (!image->This.loadFromStream(sfmlStream)) {
        delete image;
        image = NULL;
    }

    return image;
}

sfImage *sfImage_copy(const sfImage *image) {

    return new sfImage(*image);
}

void sfImage_destroy(sfImage *image) {
    delete image;
}

sfBool sfImage_saveToFile(const sfImage *image, const char *filename) {
    return image->This.saveToFile(filename);
}

void sfImage_createMaskFromColor(sfImage *image, sfColor colorKey, sfUint8 alpha) {
    image->This.createMaskFromColor(sf::Color(colorKey.r, colorKey.g, colorKey.b, colorKey.a), alpha);
}

void sfImage_copyImage(sfImage *image, const sfImage *source, unsigned int destX, unsigned int destY, sfIntRect sourceRect, sfBool applyAlpha) {

    sf::IntRect sfmlRect(sourceRect.left, sourceRect.top, sourceRect.width, sourceRect.height);
    image->This.copy(source->This, destX, destY, sfmlRect, applyAlpha == sfTrue);
}

void sfImage_setPixel(sfImage *image, unsigned int x, unsigned int y, sfColor color) {
    image->This.setPixel(x, y, sf::Color(color.r, color.g, color.b, color.a));
}

sfColor sfImage_getPixel(const sfImage *image, unsigned int x, unsigned int y) {
    sfColor color = {0, 0, 0, 0};

    sf::Color sfmlColor = image->This.getPixel(x, y);

    return sfColor_fromRGBA(sfmlColor.r, sfmlColor.g, sfmlColor.b, sfmlColor.a);
}

const sfUint8 *sfImage_getPixelsPtr(const sfImage *image) {
    return image->This.getPixelsPtr();
}

sfVector2u sfImage_getSize(const sfImage *image) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = image->This.getSize();

    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

void sfImage_flipHorizontally(sfImage *image) {
    image->This.flipHorizontally();
}

void sfImage_flipVertically(sfImage *image) {
    image->This.flipVertically();
}
