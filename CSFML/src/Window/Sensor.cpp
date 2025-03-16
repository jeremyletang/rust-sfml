#include "Window/Sensor.hpp"
#include "System/Vector3.hpp"
#include <SFML/Window/Sensor.hpp>

extern "C" bool sfSensor_isAvailable(sfSensorType sensor) {
    return sf::Sensor::isAvailable(static_cast<sf::Sensor::Type>(sensor));
}

extern "C" void sfSensor_setEnabled(sfSensorType sensor, bool enabled) {
    sf::Sensor::setEnabled(static_cast<sf::Sensor::Type>(sensor), enabled);
}

extern "C" sfVector3f sfSensor_getValue(sfSensorType sensor) {
    return convertVector3(sf::Sensor::getValue(static_cast<sf::Sensor::Type>(sensor)));
}
