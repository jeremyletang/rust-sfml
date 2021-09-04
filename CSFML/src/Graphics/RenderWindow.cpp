
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
#include "Graphics/RenderWindowStruct.h"
#include "Graphics/ShapeStruct.h"
#include "Graphics/SpriteStruct.h"
#include "Graphics/TextStruct.h"
#include "Graphics/VertexBufferStruct.h"
#include "Window/ContextSettingsInternal.h"
#include "Window/CursorStruct.h"
#include <SFML/Window/Touch.hpp>
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
    sfRenderWindow *renderWindow = new sfRenderWindow;
    renderWindow->This.create(videoMode, title, style, params);
    renderWindow->DefaultView.This = renderWindow->This.getDefaultView();
    renderWindow->CurrentView.This = renderWindow->This.getView();

    return renderWindow;
}

sfRenderWindow *sfRenderWindow_createFromHandle(sfWindowHandle handle, const sfContextSettings *settings) {
    // Convert context settings
    sf::ContextSettings params;
    if (settings) {
        priv::sfContextSettings_writeToCpp(*settings, params);
    }

    // Create the window
    sfRenderWindow *renderWindow = new sfRenderWindow;
    renderWindow->This.create(handle, params);
    renderWindow->DefaultView.This = renderWindow->This.getDefaultView();
    renderWindow->CurrentView.This = renderWindow->This.getView();

    return renderWindow;
}

void sfRenderWindow_destroy(sfRenderWindow *renderWindow) {
    delete renderWindow;
}

void sfRenderWindow_close(sfRenderWindow *renderWindow) {
    renderWindow->This.close();
}

sfBool sfRenderWindow_isOpen(const sfRenderWindow *renderWindow) {
    return renderWindow->This.isOpen();
}

sfContextSettings sfRenderWindow_getSettings(const sfRenderWindow *renderWindow) {
    sfContextSettings settings = priv::sfContextSettings_null();

    const sf::ContextSettings &params = renderWindow->This.getSettings();
    priv::sfContextSettings_readFromCpp(params, settings);

    return settings;
}

sfBool sfRenderWindow_pollEvent(sfRenderWindow *renderWindow, sfEvent *event) {

    // Get the event
    sf::Event SFMLEvent;
    sfBool ret = renderWindow->This.pollEvent(SFMLEvent);

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
    sfBool ret = renderWindow->This.waitEvent(SFMLEvent);

    // Error, return
    if (!ret)
        return sfFalse;

    // Convert the sf::Event event to a sfEvent
    convertEvent(SFMLEvent, event);

    return sfTrue;
}

