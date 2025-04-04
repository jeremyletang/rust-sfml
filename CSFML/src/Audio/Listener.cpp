#include "System/Vector3.hpp"
#include <SFML/Audio/Listener.hpp>

extern "C" void sfListener_setGlobalVolume(float volume) {
    sf::Listener::setGlobalVolume(volume);
}

extern "C" float sfListener_getGlobalVolume(void) {
    return sf::Listener::getGlobalVolume();
}

extern "C" void sfListener_setPosition(sf::Vector3f position) {
    sf::Listener::setPosition(position);
}

extern "C" sf::Vector3f sfListener_getPosition() {
    sf::Vector3f pos = sf::Listener::getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" void sfListener_setDirection(sf::Vector3f direction) {
    sf::Listener::setDirection(direction);
}

extern "C" sf::Vector3f sfListener_getDirection() {
    sf::Vector3f dir = sf::Listener::getDirection();
    return {dir.x, dir.y, dir.z};
}

extern "C" void sfListener_setUpVector(sf::Vector3f upVector) {
    sf::Listener::setUpVector(upVector);
}

extern "C" sf::Vector3f sfListener_getUpVector() {
    sf::Vector3f vec = sf::Listener::getUpVector();
    return {vec.x, vec.y, vec.z};
}
