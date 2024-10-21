#include "System/Vector3.hpp"
#include <SFML/Window/Sensor.hpp>

extern "C" bool sfSensor_isAvailable(sf::Sensor::Type sensor) {
    return sf::Sensor::isAvailable(sensor);
}

extern "C" void sfSensor_setEnabled(sf::Sensor::Type sensor, bool enabled) {
    sf::Sensor::setEnabled(sensor, enabled);
}

extern "C" sfVector3f sfSensor_getValue(sf::Sensor::Type sensor) {
    sf::Vector3f val = sf::Sensor::getValue(sensor);
    return {val.x, val.y, val.z};
}
