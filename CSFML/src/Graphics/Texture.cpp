
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
#include "Graphics/Types.h"
#include "System/InputStreamStruct.h"
#include "System/Vector2.h"
#include "Window/WindowStruct.h"
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Texture.hpp>
#include <cstddef>

extern "C" sfTexture *sfTexture_new() {
    sf::Texture *texture = new sf::Texture;
    return reinterpret_cast<sfTexture *>(texture);
}

extern "C" sfBool sfTexture_create(sfTexture *tex, unsigned int width, unsigned int height) {
    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);
    return texture->create(width, height);
}

extern "C" sfBool sfTexture_loadFromFile(sfTexture *tex, const char *filename, const sfIntRect area) {
    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);

    sf::IntRect rect = sf::IntRect(area.left, area.top, area.width, area.height);

    return texture->loadFromFile(filename, rect);
}

extern "C" sfBool sfTexture_loadFromMemory(sfTexture *tex, const void *data, size_t sizeInBytes, const sfIntRect area) {
    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);

    sf::IntRect rect = sf::IntRect(area.left, area.top, area.width, area.height);

    return texture->loadFromMemory(data, sizeInBytes, rect);
}

extern "C" sfBool sfTexture_loadFromStream(sfTexture *tex, sfInputStream *stream, const sfIntRect area) {

    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);

    sf::IntRect rect = sf::IntRect(area.left, area.top, area.width, area.height);

    return texture->loadFromStream(*stream, rect);
}

extern "C" sfBool sfTexture_loadFromImage(sfTexture *tex, const sf::Image *image, const sfIntRect area) {

    sf::Texture *texture = reinterpret_cast<sf::Texture *>(tex);

    sf::IntRect rect = sf::IntRect(area.left, area.top, area.width, area.height);

    return texture->loadFromImage(*image, rect);
}

extern "C" sfTexture *sfTexture_copy(const sfTexture *texture) {
    const sf::Texture *texture_ = reinterpret_cast<const sf::Texture *>(texture);
    sf::Texture *newTexture = new sf::Texture(*texture_);
    return reinterpret_cast<sfTexture *>(newTexture);
}

extern "C" void sfTexture_destroy(sfTexture *texture) {
    delete reinterpret_cast<sf::Texture *>(texture);
}

extern "C" sfVector2u sfTexture_getSize(const sfTexture *texture) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = reinterpret_cast<const sf::Texture *>(texture)->getSize();

    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" sf::Image *sfTexture_copyToImage(const sfTexture *texture) {

    sf::Image *image = new sf::Image;
    *image = reinterpret_cast<const sf::Texture *>(texture)->copyToImage();

    return image;
}

extern "C" void sfTexture_updateFromPixels(sfTexture *texture, const sfUint8 *pixels, unsigned int width, unsigned int height, unsigned int x, unsigned int y) {
    reinterpret_cast<sf::Texture *>(texture)->update(pixels, width, height, x, y);
}

extern "C" void sfTexture_updateFromTexture(sfTexture *destination, const sfTexture *texture, unsigned int x, unsigned int y) {
    sf::Texture *destination_ = reinterpret_cast<sf::Texture *>(destination);
    const sf::Texture *texture_ = reinterpret_cast<const sf::Texture *>(texture);
    destination_->update(*texture_, x, y);
}

extern "C" void sfTexture_updateFromImage(sfTexture *texture, const sf::Image *image, unsigned int x, unsigned int y) {

    reinterpret_cast<sf::Texture *>(texture)->update(*image, x, y);
}

extern "C" void sfTexture_updateFromWindow(sfTexture *texture, const sfWindow *window, unsigned int x, unsigned int y) {

    reinterpret_cast<sf::Texture *>(texture)->update(window->This, x, y);
}

extern "C" void sfTexture_updateFromRenderWindow(sfTexture *texture, const sfRenderWindow *renderWindow, unsigned int x, unsigned int y) {
    const sf::RenderWindow *win = reinterpret_cast<const sf::RenderWindow *>(renderWindow);
    reinterpret_cast<sf::Texture *>(texture)->update(*win, x, y);
}

extern "C" void sfTexture_setSmooth(sfTexture *texture, sfBool smooth) {
    reinterpret_cast<sf::Texture *>(texture)->setSmooth(smooth == sfTrue);
}

extern "C" sfBool sfTexture_isSmooth(const sfTexture *texture) {

    return reinterpret_cast<const sf::Texture *>(texture)->isSmooth();
}

extern "C" void sfTexture_setSrgb(sfTexture *texture, sfBool sRgb) {
    reinterpret_cast<sf::Texture *>(texture)->setSrgb(sRgb == sfTrue);
}

extern "C" sfBool sfTexture_isSrgb(const sfTexture *texture) {
    return reinterpret_cast<const sf::Texture *>(texture)->isSrgb();
}

extern "C" void sfTexture_setRepeated(sfTexture *texture, sfBool repeated) {
    reinterpret_cast<sf::Texture *>(texture)->setRepeated(repeated == sfTrue);
}

extern "C" sfBool sfTexture_isRepeated(const sfTexture *texture) {

    return reinterpret_cast<const sf::Texture *>(texture)->isRepeated();
}

extern "C" sfBool sfTexture_generateMipmap(sfTexture *texture) {
    return reinterpret_cast<sf::Texture *>(texture)->generateMipmap();
}

extern "C" void sfTexture_swap(sfTexture *left, sfTexture *right) {
    sf::Texture *left_ = reinterpret_cast<sf::Texture *>(left);
    sf::Texture *right_ = reinterpret_cast<sf::Texture *>(right);
    left_->swap(*right_);
}

extern "C" unsigned int sfTexture_getNativeHandle(const sfTexture *texture) {
    return reinterpret_cast<const sf::Texture *>(texture)->getNativeHandle();
}

extern "C" void sfTexture_bind(const sfTexture *texture) {
    sf::Texture::bind(reinterpret_cast<const sf::Texture *>(texture));
}

extern "C" unsigned int sfTexture_getMaximumSize() {
    return sf::Texture::getMaximumSize();
}
