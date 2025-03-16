#include "Graphics/Rect.hpp"
#include "SFML/Graphics/Image.hpp"
#include "System/InputStreamHelper.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Texture.hpp>
#include <cstddef>

extern "C" sf::Texture *sfTexture_new() {
    return new sf::Texture;
}

extern "C" sf::Texture *sfTexture_cpy(const sf::Texture *texture) {
    return new sf::Texture(*texture);
}

extern "C" void sfTexture_del(sf::Texture *texture) {
    delete texture;
}

extern "C" bool sfTexture_resize(sf::Texture *texture, sfVector2u size, bool sRgb) {
    return texture->resize(convertVector2(size), sRgb);
}

extern "C" bool sfTexture_loadFromFile(sf::Texture *tex, const char *filename, bool sRgb, const sfIntRect area) {
    return tex->loadFromFile(filename, sRgb, convertRect(area));
}

extern "C" bool sfTexture_loadFromMemory(sf::Texture *tex, const void *data, size_t sizeInBytes, bool sRgb, const sfIntRect area) {
    return tex->loadFromMemory(data, sizeInBytes, sRgb, convertRect(area));
}

extern "C" bool sfTexture_loadFromStream(sf::Texture *tex, sfInputStreamHelper *stream, bool sRgb, const sfIntRect area) {
    return tex->loadFromStream(*stream, sRgb, convertRect(area));
}

extern "C" bool sfTexture_loadFromImage(sf::Texture *tex, const sf::Image *image, bool sRgb, const sfIntRect area) {
    return tex->loadFromImage(*image, sRgb, convertRect(area));
}

extern "C" sfVector2u sfTexture_getSize(const sf::Texture *texture) {
    return convertVector2(texture->getSize());
}

extern "C" sf::Image *sfTexture_copyToImage(const sf::Texture *texture) {
    return new sf::Image(texture->copyToImage());
}

extern "C" void sfTexture_updateFromPixels(sf::Texture *texture, const uint8_t *pixels, sfVector2u size, sfVector2u dest) {
    texture->update(pixels, convertVector2(size), convertVector2(dest));
}

extern "C" void sfTexture_updateFromTexture(sf::Texture *destination, const sf::Texture *texture, sfVector2u dest) {
    destination->update(*texture, convertVector2(dest));
}

extern "C" void sfTexture_updateFromImage(sf::Texture *texture, const sf::Image *image, sfVector2u dest) {
    texture->update(*image, convertVector2(dest));
}

extern "C" void sfTexture_updateFromWindow(sf::Texture *texture, const sf::Window *window, sfVector2u dest) {
    texture->update(*window, convertVector2(dest));
}

extern "C" void sfTexture_updateFromRenderWindow(sf::Texture *texture, const sf::RenderWindow *renderWindow, sfVector2u dest) {
    texture->update(*renderWindow, convertVector2(dest));
}

extern "C" void sfTexture_setSmooth(sf::Texture *texture, bool smooth) {
    texture->setSmooth(smooth);
}

extern "C" bool sfTexture_isSmooth(const sf::Texture *texture) {
    return texture->isSmooth();
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
