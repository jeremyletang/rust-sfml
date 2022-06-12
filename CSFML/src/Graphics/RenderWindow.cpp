
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

#include "Graphics/PrimitiveType.h"
#include "Graphics/Rect.h"
#include "Graphics/ShapeStruct.h"
#include "Graphics/Vertex.h"
#include "Graphics/VertexBufferStruct.h"
#include "Window/VideoMode.h"
#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/Text.hpp>
#include <SFML/Window/Mouse.hpp>
#include <SFML/Window/Touch.hpp>
#include <cstddef>

extern "C" sf::RenderWindow *sfRenderWindow_createUnicode(sfVideoMode mode, const sfUint32 *title, sfUint32 style, const sf::ContextSettings *settings) {
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Create the window
    sf::RenderWindow *renderWindow = new sf::RenderWindow;
    renderWindow->create(videoMode, title, style, *settings);

    return renderWindow;
}

extern "C" sf::RenderWindow *sfRenderWindow_createFromHandle(sf::WindowHandle handle, const sf::ContextSettings *settings) {
    // Create the window
    sf::RenderWindow *renderWindow = new sf::RenderWindow;
    renderWindow->create(handle, *settings);

    return renderWindow;
}

extern "C" void sfRenderWindow_destroy(sf::RenderWindow *renderWindow) {
    delete renderWindow;
}

extern "C" void sfRenderWindow_close(sf::RenderWindow *renderWindow) {
    renderWindow->close();
}

extern "C" sfBool sfRenderWindow_isOpen(const sf::RenderWindow *renderWindow) {
    return renderWindow->isOpen();
}

extern "C" const sf::ContextSettings *sfRenderWindow_getSettings(const sf::RenderWindow *renderWindow) {
    return &renderWindow->getSettings();
}

extern "C" sfBool sfRenderWindow_pollEvent(sf::RenderWindow *renderWindow, sf::Event *event) {
    return renderWindow->pollEvent(*event);
}

extern "C" sfBool sfRenderWindow_waitEvent(sf::RenderWindow *renderWindow, sf::Event *event) {
    return renderWindow->waitEvent(*event);
}

extern "C" sfVector2i sfRenderWindow_getPosition(const sf::RenderWindow *renderWindow) {
    sfVector2i position = {0, 0};

    sf::Vector2i sfmlPos = renderWindow->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" void sfRenderWindow_setPosition(sf::RenderWindow *renderWindow, sfVector2i position) {
    renderWindow->setPosition(sf::Vector2i(position.x, position.y));
}

extern "C" sfVector2u sfRenderWindow_getSize(const sf::RenderWindow *renderWindow) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = renderWindow->getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" void sfRenderWindow_setSize(sf::RenderWindow *renderWindow, sfVector2u size) {
    renderWindow->setSize(sf::Vector2u(size.x, size.y));
}

extern "C" void sfRenderWindow_setUnicodeTitle(sf::RenderWindow *renderWindow, const sfUint32 *title) {
    renderWindow->setTitle(title);
}

extern "C" void sfRenderWindow_setIcon(sf::RenderWindow *renderWindow, unsigned int width, unsigned int height, const sfUint8 *pixels) {
    renderWindow->setIcon(width, height, pixels);
}

extern "C" void sfRenderWindow_setVisible(sf::RenderWindow *renderWindow, sfBool visible) {
    renderWindow->setVisible(visible == sfTrue);
}

extern "C" void sfRenderWindow_setVerticalSyncEnabled(sf::RenderWindow *renderWindow, sfBool enabled) {
    renderWindow->setVerticalSyncEnabled(enabled == sfTrue);
}

extern "C" void sfRenderWindow_setMouseCursorVisible(sf::RenderWindow *renderWindow, sfBool visible) {
    renderWindow->setMouseCursorVisible(visible == sfTrue);
}

extern "C" void sfRenderWindow_setMouseCursorGrabbed(sf::RenderWindow *renderWindow, sfBool grabbed) {
    renderWindow->setMouseCursorGrabbed(grabbed == sfTrue);
}

extern "C" void sfRenderWindow_setMouseCursor(sf::RenderWindow *window, const sf::Cursor *cursor) {
    reinterpret_cast<sf::RenderWindow *>(window)->setMouseCursor(*cursor);
}

extern "C" void sfRenderWindow_setKeyRepeatEnabled(sf::RenderWindow *renderWindow, sfBool enabled) {
    renderWindow->setKeyRepeatEnabled(enabled == sfTrue);
}

extern "C" sfBool sfRenderWindow_setActive(sf::RenderWindow *renderWindow, sfBool active) {
    return renderWindow->setActive(active == sfTrue);
}

extern "C" void sfRenderWindow_requestFocus(sf::RenderWindow *renderWindow) {
    renderWindow->requestFocus();
}

extern "C" sfBool sfRenderWindow_hasFocus(const sf::RenderWindow *renderWindow) {
    return renderWindow->hasFocus();
}

extern "C" void sfRenderWindow_display(sf::RenderWindow *renderWindow) {
    renderWindow->display();
}

extern "C" void sfRenderWindow_setFramerateLimit(sf::RenderWindow *renderWindow, unsigned int limit) {
    renderWindow->setFramerateLimit(limit);
}

extern "C" void sfRenderWindow_setJoystickThreshold(sf::RenderWindow *renderWindow, float threshold) {
    renderWindow->setJoystickThreshold(threshold);
}

extern "C" sf::WindowHandle sfRenderWindow_getSystemHandle(const sf::RenderWindow *renderWindow) {

    return renderWindow->getSystemHandle();
}

