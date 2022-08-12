#include "System/Vector3.h"
#include <SFML/Audio/Listener.hpp>

extern "C" void sfListener_setGlobalVolume(float volume) {
    sf::Listener::setGlobalVolume(volume);
}

extern "C" float sfListener_getGlobalVolume(void) {
    return sf::Listener::getGlobalVolume();
}

extern "C" void sfListener_setPosition(sfVector3f position) {
    sf::Listener::setPosition(position.x, position.y, position.z);
}

extern "C" sfVector3f sfListener_getPosition() {
    sf::Vector3f pos = sf::Listener::getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" void sfListener_setDirection(sfVector3f direction) {
    sf::Listener::setDirection(direction.x, direction.y, direction.z);
}

extern "C" sfVector3f sfListener_getDirection() {
    sf::Vector3f dir = sf::Listener::getDirection();
    return {dir.x, dir.y, dir.z};
}

extern "C" void sfListener_setUpVector(sfVector3f upVector) {
    sf::Listener::setUpVector(upVector.x, upVector.y, upVector.z);
}

extern "C" sfVector3f sfListener_getUpVector() {
    sf::Vector3f vec = sf::Listener::getUpVector();
    return {vec.x, vec.y, vec.z};
}
