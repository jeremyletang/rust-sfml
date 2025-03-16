#include "System/Vector2.hpp"
#include <SFML/Window/Touch.hpp>
#include <SFML/Window/Window.hpp>

extern "C" bool sfTouch_isDown(unsigned int finger) {
    return sf::Touch::isDown(finger);
}

extern "C" sfVector2i sfTouch_getPosition(unsigned int finger) {
    return convertVector2(sf::Touch::getPosition(finger));
}

extern "C" sfVector2i sfTouch_getPositionRelativeTo(unsigned int finger, const sf::Window *relativeTo) {
    return convertVector2(sf::Touch::getPosition(finger, *relativeTo));
}
