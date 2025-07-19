#pragma once

#include "SFML/System/Vector2.hpp"
struct sfVector2i {
    int x;
    int y;
};

struct sfVector2u {
    unsigned int x;
    unsigned int y;
};

struct sfVector2f {
    float x;
    float y;
};

////////////////////////////////////////////////////////////
// Convert sf::Vector2 to sfVector2
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfVector2i convertVector2(const sf::Vector2i vector) {
    return {vector.x, vector.y};
}
[[nodiscard]] inline sfVector2u convertVector2(const sf::Vector2u vector) {
    return {vector.x, vector.y};
}
[[nodiscard]] inline sfVector2f convertVector2(const sf::Vector2f vector) {
    return {vector.x, vector.y};
}

////////////////////////////////////////////////////////////
// Convert sfVector2 to sf::Vector2
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::Vector2i convertVector2(const sfVector2i vector) {
    return {vector.x, vector.y};
}
[[nodiscard]] inline sf::Vector2u convertVector2(const sfVector2u vector) {
    return {vector.x, vector.y};
}
[[nodiscard]] inline sf::Vector2f convertVector2(const sfVector2f vector) {
    return {vector.x, vector.y};
}
