
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

#include "Graphics/Rect.h"
#include "System/InputStreamStruct.h"
#include "System/Vector2.h"
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Texture.hpp>
#include <cstddef>

extern "C" sf::Texture *sfTexture_new() {
    sf::Texture *texture = new sf::Texture;
    return reinterpret_cast<sf::Texture *>(texture);
}

extern "C" bool sfTexture_create(sf::Texture *tex, unsigned int width, unsigned int height) {
    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);
    return texture->create(width, height);
}

extern "C" bool sfTexture_loadFromFile(sf::Texture *tex, const char *filename, const sfIntRect area) {
    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);

    sf::IntRect rect = sf::IntRect(area.left, area.top, area.width, area.height);

    return texture->loadFromFile(filename, rect);
}

extern "C" bool sfTexture_loadFromMemory(sf::Texture *tex, const void *data, size_t sizeInBytes, const sfIntRect area) {
    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);

    sf::IntRect rect = sf::IntRect(area.left, area.top, area.width, area.height);

    return texture->loadFromMemory(data, sizeInBytes, rect);
}

extern "C" bool sfTexture_loadFromStream(sf::Texture *tex, sfInputStream *stream, const sfIntRect area) {

    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);

    sf::IntRect rect = sf::IntRect(area.left, area.top, area.width, area.height);

    return texture->loadFromStream(*stream, rect);
}

extern "C" bool sfTexture_loadFromImage(sf::Texture *tex, const sf::Image *image, const sfIntRect area) {

    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);

    sf::IntRect rect = sf::IntRect(area.left, area.top, area.width, area.height);

    return texture->loadFromImage(*image, rect);
}

extern "C" sf::Texture *sfTexture_copy(const sf::Texture *texture) {
    const sf::Texture *texture_ = reinterpret_cast<const sf::Texture *>(texture);
    sf::Texture *newTexture = new sf::Texture(*texture_);
    return reinterpret_cast<sf::Texture *>(newTexture);
}

extern "C" void sfTexture_destroy(sf::Texture *texture) {
    delete reinterpret_cast<sf::Texture *>(texture);
}

extern "C" sfVector2u sfTexture_getSize(const sf::Texture *texture) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = reinterpret_cast<const sf::Texture *>(texture)->getSize();

    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" sf::Image *sfTexture_copyToImage(const sf::Texture *texture) {

    sf::Image *image = new sf::Image;
    *image = reinterpret_cast<const sf::Texture *>(texture)->copyToImage();

    return image;
}

extern "C" void sfTexture_updateFromPixels(sf::Texture *texture, const sfUint8 *pixels, unsigned int width, unsigned int height, unsigned int x, unsigned int y) {
    reinterpret_cast<sf::Texture *>(texture)->update(pixels, width, height, x, y);
}

extern "C" void sfTexture_updateFromTexture(sf::Texture *destination, const sf::Texture *texture, unsigned int x, unsigned int y) {
    sf::Texture *destination_ = reinterpret_cast<sf::Texture *>(destination);
    const sf::Texture *texture_ = reinterpret_cast<const sf::Texture *>(texture);
    destination_->update(*texture_, x, y);
}

extern "C" void sfTexture_updateFromImage(sf::Texture *texture, const sf::Image *image, unsigned int x, unsigned int y) {

    reinterpret_cast<sf::Texture *>(texture)->update(*image, x, y);
}

extern "C" void sfTexture_updateFromWindow(sf::Texture *texture, const sf::Window *window, unsigned int x, unsigned int y) {

    reinterpret_cast<sf::Texture *>(texture)->update(*window, x, y);
}

extern "C" void sfTexture_updateFromRenderWindow(sf::Texture *texture, const sf::RenderWindow *renderWindow, unsigned int x, unsigned int y) {
    const sf::RenderWindow *win = reinterpret_cast<const sf::RenderWindow *>(renderWindow);
    reinterpret_cast<sf::Texture *>(texture)->update(*win, x, y);
}

extern "C" void sfTexture_setSmooth(sf::Texture *texture, bool smooth) {
    reinterpret_cast<sf::Texture *>(texture)->setSmooth(smooth);
}

extern "C" bool sfTexture_isSmooth(const sf::Texture *texture) {

    return reinterpret_cast<const sf::Texture *>(texture)->isSmooth();
}

extern "C" void sfTexture_setSrgb(sf::Texture *texture, bool sRgb) {
    reinterpret_cast<sf::Texture *>(texture)->setSrgb(sRgb);
}

extern "C" bool sfTexture_isSrgb(const sf::Texture *texture) {
    return reinterpret_cast<const sf::Texture *>(texture)->isSrgb();
}

extern "C" void sfTexture_setRepeated(sf::Texture *texture, bool repeated) {
    reinterpret_cast<sf::Texture *>(texture)->setRepeated(repeated);
}

extern "C" bool sfTexture_isRepeated(const sf::Texture *texture) {

    return reinterpret_cast<const sf::Texture *>(texture)->isRepeated();
}

extern "C" bool sfTexture_generateMipmap(sf::Texture *texture) {
    return reinterpret_cast<sf::Texture *>(texture)->generateMipmap();
}

extern "C" void sfTexture_swap(sf::Texture *left, sf::Texture *right) {
    sf::Texture *left_ = reinterpret_cast<sf::Texture *>(left);
    sf::Texture *right_ = reinterpret_cast<sf::Texture *>(right);
    left_->swap(*right_);
}

extern "C" unsigned int sfTexture_getNativeHandle(const sf::Texture *texture) {
    return reinterpret_cast<const sf::Texture *>(texture)->getNativeHandle();
}

extern "C" void sfTexture_bind(const sf::Texture *texture) {
    sf::Texture::bind(reinterpret_cast<const sf::Texture *>(texture));
}

extern "C" unsigned int sfTexture_getMaximumSize() {
    return sf::Texture::getMaximumSize();
}
