
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

#include "Audio/SoundBuffer.h"
#include "System/InputStreamStruct.h"
#include <SFML/Audio/SoundBuffer.hpp>
#include <cstddef>

sfSoundBuffer *sfSoundBuffer_createFromFile(const char *filename) {
    sf::SoundBuffer *buffer = new sf::SoundBuffer;

    if (!buffer->loadFromFile(filename)) {
        delete buffer;
        buffer = NULL;
    }

    return reinterpret_cast<sfSoundBuffer *>(buffer);
}

sfSoundBuffer *sfSoundBuffer_createFromMemory(const void *data, size_t sizeInBytes) {
    sf::SoundBuffer *buffer = new sf::SoundBuffer;

    if (!buffer->loadFromMemory(data, sizeInBytes)) {
        delete buffer;
        buffer = NULL;
    }

    return reinterpret_cast<sfSoundBuffer *>(buffer);
}

sfSoundBuffer *sfSoundBuffer_createFromStream(sfInputStream *stream) {

    sf::SoundBuffer *buffer = new sf::SoundBuffer;
    if (!buffer->loadFromStream(*stream)) {
        delete buffer;
        buffer = NULL;
    }

    return reinterpret_cast<sfSoundBuffer *>(buffer);
}

sfSoundBuffer *sfSoundBuffer_createFromSamples(const int16_t *samples, uint64_t sampleCount, unsigned int channelCount, unsigned int sampleRate) {
    sf::SoundBuffer *buffer = new sf::SoundBuffer;

    if (!buffer->loadFromSamples(samples, sampleCount, channelCount, sampleRate)) {
        delete buffer;
        buffer = NULL;
    }

    return reinterpret_cast<sfSoundBuffer *>(buffer);
}

sfSoundBuffer *sfSoundBuffer_copy(const sfSoundBuffer *soundBuffer) {
    const sf::SoundBuffer *src = reinterpret_cast<const sf::SoundBuffer *>(soundBuffer);
    return reinterpret_cast<sfSoundBuffer *>(new sf::SoundBuffer(*src));
}

void sfSoundBuffer_destroy(sfSoundBuffer *soundBuffer) {
    delete reinterpret_cast<sf::SoundBuffer *>(soundBuffer);
}

bool sfSoundBuffer_saveToFile(const sfSoundBuffer *soundBuffer, const char *filename) {
    return reinterpret_cast<const sf::SoundBuffer *>(soundBuffer)->saveToFile(filename);
}

const int16_t *sfSoundBuffer_getSamples(const sfSoundBuffer *soundBuffer) {
    return reinterpret_cast<const sf::SoundBuffer *>(soundBuffer)->getSamples();
}

uint64_t sfSoundBuffer_getSampleCount(const sfSoundBuffer *soundBuffer) {
    return reinterpret_cast<const sf::SoundBuffer *>(soundBuffer)->getSampleCount();
}

unsigned int sfSoundBuffer_getSampleRate(const sfSoundBuffer *soundBuffer) {
    return reinterpret_cast<const sf::SoundBuffer *>(soundBuffer)->getSampleRate();
}

unsigned int sfSoundBuffer_getChannelCount(const sfSoundBuffer *soundBuffer) {
    return reinterpret_cast<const sf::SoundBuffer *>(soundBuffer)->getChannelCount();
}

int64_t sfSoundBuffer_getDuration(const sfSoundBuffer *soundBuffer) {
    int64_t time = reinterpret_cast<const sf::SoundBuffer *>(soundBuffer)->getDuration().asMicroseconds();
    return time;
}
