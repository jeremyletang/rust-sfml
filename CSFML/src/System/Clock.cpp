
//
// SFML - Simple and Fast Multimedia Library
// Copyright (C) 2007-2018 Laurent Gomila (laurent@sfml-dev.org)
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it freely,
// subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented;
//    you must not claim that you wrote the original software.
//    If you use this software in a product, an acknowledgment
//    in the product documentation would be appreciated but is not required.
//
// 2. Altered source versions must be plainly marked as such,
//    and must not be misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

// Headers

#include <SFML/System/Clock.hpp>
#include <cstddef>

struct sfClock {
    sf::Clock This;
};

extern "C" sfClock *sfClock_create(void) {
    return new sfClock;
}

extern "C" sfClock *sfClock_copy(const sfClock *clock) {

    return new sfClock(*clock);
}

extern "C" void sfClock_destroy(sfClock *clock) {
    delete clock;
}

extern "C" sf::Int64 sfClock_getElapsedTime(const sfClock *clock) {
    sf::Time time = clock->This.getElapsedTime();
    return time.asMicroseconds();
}

extern "C" sf::Int64 sfClock_restart(sfClock *clock) {
    sf::Time time = clock->This.restart();
    return time.asMicroseconds();
}
