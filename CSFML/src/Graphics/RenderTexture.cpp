
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

#include "Graphics/CircleShapeStruct.h"
#include "Graphics/ConvexShapeStruct.h"
#include "Graphics/ShapeStruct.h"
#include "Graphics/VertexBufferStruct.h"
#include "Graphics/Vertex.h"
#include "Graphics/PrimitiveType.h"
#include "Window/ContextSettingsInternal.h"
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/Text.hpp>
#include <SFML/Graphics/RenderTexture.hpp>
#include <cstddef>

extern "C" sf::RenderTexture *sfRenderTexture_createWithSettings(unsigned int width, unsigned int height, const sfContextSettings *settings) {
    // Convert context settings
    sf::ContextSettings params;
    if (settings) {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the render texture
    sf::RenderTexture *renderTexture = new sf::RenderTexture;
    renderTexture->create(width, height, params);

    return renderTexture;
}

extern "C" void sfRenderTexture_destroy(sf::RenderTexture *renderTexture) {
    delete renderTexture;
}

extern "C" sfVector2u sfRenderTexture_getSize(const sf::RenderTexture *renderTexture) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = renderTexture->getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" sfBool sfRenderTexture_setActive(sf::RenderTexture *renderTexture, sfBool active) {
    return renderTexture->setActive(active == sfTrue);
}

extern "C" void sfRenderTexture_display(sf::RenderTexture *renderTexture) {
    renderTexture->display();
}

extern "C" void sfRenderTexture_clear(sf::RenderTexture *renderTexture, sfColor color) {
    sf::Color SFMLColor(color.r, color.g, color.b, color.a);

    renderTexture->clear(SFMLColor);
}

extern "C" void sfRenderTexture_setView(sf::RenderTexture *renderTexture, const sfView *view) {
    const sf::View *view_ = reinterpret_cast<const sf::View *>(view);
    renderTexture->setView(*view_);
}

extern "C" const sfView *sfRenderTexture_getView(const sf::RenderTexture *renderTexture) {
    return reinterpret_cast<const sfView *>(&renderTexture->getView());
}

extern "C" const sfView *sfRenderTexture_getDefaultView(const sf::RenderTexture *renderTexture) {
    return reinterpret_cast<const sfView *>(&renderTexture->getDefaultView());
}

extern "C" sfIntRect sfRenderTexture_getViewport(const sf::RenderTexture *renderTexture, const sfView *view) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect SFMLrect = renderTexture->getViewport(*reinterpret_cast<const sf::View *>(view));
    rect.left = SFMLrect.left;
    rect.top = SFMLrect.top;
    rect.width = SFMLrect.width;
    rect.height = SFMLrect.height;

    return rect;
}

extern "C" sfVector2f sfRenderTexture_mapPixelToCoords(const sf::RenderTexture *renderTexture, sfVector2i point, const sfView *targetView) {
    sfVector2f result = {0, 0};

    sf::Vector2f sfmlPoint;
    if (targetView)
        sfmlPoint = renderTexture->mapPixelToCoords(sf::Vector2i(point.x, point.y), *reinterpret_cast<const sf::View *>(targetView));
    else
        sfmlPoint = renderTexture->mapPixelToCoords(sf::Vector2i(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

extern "C" sfVector2i sfRenderTexture_mapCoordsToPixel(const sf::RenderTexture *renderTexture, sfVector2f point, const sfView *targetView) {
    sfVector2i result = {0, 0};

    sf::Vector2i sfmlPoint;
    if (targetView)
        sfmlPoint = renderTexture->mapCoordsToPixel(sf::Vector2f(point.x, point.y), *reinterpret_cast<const sf::View *>(targetView));
    else
        sfmlPoint = renderTexture->mapCoordsToPixel(sf::Vector2f(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

extern "C" void sfRenderTexture_drawSprite(sf::RenderTexture *renderTexture, const sfSprite *object, const sf::RenderStates *states) {

    renderTexture->draw(*reinterpret_cast<const sf::Sprite *>(object), *states);
}
extern "C" void sfRenderTexture_drawText(sf::RenderTexture *renderTexture, const sfText *object, const sf::RenderStates *states) {

    renderTexture->draw(*reinterpret_cast<const sf::Text *>(object), *states);
}
extern "C" void sfRenderTexture_drawShape(sf::RenderTexture *renderTexture, const sfShape *object, const sf::RenderStates *states) {

    renderTexture->draw(object->This, *states);
}
extern "C" void sfRenderTexture_drawCircleShape(sf::RenderTexture *renderTexture, const sfCircleShape *object, const sf::RenderStates *states) {

    renderTexture->draw(object->This, *states);
}
extern "C" void sfRenderTexture_drawConvexShape(sf::RenderTexture *renderTexture, const sfConvexShape *object, const sf::RenderStates *states) {

    renderTexture->draw(object->This, *states);
}
extern "C" void sfRenderTexture_drawRectangleShape(sf::RenderTexture *renderTexture, const sfRectangleShape *object, const sf::RenderStates *states) {

    renderTexture->draw(*reinterpret_cast<const sf::RectangleShape *>(object), *states);
}
extern "C" void sfRenderTexture_drawVertexBuffer(sf::RenderTexture *renderTexture, const sfVertexBuffer *object, const sf::RenderStates *states) {

    renderTexture->draw(object->This, *states);
}

extern "C" void sfRenderTexture_drawPrimitives(sf::RenderTexture *renderTexture,
                                    const sfVertex *vertices, size_t vertexCount,
                                    sfPrimitiveType type, const sf::RenderStates *states) {
    renderTexture->draw(reinterpret_cast<const sf::Vertex *>(vertices), vertexCount,
                             static_cast<sf::PrimitiveType>(type), *states);
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

extern "C" const sfTexture *sfRenderTexture_getTexture(const sf::RenderTexture *renderTexture) {
    return reinterpret_cast<const sfTexture *>(&renderTexture->getTexture());
}

extern "C" void sfRenderTexture_setSmooth(sf::RenderTexture *renderTexture, sfBool smooth) {
    renderTexture->setSmooth(smooth == sfTrue);
}

extern "C" unsigned int sfRenderTexture_getMaximumAntialiasingLevel() {
    return sf::RenderTexture::getMaximumAntialiasingLevel();
}

extern "C" sfBool sfRenderTexture_isSmooth(const sf::RenderTexture *renderTexture) {
    return renderTexture->isSmooth();
}

extern "C" void sfRenderTexture_setRepeated(sf::RenderTexture *renderTexture, sfBool repeated) {
    renderTexture->setRepeated(repeated == sfTrue);
}

extern "C" sfBool sfRenderTexture_isRepeated(const sf::RenderTexture *renderTexture) {
    return renderTexture->isRepeated();
}

extern "C" sfBool sfRenderTexture_generateMipmap(sf::RenderTexture *renderTexture) {
    return renderTexture->generateMipmap();
}
