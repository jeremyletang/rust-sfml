#include <SFML/System/Time.hpp>
#include <SFML/System/Sleep.hpp>
#include <cstdint>

extern "C" void sfSleep(int64_t duration_ms) {
    sf::sleep(sf::microseconds(duration_ms));
}
