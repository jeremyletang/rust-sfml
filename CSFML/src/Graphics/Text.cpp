#include "Graphics/Color.hpp"
#include "Graphics/Rect.hpp"
#include "Graphics/TextEnums.hpp"
#include "SFML/Graphics/Font.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Text.hpp>
#include <cstddef>
#include <vector>

extern "C" sf::Text *sfText_new(const sf::Font *font, const uint32_t *string, unsigned int size) {
    return new sf::Text(*font, (char32_t *)string, size);
}

extern "C" sf::Text *sfText_cpy(const sf::Text *text) {
    return new sf::Text(*text);
}

extern "C" void sfText_del(sf::Text *text) {
    delete text;
}

extern "C" void sfText_setPosition(sf::Text *text, sfVector2f position) {
    text->setPosition(convertVector2(position));
}

extern "C" void sfText_setRotation(sf::Text *text, float angle) {
    text->setRotation(sf::degrees(angle));
}

extern "C" void sfText_setScale(sf::Text *text, sfVector2f scale) {
    text->setScale(convertVector2(scale));
}

extern "C" void sfText_setOrigin(sf::Text *text, sfVector2f origin) {
    text->setOrigin(convertVector2(origin));
}

extern "C" sfVector2f sfText_getPosition(const sf::Text *text) {
    return convertVector2(text->getPosition());
}

extern "C" float sfText_getRotation(const sf::Text *text) {
    return text->getRotation().asDegrees();
}

extern "C" sfVector2f sfText_getScale(const sf::Text *text) {
    return convertVector2(text->getScale());
}

extern "C" sfVector2f sfText_getOrigin(const sf::Text *text) {
    return convertVector2(text->getOrigin());
}

extern "C" void sfText_move(sf::Text *text, sfVector2f offset) {
    text->move(convertVector2(offset));
}

extern "C" void sfText_rotate(sf::Text *text, float angle) {
    text->rotate(sf::degrees(angle));
}

extern "C" void sfText_scale(sf::Text *text, sfVector2f factors) {
    text->scale(convertVector2(factors));
}

extern "C" sf::Transform const *sfText_getTransform(const sf::Text *text) {
    return &text->getTransform();
}

extern "C" sf::Transform const *sfText_getInverseTransform(const sf::Text *text) {
    return &text->getInverseTransform();
}

extern "C" void sfText_setUnicodeString(sf::Text *text, const uint32_t *string) {
    text->setString(sf::String((char32_t *)string));
}

extern "C" void sfText_setFont(sf::Text *text, const sf::Font *font) {
    text->setFont(*font);
}

extern "C" void sfText_setCharacterSize(sf::Text *text, unsigned int size) {
    text->setCharacterSize(size);
}

extern "C" void sfText_setLineSpacing(sf::Text *text, float spacingFactor) {
    text->setLineSpacing(spacingFactor);
}

extern "C" void sfText_setLetterSpacing(sf::Text *text, float spacingFactor) {
    text->setLetterSpacing(spacingFactor);
}

extern "C" void sfText_setStyle(sf::Text *text, uint32_t style) {
    text->setStyle(style);
}

extern "C" void sfText_setFillColor(sf::Text *text, sfColor color) {
    text->setFillColor(convertColor(color));
}

extern "C" void sfText_setOutlineColor(sf::Text *text, sfColor color) {
    text->setOutlineColor(convertColor(color));
}

extern "C" void sfText_setOutlineThickness(sf::Text *text, float thickness) {
    text->setOutlineThickness(thickness);
}

extern "C" const uint32_t *sfText_getUnicodeString(const sf::Text *text) {
    return (uint32_t *)text->getString().getData();
}

extern "C" const sf::Font *sfText_getFont(const sf::Text *text) {
    return &text->getFont();
}

extern "C" unsigned int sfText_getCharacterSize(const sf::Text *text) {
    return text->getCharacterSize();
}

extern "C" float sfText_getLetterSpacing(const sf::Text *text) {
    return text->getLetterSpacing();
}

extern "C" float sfText_getLineSpacing(const sf::Text *text) {
    return text->getLineSpacing();
}

extern "C" uint32_t sfText_getStyle(const sf::Text *text) {
    return text->getStyle();
}

