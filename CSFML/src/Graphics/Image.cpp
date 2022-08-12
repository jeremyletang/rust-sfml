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

extern "C" sf::Image *sfImage_createFromPixels(unsigned int width, unsigned int height, const uint8_t *data) {
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

extern "C" bool sfImage_saveToFile(const sf::Image *image, const char *filename) {
    return image->saveToFile(filename);
}

extern "C" void sfImage_createMaskFromColor(sf::Image *image, sfColor colorKey, uint8_t alpha) {
    image->createMaskFromColor(sf::Color(colorKey.r, colorKey.g, colorKey.b, colorKey.a), alpha);
}

extern "C" void sfImage_copyImage(sf::Image *image, const sf::Image *source, unsigned int destX, unsigned int destY, sfIntRect sourceRect, bool applyAlpha) {
    sf::IntRect sfmlRect(sourceRect.left, sourceRect.top, sourceRect.width, sourceRect.height);
    image->copy(*source, destX, destY, sfmlRect, applyAlpha);
}

extern "C" void sfImage_setPixel(sf::Image *image, unsigned int x, unsigned int y, sfColor color) {
    image->setPixel(x, y, sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" sfColor sfImage_getPixel(const sf::Image *image, unsigned int x, unsigned int y) {
    sf::Color color = image->getPixel(x, y);
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
