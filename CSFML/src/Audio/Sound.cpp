
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

#include "Audio/Sound.h"
#include <SFML/Audio/Sound.hpp>
#include <cstddef>

sfSound *sfSound_create(void) {
    return reinterpret_cast<sfSound *>(new sf::Sound);
}

sfSound *sfSound_copy(const sfSound *sound) {
    const sf::Sound *src = reinterpret_cast<const sf::Sound *>(sound);
    return reinterpret_cast<sfSound *>(new sf::Sound(*src));
}

void sfSound_destroy(sfSound *sound) {
    delete reinterpret_cast<sf::Sound *>(sound);
}

void sfSound_play(sfSound *sound) {
    reinterpret_cast<sf::Sound *>(sound)->play();
}

void sfSound_pause(sfSound *sound) {
    reinterpret_cast<sf::Sound *>(sound)->pause();
}

void sfSound_stop(sfSound *sound) {
    reinterpret_cast<sf::Sound *>(sound)->stop();
}

void sfSound_setBuffer(sfSound *sound, const sfSoundBuffer *buffer) {
    reinterpret_cast<sf::Sound *>(sound)->setBuffer(*reinterpret_cast<const sf::SoundBuffer *>(buffer));
}

const sfSoundBuffer *sfSound_getBuffer(const sfSound *sound) {
    const sf::Sound *s = reinterpret_cast<const sf::Sound *>(sound);
    return reinterpret_cast<const sfSoundBuffer *>(s->getBuffer());
}

void sfSound_setLoop(sfSound *sound, bool loop) {
    reinterpret_cast<sf::Sound *>(sound)->setLoop(loop);
}

bool sfSound_getLoop(const sfSound *sound) {
    return reinterpret_cast<const sf::Sound *>(sound)->getLoop();
}

sfSoundStatus sfSound_getStatus(const sfSound *sound) {

    return static_cast<sfSoundStatus>(reinterpret_cast<const sf::Sound *>(sound)->getStatus());
}

void sfSound_setPitch(sfSound *sound, float pitch) {
    reinterpret_cast<sf::Sound *>(sound)->setPitch(pitch);
}

void sfSound_setVolume(sfSound *sound, float volume) {
    reinterpret_cast<sf::Sound *>(sound)->setVolume(volume);
}

void sfSound_setPosition(sfSound *sound, sfVector3f position) {
    reinterpret_cast<sf::Sound *>(sound)->setPosition(sf::Vector3f(position.x, position.y, position.z));
}

void sfSound_setRelativeToListener(sfSound *sound, bool relative) {
    reinterpret_cast<sf::Sound *>(sound)->setRelativeToListener(relative);
}

void sfSound_setMinDistance(sfSound *sound, float distance) {
    reinterpret_cast<sf::Sound *>(sound)->setMinDistance(distance);
}

void sfSound_setAttenuation(sfSound *sound, float attenuation) {
    reinterpret_cast<sf::Sound *>(sound)->setAttenuation(attenuation);
}

void sfSound_setPlayingOffset(sfSound *sound, sfInt64 timeOffset) {
    reinterpret_cast<sf::Sound *>(sound)->setPlayingOffset(sf::microseconds(timeOffset));
}

float sfSound_getPitch(const sfSound *sound) {
    return reinterpret_cast<const sf::Sound *>(sound)->getPitch();
}

float sfSound_getVolume(const sfSound *sound) {
    return reinterpret_cast<const sf::Sound *>(sound)->getVolume();
}

sfVector3f sfSound_getPosition(const sfSound *sound) {
    sfVector3f position = {0, 0, 0};

    sf::Vector3f sfmlPos = reinterpret_cast<const sf::Sound *>(sound)->getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;
    position.z = sfmlPos.z;

    return position;
}

bool sfSound_isRelativeToListener(const sfSound *sound) {
    return reinterpret_cast<const sf::Sound *>(sound)->isRelativeToListener();
}

float sfSound_getMinDistance(const sfSound *sound) {
    return reinterpret_cast<const sf::Sound *>(sound)->getMinDistance();
}

float sfSound_getAttenuation(const sfSound *sound) {
    return reinterpret_cast<const sf::Sound *>(sound)->getAttenuation();
}

sfInt64 sfSound_getPlayingOffset(const sfSound *sound) {
    sfInt64 time = reinterpret_cast<const sf::Sound *>(sound)->getPlayingOffset().asMicroseconds();
    return time;
}