extern "C" void sfRenderWindow_clear(sf::RenderWindow *renderWindow, sfColor color) {
    sf::Color SFMLColor(color.r, color.g, color.b, color.a);

    renderWindow->clear(SFMLColor);
}

extern "C" void sfRenderWindow_setView(sf::RenderWindow *renderWindow, const sf::View *view) {
    renderWindow->setView(*reinterpret_cast<const sf::View *>(view));
}

extern "C" const sf::View *sfRenderWindow_getView(const sf::RenderWindow *renderWindow) {
    return reinterpret_cast<const sf::View *>(&renderWindow->getView());
}

extern "C" const sf::View *sfRenderWindow_getDefaultView(const sf::RenderWindow *renderWindow) {
    return reinterpret_cast<const sf::View *>(&renderWindow->getDefaultView());
}

extern "C" sfIntRect sfRenderWindow_getViewport(const sf::RenderWindow *renderWindow, const sf::View *view) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect SFMLrect = renderWindow->getViewport(*reinterpret_cast<const sf::View *>(view));
    rect.left = SFMLrect.left;
    rect.top = SFMLrect.top;
    rect.width = SFMLrect.width;
    rect.height = SFMLrect.height;

    return rect;
}

extern "C" sfVector2f sfRenderWindow_mapPixelToCoords(const sf::RenderWindow *renderWindow, sfVector2i point, const sf::View *targetView) {
    sfVector2f result = {0, 0};

    sf::Vector2f sfmlPoint;
    if (targetView)
        sfmlPoint = renderWindow->mapPixelToCoords(sf::Vector2i(point.x, point.y), *reinterpret_cast<const sf::View *>(targetView));
    else
        sfmlPoint = renderWindow->mapPixelToCoords(sf::Vector2i(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

extern "C" sfVector2i sfRenderWindow_mapCoordsToPixel(const sf::RenderWindow *renderWindow, sfVector2f point, const sf::View *targetView) {
    sfVector2i result = {0, 0};

    sf::Vector2i sfmlPoint;
    if (targetView)
        sfmlPoint = renderWindow->mapCoordsToPixel(sf::Vector2f(point.x, point.y), *reinterpret_cast<const sf::View *>(targetView));
    else
        sfmlPoint = renderWindow->mapCoordsToPixel(sf::Vector2f(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

extern "C" void sfRenderWindow_drawSprite(sf::RenderWindow *renderWindow, const sf::Sprite *object, const sf::RenderStates *states) {

    renderWindow->draw(*reinterpret_cast<const sf::Sprite *>(object), *states);
}
extern "C" void sfRenderWindow_drawText(sf::RenderWindow *renderWindow, const sf::Text *object, const sf::RenderStates *states) {

    renderWindow->draw(*reinterpret_cast<const sf::Text *>(object), *states);
}
extern "C" void sfRenderWindow_drawShape(sf::RenderWindow *renderWindow, const sf::Shape *object, const sf::RenderStates *states) {

    renderWindow->draw(*object, *states);
}
extern "C" void sfRenderWindow_drawCircleShape(sf::RenderWindow *renderWindow, const sf::CircleShape *object, const sf::RenderStates *states) {

    renderWindow->draw(*object, *states);
}
extern "C" void sfRenderWindow_drawConvexShape(sf::RenderWindow *renderWindow, const sf::ConvexShape *object, const sf::RenderStates *states) {

    renderWindow->draw(*object, *states);
}
extern "C" void sfRenderWindow_drawRectangleShape(sf::RenderWindow *renderWindow, const sf::RectangleShape *object, const sf::RenderStates *states) {

    renderWindow->draw(*reinterpret_cast<const sf::RectangleShape *>(object), *states);
}
extern "C" void sfRenderWindow_drawVertexBuffer(sf::RenderWindow *renderWindow, const sf::VertexBuffer *object, const sf::RenderStates *states) {

    renderWindow->draw(*object, *states);
}

extern "C" void sfRenderWindow_drawPrimitives(sf::RenderWindow *renderWindow,
                                              const sfVertex *vertices, size_t vertexCount,
                                              sfPrimitiveType type, const sf::RenderStates *states) {
    renderWindow->draw(reinterpret_cast<const sf::Vertex *>(vertices), vertexCount, static_cast<sf::PrimitiveType>(type), *states);
}

extern "C" void sfRenderWindow_pushGLStates(sf::RenderWindow *renderWindow) {
    renderWindow->pushGLStates();
}

extern "C" void sfRenderWindow_popGLStates(sf::RenderWindow *renderWindow) {
    renderWindow->popGLStates();
}

extern "C" void sfRenderWindow_resetGLStates(sf::RenderWindow *renderWindow) {
    renderWindow->resetGLStates();
}

extern "C" sfVector2i sfMouse_getPositionRenderWindow(const sf::RenderWindow *relativeTo) {
    sf::Vector2i sfmlPos;
    if (relativeTo)
        sfmlPos = sf::Mouse::getPosition(*reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sfmlPos = sf::Mouse::getPosition();

    sfVector2i position = {sfmlPos.x, sfmlPos.y};
    return position;
}

extern "C" void sfMouse_setPositionRenderWindow(sfVector2i position, const sf::RenderWindow *relativeTo) {
    if (relativeTo)
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y), *reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y));
}

extern "C" sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sf::RenderWindow *relativeTo) {
    sf::Vector2i sfmlPosition;

    if (relativeTo)
        sfmlPosition = sf::Touch::getPosition(finger, *reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sfmlPosition = sf::Touch::getPosition(finger);

    sfVector2i position = {sfmlPosition.x, sfmlPosition.y};
    return position;
}
