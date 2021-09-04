
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

#ifndef SFML_SOUNDBUFFERRECORDER_H
#define SFML_SOUNDBUFFERRECORDER_H

// Headers

#include "Audio/Types.h"
#include "Config.h"

extern "C" sfSoundBufferRecorder *sfSoundBufferRecorder_create(void);

extern "C" void sfSoundBufferRecorder_destroy(sfSoundBufferRecorder *soundBufferRecorder);

extern "C" sfBool sfSoundBufferRecorder_start(sfSoundBufferRecorder *soundBufferRecorder, unsigned int sampleRate);

extern "C" void sfSoundBufferRecorder_stop(sfSoundBufferRecorder *soundBufferRecorder);

extern "C" unsigned int sfSoundBufferRecorder_getSampleRate(const sfSoundBufferRecorder *soundBufferRecorder);

extern "C" const sfSoundBuffer *sfSoundBufferRecorder_getBuffer(const sfSoundBufferRecorder *soundBufferRecorder);

extern "C" sfBool sfSoundBufferRecorder_setDevice(sfSoundBufferRecorder *soundBufferRecorder, const char *name);

extern "C" const char *sfSoundBufferRecorder_getDevice(sfSoundBufferRecorder *soundBufferRecorder);

#endif // SFML_SOUNDBUFFERRECORDER_H
