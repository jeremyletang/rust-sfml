
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

#include "Audio/SoundStreamStruct.h"
#include "System/Vector3.h"
#include <cstddef>

extern "C" sfSoundStream *sfSoundStream_create(sfSoundStreamGetDataCallback onGetData,
                                    sfSoundStreamSeekCallback onSeek,
                                    unsigned int channelCount,
                                    unsigned int sampleRate,
                                    void *userData) {
    return new sfSoundStream(onGetData, onSeek, channelCount, sampleRate, userData);
}

extern "C" void sfSoundStream_destroy(sfSoundStream *soundStream) {
    delete soundStream;
}

extern "C" void sfSoundStream_play(sfSoundStream *soundStream) {
    soundStream->This.play();
}

extern "C" void sfSoundStream_pause(sfSoundStream *soundStream) {
    soundStream->This.pause();
}

extern "C" void sfSoundStream_stop(sfSoundStream *soundStream) {
    soundStream->This.stop();
}

extern "C" sf::SoundStream::Status sfSoundStream_getStatus(const sfSoundStream *soundStream) {

    return soundStream->This.getStatus();
}

extern "C" unsigned int sfSoundStream_getChannelCount(const sfSoundStream *soundStream) {
    return soundStream->This.getChannelCount();
}

extern "C" unsigned int sfSoundStream_getSampleRate(const sfSoundStream *soundStream) {
    return soundStream->This.getSampleRate();
}

extern "C" void sfSoundStream_setPitch(sfSoundStream *soundStream, float pitch) {
    soundStream->This.setPitch(pitch);
}

extern "C" void sfSoundStream_setVolume(sfSoundStream *soundStream, float volume) {
    soundStream->This.setVolume(volume);
}

extern "C" void sfSoundStream_setPosition(sfSoundStream *soundStream, sfVector3f position) {
    soundStream->This.setPosition(position.x, position.y, position.z);
}

extern "C" void sfSoundStream_setRelativeToListener(sfSoundStream *soundStream, bool relative) {
    soundStream->This.setRelativeToListener(relative);
}

extern "C" void sfSoundStream_setMinDistance(sfSoundStream *soundStream, float distance) {
    soundStream->This.setMinDistance(distance);
}

extern "C" void sfSoundStream_setAttenuation(sfSoundStream *soundStream, float attenuation) {
    soundStream->This.setAttenuation(attenuation);
}

extern "C" void sfSoundStream_setPlayingOffset(sfSoundStream *soundStream, int64_t timeOffset) {
    soundStream->This.setPlayingOffset(sf::microseconds(timeOffset));
}

extern "C" void sfSoundStream_setLoop(sfSoundStream *soundStream, bool loop) {
    soundStream->This.setLoop(loop);
}

extern "C" float sfSoundStream_getPitch(const sfSoundStream *soundStream) {
    return soundStream->This.getPitch();
}

extern "C" float sfSoundStream_getVolume(const sfSoundStream *soundStream) {
    return soundStream->This.getVolume();
}

extern "C" sfVector3f sfSoundStream_getPosition(const sfSoundStream *soundStream) {
    sfVector3f position = {0, 0, 0};

    sf::Vector3f sfmlPos = soundStream->This.getPosition();
    position.x = sfmlPos.x;
    position.y = sfmlPos.y;
    position.z = sfmlPos.z;

    return position;
}

extern "C" bool sfSoundStream_isRelativeToListener(const sfSoundStream *soundStream) {
    return soundStream->This.isRelativeToListener();
}

extern "C" float sfSoundStream_getMinDistance(const sfSoundStream *soundStream) {
    return soundStream->This.getMinDistance();
}

extern "C" float sfSoundStream_getAttenuation(const sfSoundStream *soundStream) {
    return soundStream->This.getAttenuation();
}

extern "C" bool sfSoundStream_getLoop(const sfSoundStream *soundStream) {
    return soundStream->This.getLoop();
}

extern "C" int64_t sfSoundStream_getPlayingOffset(const sfSoundStream *soundStream) {
    int64_t time = soundStream->This.getPlayingOffset().asMicroseconds();
    return time;
}
