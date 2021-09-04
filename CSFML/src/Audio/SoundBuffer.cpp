
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
#include "Audio/SoundBufferStruct.h"
#include "System/InputStreamStruct.h"
#include <cstddef>

sfSoundBuffer *sfSoundBuffer_createFromFile(const char *filename) {
    sfSoundBuffer *buffer = new sfSoundBuffer;

    if (!buffer->This.loadFromFile(filename)) {
        delete buffer;
        buffer = NULL;
    }

    return buffer;
}

sfSoundBuffer *sfSoundBuffer_createFromMemory(const void *data, size_t sizeInBytes) {
    sfSoundBuffer *buffer = new sfSoundBuffer;

    if (!buffer->This.loadFromMemory(data, sizeInBytes)) {
        delete buffer;
        buffer = NULL;
    }

    return buffer;
}

sfSoundBuffer *sfSoundBuffer_createFromStream(sfInputStream *stream) {

    sfSoundBuffer *buffer = new sfSoundBuffer;
    if (!buffer->This.loadFromStream(*stream)) {
        delete buffer;
        buffer = NULL;
    }

    return buffer;
}

sfSoundBuffer *sfSoundBuffer_createFromSamples(const sfInt16 *samples, sfUint64 sampleCount, unsigned int channelCount, unsigned int sampleRate) {
    sfSoundBuffer *buffer = new sfSoundBuffer;

    if (!buffer->This.loadFromSamples(samples, sampleCount, channelCount, sampleRate)) {
        delete buffer;
        buffer = NULL;
    }

    return buffer;
}

sfSoundBuffer *sfSoundBuffer_copy(const sfSoundBuffer *soundBuffer) {

    return new sfSoundBuffer(*soundBuffer);
}

void sfSoundBuffer_destroy(sfSoundBuffer *soundBuffer) {
    delete soundBuffer;
}

sfBool sfSoundBuffer_saveToFile(const sfSoundBuffer *soundBuffer, const char *filename) {
    return soundBuffer->This.saveToFile(filename);
}

const sfInt16 *sfSoundBuffer_getSamples(const sfSoundBuffer *soundBuffer) {
    return soundBuffer->This.getSamples();
}

sfUint64 sfSoundBuffer_getSampleCount(const sfSoundBuffer *soundBuffer) {
    return soundBuffer->This.getSampleCount();
}

unsigned int sfSoundBuffer_getSampleRate(const sfSoundBuffer *soundBuffer) {
    return soundBuffer->This.getSampleRate();
}

unsigned int sfSoundBuffer_getChannelCount(const sfSoundBuffer *soundBuffer) {
    return soundBuffer->This.getChannelCount();
}

sfTime sfSoundBuffer_getDuration(const sfSoundBuffer *soundBuffer) {
    sfTime time = {0};

    time.microseconds = soundBuffer->This.getDuration().asMicroseconds();
    return time;
}
