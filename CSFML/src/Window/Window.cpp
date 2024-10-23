#include "System/Vector2.hpp"
#include "Window/VideoMode.hpp"
#include <SFML/Window/Window.hpp>
#include <SFML/Window/Touch.hpp>
#include <cstdint>

extern "C" sf::Window *sfWindow_new() {
    return new sf::Window;
}

extern "C" void sfWindow_del(sf::Window *window) {
    delete window;
}

// Create with (mode, title, style, settings)
extern "C" void sfWindow_create_mtss(sf::Window *window, sfVideoMode mode, const uint32_t *title, uint32_t style, const sf::ContextSettings *settings) {
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);
    window->create(videoMode, title, style, *settings);
}

extern "C" void sfWindow_create_handle_settings(sf::Window *window, sf::WindowHandle handle, const sf::ContextSettings *settings) {
    window->create(handle, *settings);
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
