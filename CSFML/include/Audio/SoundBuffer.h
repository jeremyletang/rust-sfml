
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

#ifndef SFML_SOUNDBUFFER_H
#define SFML_SOUNDBUFFER_H

// Headers

#include "Audio/Types.h"
#include <cstdint>
#include <stddef.h>

struct sfInputStream;

extern "C" sfSoundBuffer *sfSoundBuffer_createFromFile(const char *filename);

extern "C" sfSoundBuffer *sfSoundBuffer_createFromMemory(const void *data, size_t sizeInBytes);

extern "C" sfSoundBuffer *sfSoundBuffer_createFromStream(sfInputStream *stream);

extern "C" sfSoundBuffer *sfSoundBuffer_createFromSamples(const int16_t *samples, uint64_t sampleCount, unsigned int channelCount, unsigned int sampleRate);

extern "C" sfSoundBuffer *sfSoundBuffer_copy(const sfSoundBuffer *soundBuffer);

extern "C" void sfSoundBuffer_destroy(sfSoundBuffer *soundBuffer);

extern "C" bool sfSoundBuffer_saveToFile(const sfSoundBuffer *soundBuffer, const char *filename);

extern "C" const int16_t *sfSoundBuffer_getSamples(const sfSoundBuffer *soundBuffer);

extern "C" uint64_t sfSoundBuffer_getSampleCount(const sfSoundBuffer *soundBuffer);

extern "C" unsigned int sfSoundBuffer_getSampleRate(const sfSoundBuffer *soundBuffer);

extern "C" unsigned int sfSoundBuffer_getChannelCount(const sfSoundBuffer *soundBuffer);

extern "C" int64_t sfSoundBuffer_getDuration(const sfSoundBuffer *soundBuffer);

#endif // SFML_SOUNDBUFFER_H
