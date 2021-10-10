
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

#include "Graphics/ConvexShapeStruct.h"
#include "Graphics/PrimitiveType.h"
#include "Graphics/ShapeStruct.h"
#include "Graphics/Vertex.h"
#include "Graphics/VertexBufferStruct.h"
#include "Window/CursorStruct.h"
#include "Window/VideoMode.h"
#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/Text.hpp>
#include <SFML/Window/Mouse.hpp>
#include <SFML/Window/Touch.hpp>
#include <cstddef>

extern "C" sfRenderWindow *sfRenderWindow_createUnicode(sfVideoMode mode, const sfUint32 *title, sfUint32 style, const sf::ContextSettings *settings) {
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Create the window
    sf::RenderWindow *renderWindow = new sf::RenderWindow;
    renderWindow->create(videoMode, title, style, *settings);

    return reinterpret_cast<sfRenderWindow *>(renderWindow);
}

extern "C" sfRenderWindow *sfRenderWindow_createFromHandle(sf::WindowHandle handle, const sf::ContextSettings *settings) {
    // Create the window
    sf::RenderWindow *renderWindow = new sf::RenderWindow;
    renderWindow->create(handle, *settings);

    return reinterpret_cast<sfRenderWindow *>(renderWindow);
}

extern "C" void sfRenderWindow_destroy(sfRenderWindow *renderWindow) {
    delete reinterpret_cast<sf::RenderWindow *>(renderWindow);
}

extern "C" void sfRenderWindow_close(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->close();
}

extern "C" sfBool sfRenderWindow_isOpen(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sf::RenderWindow *>(renderWindow)->isOpen();
}

extern "C" const sf::ContextSettings *sfRenderWindow_getSettings(const sfRenderWindow *renderWindow) {
    return &reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getSettings();
}

extern "C" sfBool sfRenderWindow_pollEvent(sfRenderWindow *renderWindow, sf::Event *event) {
    return reinterpret_cast<sf::RenderWindow *>(renderWindow)->pollEvent(*event);
}

extern "C" sfBool sfRenderWindow_waitEvent(sfRenderWindow *renderWindow, sf::Event *event) {
    return reinterpret_cast<sf::RenderWindow *>(renderWindow)->waitEvent(*event);
}

extern "C" sfVector2i sfRenderWindow_getPosition(const sfRenderWindow *renderWindow) {
    sfVector2i position = {0, 0};

    sf::Vector2i sfmlPos = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" void sfRenderWindow_setPosition(sfRenderWindow *renderWindow, sfVector2i position) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setPosition(sf::Vector2i(position.x, position.y));
}

extern "C" sfVector2u sfRenderWindow_getSize(const sfRenderWindow *renderWindow) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" void sfRenderWindow_setSize(sfRenderWindow *renderWindow, sfVector2u size) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setSize(sf::Vector2u(size.x, size.y));
}

extern "C" void sfRenderWindow_setUnicodeTitle(sfRenderWindow *renderWindow, const sfUint32 *title) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setTitle(title);
}

extern "C" void sfRenderWindow_setIcon(sfRenderWindow *renderWindow, unsigned int width, unsigned int height, const sfUint8 *pixels) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setIcon(width, height, pixels);
}

extern "C" void sfRenderWindow_setVisible(sfRenderWindow *renderWindow, sfBool visible) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setVisible(visible == sfTrue);
}

extern "C" void sfRenderWindow_setVerticalSyncEnabled(sfRenderWindow *renderWindow, sfBool enabled) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setVerticalSyncEnabled(enabled == sfTrue);
}

extern "C" void sfRenderWindow_setMouseCursorVisible(sfRenderWindow *renderWindow, sfBool visible) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setMouseCursorVisible(visible == sfTrue);
}

extern "C" void sfRenderWindow_setMouseCursorGrabbed(sfRenderWindow *renderWindow, sfBool grabbed) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setMouseCursorGrabbed(grabbed == sfTrue);
}

extern "C" void sfRenderWindow_setMouseCursor(sfRenderWindow *window, const sfCursor *cursor) {
    reinterpret_cast<sf::RenderWindow *>(window)->setMouseCursor(cursor->This);
}

extern "C" void sfRenderWindow_setKeyRepeatEnabled(sfRenderWindow *renderWindow, sfBool enabled) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setKeyRepeatEnabled(enabled == sfTrue);
}

extern "C" sfBool sfRenderWindow_setActive(sfRenderWindow *renderWindow, sfBool active) {
    return reinterpret_cast<sf::RenderWindow *>(renderWindow)->setActive(active == sfTrue);
}

extern "C" void sfRenderWindow_requestFocus(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->requestFocus();
}

extern "C" sfBool sfRenderWindow_hasFocus(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sf::RenderWindow *>(renderWindow)->hasFocus();
}

extern "C" void sfRenderWindow_display(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->display();
}

extern "C" void sfRenderWindow_setFramerateLimit(sfRenderWindow *renderWindow, unsigned int limit) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setFramerateLimit(limit);
}