extern "C" sfColor sfText_getFillColor(const sf::Text *text) {
    return convertColor(text->getFillColor());
}

extern "C" sfColor sfText_getOutlineColor(const sf::Text *text) {
    return convertColor(text->getOutlineColor());
}

extern "C" float sfText_getOutlineThickness(const sf::Text *text) {
    return text->getOutlineThickness();
}

extern "C" sfFloatRect sfText_getLocalBounds(const sf::Text *text) {
    return convertRect(text->getLocalBounds());
}

extern "C" sfFloatRect sfText_getGlobalBounds(const sf::Text *text) {
    return convertRect(text->getGlobalBounds());
}

extern "C" const std::vector<sf::Text::ShapedGlyph> *sfText_getShapedGlyphs(const sf::Text *text) {
    return &text->getShapedGlyphs();
}

extern "C" std::size_t sfShapedGlyphVector_getLength(const std::vector<sf::Text::ShapedGlyph> *vec) {
    return vec->size();
}

extern "C" const sf::Text::ShapedGlyph *sfShapedGlyphVector_getData(const std::vector<sf::Text::ShapedGlyph> *vec) {
    return vec->data();
}

extern "C" void sfText_setLineAlignment(sf::Text *text, sfLineAlignment lineAlignment) {
    text->setLineAlignment(static_cast<sf::Text::LineAlignment>(lineAlignment));
}

extern "C" sfLineAlignment sfText_getLineAlignment(const sf::Text *text) {
    return static_cast<sfLineAlignment>(text->getLineAlignment());
}

extern "C" void sfText_setTextOrientation(sf::Text *text, sfTextOrientation textOrientation) {
    text->setTextOrientation(static_cast<sf::Text::TextOrientation>(textOrientation));
}

extern "C" sfTextOrientation sfText_getTextOrientation(const sf::Text *text) {
    return static_cast<sfTextOrientation>(text->getTextOrientation());
}

extern "C" void sfText_setClusterGrouping(sf::Text *text, sfClusterGrouping clusterGrouping) {
    text->setClusterGrouping(static_cast<sf::Text::ClusterGrouping>(clusterGrouping));
}

extern "C" sfClusterGrouping sfText_getClusterGrouping(const sf::Text *text) {
    return static_cast<sfClusterGrouping>(text->getClusterGrouping());
}

typedef void (*sfTextGlyphPreProcessorCb)(const sf::Text::ShapedGlyph *shapedGlyph,
                                          std::uint32_t *style,
                                          sfColor *fillColor,
                                          sfColor *outlineColor,
                                          float *outlineThickness,
                                          void *userData);

extern "C" void sfText_setGlyphPreProcessor(sf::Text *text, sfTextGlyphPreProcessorCb callback, void *userData) {
    if (!callback) {
        text->setGlyphPreProcessor({});
        return;
    }

    text->setGlyphPreProcessor([callback, userData](const sf::Text::ShapedGlyph &shapedGlyph,
                                                    std::uint32_t &style,
                                                    sf::Color &fillColor,
                                                    sf::Color &outlineColor,
                                                    float &outlineThickness) {
        sfColor fill = convertColor(fillColor);
        sfColor outline = convertColor(outlineColor);
        callback(&shapedGlyph, &style, &fill, &outline, &outlineThickness, userData);
        fillColor = convertColor(fill);
        outlineColor = convertColor(outline);
    });
}

static sf::Vertex *vertexArrayData(sf::VertexArray &vertices, std::size_t *vertexCount) {
    *vertexCount = vertices.getVertexCount();
    return *vertexCount ? &vertices[0] : nullptr;
}

// getVertexData and getOutlineVertexData don't regenerate the geometry themselves,
// so query the local bounds first to make sure the vertices are up to date
extern "C" sf::Vertex *sfText_getVertexData(sf::Text *text, std::size_t *vertexCount) {
    (void)text->getLocalBounds();
    return vertexArrayData(text->getVertexData(), vertexCount);
}

extern "C" sf::Vertex *sfText_getOutlineVertexData(sf::Text *text, std::size_t *vertexCount) {
    (void)text->getLocalBounds();
    return vertexArrayData(text->getOutlineVertexData(), vertexCount);
}
