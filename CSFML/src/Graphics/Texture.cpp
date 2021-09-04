
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

#include "Graphics/ImageStruct.h"
#include "Graphics/Texture.h"
#include "Window/WindowStruct.h"
#include <cstddef>
#include <SFML/Graphics/Texture.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include "System/InputStreamStruct.h"

sfTexture *sfTexture_create(unsigned int width, unsigned int height) {
    sf::Texture *texture = new sf::Texture;

    if (!texture->create(width, height)) {
        delete texture;
        texture = NULL;
    }

    return reinterpret_cast<sfTexture *>(texture);
}

sfTexture *sfTexture_createFromFile(const char *filename, const sfIntRect *area) {
    sf::Texture *texture = new sf::Texture;

    sf::IntRect rect;
    if (area)
        rect = sf::IntRect(area->left, area->top, area->width, area->height);

    if (!texture->loadFromFile(filename, rect)) {
        delete texture;
        texture = NULL;
    }

    return reinterpret_cast<sfTexture *>(texture);
}

sfTexture *sfTexture_createFromMemory(const void *data, size_t sizeInBytes, const sfIntRect *area) {
    sf::Texture *texture = new sf::Texture;

    sf::IntRect rect;
    if (area)
        rect = sf::IntRect(area->left, area->top, area->width, area->height);

    if (!texture->loadFromMemory(data, sizeInBytes, rect)) {
        delete texture;
        texture = NULL;
    }

    return reinterpret_cast<sfTexture *>(texture);
}

sfTexture *sfTexture_createFromStream(sfInputStream *stream, const sfIntRect *area) {

    sf::Texture *texture = new sf::Texture;

    sf::IntRect rect;
    if (area)
        rect = sf::IntRect(area->left, area->top, area->width, area->height);

    if (!texture->loadFromStream(*stream, rect)) {
        delete texture;
        texture = NULL;
    }

    return reinterpret_cast<sfTexture *>(texture);
}

sfTexture *sfTexture_createFromImage(const sfImage *image, const sfIntRect *area) {

    sf::Texture *texture = new sf::Texture;

    sf::IntRect rect;
    if (area)
        rect = sf::IntRect(area->left, area->top, area->width, area->height);

    if (!texture->loadFromImage(image->This, rect)) {
        delete texture;
        texture = NULL;
    }

    return reinterpret_cast<sfTexture *>(texture);
}

sfTexture *sfTexture_copy(const sfTexture *texture) {
    const sf::Texture * texture_ = reinterpret_cast<const sf::Texture *>(texture);
    sf::Texture * newTexture = new sf::Texture(*texture_);
    return reinterpret_cast<sfTexture*>(newTexture);
}

void sfTexture_destroy(sfTexture *texture) {
    delete reinterpret_cast<sf::Texture*>(texture);
}

sfVector2u sfTexture_getSize(const sfTexture *texture) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = reinterpret_cast<const sf::Texture*>(texture)->getSize();

    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

sfImage *sfTexture_copyToImage(const sfTexture *texture) {

    sfImage *image = new sfImage;
    image->This = reinterpret_cast<const sf::Texture*>(texture)->copyToImage();

    return image;
}

void sfTexture_updateFromPixels(sfTexture *texture, const sfUint8 *pixels, unsigned int width, unsigned int height, unsigned int x, unsigned int y) {
    reinterpret_cast<sf::Texture*>(texture)->update(pixels, width, height, x, y);
}

void sfTexture_updateFromTexture(sfTexture *destination, const sfTexture *texture, unsigned int x, unsigned int y) {
    sf::Texture * destination_ = reinterpret_cast<sf::Texture*>(destination);
    const sf::Texture * texture_ = reinterpret_cast<const sf::Texture*>(texture);
    destination_->update(*texture_, x, y);
}

void sfTexture_updateFromImage(sfTexture *texture, const sfImage *image, unsigned int x, unsigned int y) {

    reinterpret_cast<sf::Texture*>(texture)->update(image->This, x, y);
}

void sfTexture_updateFromWindow(sfTexture *texture, const sfWindow *window, unsigned int x, unsigned int y) {

    reinterpret_cast<sf::Texture*>(texture)->update(window->This, x, y);
}

void sfTexture_updateFromRenderWindow(sfTexture *texture, const sfRenderWindow *renderWindow, unsigned int x, unsigned int y) {
    const sf::RenderWindow * win = reinterpret_cast<const sf::RenderWindow *>(renderWindow);
    reinterpret_cast<sf::Texture*>(texture)->update(*win, x, y);
}

void sfTexture_setSmooth(sfTexture *texture, sfBool smooth) {
    reinterpret_cast<sf::Texture*>(texture)->setSmooth(smooth == sfTrue);
}

sfBool sfTexture_isSmooth(const sfTexture *texture) {

    return reinterpret_cast<const sf::Texture*>(texture)->isSmooth();
}

void sfTexture_setSrgb(sfTexture *texture, sfBool sRgb) {
    reinterpret_cast<sf::Texture*>(texture)->setSrgb(sRgb == sfTrue);
}

sfBool sfTexture_isSrgb(const sfTexture *texture) {
    return reinterpret_cast<const sf::Texture*>(texture)->isSrgb();
}

void sfTexture_setRepeated(sfTexture *texture, sfBool repeated) {
    reinterpret_cast<sf::Texture*>(texture)->setRepeated(repeated == sfTrue);
}

sfBool sfTexture_isRepeated(const sfTexture *texture) {

    return reinterpret_cast<const sf::Texture*>(texture)->isRepeated();
}

sfBool sfTexture_generateMipmap(sfTexture *texture) {
    return reinterpret_cast<sf::Texture*>(texture)->generateMipmap();
}

void sfTexture_swap(sfTexture *left, sfTexture *right) {
    sf::Texture * left_ = reinterpret_cast<sf::Texture*>(left);
    sf::Texture * right_ = reinterpret_cast<sf::Texture*>(right);
    left_->swap(*right_);
}

unsigned int sfTexture_getNativeHandle(const sfTexture *texture) {
    return reinterpret_cast<const sf::Texture*>(texture)->getNativeHandle();
}

void sfTexture_bind(const sfTexture *texture) {
    sf::Texture::bind(reinterpret_cast<const sf::Texture*>(texture));
}

unsigned int sfTexture_getMaximumSize() {
    return sf::Texture::getMaximumSize();
}
