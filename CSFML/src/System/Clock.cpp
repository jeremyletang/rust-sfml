#include <SFML/System/Time.hpp>
#include <SFML/System/Clock.hpp>

extern "C" sf::Clock *sfClock_new(void) {
    return new sf::Clock;
}

extern "C" void sfClock_delete(sf::Clock *clock) {
    delete clock;
}

extern "C" int64_t sfClock_getElapsedTime(const sf::Clock *clock) {
    return clock->getElapsedTime().asMicroseconds();
}

extern "C" int64_t sfClock_restart(sf::Clock *clock) {
    return clock->restart().asMicroseconds();
}
