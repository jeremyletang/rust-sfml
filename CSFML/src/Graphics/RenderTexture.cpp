#include "Graphics/PrimitiveType.hpp"
#include "Graphics/Rect.hpp"
#include "Graphics/Color.hpp"
#include "SFML/Graphics/PrimitiveType.hpp"
#include "SFML/Window/ContextSettings.hpp"
#include "System/Vector2.hpp"
#include <SFML/Graphics/RenderTarget.hpp>
#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderTexture.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/Text.hpp>
#include <cstddef>

extern "C" sf::RenderTexture *sfRenderTexture_new() {
    return new sf::RenderTexture;
}

extern "C" void sfRenderTexture_del(sf::RenderTexture *renderTexture) {
    delete renderTexture;
}

extern "C" bool sfRenderTexture_resize(sf::RenderTexture *texture, sfVector2u size, const sf::ContextSettings *settings) {
    return texture->resize({size.x, size.y}, *settings);
}

extern "C" sfVector2u sfRenderTexture_getSize(const sf::RenderTexture *renderTexture) {
    sf::Vector2u size = renderTexture->getSize();
    return {size.x, size.y};
}

extern "C" bool sfRenderTexture_isSrgb(const sf::RenderTexture *renderTexture) {
    return renderTexture->isSrgb();
}

extern "C" bool sfRenderTexture_setActive(sf::RenderTexture *renderTexture, bool active) {
    return renderTexture->setActive(active);
}

extern "C" void sfRenderTexture_display(sf::RenderTexture *renderTexture) {
    renderTexture->display();
}

extern "C" void sfRenderTexture_clear(sf::RenderTexture *renderTexture, sfColor color) {
    renderTexture->clear(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfRenderTexture_setView(sf::RenderTexture *renderTexture, const sf::View *view) {
    renderTexture->setView(*view);
}

extern "C" const sf::View *sfRenderTexture_getView(const sf::RenderTexture *renderTexture) {
    return &renderTexture->getView();
}

extern "C" const sf::View *sfRenderTexture_getDefaultView(const sf::RenderTexture *renderTexture) {
    return &renderTexture->getDefaultView();
}

extern "C" sfIntRect sfRenderTexture_getViewport(const sf::RenderTexture *renderTexture, const sf::View *view) {
    sf::IntRect rect = renderTexture->getViewport(*view);
    return {rect.position.x, rect.position.y, rect.size.x, rect.size.y};
}

extern "C" sfVector2f sfRenderTexture_mapPixelToCoords(const sf::RenderTexture *renderTexture, sfVector2i point) {
    sf::Vector2f result = renderTexture->mapPixelToCoords(sf::Vector2i({point.x, point.y}));
    return {result.x, result.y};
}

extern "C" sfVector2f sfRenderTexture_mapPixelToCoords_View(const sf::RenderTexture *renderTexture, sfVector2i point, const sf::View *targetView) {
    sf::Vector2f result = renderTexture->mapPixelToCoords(sf::Vector2i({point.x, point.y}), *targetView);
    return {result.x, result.y};
}

extern "C" sfVector2i sfRenderTexture_mapCoordsToPixel(const sf::RenderTexture *renderTexture, sfVector2f point) {
    sf::Vector2i result = renderTexture->mapCoordsToPixel(sf::Vector2f({point.x, point.y}));
    return {result.x, result.y};
}

extern "C" sfVector2i sfRenderTexture_mapCoordsToPixel_View(const sf::RenderTexture *renderTexture, sfVector2f point, const sf::View *targetView) {
    sf::Vector2i result = renderTexture->mapCoordsToPixel(sf::Vector2f({point.x, point.y}), *targetView);
    return {result.x, result.y};
}

extern "C" void sfRenderTexture_drawSprite(sf::RenderTexture *renderTexture, const sf::Sprite *object, const sf::RenderStates *states) {
    renderTexture->draw(*object, *states);
}
extern "C" void sfRenderTexture_drawText(sf::RenderTexture *renderTexture, const sf::Text *object, const sf::RenderStates *states) {
    renderTexture->draw(*object, *states);
}
extern "C" void sfRenderTexture_drawShape(sf::RenderTexture *renderTexture, const sf::Shape *object, const sf::RenderStates *states) {
    renderTexture->draw(*object, *states);
}
extern "C" void sfRenderTexture_drawCircleShape(sf::RenderTexture *renderTexture, const sf::CircleShape *object, const sf::RenderStates *states) {
    renderTexture->draw(*object, *states);
}
extern "C" void sfRenderTexture_drawConvexShape(sf::RenderTexture *renderTexture, const sf::ConvexShape *object, const sf::RenderStates *states) {
    renderTexture->draw(*object, *states);
}
extern "C" void sfRenderTexture_drawRectangleShape(sf::RenderTexture *renderTexture, const sf::RectangleShape *object, const sf::RenderStates *states) {
    renderTexture->draw(*object, *states);
}
extern "C" void sfRenderTexture_drawVertexBuffer(sf::RenderTexture *renderTexture, const sf::VertexBuffer *object, const sf::RenderStates *states) {
    renderTexture->draw(*object, *states);
}

extern "C" void sfRenderTexture_drawPrimitives(sf::RenderTexture *renderTexture,
                                               const sf::Vertex *vertices, size_t vertexCount,
                                               sfPrimitiveType type, const sf::RenderStates *states) {
    renderTexture->draw(vertices, vertexCount, static_cast<sf::PrimitiveType>(type), *states);
}

extern "C" void sfRenderTexture_pushGLStates(sf::RenderTexture *renderTexture) {
    renderTexture->pushGLStates();
}

extern "C" void sfRenderTexture_popGLStates(sf::RenderTexture *renderTexture) {
    renderTexture->popGLStates();
}

extern "C" void sfRenderTexture_resetGLStates(sf::RenderTexture *renderTexture) {
    renderTexture->resetGLStates();
}

extern "C" const sf::Texture *sfRenderTexture_getTexture(const sf::RenderTexture *renderTexture) {
    return &renderTexture->getTexture();
}

extern "C" void sfRenderTexture_setSmooth(sf::RenderTexture *renderTexture, bool smooth) {
    renderTexture->setSmooth(smooth);
}

extern "C" unsigned int sfRenderTexture_getMaximumAntialiasingLevel() {
    return sf::RenderTexture::getMaximumAntiAliasingLevel();
}

extern "C" bool sfRenderTexture_isSmooth(const sf::RenderTexture *renderTexture) {
    return renderTexture->isSmooth();
}

extern "C" void sfRenderTexture_setRepeated(sf::RenderTexture *renderTexture, bool repeated) {
    renderTexture->setRepeated(repeated);
}

extern "C" bool sfRenderTexture_isRepeated(const sf::RenderTexture *renderTexture) {
    return renderTexture->isRepeated();
}

extern "C" bool sfRenderTexture_generateMipmap(sf::RenderTexture *renderTexture) {
    return renderTexture->generateMipmap();
}
