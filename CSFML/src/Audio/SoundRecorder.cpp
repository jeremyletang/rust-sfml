
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

#include "Audio/SoundRecorderStruct.h"
#include <cstddef>
#include <cstdint>
#include <vector>

extern "C" sfSoundRecorder *sfSoundRecorder_create(sfSoundRecorderStartCallback onStart,
                                        sfSoundRecorderProcessCallback onProcess,
                                        sfSoundRecorderStopCallback onStop,
                                        void *userData) {
    return new sfSoundRecorder(onStart, onProcess, onStop, userData);
}

extern "C" void sfSoundRecorder_destroy(sfSoundRecorder *soundRecorder) {
    delete soundRecorder;
}

extern "C" bool sfSoundRecorder_start(sfSoundRecorder *soundRecorder, unsigned int sampleRate) {
    return soundRecorder->This.start(sampleRate);
}

extern "C" void sfSoundRecorder_stop(sfSoundRecorder *soundRecorder) {
    soundRecorder->This.stop();
}

extern "C" unsigned int sfSoundRecorder_getSampleRate(const sfSoundRecorder *soundRecorder) {
    return soundRecorder->This.getSampleRate();
}

extern "C" bool sfSoundRecorder_isAvailable(void) {
    return sf::SoundRecorder::isAvailable();
}

extern "C" void sfSoundRecorder_setProcessingInterval(sfSoundRecorder *soundRecorder, int64_t interval) {
    soundRecorder->This.setProcessingInterval(interval);
}

extern "C" std::vector<std::string> *sfSoundRecorder_getAvailableDevices() {
    std::vector<std::string> devices = sf::SoundRecorder::getAvailableDevices();
    std::vector<std::string> *copy = new std::vector<std::string>(devices);
    return copy;
}

extern "C" std::string *sfSoundRecorder_getDefaultDevice() {
    std::string defaultDevice = sf::SoundRecorder::getDefaultDevice();
    std::string *copy = new std::string(defaultDevice);
    return copy;
}

extern "C" bool sfSoundRecorder_setDevice(sfSoundRecorder *soundRecorder, const char *name) {
    return soundRecorder->This.setDevice(name);
}

extern "C" const std::string *sfSoundRecorder_getDevice(sfSoundRecorder *soundRecorder) {
    return &soundRecorder->This.getDevice();
}

extern "C" void sfSoundRecorder_setChannelCount(sfSoundRecorder *soundRecorder, unsigned int channelCount) {
    soundRecorder->This.setChannelCount(channelCount);
}

extern "C" unsigned int sfSoundRecorder_getChannelCount(const sfSoundRecorder *soundRecorder) {
    return soundRecorder->This.getChannelCount();
}
