#include "System/Vector3.hpp"
#include <SFML/Audio/Listener.hpp>

typedef struct
{
    float innerAngle; //!< Inner angle, in degrees
    float outerAngle; //!< Outer angle, in degrees
    float outerGain;  //!< Outer gain
} sfListenerCone;

////////////////////////////////////////////////////////////
// Convert sf::Listener::Cone to sfListenerCone
////////////////////////////////////////////////////////////
[[nodiscard]] inline sfListenerCone convertCone(const sf::Listener::Cone cone) {
    return {cone.innerAngle.asDegrees(), cone.outerAngle.asDegrees(), cone.outerGain};
}

////////////////////////////////////////////////////////////
// Convert sfVector3f to sf::Vector3f
////////////////////////////////////////////////////////////
[[nodiscard]] inline sf::Listener::Cone convertCone(const sfListenerCone cone) {
    return {sf::degrees(cone.innerAngle), sf::degrees(cone.outerAngle), cone.outerGain};
}

extern "C" void sfListener_setGlobalVolume(float volume) {
    sf::Listener::setGlobalVolume(volume);
}

extern "C" float sfListener_getGlobalVolume(void) {
    return sf::Listener::getGlobalVolume();
}

extern "C" void sfListener_setPosition(sfVector3f position) {
    sf::Listener::setPosition(convertVector3(position));
}

extern "C" sfVector3f sfListener_getPosition() {
    sf::Vector3f pos = sf::Listener::getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" void sfListener_setDirection(sfVector3f direction) {
    sf::Listener::setDirection(convertVector3(direction));
}

extern "C" sfVector3f sfListener_getDirection() {
    return convertVector3(sf::Listener::getDirection());
}

extern "C" void sfListener_setUpVector(sfVector3f upVector) {
    sf::Listener::setUpVector(convertVector3(upVector));
}

extern "C" sfVector3f sfListener_getUpVector() {
    return convertVector3(sf::Listener::getUpVector());
}

extern "C" void sfListener_setVelocity(sfVector3f velocity) {
    sf::Listener::setVelocity(convertVector3(velocity));
}

extern "C" sfVector3f sfListener_getVelocity() {
    return convertVector3(sf::Listener::getVelocity());
}

extern "C" void sfListener_setCone(sfListenerCone cone) {
    sf::Listener::setCone(convertCone(cone));
}

extern "C" sfListenerCone sfListener_getCone() {
    return convertCone(sf::Listener::getCone());
}
