#include "Graphics/Rect.hpp"
#include "System/InputStreamHelper.hpp"
#include <SFML/Graphics/Font.hpp>
#include <cstddef>

struct sfGlyph {
    float advance;         ///< Offset to move horizontically to the next character
    sfFloatRect bounds;    ///< Bounding rectangle of the glyph, in coordinates relative to the baseline
    sfIntRect textureRect; ///< Texture coordinates of the glyph inside the font's image
};

extern "C" sf::Font *sfFont_new() {
    return new sf::Font;
}

extern "C" void sfFont_del(sf::Font *font) {
    delete font;
}

extern "C" sf::Font *sfFont_cpy(const sf::Font *font) {
    return new sf::Font(*font);
}

extern "C" bool sfFont_openFromFile(sf::Font *font, const char *filename) {
    return font->openFromFile(filename);
}

extern "C" bool sfFont_openFromMemory(sf::Font *font, const uint8_t *data, size_t sizeInBytes) {
    return font->openFromMemory(data, sizeInBytes);
}

extern "C" bool sfFont_openFromStream(sf::Font *font, sfInputStreamHelper *stream) {
    return font->openFromStream(*stream);
}

extern "C" sfGlyph sfFont_getGlyph(const sf::Font *font, uint32_t codePoint, unsigned int characterSize, bool bold, float outlineThickness) {
    sf::Glyph glyph = font->getGlyph(codePoint, characterSize, bold, outlineThickness);
    return {
        glyph.advance,
        {glyph.bounds.position.x, glyph.bounds.position.y, glyph.bounds.size.x, glyph.bounds.size.y},
        {glyph.textureRect.position.x, glyph.textureRect.position.y, glyph.textureRect.size.x, glyph.textureRect.size.y}};
}

extern "C" bool sfFont_hasGlyph(const sf::Font *font, uint32_t codePoint) {
    return font->hasGlyph(codePoint);
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

struct sfFontInfo {
    const char *family;
};

extern "C" sfFontInfo sfFont_getInfo(const sf::Font *font) {
    return {font->getInfo().family.c_str()};
}
