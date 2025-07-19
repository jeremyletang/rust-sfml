#include "Window/Cursor.hpp"
#include "System/Vector2.hpp"
#include <SFML/Window/Cursor.hpp>

extern "C" sf::Cursor *sfCursor_createFromPixels(const uint8_t *pixels, sfVector2u size, sfVector2u hotspot) {
    auto cursor = sf::Cursor::createFromPixels(pixels, {size.x, size.y}, {hotspot.x, hotspot.y});
    if (!cursor)
        return nullptr;

    return new sf::Cursor{std::move(*cursor)};
}

extern "C" sf::Cursor *sfCursor_createFromSystem(sfCursorType type) {
    auto cursor = sf::Cursor::createFromSystem(static_cast<sf::Cursor::Type>(type));
    if (!cursor)
        return nullptr;

    return new sf::Cursor{std::move(*cursor)};
}

extern "C" void sfCursor_del(sf::Cursor *cursor) {
    delete cursor;
}
