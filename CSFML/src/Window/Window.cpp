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

#include "SFML/Window.hpp"
#include "System/Vector2.h"
#include "Window/VideoMode.h"
#include <SFML/Window/Context.hpp>
#include <SFML/Window/Touch.hpp>
#include <cstddef>
#include <cstdint>

extern "C" sf::Window *sfWindow_createUnicode(sfVideoMode mode, const uint32_t *title, uint32_t style, const sf::ContextSettings *settings) {
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);

    // Create the window
    sf::Window *window = new sf::Window;
    window->create(videoMode, title, style, *settings);

    return window;
}

extern "C" sf::Window *sfWindow_createFromHandle(sf::WindowHandle handle, const sf::ContextSettings *settings) {
    // Create the window
    sf::Window *window = new sf::Window;
    window->create(handle, *settings);

    return window;
}

extern "C" void sfWindow_destroy(sf::Window *window) {
    delete window;
}

extern "C" void sfWindow_close(sf::Window *window) {
    window->close();
}

extern "C" bool sfWindow_isOpen(const sf::Window *window) {
    return window->isOpen();
}

extern "C" const sf::ContextSettings *sfWindow_getSettings(const sf::Window *window) {
    return &window->getSettings();
}

extern "C" bool sfWindow_pollEvent(sf::Window *window, sf::Event *event) {
    return window->pollEvent(*event);
}

extern "C" bool sfWindow_waitEvent(sf::Window *window, sf::Event *event) {
    return window->waitEvent(*event);
}

extern "C" sfVector2i sfWindow_getPosition(const sf::Window *window) {
    sf::Vector2i pos = window->getPosition();
    return {pos.x, pos.y};
}

extern "C" void sfWindow_setPosition(sf::Window *window, sfVector2i position) {
    window->setPosition(sf::Vector2i(position.x, position.y));
}

extern "C" sfVector2u sfWindow_getSize(const sf::Window *window) {
    sf::Vector2u size = window->getSize();
    return {size.x, size.y};
}

extern "C" void sfWindow_setSize(sf::Window *window, sfVector2u size) {
    window->setSize(sf::Vector2u(size.x, size.y));
}

extern "C" void sfWindow_setUnicodeTitle(sf::Window *window, const uint32_t *title) {
    window->setTitle(title);
}

extern "C" void sfWindow_setIcon(sf::Window *window, unsigned int width, unsigned int height, const uint8_t *pixels) {
    window->setIcon(width, height, pixels);
}

extern "C" void sfWindow_setVisible(sf::Window *window, bool visible) {
    window->setVisible(visible);
}

extern "C" void sfWindow_setMouseCursorVisible(sf::Window *window, bool visible) {
    window->setMouseCursorVisible(visible);
}

extern "C" void sfWindow_setMouseCursorGrabbed(sf::Window *window, bool grabbed) {
    window->setMouseCursorGrabbed(grabbed);
}

extern "C" void sfWindow_setMouseCursor(sf::Window *window, const sf::Cursor *cursor) {

    window->setMouseCursor(*cursor);
}

extern "C" void sfWindow_setVerticalSyncEnabled(sf::Window *window, bool enabled) {
    window->setVerticalSyncEnabled(enabled);
}

extern "C" void sfWindow_setKeyRepeatEnabled(sf::Window *window, bool enabled) {
    window->setKeyRepeatEnabled(enabled);
}

extern "C" bool sfWindow_setActive(sf::Window *window, bool active) {
    return window->setActive(active);
}

extern "C" void sfWindow_requestFocus(sf::Window *window) {
    window->requestFocus();
}

extern "C" bool sfWindow_hasFocus(const sf::Window *window) {
    return window->hasFocus();
}

extern "C" void sfWindow_display(sf::Window *window) {
    window->display();
}

extern "C" void sfWindow_setFramerateLimit(sf::Window *window, unsigned int limit) {
    window->setFramerateLimit(limit);
}

extern "C" void sfWindow_setJoystickThreshold(sf::Window *window, float threshold) {
    window->setJoystickThreshold(threshold);
}

extern "C" sf::WindowHandle sfWindow_getSystemHandle(const sf::Window *window) {
    return window->getSystemHandle();
}

extern "C" sf::Context *sfContext_create(void) {
    return new sf::Context;
}

extern "C" void sfContext_destroy(sf::Context *context) {
    delete context;
}

extern "C" bool sfContext_setActive(sf::Context *context, bool active) {
    return context->setActive(active);
}

extern "C" const sf::ContextSettings *sfContext_getSettings(const sf::Context *context) {
    return &context->getSettings();
}

extern "C" uint64_t sfContext_getActiveContextId() {
    return sf::Context::getActiveContextId();
}
