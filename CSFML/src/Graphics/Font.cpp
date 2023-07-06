#include "Graphics/Rect.h"
#include "SFML/Config.h"
#include "System/InputStreamStruct.h"
#include <SFML/Graphics/Font.hpp>
#include <cstddef>

typedef struct
{
    float advance;         ///< Offset to move horizontically to the next character
    sfFloatRect bounds;    ///< Bounding rectangle of the glyph, in coordinates relative to the baseline
    sfIntRect textureRect; ///< Texture coordinates of the glyph inside the font's image
} sfGlyph;

extern "C" sf::Font *sfFont_createFromFile(const char *filename) {
    sf::Font *font = new sf::Font;
    if (!font->loadFromFile(filename)) {
        delete font;
        font = NULL;
    }

    return font;
}

extern "C" sf::Font *sfFont_createFromMemory(const void *data, size_t sizeInBytes) {
    sf::Font *font = new sf::Font;
    if (!font->loadFromMemory(data, sizeInBytes)) {
        delete font;
        font = NULL;
    }

    return font;
}

extern "C" sf::Font *sfFont_createFromStream(sfInputStream *stream) {

    sf::Font *font = new sf::Font;
    if (!font->loadFromStream(*stream)) {
        delete font;
        font = NULL;
    }

    return font;
}

extern "C" sf::Font *sfFont_copy(const sf::Font *font) {
    return new sf::Font(*font);
}

extern "C" void sfFont_destroy(sf::Font *font) {
    delete font;
}

extern "C" sfGlyph sfFont_getGlyph(const sf::Font *font, uint32_t codePoint, unsigned int characterSize, bool bold, float outlineThickness) {
    sf::Glyph glyph = font->getGlyph(codePoint, characterSize, bold, outlineThickness);
    return {
        glyph.advance,
        {glyph.bounds.left, glyph.bounds.top, glyph.bounds.width, glyph.bounds.height},
        {glyph.textureRect.left, glyph.textureRect.top, glyph.textureRect.width, glyph.textureRect.height}};
}

extern "C" float sfFont_getKerning(const sf::Font *font, uint32_t first, uint32_t second, unsigned int characterSize) {
    return font->getKerning(first, second, characterSize);
}

extern "C" float sfFont_getBoldKerning(const sf::Font *font, uint32_t first, uint32_t second, unsigned int characterSize) {
    return font->getKerning(first, second, characterSize, true);
}

extern "C" float sfFont_getLineSpacing(const sf::Font *font, unsigned int characterSize) {
    return font->getLineSpacing(characterSize);
}

extern "C" float sfFont_getUnderlinePosition(const sf::Font *font, unsigned int characterSize) {
    return font->getUnderlinePosition(characterSize);
}

extern "C" float sfFont_getUnderlineThickness(const sf::Font *font, unsigned int characterSize) {
    return font->getUnderlineThickness(characterSize);
}

extern "C" const sf::Texture *sfFont_getTexture(const sf::Font *font, unsigned int characterSize) {
    return &font->getTexture(characterSize);
}

extern "C" bool sfFont_isSmooth(const sf::Font *font) {
    return font->isSmooth();
}

extern "C" void sfFont_setSmooth(sf::Font *font, bool smooth) {
    font->setSmooth(smooth);
}

typedef struct
{
    const char *family;
} sfFontInfo;

extern "C" sfFontInfo sfFont_getInfo(const sf::Font *font) {
    return {font->getInfo().family.c_str()};
}