extern "C" void sfRenderWindow_setJoystickThreshold(sfRenderWindow *renderWindow, float threshold) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setJoystickThreshold(threshold);
}

extern "C" sf::WindowHandle sfRenderWindow_getSystemHandle(const sfRenderWindow *renderWindow) {

    return reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getSystemHandle();
}

extern "C" void sfRenderWindow_clear(sfRenderWindow *renderWindow, sfColor color) {
    sf::Color SFMLColor(color.r, color.g, color.b, color.a);

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->clear(SFMLColor);
}

extern "C" void sfRenderWindow_setView(sfRenderWindow *renderWindow, const sfView *view) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setView(*reinterpret_cast<const sf::View *>(view));
}

extern "C" const sfView *sfRenderWindow_getView(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sfView *>(&reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getView());
}

extern "C" const sfView *sfRenderWindow_getDefaultView(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sfView *>(&reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getDefaultView());
}

extern "C" sfIntRect sfRenderWindow_getViewport(const sfRenderWindow *renderWindow, const sfView *view) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect SFMLrect = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getViewport(*reinterpret_cast<const sf::View *>(view));
    rect.left = SFMLrect.left;
    rect.top = SFMLrect.top;
    rect.width = SFMLrect.width;
    rect.height = SFMLrect.height;

    return rect;
}

extern "C" sfVector2f sfRenderWindow_mapPixelToCoords(const sfRenderWindow *renderWindow, sfVector2i point, const sfView *targetView) {
    sfVector2f result = {0, 0};

    sf::Vector2f sfmlPoint;
    if (targetView)
        sfmlPoint = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->mapPixelToCoords(sf::Vector2i(point.x, point.y), *reinterpret_cast<const sf::View *>(targetView));
    else
        sfmlPoint = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->mapPixelToCoords(sf::Vector2i(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

extern "C" sfVector2i sfRenderWindow_mapCoordsToPixel(const sfRenderWindow *renderWindow, sfVector2f point, const sfView *targetView) {
    sfVector2i result = {0, 0};

    sf::Vector2i sfmlPoint;
    if (targetView)
        sfmlPoint = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->mapCoordsToPixel(sf::Vector2f(point.x, point.y), *reinterpret_cast<const sf::View *>(targetView));
    else
        sfmlPoint = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->mapCoordsToPixel(sf::Vector2f(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

extern "C" void sfRenderWindow_drawSprite(sfRenderWindow *renderWindow, const sfSprite *object, const sf::RenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(*reinterpret_cast<const sf::Sprite *>(object), *states);
}
extern "C" void sfRenderWindow_drawText(sfRenderWindow *renderWindow, const sfText *object, const sf::RenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(*reinterpret_cast<const sf::Text *>(object), *states);
}
extern "C" void sfRenderWindow_drawShape(sfRenderWindow *renderWindow, const sfShape *object, const sf::RenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(object->This, *states);
}
extern "C" void sfRenderWindow_drawCircleShape(sfRenderWindow *renderWindow, const sf::CircleShape *object, const sf::RenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(*object, *states);
}
extern "C" void sfRenderWindow_drawConvexShape(sfRenderWindow *renderWindow, const sfConvexShape *object, const sf::RenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(object->This, *states);
}
extern "C" void sfRenderWindow_drawRectangleShape(sfRenderWindow *renderWindow, const sfRectangleShape *object, const sf::RenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(*reinterpret_cast<const sf::RectangleShape *>(object), *states);
}
extern "C" void sfRenderWindow_drawVertexBuffer(sfRenderWindow *renderWindow, const sfVertexBuffer *object, const sf::RenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(object->This, *states);
}

extern "C" void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
                                              const sfVertex *vertices, size_t vertexCount,
                                              sfPrimitiveType type, const sf::RenderStates *states) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(reinterpret_cast<const sf::Vertex *>(vertices), vertexCount, static_cast<sf::PrimitiveType>(type), *states);
}

extern "C" void sfRenderWindow_pushGLStates(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->pushGLStates();
}

extern "C" void sfRenderWindow_popGLStates(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->popGLStates();
}

extern "C" void sfRenderWindow_resetGLStates(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->resetGLStates();
}

extern "C" sfVector2i sfMouse_getPositionRenderWindow(const sfRenderWindow *relativeTo) {
    sf::Vector2i sfmlPos;
    if (relativeTo)
        sfmlPos = sf::Mouse::getPosition(*reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sfmlPos = sf::Mouse::getPosition();

    sfVector2i position = {sfmlPos.x, sfmlPos.y};
    return position;
}

extern "C" void sfMouse_setPositionRenderWindow(sfVector2i position, const sfRenderWindow *relativeTo) {
    if (relativeTo)
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y), *reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y));
}

extern "C" sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sfRenderWindow *relativeTo) {
    sf::Vector2i sfmlPosition;

    if (relativeTo)
        sfmlPosition = sf::Touch::getPosition(finger, *reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sfmlPosition = sf::Touch::getPosition(finger);

    sfVector2i position = {sfmlPosition.x, sfmlPosition.y};
    return position;
}
