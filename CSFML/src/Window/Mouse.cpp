#include "Window/Mouse.hpp"
#include "System/Vector2.hpp"
#include <SFML/Window/Mouse.hpp>
#include <SFML/Window/Window.hpp>

extern "C" bool sfMouse_isButtonPressed(sfMouseButton button) {
    return sf::Mouse::isButtonPressed(static_cast<sf::Mouse::Button>(button));
}

extern "C" sfVector2i sfMouse_getPosition() {
    return convertVector2(sf::Mouse::getPosition());
}

extern "C" sfVector2i sfMouse_getPositionRelativeTo(const sf::Window *relativeTo) {
    return convertVector2(sf::Mouse::getPosition(*relativeTo));
}

extern "C" void sfMouse_setPosition(sfVector2i position) {
    sf::Mouse::setPosition(convertVector2(position));
}

extern "C" void sfMouse_setPositionRelativeTo(sfVector2i pos, const sf::Window *relativeTo) {
    sf::Mouse::setPosition(sf::Vector2i(pos.x, pos.y), *relativeTo);
}
