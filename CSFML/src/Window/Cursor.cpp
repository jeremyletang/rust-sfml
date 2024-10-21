#include "System/Vector2.hpp"
#include <SFML/Window/Cursor.hpp>
#include <cstdint>

extern "C" sf::Cursor *sfCursor_new() {
    return new sf::Cursor;
}

extern "C" void sfCursor_del(sf::Cursor *cursor) {
    delete cursor;
}

extern "C" bool sfCursor_loadFromPixels(sf::Cursor *cursor, const uint8_t *pixels, sfVector2u size, sfVector2u hotspot) {
    return cursor->loadFromPixels(pixels, sf::Vector2u(size.x, size.y), sf::Vector2u(hotspot.x, hotspot.y));
}

extern "C" bool sfCursor_loadFromSystem(sf::Cursor *cursor, sf::Cursor::Type type) {
    return cursor->loadFromSystem(type);
}
