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
    soundStream->play();
}

extern "C" void sfSoundStream_pause(sfSoundStream *soundStream) {
    soundStream->pause();
}

extern "C" void sfSoundStream_stop(sfSoundStream *soundStream) {
    soundStream->stop();
}

extern "C" sf::SoundStream::Status sfSoundStream_getStatus(const sfSoundStream *soundStream) {

    return soundStream->getStatus();
}

extern "C" unsigned int sfSoundStream_getChannelCount(const sfSoundStream *soundStream) {
    return soundStream->getChannelCount();
}

extern "C" unsigned int sfSoundStream_getSampleRate(const sfSoundStream *soundStream) {
    return soundStream->getSampleRate();
}

extern "C" void sfSoundStream_setPitch(sfSoundStream *soundStream, float pitch) {
    soundStream->setPitch(pitch);
}

extern "C" void sfSoundStream_setVolume(sfSoundStream *soundStream, float volume) {
    soundStream->setVolume(volume);
}

extern "C" void sfSoundStream_setPosition(sfSoundStream *soundStream, sfVector3f position) {
    soundStream->setPosition(position.x, position.y, position.z);
}

extern "C" void sfSoundStream_setRelativeToListener(sfSoundStream *soundStream, bool relative) {
    soundStream->setRelativeToListener(relative);
}

extern "C" void sfSoundStream_setMinDistance(sfSoundStream *soundStream, float distance) {
    soundStream->setMinDistance(distance);
}

extern "C" void sfSoundStream_setAttenuation(sfSoundStream *soundStream, float attenuation) {
    soundStream->setAttenuation(attenuation);
}

extern "C" void sfSoundStream_setPlayingOffset(sfSoundStream *soundStream, int64_t timeOffset) {
    soundStream->setPlayingOffset(sf::microseconds(timeOffset));
}

extern "C" void sfSoundStream_setLoop(sfSoundStream *soundStream, bool loop) {
    soundStream->setLoop(loop);
}

extern "C" float sfSoundStream_getPitch(const sfSoundStream *soundStream) {
    return soundStream->getPitch();
}

extern "C" float sfSoundStream_getVolume(const sfSoundStream *soundStream) {
    return soundStream->getVolume();
}

extern "C" sfVector3f sfSoundStream_getPosition(const sfSoundStream *soundStream) {
    sf::Vector3f pos = soundStream->getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" bool sfSoundStream_isRelativeToListener(const sfSoundStream *soundStream) {
    return soundStream->isRelativeToListener();
}

extern "C" float sfSoundStream_getMinDistance(const sfSoundStream *soundStream) {
    return soundStream->getMinDistance();
}

extern "C" float sfSoundStream_getAttenuation(const sfSoundStream *soundStream) {
    return soundStream->getAttenuation();
}

extern "C" bool sfSoundStream_getLoop(const sfSoundStream *soundStream) {
    return soundStream->getLoop();
}

extern "C" int64_t sfSoundStream_getPlayingOffset(const sfSoundStream *soundStream) {
    return soundStream->getPlayingOffset().asMicroseconds();
}
