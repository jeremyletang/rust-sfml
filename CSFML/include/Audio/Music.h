
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

#ifndef SFML_MUSIC_H
#define SFML_MUSIC_H

// Headers

#include "Audio/SoundStatus.h"
#include "Audio/Types.h"
#include "System/Vector3.h"
#include <stddef.h>
#include <cstdint>

struct sfInputStream;

typedef struct
{
    int64_t offset; ///< The beginning offset of the time range
    int64_t length; ///< The length of the time range
} sfTimeSpan;

extern "C" sfMusic *sfMusic_createFromFile(const char *filename);

extern "C" sfMusic *sfMusic_createFromMemory(const void *data, size_t sizeInBytes);

extern "C" sfMusic *sfMusic_createFromStream(sfInputStream *stream);

extern "C" void sfMusic_destroy(sfMusic *music);

extern "C" void sfMusic_setLoop(sfMusic *music, bool loop);

extern "C" bool sfMusic_getLoop(const sfMusic *music);

extern "C" int64_t sfMusic_getDuration(const sfMusic *music);

extern "C" sfTimeSpan sfMusic_getLoopPoints(const sfMusic *music);

extern "C" void sfMusic_setLoopPoints(sfMusic *music, sfTimeSpan timePoints);

extern "C" void sfMusic_play(sfMusic *music);

extern "C" void sfMusic_pause(sfMusic *music);

extern "C" void sfMusic_stop(sfMusic *music);

extern "C" unsigned int sfMusic_getChannelCount(const sfMusic *music);

extern "C" unsigned int sfMusic_getSampleRate(const sfMusic *music);

extern "C" sfSoundStatus sfMusic_getStatus(const sfMusic *music);

extern "C" int64_t sfMusic_getPlayingOffset(const sfMusic *music);

extern "C" void sfMusic_setPitch(sfMusic *music, float pitch);

extern "C" void sfMusic_setVolume(sfMusic *music, float volume);

extern "C" void sfMusic_setPosition(sfMusic *music, sfVector3f position);

extern "C" void sfMusic_setRelativeToListener(sfMusic *music, bool relative);

extern "C" void sfMusic_setMinDistance(sfMusic *music, float distance);

extern "C" void sfMusic_setAttenuation(sfMusic *music, float attenuation);

extern "C" void sfMusic_setPlayingOffset(sfMusic *music, int64_t timeOffset);

extern "C" float sfMusic_getPitch(const sfMusic *music);

extern "C" float sfMusic_getVolume(const sfMusic *music);

extern "C" sfVector3f sfMusic_getPosition(const sfMusic *music);

extern "C" bool sfMusic_isRelativeToListener(const sfMusic *music);

extern "C" float sfMusic_getMinDistance(const sfMusic *music);

extern "C" float sfMusic_getAttenuation(const sfMusic *music);

#endif // SFML_MUSIC_H
