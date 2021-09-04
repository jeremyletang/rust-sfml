
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

#include "ConvertEvent.h"
#include "Graphics/CircleShapeStruct.h"
#include "Graphics/ConvertRenderStates.hpp"
#include "Graphics/ConvexShapeStruct.h"
#include "Graphics/ImageStruct.h"
#include "Graphics/RectangleShapeStruct.h"
#include "Graphics/RenderWindow.h"
#include "Graphics/ShapeStruct.h"
#include "Graphics/VertexBufferStruct.h"
#include "Window/ContextSettingsInternal.h"
#include "Window/CursorStruct.h"
#include <SFML/Window/Touch.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Text.hpp>
#include <cstddef>

sfRenderWindow *sfRenderWindow_createUnicode(sfVideoMode mode, const sfUint32 *title, sfUint32 style, const sfContextSettings *settings) {
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Convert context settings
    sf::ContextSettings params;
    if (settings) {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sf::RenderWindow *renderWindow = new sf::RenderWindow;
    renderWindow->create(videoMode, title, style, params);

    return reinterpret_cast<sfRenderWindow*>(renderWindow);
}

sfRenderWindow *sfRenderWindow_createFromHandle(sfWindowHandle handle, const sfContextSettings *settings) {
    // Convert context settings
    sf::ContextSettings params;
    if (settings) {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sf::RenderWindow *renderWindow = new sf::RenderWindow;
    renderWindow->create(handle, params);

    return reinterpret_cast<sfRenderWindow *>(renderWindow);
}

void sfRenderWindow_destroy(sfRenderWindow *renderWindow) {
    delete reinterpret_cast<sf::RenderWindow*>(renderWindow);
}

void sfRenderWindow_close(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->close();
}

sfBool sfRenderWindow_isOpen(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sf::RenderWindow *>(renderWindow)->isOpen();
}

sfContextSettings sfRenderWindow_getSettings(const sfRenderWindow *renderWindow) {
    sfContextSettings settings = priv::sfContextSettings_null();

    const sf::ContextSettings &params = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getSettings();
    priv::sfContextSettings_readFromCpp(params, settings);

    return settings;
}

sfBool sfRenderWindow_pollEvent(sfRenderWindow *renderWindow, sfEvent *event) {

    // Get the event
    sf::Event SFMLEvent;
    sfBool ret = reinterpret_cast<sf::RenderWindow *>(renderWindow)->pollEvent(SFMLEvent);

    // No event, return
    if (!ret)
        return sfFalse;

    // Convert the sf::Event event to a sfEvent
    convertEvent(SFMLEvent, event);

    return sfTrue;
}

sfBool sfRenderWindow_waitEvent(sfRenderWindow *renderWindow, sfEvent *event) {

    // Get the event
    sf::Event SFMLEvent;
    sfBool ret = reinterpret_cast<sf::RenderWindow *>(renderWindow)->waitEvent(SFMLEvent);

    // Error, return
    if (!ret)
        return sfFalse;

    // Convert the sf::Event event to a sfEvent
    convertEvent(SFMLEvent, event);

    return sfTrue;
}

sfVector2i sfRenderWindow_getPosition(const sfRenderWindow *renderWindow) {
    sfVector2i position = {0, 0};

    sf::Vector2i sfmlPos = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

void sfRenderWindow_setPosition(sfRenderWindow *renderWindow, sfVector2i position) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setPosition(sf::Vector2i(position.x, position.y));
}

sfVector2u sfRenderWindow_getSize(const sfRenderWindow *renderWindow) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

void sfRenderWindow_setSize(sfRenderWindow *renderWindow, sfVector2u size) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setSize(sf::Vector2u(size.x, size.y));
}

void sfRenderWindow_setUnicodeTitle(sfRenderWindow *renderWindow, const sfUint32 *title) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setTitle(title);
}

void sfRenderWindow_setIcon(sfRenderWindow *renderWindow, unsigned int width, unsigned int height, const sfUint8 *pixels) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setIcon(width, height, pixels);
}

void sfRenderWindow_setVisible(sfRenderWindow *renderWindow, sfBool visible) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setVisible(visible == sfTrue);
}

void sfRenderWindow_setVerticalSyncEnabled(sfRenderWindow *renderWindow, sfBool enabled) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setVerticalSyncEnabled(enabled == sfTrue);
}

void sfRenderWindow_setMouseCursorVisible(sfRenderWindow *renderWindow, sfBool visible) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setMouseCursorVisible(visible == sfTrue);
}

void sfRenderWindow_setMouseCursorGrabbed(sfRenderWindow *renderWindow, sfBool grabbed) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setMouseCursorGrabbed(grabbed == sfTrue);
}

void sfRenderWindow_setMouseCursor(sfRenderWindow *window, const sfCursor *cursor) {
    reinterpret_cast<sf::RenderWindow *>(window)->setMouseCursor(cursor->This);
}

void sfRenderWindow_setKeyRepeatEnabled(sfRenderWindow *renderWindow, sfBool enabled) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setKeyRepeatEnabled(enabled == sfTrue);
}

sfBool sfRenderWindow_setActive(sfRenderWindow *renderWindow, sfBool active) {
    return reinterpret_cast<sf::RenderWindow *>(renderWindow)->setActive(active == sfTrue);
}

