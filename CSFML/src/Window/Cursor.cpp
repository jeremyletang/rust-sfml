#include "System/Vector2.hpp"
#include <SFML/Window/Cursor.hpp>

extern "C" bool sfCursor_createFromPixels(sf::Cursor *new_value, const uint8_t *pixels, sfVector2u size, sfVector2u hotspot) {
    auto maybe_cursor = sf::Cursor::createFromPixels(pixels, {size.x, size.y}, {hotspot.x, hotspot.y});
    if (maybe_cursor) {
        *new_value = std::move(maybe_cursor).value();
        return true;
    }
    return false;
}

extern "C" bool sfCursor_createFromSystem(sf::Cursor *new_value, sf::Cursor::Type type) {
    auto maybe_cursor = sf::Cursor::createFromSystem(type);
    if (maybe_cursor) {
        *new_value = std::move(maybe_cursor).value();
        return true;
    }
    return false;
}

extern "C" void sfCursor_del(sf::Cursor *cursor) {
    delete cursor;
}
