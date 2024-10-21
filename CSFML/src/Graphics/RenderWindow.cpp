#include "Graphics/Rect.hpp"
#include "Graphics/Color.hpp"
#include "System/Vector2.hpp"
#include "Window/VideoMode.hpp"
#include <SFML/Graphics/RenderTarget.hpp>
#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/Text.hpp>
#include <SFML/Window/Mouse.hpp>
#include <SFML/Window/Touch.hpp>
#include <cstddef>

extern "C" void sfRenderWindow_createUnicode(sf::RenderWindow *renderWindow, sfVideoMode mode, const uint32_t *title, uint32_t style, const sf::ContextSettings *settings) {
    // Convert video mode
    sf::VideoMode videoMode(mode.width, mode.height, mode.bitsPerPixel);
    // Create the window
    renderWindow->create(videoMode, title, style, *settings);
}

extern "C" sf::RenderWindow *sfRenderWindow_createUnicode_new(sfVideoMode mode, const uint32_t *title, uint32_t style, const sf::ContextSettings *settings) {
    sf::RenderWindow *renderWindow = new sf::RenderWindow;
    sfRenderWindow_createUnicode(renderWindow, mode, title, style, settings);
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

extern "C" bool sfRenderWindow_isOpen(const sf::RenderWindow *renderWindow) {
    return renderWindow->isOpen();
}

extern "C" const sf::ContextSettings *sfRenderWindow_getSettings(const sf::RenderWindow *renderWindow) {
    return &renderWindow->getSettings();
}

extern "C" bool sfRenderWindow_pollEvent(sf::RenderWindow *renderWindow, sf::Event *event) {
    return renderWindow->pollEvent(*event);
}

extern "C" bool sfRenderWindow_waitEvent(sf::RenderWindow *renderWindow, sf::Event *event) {
    return renderWindow->waitEvent(*event);
}

extern "C" sfVector2i sfRenderWindow_getPosition(const sf::RenderWindow *renderWindow) {
    sf::Vector2i vec2 = renderWindow->getPosition();
    return {vec2.x, vec2.y};
}

extern "C" void sfRenderWindow_setPosition(sf::RenderWindow *renderWindow, sfVector2i position) {
    renderWindow->setPosition(sf::Vector2i(position.x, position.y));
}

extern "C" sfVector2u sfRenderWindow_getSize(const sf::RenderWindow *renderWindow) {
    sf::Vector2u vec2 = renderWindow->getSize();
    return {vec2.x, vec2.y};
}

extern "C" void sfRenderWindow_setSize(sf::RenderWindow *renderWindow, sfVector2u size) {
    renderWindow->setSize(sf::Vector2u(size.x, size.y));
}

extern "C" bool sfRenderWindow_isSrgb(const sf::RenderWindow *renderWindow) {
    return renderWindow->isSrgb();
}

extern "C" void sfRenderWindow_setUnicodeTitle(sf::RenderWindow *renderWindow, const uint32_t *title) {
    renderWindow->setTitle(title);
}

extern "C" void sfRenderWindow_setIcon(sf::RenderWindow *renderWindow, unsigned int width, unsigned int height, const uint8_t *pixels) {
    renderWindow->setIcon(width, height, pixels);
}

extern "C" void sfRenderWindow_setVisible(sf::RenderWindow *renderWindow, bool visible) {
    renderWindow->setVisible(visible);
}

extern "C" void sfRenderWindow_setVerticalSyncEnabled(sf::RenderWindow *renderWindow, bool enabled) {
    renderWindow->setVerticalSyncEnabled(enabled);
}

extern "C" void sfRenderWindow_setMouseCursorVisible(sf::RenderWindow *renderWindow, bool visible) {
    renderWindow->setMouseCursorVisible(visible);
}

extern "C" void sfRenderWindow_setMouseCursorGrabbed(sf::RenderWindow *renderWindow, bool grabbed) {
    renderWindow->setMouseCursorGrabbed(grabbed);
}

extern "C" void sfRenderWindow_setMouseCursor(sf::RenderWindow *window, const sf::Cursor *cursor) {
    window->setMouseCursor(*cursor);
}

extern "C" void sfRenderWindow_setKeyRepeatEnabled(sf::RenderWindow *renderWindow, bool enabled) {
    renderWindow->setKeyRepeatEnabled(enabled);
}

extern "C" bool sfRenderWindow_setActive(sf::RenderWindow *renderWindow, bool active) {
    return renderWindow->setActive(active);
}

extern "C" void sfRenderWindow_requestFocus(sf::RenderWindow *renderWindow) {
    renderWindow->requestFocus();
}

extern "C" bool sfRenderWindow_hasFocus(const sf::RenderWindow *renderWindow) {
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
    renderWindow->clear(sf::Color(color.r, color.g, color.b, color.a));
}

extern "C" void sfRenderWindow_setView(sf::RenderWindow *renderWindow, const sf::View *view) {
    renderWindow->setView(*view);
}

extern "C" const sf::View *sfRenderWindow_getView(const sf::RenderWindow *renderWindow) {
    return &renderWindow->getView();
}

extern "C" const sf::View *sfRenderWindow_getDefaultView(const sf::RenderWindow *renderWindow) {
    return &renderWindow->getDefaultView();
}

extern "C" sfIntRect sfRenderWindow_getViewport(const sf::RenderWindow *renderWindow, const sf::View *view) {
    sf::IntRect rect = renderWindow->getViewport(*view);
    return {rect.left, rect.top, rect.width, rect.height};
}

extern "C" sfVector2f sfRenderWindow_mapPixelToCoords(const sf::RenderWindow *renderWindow, sfVector2i point) {
    sf::Vector2f vec2 = renderWindow->mapPixelToCoords(sf::Vector2i(point.x, point.y));
    return {vec2.x, vec2.y};
}

extern "C" sfVector2f sfRenderWindow_mapPixelToCoords_View(const sf::RenderWindow *renderWindow, sfVector2i point, const sf::View *targetView) {
    sf::Vector2f vec2 = renderWindow->mapPixelToCoords(sf::Vector2i(point.x, point.y), *targetView);
    return {vec2.x, vec2.y};
}

extern "C" sfVector2i sfRenderWindow_mapCoordsToPixel(const sf::RenderWindow *renderWindow, sfVector2f point) {
    sf::Vector2i vec2 = renderWindow->mapCoordsToPixel(sf::Vector2f(point.x, point.y));
    return {vec2.x, vec2.y};
}

extern "C" sfVector2i sfRenderWindow_mapCoordsToPixel_View(const sf::RenderWindow *renderWindow, sfVector2f point, const sf::View *targetView) {
    sf::Vector2i vec2 = renderWindow->mapCoordsToPixel(sf::Vector2f(point.x, point.y), *targetView);
    return {vec2.x, vec2.y};
}

extern "C" void sfRenderWindow_drawSprite(sf::RenderWindow *renderWindow, const sf::Sprite *object, const sf::RenderStates *states) {
    renderWindow->draw(*object, *states);
}
extern "C" void sfRenderWindow_drawText(sf::RenderWindow *renderWindow, const sf::Text *object, const sf::RenderStates *states) {
    renderWindow->draw(*object, *states);
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
    renderWindow->draw(*object, *states);
}
extern "C" void sfRenderWindow_drawVertexBuffer(sf::RenderWindow *renderWindow, const sf::VertexBuffer *object, const sf::RenderStates *states) {
    renderWindow->draw(*object, *states);
}

extern "C" void sfRenderWindow_drawPrimitives(sf::RenderWindow *renderWindow,
                                              const sf::Vertex *vertices, size_t vertexCount,
                                              sf::PrimitiveType type, const sf::RenderStates *states) {
    renderWindow->draw(vertices, vertexCount, type, *states);
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

extern "C" void sfMouse_setPositionRenderWindow(sfVector2i pos, const sf::RenderWindow *relativeTo) {
    sf::Mouse::setPosition(sf::Vector2i(pos.x, pos.y), *relativeTo);
}

extern "C" sfVector2i sfMouse_getPositionRenderWindow(const sf::RenderWindow *relativeTo) {
    sf::Vector2i vec2 = sf::Mouse::getPosition(*relativeTo);
    return {vec2.x, vec2.y};
}

extern "C" sfVector2i sfTouch_getPositionRenderWindow(unsigned int finger, const sf::RenderWindow *relativeTo) {
    sf::Vector2i pos = sf::Touch::getPosition(finger, *relativeTo);
    return {pos.x, pos.y};
}
