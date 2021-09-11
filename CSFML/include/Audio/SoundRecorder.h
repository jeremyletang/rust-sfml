
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

#ifndef SFML_SOUNDRECORDER_H
#define SFML_SOUNDRECORDER_H

// Headers

#include "Audio/Types.h"
#include "System/Time.h"
#include <stddef.h>

typedef sfBool (*sfSoundRecorderStartCallback)(void *);                            ///< Type of the callback used when starting a capture
typedef sfBool (*sfSoundRecorderProcessCallback)(const sfInt16 *, size_t, void *); ///< Type of the callback used to process audio data
typedef void (*sfSoundRecorderStopCallback)(void *);                               ///< Type of the callback used when stopping a capture

extern "C" sfSoundRecorder *sfSoundRecorder_create(sfSoundRecorderStartCallback onStart,
                                                   sfSoundRecorderProcessCallback onProcess,
                                                   sfSoundRecorderStopCallback onStop,
                                                   void *userData);

extern "C" void sfSoundRecorder_destroy(sfSoundRecorder *soundRecorder);

extern "C" sfBool sfSoundRecorder_start(sfSoundRecorder *soundRecorder, unsigned int sampleRate);

extern "C" void sfSoundRecorder_stop(sfSoundRecorder *soundRecorder);

extern "C" unsigned int sfSoundRecorder_getSampleRate(const sfSoundRecorder *soundRecorder);

extern "C" sfBool sfSoundRecorder_isAvailable(void);

extern "C" void sfSoundRecorder_setProcessingInterval(sfSoundRecorder *soundRecorder, sfTime interval);

extern "C" sfBool sfSoundRecorder_setDevice(sfSoundRecorder *soundRecorder, const char *name);

extern "C" void sfSoundRecorder_setChannelCount(sfSoundRecorder *soundRecorder, unsigned int channelCount);

extern "C" unsigned int sfSoundRecorder_getChannelCount(const sfSoundRecorder *soundRecorder);

#endif // SFML_SOUNDRECORDER_H
