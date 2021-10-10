
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

#include "Config.h"
#include "System/Vector2.h"
#include "Window/CursorStruct.h"
#include "Window/VideoMode.h"
#include "Window/WindowStruct.h"
#include <SFML/Window/Context.hpp>
#include <cstddef>

extern "C" sfWindow *sfWindow_createUnicode(sfVideoMode mode, const sfUint32 *title, sfUint32 style, const sf::ContextSettings *settings) {
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Create the window
    sfWindow *window = new sfWindow;
    window->This.create(videoMode, title, style, *settings);

    return window;
}

extern "C" sfWindow *sfWindow_createFromHandle(sf::WindowHandle handle, const sf::ContextSettings *settings) {
    // Create the window
    sfWindow *window = new sfWindow;
    window->This.create(handle, *settings);

    return window;
}

extern "C" void sfWindow_destroy(sfWindow *window) {
    delete window;
}

extern "C" void sfWindow_close(sfWindow *window) {
    window->This.close();
}

extern "C" sfBool sfWindow_isOpen(const sfWindow *window) {
    return window->This.isOpen();
}

extern "C" const sf::ContextSettings *sfWindow_getSettings(const sfWindow *window) {
    return &window->This.getSettings();
}

extern "C" sfBool sfWindow_pollEvent(sfWindow *window, sf::Event *event) {
    return window->This.pollEvent(*event);
}

extern "C" sfBool sfWindow_waitEvent(sfWindow *window, sf::Event *event) {
    return window->This.waitEvent(*event);
}

extern "C" sfVector2i sfWindow_getPosition(const sfWindow *window) {
    sfVector2i position = {0, 0};

    sf::Vector2i sfmlPos = window->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;

    return position;
}

extern "C" void sfWindow_setPosition(sfWindow *window, sfVector2i position) {
    window->This.setPosition(sf::Vector2i(position.x, position.y));
}

extern "C" sfVector2u sfWindow_getSize(const sfWindow *window) {
    sfVector2u size = {0, 0};

    sf::Vector2u sfmlSize = window->This.getSize();
    size.x = sfmlSize.x;
    size.y = sfmlSize.y;

    return size;
}

extern "C" void sfWindow_setSize(sfWindow *window, sfVector2u size) {
    window->This.setSize(sf::Vector2u(size.x, size.y));
}

extern "C" void sfWindow_setUnicodeTitle(sfWindow *window, const sfUint32 *title) {
    window->This.setTitle(title);
}

extern "C" void sfWindow_setIcon(sfWindow *window, unsigned int width, unsigned int height, const sfUint8 *pixels) {
    window->This.setIcon(width, height, pixels);
}

extern "C" void sfWindow_setVisible(sfWindow *window, sfBool visible) {
    window->This.setVisible(visible == sfTrue);
}

extern "C" void sfWindow_setMouseCursorVisible(sfWindow *window, sfBool visible) {
    window->This.setMouseCursorVisible(visible == sfTrue);
}

extern "C" void sfWindow_setMouseCursorGrabbed(sfWindow *window, sfBool grabbed) {
    window->This.setMouseCursorGrabbed(grabbed == sfTrue);
}

extern "C" void sfWindow_setMouseCursor(sfWindow *window, const sfCursor *cursor) {

    window->This.setMouseCursor(cursor->This);
}

extern "C" void sfWindow_setVerticalSyncEnabled(sfWindow *window, sfBool enabled) {
    window->This.setVerticalSyncEnabled(enabled == sfTrue);
}

extern "C" void sfWindow_setKeyRepeatEnabled(sfWindow *window, sfBool enabled) {
    window->This.setKeyRepeatEnabled(enabled == sfTrue);
}

extern "C" sfBool sfWindow_setActive(sfWindow *window, sfBool active) {
    return window->This.setActive(active == sfTrue);
}

extern "C" void sfWindow_requestFocus(sfWindow *window) {
    window->This.requestFocus();
}

extern "C" sfBool sfWindow_hasFocus(const sfWindow *window) {
    return window->This.hasFocus();
}

extern "C" void sfWindow_display(sfWindow *window) {
    window->This.display();
}

extern "C" void sfWindow_setFramerateLimit(sfWindow *window, unsigned int limit) {
    window->This.setFramerateLimit(limit);
}

extern "C" void sfWindow_setJoystickThreshold(sfWindow *window, float threshold) {
    window->This.setJoystickThreshold(threshold);
}

extern "C" sf::WindowHandle sfWindow_getSystemHandle(const sfWindow *window) {
    return window->This.getSystemHandle();
}

extern "C" sf::Context *sfContext_create(void) {
    return new sf::Context;
}

extern "C" void sfContext_destroy(sf::Context *context) {
    delete context;
}

extern "C" sfBool sfContext_setActive(sf::Context *context, sfBool active) {
    return context->setActive(active == sfTrue);
}

extern "C" const sf::ContextSettings *sfContext_getSettings(const sf::Context *context) {
    return &context->getSettings();
}

extern "C" sfUint64 sfContext_getActiveContextId() {
    return sf::Context::getActiveContextId();
}
