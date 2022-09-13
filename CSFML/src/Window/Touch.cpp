#include "System/Vector2.h"
#include <SFML/Window/Touch.hpp>

extern "C" bool sfTouch_isDown(unsigned int finger) {
    return sf::Touch::isDown(finger);
}

extern "C" sfVector2i sfTouch_getPosition(unsigned int finger) {
    sf::Vector2i pos = sf::Touch::getPosition(finger);
    return {pos.x, pos.y};
}

extern "C" sfVector2i sfTouch_getPositionRelativeTo(unsigned int finger, const sf::Window *relativeTo) {
    sf::Vector2i pos = sf::Touch::getPosition(finger, *relativeTo);
    return {pos.x, pos.y};
}
