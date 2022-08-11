
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

#include "System/InputStreamStruct.h"
#include "System/Vector2.h"
#include "System/Vector3.h"
#include <SFML/Audio/Music.hpp>
#include <cstddef>

typedef struct
{
    int64_t offset; ///< The beginning offset of the time range
    int64_t length; ///< The length of the time range
} sfTimeSpan;

extern "C" sf::Music *sfMusic_createFromFile(const char *filename) {
    sf::Music *music = new sf::Music;
    if (!music->openFromFile(filename)) {
        delete music;
        music = NULL;
    }

    return music;
}

extern "C" sf::Music *sfMusic_createFromMemory(const void *data, size_t sizeInBytes) {
    sf::Music *music = new sf::Music;
    if (!music->openFromMemory(data, sizeInBytes)) {
        delete music;
        music = NULL;
    }

    return music;
}

extern "C" sf::Music *sfMusic_createFromStream(sfInputStream *stream) {

    sf::Music *music = new sf::Music;
    if (!music->openFromStream(*stream)) {
        delete music;
        music = NULL;
    }

    return music;
}

extern "C" void sfMusic_destroy(sf::Music *music) {
    delete music;
}

extern "C" void sfMusic_setLoop(sf::Music *music, bool loop) {
    music->setLoop(loop != 0);
}

extern "C" bool sfMusic_getLoop(const sf::Music *music) {
    return music->getLoop();
}

extern "C" int64_t sfMusic_getDuration(const sf::Music *music) {
    return music->getDuration().asMicroseconds();
}

extern "C" sfTimeSpan sfMusic_getLoopPoints(const sf::Music *music) {
    sf::Music::TimeSpan span = music->getLoopPoints();
    sfTimeSpan timeSpan;

    timeSpan.offset = span.offset.asMicroseconds();
    timeSpan.length = span.length.asMicroseconds();

    return timeSpan;
}

extern "C" void sfMusic_setLoopPoints(sf::Music *music, sfTimeSpan timePoints) {
    music->setLoopPoints(sf::Music::TimeSpan(sf::microseconds(timePoints.offset),
                                             sf::microseconds(timePoints.length)));
}

extern "C" void sfMusic_play(sf::Music *music) {
    music->play();
}

extern "C" void sfMusic_pause(sf::Music *music) {
    music->pause();
}

extern "C" void sfMusic_stop(sf::Music *music) {
    music->stop();
}

extern "C" unsigned int sfMusic_getChannelCount(const sf::Music *music) {
    return music->getChannelCount();
}

extern "C" unsigned int sfMusic_getSampleRate(const sf::Music *music) {
    return music->getSampleRate();
}

extern "C" sf::Music::Status sfMusic_getStatus(const sf::Music *music) {

    return music->getStatus();
}

extern "C" int64_t sfMusic_getPlayingOffset(const sf::Music *music) {
    return music->getPlayingOffset().asMicroseconds();
}

extern "C" void sfMusic_setPitch(sf::Music *music, float pitch) {
    music->setPitch(pitch);
}

extern "C" void sfMusic_setVolume(sf::Music *music, float volume) {
    music->setVolume(volume);
}

extern "C" void sfMusic_setPosition(sf::Music *music, sfVector3f position) {
    music->setPosition(sf::Vector3f(position.x, position.y, position.z));
}

extern "C" void sfMusic_setRelativeToListener(sf::Music *music, bool relative) {
    music->setRelativeToListener(relative);
}

extern "C" void sfMusic_setMinDistance(sf::Music *music, float distance) {
    music->setMinDistance(distance);
}

extern "C" void sfMusic_setAttenuation(sf::Music *music, float attenuation) {
    music->setAttenuation(attenuation);
}

extern "C" void sfMusic_setPlayingOffset(sf::Music *music, int64_t timeOffset) {
    music->setPlayingOffset(sf::microseconds(timeOffset));
}

extern "C" float sfMusic_getPitch(const sf::Music *music) {
    return music->getPitch();
}

extern "C" float sfMusic_getVolume(const sf::Music *music) {
    return music->getVolume();
}

extern "C" sfVector3f sfMusic_getPosition(const sf::Music *music) {
    sf::Vector3f pos = music->getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" bool sfMusic_isRelativeToListener(const sf::Music *music) {
    return music->isRelativeToListener();
}

extern "C" float sfMusic_getMinDistance(const sf::Music *music) {
    return music->getMinDistance();
}

extern "C" float sfMusic_getAttenuation(const sf::Music *music) {
    return music->getAttenuation();
}
