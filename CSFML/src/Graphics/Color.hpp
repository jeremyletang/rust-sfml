#pragma once
#include <SFML/Graphics/Color.hpp>

struct sfColor {
    uint8_t r;
    uint8_t g;
    uint8_t b;
    uint8_t a;
};

////////////////////////////////////////////////////////////
// Convert sf::Color to sfColor
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfColor convertColor(const sf::Color color) {
    return {color.r, color.g, color.b, color.a};
}

////////////////////////////////////////////////////////////
// Convert sfColor to sf::Color
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::Color convertColor(const sfColor color) {
    return {color.r, color.g, color.b, color.a};
}