void sfRenderWindow_requestFocus(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->requestFocus();
}

sfBool sfRenderWindow_hasFocus(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sf::RenderWindow *>(renderWindow)->hasFocus();
}

void sfRenderWindow_display(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->display();
}

void sfRenderWindow_setFramerateLimit(sfRenderWindow *renderWindow, unsigned int limit) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setFramerateLimit(limit);
}

void sfRenderWindow_setJoystickThreshold(sfRenderWindow *renderWindow, float threshold) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setJoystickThreshold(threshold);
}

sfWindowHandle sfRenderWindow_getSystemHandle(const sfRenderWindow *renderWindow) {

    return (sfWindowHandle)reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getSystemHandle();
}

void sfRenderWindow_clear(sfRenderWindow *renderWindow, sfColor color) {
    sf::Color SFMLColor(color.r, color.g, color.b, color.a);

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->clear(SFMLColor);
}

void sfRenderWindow_setView(sfRenderWindow *renderWindow, const sfView *view) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->setView(*reinterpret_cast<const sf::View*>(view));
}

const sfView *sfRenderWindow_getView(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sfView *>(&reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getView());
}

const sfView *sfRenderWindow_getDefaultView(const sfRenderWindow *renderWindow) {
    return reinterpret_cast<const sfView *>(&reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getDefaultView());
}

sfIntRect sfRenderWindow_getViewport(const sfRenderWindow *renderWindow, const sfView *view) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect SFMLrect = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->getViewport(*reinterpret_cast<const sf::View*>(view));
    rect.left = SFMLrect.left;
    rect.top = SFMLrect.top;
    rect.width = SFMLrect.width;
    rect.height = SFMLrect.height;

    return rect;
}

sfVector2f sfRenderWindow_mapPixelToCoords(const sfRenderWindow *renderWindow, sfVector2i point, const sfView *targetView) {
    sfVector2f result = {0, 0};

    sf::Vector2f sfmlPoint;
    if (targetView)
        sfmlPoint = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->mapPixelToCoords(sf::Vector2i(point.x, point.y), *reinterpret_cast<const sf::View*>(targetView));
    else
        sfmlPoint = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->mapPixelToCoords(sf::Vector2i(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

sfVector2i sfRenderWindow_mapCoordsToPixel(const sfRenderWindow *renderWindow, sfVector2f point, const sfView *targetView) {
    sfVector2i result = {0, 0};

    sf::Vector2i sfmlPoint;
    if (targetView)
        sfmlPoint = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->mapCoordsToPixel(sf::Vector2f(point.x, point.y), *reinterpret_cast<const sf::View*>(targetView));
    else
        sfmlPoint = reinterpret_cast<const sf::RenderWindow *>(renderWindow)->mapCoordsToPixel(sf::Vector2f(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

void sfRenderWindow_drawSprite(sfRenderWindow *renderWindow, const sfSprite *object, const sfRenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(*reinterpret_cast<const sf::Sprite*>(object), convertRenderStates(states));
}
void sfRenderWindow_drawText(sfRenderWindow *renderWindow, const sfText *object, const sfRenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(*reinterpret_cast<const sf::Text*>(object), convertRenderStates(states));
}
void sfRenderWindow_drawShape(sfRenderWindow *renderWindow, const sfShape *object, const sfRenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawCircleShape(sfRenderWindow *renderWindow, const sfCircleShape *object, const sfRenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawConvexShape(sfRenderWindow *renderWindow, const sfConvexShape *object, const sfRenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawRectangleShape(sfRenderWindow *renderWindow, const sfRectangleShape *object, const sfRenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawVertexBuffer(sfRenderWindow *renderWindow, const sfVertexBuffer *object, const sfRenderStates *states) {

    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(object->This, convertRenderStates(states));
}

void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
                                   const sfVertex *vertices, size_t vertexCount,
                                   sfPrimitiveType type, const sfRenderStates *states) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->draw(reinterpret_cast<const sf::Vertex *>(vertices), vertexCount,
                            static_cast<sf::PrimitiveType>(type), convertRenderStates(states));
}

void sfRenderWindow_pushGLStates(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->pushGLStates();
}

void sfRenderWindow_popGLStates(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->popGLStates();
}

void sfRenderWindow_resetGLStates(sfRenderWindow *renderWindow) {
    reinterpret_cast<sf::RenderWindow *>(renderWindow)->resetGLStates();
}

sfVector2i sfMouse_getPositionRenderWindow(const sfRenderWindow *relativeTo) {
    sf::Vector2i sfmlPos;
    if (relativeTo)
        sfmlPos = sf::Mouse::getPosition(*reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sfmlPos = sf::Mouse::getPosition();

    sfVector2i position = {sfmlPos.x, sfmlPos.y};
    return position;
}

void sfMouse_setPositionRenderWindow(sfVector2i position, const sfRenderWindow *relativeTo) {
    if (relativeTo)
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y), *reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y));
}

sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sfRenderWindow *relativeTo) {
    sf::Vector2i sfmlPosition;

    if (relativeTo)
        sfmlPosition = sf::Touch::getPosition(finger, *reinterpret_cast<const sf::RenderWindow *>(relativeTo));
    else
        sfmlPosition = sf::Touch::getPosition(finger);

    sfVector2i position = {sfmlPosition.x, sfmlPosition.y};
    return position;
}
