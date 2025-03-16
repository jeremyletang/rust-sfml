#pragma once

#include "SFML/System/Vector3.hpp"
struct sfVector3f {
    float x;
    float y;
    float z;
};

////////////////////////////////////////////////////////////
// Convert sf::Vector3f to sfVector3f
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfVector3f convertVector3(const sf::Vector3f vector) {
    return {vector.x, vector.y, vector.z};
}

////////////////////////////////////////////////////////////
// Convert sfVector3f to sf::Vector3f
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::Vector3f convertVector3(const sfVector3f vector) {
    return {vector.x, vector.y, vector.z};
}
