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

extern "C" bool sfClock_isRunning(const sf::Clock *clock) {
    return clock->isRunning();
}

extern "C" void sfClock_start(sf::Clock *clock) {
    clock->start();
}

extern "C" void sfClock_stop(sf::Clock *clock) {
    clock->stop();
}

extern "C" int64_t sfClock_restart(sf::Clock *clock) {
    return clock->restart().asMicroseconds();
}

extern "C" int64_t sfClock_reset(sf::Clock *clock) {
    return clock->reset().asMicroseconds();
}
