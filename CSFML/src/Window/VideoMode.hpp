#pragma once

#include <SFML/Window/VideoMode.hpp>
#include "System/Vector2.hpp"
struct sfVideoMode {
    sfVector2u size;
    unsigned int bitsPerPixel;
};

////////////////////////////////////////////////////////////
// Convert sf::VideoMode to sfVideoMode
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfVideoMode convertVideoMode(const sf::VideoMode &videoMode) {
    return {{videoMode.size.x, videoMode.size.y}, videoMode.bitsPerPixel};
}

////////////////////////////////////////////////////////////
// Convert sfVideoMode to sf::VideoMode
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::VideoMode convertVideoMode(const sfVideoMode &videoMode) {
    return sf::VideoMode({videoMode.size.x, videoMode.size.y}, videoMode.bitsPerPixel);
}
