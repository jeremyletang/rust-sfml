#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "System/InputStreamHelper.hpp"
#include "System/Vector2.hpp"
#include "System/Buffer.hpp"
#include <SFML/Graphics/Image.hpp>
#include <cstddef>
#include <cstdint>

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
    image->resize(convertVector2(size), convertColor(color));
}

extern "C" void sfImage_resizeWithPixels(sf::Image *image, sfVector2u size, const uint8_t *pixels) {
    image->resize(convertVector2(size), pixels);
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

extern "C" sfBuffer *sfImage_saveToMemory(const sf::Image *image, const char *format) {
    if (auto data = image->saveToMemory(format)) {
        return new sfBuffer{std::move(*data)};
    }
    return nullptr;
}

extern "C" void sfImage_createMaskFromColor(sf::Image *image, sfColor colorKey, uint8_t alpha) {
    image->createMaskFromColor(convertColor(colorKey), alpha);
}

extern "C" bool sfImage_copy(sf::Image *image, const sf::Image *source, sfVector2u dest, sfIntRect sourceRect, bool applyAlpha) {
    return image->copy(*source, convertVector2(dest), convertRect(sourceRect), applyAlpha);
}

extern "C" void sfImage_setPixel(sf::Image *image, sfVector2u coords, sfColor color) {
    image->setPixel(convertVector2(coords), convertColor(color));
}

extern "C" sfColor sfImage_getPixel(const sf::Image *image, sfVector2u coords) {
    return convertColor(image->getPixel(convertVector2(coords)));
}

extern "C" const uint8_t *sfImage_getPixelsPtr(const sf::Image *image) {
    return image->getPixelsPtr();
}

extern "C" sfVector2u sfImage_getSize(const sf::Image *image) {
    return convertVector2(image->getSize());
}

extern "C" void sfImage_flipHorizontally(sf::Image *image) {
    image->flipHorizontally();
}

extern "C" void sfImage_flipVertically(sf::Image *image) {
    image->flipVertically();
}
