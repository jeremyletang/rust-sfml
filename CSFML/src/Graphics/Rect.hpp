#pragma once

#include <SFML/Graphics/Rect.hpp>
#include "System/Vector2.hpp"
struct sfFloatRect {
    sfVector2f position;
    sfVector2f size;
};

struct sfIntRect {
    sfVector2i position;
    sfVector2i size;
};

////////////////////////////////////////////////////////////
// Convert sf::Rect<T> to CSFML Rect
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfFloatRect convertRect(const sf::FloatRect &rect) {
    return {{rect.position.x, rect.position.y}, {rect.size.x, rect.size.y}};
}
[[nodiscard]] inline sfIntRect convertRect(const sf::IntRect &rect) {
    return {{rect.position.x, rect.position.y}, {rect.size.x, rect.size.y}};
}

////////////////////////////////////////////////////////////
// Convert CSFML Rect to sf::Rect<T>
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::FloatRect convertRect(const sfFloatRect &rect) {
    return {{rect.position.x, rect.position.y}, {rect.size.x, rect.size.y}};
}
[[nodiscard]] inline sf::IntRect convertRect(const sfIntRect &rect) {
    return {{rect.position.x, rect.position.y}, {rect.size.x, rect.size.y}};
}
