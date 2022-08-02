
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

#include <SFML/Audio/SoundBufferRecorder.hpp>
#include <cstddef>

extern "C" sf::SoundBufferRecorder *sfSoundBufferRecorder_create(void) {
    return new sf::SoundBufferRecorder;
}

extern "C" void sfSoundBufferRecorder_destroy(sf::SoundBufferRecorder *soundBufferRecorder) {
    delete soundBufferRecorder;
}

extern "C" bool sfSoundBufferRecorder_start(sf::SoundBufferRecorder *soundBufferRecorder, unsigned int sampleRate) {
    return soundBufferRecorder->start(sampleRate);
}

extern "C" void sfSoundBufferRecorder_stop(sf::SoundBufferRecorder *soundBufferRecorder) {
    soundBufferRecorder->stop();
}

extern "C" unsigned int sfSoundBufferRecorder_getSampleRate(const sf::SoundBufferRecorder *soundBufferRecorder) {
    return soundBufferRecorder->getSampleRate();
}

extern "C" const sf::SoundBuffer *sfSoundBufferRecorder_getBuffer(const sf::SoundBufferRecorder *soundBufferRecorder) {
    const sf::SoundBuffer *buf = &soundBufferRecorder->getBuffer();
    return buf;
}

extern "C" bool sfSoundBufferRecorder_setDevice(sf::SoundBufferRecorder *soundBufferRecorder, const char *name) {
    return soundBufferRecorder->setDevice(name);
}

extern "C" const std::string *sfSoundBufferRecorder_getDevice(sf::SoundBufferRecorder *soundBufferRecorder) {
    const std::string *device = &soundBufferRecorder->getDevice();
    return device;
}
