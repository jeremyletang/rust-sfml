
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

#ifndef SFML_SOUND_H
#define SFML_SOUND_H

// Headers

#include "Audio/SoundStatus.h"
#include "Audio/Types.h"
#include "System/Vector3.h"
#include "Config.h"

extern "C" sfSound *sfSound_create(void);

extern "C" sfSound *sfSound_copy(const sfSound *sound);

extern "C" void sfSound_destroy(sfSound *sound);

extern "C" void sfSound_play(sfSound *sound);

extern "C" void sfSound_pause(sfSound *sound);

extern "C" void sfSound_stop(sfSound *sound);

extern "C" void sfSound_setBuffer(sfSound *sound, const sfSoundBuffer *buffer);

extern "C" const sfSoundBuffer *sfSound_getBuffer(const sfSound *sound);

extern "C" void sfSound_setLoop(sfSound *sound, sfBool loop);

extern "C" sfBool sfSound_getLoop(const sfSound *sound);

extern "C" sfSoundStatus sfSound_getStatus(const sfSound *sound);

extern "C" void sfSound_setPitch(sfSound *sound, float pitch);

extern "C" void sfSound_setVolume(sfSound *sound, float volume);

extern "C" void sfSound_setPosition(sfSound *sound, sfVector3f position);

extern "C" void sfSound_setRelativeToListener(sfSound *sound, sfBool relative);

extern "C" void sfSound_setMinDistance(sfSound *sound, float distance);

extern "C" void sfSound_setAttenuation(sfSound *sound, float attenuation);

extern "C" void sfSound_setPlayingOffset(sfSound *sound, sfInt64 timeOffset);

extern "C" float sfSound_getPitch(const sfSound *sound);

extern "C" float sfSound_getVolume(const sfSound *sound);

extern "C" sfVector3f sfSound_getPosition(const sfSound *sound);

extern "C" sfBool sfSound_isRelativeToListener(const sfSound *sound);

extern "C" float sfSound_getMinDistance(const sfSound *sound);

extern "C" float sfSound_getAttenuation(const sfSound *sound);

extern "C" sfInt64 sfSound_getPlayingOffset(const sfSound *sound);

#endif // SFML_SOUND_H
