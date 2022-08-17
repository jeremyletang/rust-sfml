#include <SFML/System/Sleep.hpp>

extern "C" void sfSleep(sf::Int64 duration_ms) {
    sf::sleep(sf::microseconds(duration_ms));
}
