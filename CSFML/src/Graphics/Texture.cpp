#include "Graphics/Rect.h"
#include "System/InputStreamStruct.h"
#include "System/Vector2.h"
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Texture.hpp>
#include <cstddef>

extern "C" sf::Texture *sfTexture_new() {
    return new sf::Texture;
}

extern "C" bool sfTexture_create(sf::Texture *tex, unsigned int width, unsigned int height) {
    return tex->create(width, height);
}

extern "C" bool sfTexture_loadFromFile(sf::Texture *tex, const char *filename, const sfIntRect area) {
    return tex->loadFromFile(filename, sf::IntRect(area.left, area.top, area.width, area.height));
}

extern "C" bool sfTexture_loadFromMemory(sf::Texture *tex, const void *data, size_t sizeInBytes, const sfIntRect area) {
    return tex->loadFromMemory(data, sizeInBytes, sf::IntRect(area.left, area.top, area.width, area.height));
}

extern "C" bool sfTexture_loadFromStream(sf::Texture *tex, sfInputStream *stream, const sfIntRect area) {
    return tex->loadFromStream(*stream, sf::IntRect(area.left, area.top, area.width, area.height));
}

extern "C" bool sfTexture_loadFromImage(sf::Texture *tex, const sf::Image *image, const sfIntRect area) {
    return tex->loadFromImage(*image, sf::IntRect(area.left, area.top, area.width, area.height));
}

extern "C" sf::Texture *sfTexture_copy(const sf::Texture *texture) {
    return new sf::Texture(*texture);
}

extern "C" void sfTexture_destroy(sf::Texture *texture) {
    delete texture;
}

extern "C" sfVector2u sfTexture_getSize(const sf::Texture *texture) {
    sf::Vector2u vec2 = texture->getSize();
    return {vec2.x, vec2.y};
}

extern "C" sf::Image *sfTexture_copyToImage(const sf::Texture *texture) {
    return new sf::Image(texture->copyToImage());
}

extern "C" void sfTexture_updateFromPixels(sf::Texture *texture, const uint8_t *pixels, unsigned int width, unsigned int height, unsigned int x, unsigned int y) {
    texture->update(pixels, width, height, x, y);
}

extern "C" void sfTexture_updateFromTexture(sf::Texture *destination, const sf::Texture *texture, unsigned int x, unsigned int y) {
    destination->update(*texture, x, y);
}

extern "C" void sfTexture_updateFromImage(sf::Texture *texture, const sf::Image *image, unsigned int x, unsigned int y) {
    texture->update(*image, x, y);
}

extern "C" void sfTexture_updateFromWindow(sf::Texture *texture, const sf::Window *window, unsigned int x, unsigned int y) {
    texture->update(*window, x, y);
}

extern "C" void sfTexture_updateFromRenderWindow(sf::Texture *texture, const sf::RenderWindow *renderWindow, unsigned int x, unsigned int y) {
    texture->update(*renderWindow, x, y);
}

extern "C" void sfTexture_setSmooth(sf::Texture *texture, bool smooth) {
    texture->setSmooth(smooth);
}

extern "C" bool sfTexture_isSmooth(const sf::Texture *texture) {
    return texture->isSmooth();
}

extern "C" void sfTexture_setSrgb(sf::Texture *texture, bool sRgb) {
    texture->setSrgb(sRgb);
}

extern "C" bool sfTexture_isSrgb(const sf::Texture *texture) {
    return texture->isSrgb();
}

extern "C" void sfTexture_setRepeated(sf::Texture *texture, bool repeated) {
    texture->setRepeated(repeated);
}

extern "C" bool sfTexture_isRepeated(const sf::Texture *texture) {

    return texture->isRepeated();
}

extern "C" bool sfTexture_generateMipmap(sf::Texture *texture) {
    return texture->generateMipmap();
}

extern "C" void sfTexture_swap(sf::Texture *left, sf::Texture *right) {
    left->swap(*right);
}

extern "C" unsigned int sfTexture_getNativeHandle(const sf::Texture *texture) {
    return texture->getNativeHandle();
}

extern "C" void sfTexture_bind(const sf::Texture *texture) {
    sf::Texture::bind(texture);
}

extern "C" unsigned int sfTexture_getMaximumSize() {
    return sf::Texture::getMaximumSize();
}
