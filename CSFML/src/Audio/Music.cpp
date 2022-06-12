
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

#include "Audio/Music.h"
#include "System/InputStreamStruct.h"
#include <SFML/Audio/Music.hpp>
#include <cstddef>

sfMusic *sfMusic_createFromFile(const char *filename) {
    sf::Music *music = new sf::Music;
    if (!music->openFromFile(filename)) {
        delete music;
        music = NULL;
    }

    return reinterpret_cast<sfMusic *>(music);
}

sfMusic *sfMusic_createFromMemory(const void *data, size_t sizeInBytes) {
    sf::Music *music = new sf::Music;
    if (!music->openFromMemory(data, sizeInBytes)) {
        delete music;
        music = NULL;
    }

    return reinterpret_cast<sfMusic *>(music);
}

sfMusic *sfMusic_createFromStream(sfInputStream *stream) {

    sf::Music *music = new sf::Music;
    if (!music->openFromStream(*stream)) {
        delete music;
        music = NULL;
    }

    return reinterpret_cast<sfMusic *>(music);
}

void sfMusic_destroy(sfMusic *music) {
    delete reinterpret_cast<sf::Music *>(music);
}

void sfMusic_setLoop(sfMusic *music, sfBool loop) {
    reinterpret_cast<sf::Music *>(music)->setLoop(loop != 0);
}

sfBool sfMusic_getLoop(const sfMusic *music) {
    return reinterpret_cast<const sf::Music *>(music)->getLoop();
}

sfInt64 sfMusic_getDuration(const sfMusic *music) {
    sfInt64 time = reinterpret_cast<const sf::Music *>(music)->getDuration().asMicroseconds();
    return time;
}

sfTimeSpan sfMusic_getLoopPoints(const sfMusic *music) {
    sf::Music::TimeSpan span = reinterpret_cast<const sf::Music *>(music)->getLoopPoints();
    sfTimeSpan timeSpan;

    timeSpan.offset = span.offset.asMicroseconds();
    timeSpan.length = span.length.asMicroseconds();

    return timeSpan;
}

void sfMusic_setLoopPoints(sfMusic *music, sfTimeSpan timePoints) {
    reinterpret_cast<sf::Music *>(music)->setLoopPoints(sf::Music::TimeSpan(sf::microseconds(timePoints.offset),
                                                                            sf::microseconds(timePoints.length)));
}

void sfMusic_play(sfMusic *music) {
    reinterpret_cast<sf::Music *>(music)->play();
}

void sfMusic_pause(sfMusic *music) {
    reinterpret_cast<sf::Music *>(music)->pause();
}

void sfMusic_stop(sfMusic *music) {
    reinterpret_cast<sf::Music *>(music)->stop();
}

unsigned int sfMusic_getChannelCount(const sfMusic *music) {
    return reinterpret_cast<const sf::Music *>(music)->getChannelCount();
}

unsigned int sfMusic_getSampleRate(const sfMusic *music) {
    return reinterpret_cast<const sf::Music *>(music)->getSampleRate();
}

sfSoundStatus sfMusic_getStatus(const sfMusic *music) {

    return static_cast<sfSoundStatus>(reinterpret_cast<const sf::Music *>(music)->getStatus());
}

sfInt64 sfMusic_getPlayingOffset(const sfMusic *music) {
    sfInt64 time = reinterpret_cast<const sf::Music *>(music)->getPlayingOffset().asMicroseconds();
    return time;
}

void sfMusic_setPitch(sfMusic *music, float pitch) {
    reinterpret_cast<sf::Music *>(music)->setPitch(pitch);
}

void sfMusic_setVolume(sfMusic *music, float volume) {
    reinterpret_cast<sf::Music *>(music)->setVolume(volume);
}

void sfMusic_setPosition(sfMusic *music, sfVector3f position) {
    reinterpret_cast<sf::Music *>(music)->setPosition(sf::Vector3f(position.x, position.y, position.z));
}

void sfMusic_setRelativeToListener(sfMusic *music, sfBool relative) {
    reinterpret_cast<sf::Music *>(music)->setRelativeToListener(relative == sfTrue);
}

void sfMusic_setMinDistance(sfMusic *music, float distance) {
    reinterpret_cast<sf::Music *>(music)->setMinDistance(distance);
}

void sfMusic_setAttenuation(sfMusic *music, float attenuation) {
    reinterpret_cast<sf::Music *>(music)->setAttenuation(attenuation);
}

void sfMusic_setPlayingOffset(sfMusic *music, sfInt64 timeOffset) {
    reinterpret_cast<sf::Music *>(music)->setPlayingOffset(sf::microseconds(timeOffset));
}

float sfMusic_getPitch(const sfMusic *music) {
    return reinterpret_cast<const sf::Music *>(music)->getPitch();
}

float sfMusic_getVolume(const sfMusic *music) {
    return reinterpret_cast<const sf::Music *>(music)->getVolume();
}

sfVector3f sfMusic_getPosition(const sfMusic *music) {
    sfVector3f position = {0, 0, 0};

    sf::Vector3f sfmlPos = reinterpret_cast<const sf::Music *>(music)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;
    position.z = sfmlPos.z;

    return position;
}

sfBool sfMusic_isRelativeToListener(const sfMusic *music) {
    return reinterpret_cast<const sf::Music *>(music)->isRelativeToListener();
}

float sfMusic_getMinDistance(const sfMusic *music) {
    return reinterpret_cast<const sf::Music *>(music)->getMinDistance();
}

float sfMusic_getAttenuation(const sfMusic *music) {
    return reinterpret_cast<const sf::Music *>(music)->getAttenuation();
}
