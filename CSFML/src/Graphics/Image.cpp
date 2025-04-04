#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "System/InputStreamHelper.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Image.hpp>
#include <cstddef>

extern "C" sf::Image *sfImage_new() {
    return new sf::Image;
}

extern "C" sf::Image *sfImage_cpy(const sf::Image *image) {
    return new sf::Image(*image);
}

extern "C" void sfImage_del(sf::Image *image) {
    delete image;
}

extern "C" void sfImage_resizeWithColor(sf::Image *image, sfVector2u size, sfColor color) {
    image->resize({size.x, size.y}, sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfImage_resizeWithPixels(sf::Image *image, sfVector2u size, const uint8_t *pixels) {
    image->resize({size.x, size.y}, pixels);
}

extern "C" bool sfImage_loadFromFile(sf::Image *image, const char *filename) {
    return image->loadFromFile(filename);
}

extern "C" bool sfImage_loadFromMemory(sf::Image *image, const uint8_t *data, size_t sizeInBytes) {
    return image->loadFromMemory(data, sizeInBytes);
}

extern "C" bool sfImage_loadFromStream(sf::Image *image, sfInputStreamHelper *stream) {
    return image->loadFromStream(*stream);
}

extern "C" bool sfImage_saveToFile(const sf::Image *image, const char *filename) {
    return image->saveToFile(filename);
}

extern "C" void sfImage_createMaskFromColor(sf::Image *image, sfColor colorKey, uint8_t alpha) {
    image->createMaskFromColor(sf::Color(colorKey.r, colorKey.g, colorKey.b, colorKey.a), alpha);
}

extern "C" bool sfImage_copy(sf::Image *image, const sf::Image *source, sfVector2u dest, sfIntRect sourceRect, bool applyAlpha) {
    sf::IntRect sfmlRect = {{sourceRect.position.x, sourceRect.position.y}, {sourceRect.size.x, sourceRect.size.y}};
    return image->copy(*source, sf::Vector2u(dest.x, dest.y), sfmlRect, applyAlpha);
}

extern "C" void sfImage_setPixel(sf::Image *image, sfVector2u coords, sfColor color) {
    image->setPixel(sf::Vector2u(coords.x, coords.y), sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" sfColor sfImage_getPixel(const sf::Image *image, sfVector2u coords) {
    sf::Color color = image->getPixel(sf::Vector2u(coords.x, coords.y));
    return sfColor{color.r, color.g, color.b, color.a};
}

extern "C" const uint8_t *sfImage_getPixelsPtr(const sf::Image *image) {
    return image->getPixelsPtr();
}

extern "C" sfVector2u sfImage_getSize(const sf::Image *image) {
    sf::Vector2u size = image->getSize();
    return {size.x, size.y};
}

extern "C" void sfImage_flipHorizontally(sf::Image *image) {
    image->flipHorizontally();
}

extern "C" void sfImage_flipVertically(sf::Image *image) {
    image->flipVertically();
}