sfVector2i sfRenderWindow_getPosition(const sfRenderWindow *renderWindow) {
    sfVector2i position = {0, 0};

    sf::Vector2i sfmlPos = renderWindow->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

void sfRenderWindow_setPosition(sfRenderWindow *renderWindow, sfVector2i position) {
    renderWindow->This.setPosition(sf::Vector2i(position.x, position.y));
}

sfVector2u sfRenderWindow_getSize(const sfRenderWindow *renderWindow) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = renderWindow->This.getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

void sfRenderWindow_setSize(sfRenderWindow *renderWindow, sfVector2u size) {
    renderWindow->This.setSize(sf::Vector2u(size.x, size.y));
}

void sfRenderWindow_setUnicodeTitle(sfRenderWindow *renderWindow, const sfUint32 *title) {
    renderWindow->This.setTitle(title);
}

void sfRenderWindow_setIcon(sfRenderWindow *renderWindow, unsigned int width, unsigned int height, const sfUint8 *pixels) {
    renderWindow->This.setIcon(width, height, pixels);
}

void sfRenderWindow_setVisible(sfRenderWindow *renderWindow, sfBool visible) {
    renderWindow->This.setVisible(visible == sfTrue);
}

void sfRenderWindow_setVerticalSyncEnabled(sfRenderWindow *renderWindow, sfBool enabled) {
    renderWindow->This.setVerticalSyncEnabled(enabled == sfTrue);
}

void sfRenderWindow_setMouseCursorVisible(sfRenderWindow *renderWindow, sfBool visible) {
    renderWindow->This.setMouseCursorVisible(visible == sfTrue);
}

void sfRenderWindow_setMouseCursorGrabbed(sfRenderWindow *renderWindow, sfBool grabbed) {
    renderWindow->This.setMouseCursorGrabbed(grabbed == sfTrue);
}

void sfRenderWindow_setMouseCursor(sfRenderWindow *window, const sfCursor *cursor) {

    window->This.setMouseCursor(cursor->This);
}

void sfRenderWindow_setKeyRepeatEnabled(sfRenderWindow *renderWindow, sfBool enabled) {
    renderWindow->This.setKeyRepeatEnabled(enabled == sfTrue);
}

sfBool sfRenderWindow_setActive(sfRenderWindow *renderWindow, sfBool active) {
    return renderWindow->This.setActive(active == sfTrue);
}

void sfRenderWindow_requestFocus(sfRenderWindow *renderWindow) {
    renderWindow->This.requestFocus();
}

sfBool sfRenderWindow_hasFocus(const sfRenderWindow *renderWindow) {
    return renderWindow->This.hasFocus();
}

void sfRenderWindow_display(sfRenderWindow *renderWindow) {
    renderWindow->This.display();
}

void sfRenderWindow_setFramerateLimit(sfRenderWindow *renderWindow, unsigned int limit) {
    renderWindow->This.setFramerateLimit(limit);
}

void sfRenderWindow_setJoystickThreshold(sfRenderWindow *renderWindow, float threshold) {
    renderWindow->This.setJoystickThreshold(threshold);
}

sfWindowHandle sfRenderWindow_getSystemHandle(const sfRenderWindow *renderWindow) {

    return (sfWindowHandle)renderWindow->This.getSystemHandle();
}

void sfRenderWindow_clear(sfRenderWindow *renderWindow, sfColor color) {
    sf::Color SFMLColor(color.r, color.g, color.b, color.a);

    renderWindow->This.clear(SFMLColor);
}

void sfRenderWindow_setView(sfRenderWindow *renderWindow, const sfView *view) {

    renderWindow->This.setView(view->This);
    renderWindow->CurrentView.This = view->This;
}

const sfView *sfRenderWindow_getView(const sfRenderWindow *renderWindow) {

    return &renderWindow->CurrentView;
}

const sfView *sfRenderWindow_getDefaultView(const sfRenderWindow *renderWindow) {

    return &renderWindow->DefaultView;
}

sfIntRect sfRenderWindow_getViewport(const sfRenderWindow *renderWindow, const sfView *view) {
    sfIntRect rect = {0, 0, 0, 0};

    sf::IntRect SFMLrect = renderWindow->This.getViewport(view->This);
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
        sfmlPoint = renderWindow->This.mapPixelToCoords(sf::Vector2i(point.x, point.y), targetView->This);
    else
        sfmlPoint = renderWindow->This.mapPixelToCoords(sf::Vector2i(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

sfVector2i sfRenderWindow_mapCoordsToPixel(const sfRenderWindow *renderWindow, sfVector2f point, const sfView *targetView) {
    sfVector2i result = {0, 0};

    sf::Vector2i sfmlPoint;
    if (targetView)
        sfmlPoint = renderWindow->This.mapCoordsToPixel(sf::Vector2f(point.x, point.y), targetView->This);
    else
        sfmlPoint = renderWindow->This.mapCoordsToPixel(sf::Vector2f(point.x, point.y));

    result.x = sfmlPoint.x;
    result.y = sfmlPoint.y;

    return result;
}

void sfRenderWindow_drawSprite(sfRenderWindow *renderWindow, const sfSprite *object, const sfRenderStates *states) {

    renderWindow->This.draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawText(sfRenderWindow *renderWindow, const sfText *object, const sfRenderStates *states) {

    renderWindow->This.draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawShape(sfRenderWindow *renderWindow, const sfShape *object, const sfRenderStates *states) {

    renderWindow->This.draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawCircleShape(sfRenderWindow *renderWindow, const sfCircleShape *object, const sfRenderStates *states) {

    renderWindow->This.draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawConvexShape(sfRenderWindow *renderWindow, const sfConvexShape *object, const sfRenderStates *states) {

    renderWindow->This.draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawRectangleShape(sfRenderWindow *renderWindow, const sfRectangleShape *object, const sfRenderStates *states) {

    renderWindow->This.draw(object->This, convertRenderStates(states));
}
void sfRenderWindow_drawVertexBuffer(sfRenderWindow *renderWindow, const sfVertexBuffer *object, const sfRenderStates *states) {

    renderWindow->This.draw(object->This, convertRenderStates(states));
}

void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
                                   const sfVertex *vertices, size_t vertexCount,
                                   sfPrimitiveType type, const sfRenderStates *states) {
    renderWindow->This.draw(reinterpret_cast<const sf::Vertex *>(vertices), vertexCount,
                            static_cast<sf::PrimitiveType>(type), convertRenderStates(states));
}

void sfRenderWindow_pushGLStates(sfRenderWindow *renderWindow) {
    renderWindow->This.pushGLStates();
}

void sfRenderWindow_popGLStates(sfRenderWindow *renderWindow) {
    renderWindow->This.popGLStates();
}

void sfRenderWindow_resetGLStates(sfRenderWindow *renderWindow) {
    renderWindow->This.resetGLStates();
}

sfVector2i sfMouse_getPositionRenderWindow(const sfRenderWindow *relativeTo) {
    sf::Vector2i sfmlPos;
    if (relativeTo)
        sfmlPos = sf::Mouse::getPosition(relativeTo->This);
    else
        sfmlPos = sf::Mouse::getPosition();

    sfVector2i position = {sfmlPos.x, sfmlPos.y};
    return position;
}

void sfMouse_setPositionRenderWindow(sfVector2i position, const sfRenderWindow *relativeTo) {
    if (relativeTo)
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y), relativeTo->This);
    else
        sf::Mouse::setPosition(sf::Vector2i(position.x, position.y));
}

sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sfRenderWindow *relativeTo) {
    sf::Vector2i sfmlPosition;

    if (relativeTo)
        sfmlPosition = sf::Touch::getPosition(finger, relativeTo->This);
    else
        sfmlPosition = sf::Touch::getPosition(finger);

    sfVector2i position = {sfmlPosition.x, sfmlPosition.y};
    return position;
}
