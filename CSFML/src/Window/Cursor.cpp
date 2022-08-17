#include "System/Vector2.h"
#include <SFML/Window/Cursor.hpp>
#include <cstddef>
#include <cstdint>

extern "C" sf::Cursor *sfCursor_createFromPixels(const uint8_t *pixels, sfVector2u size, sfVector2u hotspot) {
    sf::Cursor *cursor = new sf::Cursor;

    if (!cursor->loadFromPixels(pixels, sf::Vector2u(size.x, size.y), sf::Vector2u(hotspot.x, hotspot.y))) {
        delete cursor;
        cursor = NULL;
    }

    return cursor;
}

extern "C" sf::Cursor *sfCursor_createFromSystem(sf::Cursor::Type type) {
    sf::Cursor *cursor = new sf::Cursor;

    if (!cursor->loadFromSystem(type)) {
        delete cursor;
        cursor = NULL;
    }

    return cursor;
}

extern "C" void sfCursor_destroy(sf::Cursor *cursor) {
    delete cursor;
}
