#include "System/Vector3.h"
#include <SFML/Audio/SoundStream.hpp>
#include <cstddef>
#include <cstdint>

typedef bool (*sfCustomSoundStreamGetDataCallback)(sf::SoundStream::Chunk *, void *);
typedef void (*sfCustomSoundStreamSeekCallback)(int64_t, void *);

class sfCustomSoundStream : public sf::SoundStream {
  public:
    sfCustomSoundStream(sfCustomSoundStreamGetDataCallback onGetData,
                        sfCustomSoundStreamSeekCallback onSeek,
                        unsigned int channelCount,
                        unsigned int sampleRate,
                        void *userData) : myGetDataCallback(onGetData),
                                          mySeekCallback(onSeek),
                                          myUserData(userData) {
        initialize(channelCount, sampleRate);
    }

  private:
    virtual bool onGetData(Chunk &data) {
        return (myGetDataCallback(&data, myUserData));
    }

    virtual void onSeek(sf::Time timeOffset) {
        if (mySeekCallback) {
            int64_t time = {timeOffset.asMicroseconds()};
            mySeekCallback(time, myUserData);
        }
    }

    sfCustomSoundStreamGetDataCallback myGetDataCallback;
    sfCustomSoundStreamSeekCallback mySeekCallback;
    void *myUserData;
};

extern "C" sfCustomSoundStream *sfCustomSoundStream_create(sfCustomSoundStreamGetDataCallback onGetData,
                                                           sfCustomSoundStreamSeekCallback onSeek,
                                                           unsigned int channelCount,
                                                           unsigned int sampleRate,
                                                           void *userData) {
    return new sfCustomSoundStream(onGetData, onSeek, channelCount, sampleRate, userData);
}

extern "C" void sfCustomSoundStream_destroy(sfCustomSoundStream *soundStream) {
    delete soundStream;
}

extern "C" void sfCustomSoundStream_play(sfCustomSoundStream *soundStream) {
    soundStream->play();
}

extern "C" void sfCustomSoundStream_pause(sfCustomSoundStream *soundStream) {
    soundStream->pause();
}

extern "C" void sfCustomSoundStream_stop(sfCustomSoundStream *soundStream) {
    soundStream->stop();
}

extern "C" sf::SoundStream::Status sfCustomSoundStream_getStatus(const sfCustomSoundStream *soundStream) {

    return soundStream->getStatus();
}

extern "C" unsigned int sfCustomSoundStream_getChannelCount(const sfCustomSoundStream *soundStream) {
    return soundStream->getChannelCount();
}

extern "C" unsigned int sfCustomSoundStream_getSampleRate(const sfCustomSoundStream *soundStream) {
    return soundStream->getSampleRate();
}

extern "C" void sfCustomSoundStream_setPitch(sfCustomSoundStream *soundStream, float pitch) {
    soundStream->setPitch(pitch);
}

extern "C" void sfCustomSoundStream_setVolume(sfCustomSoundStream *soundStream, float volume) {
    soundStream->setVolume(volume);
}

extern "C" void sfCustomSoundStream_setPosition(sfCustomSoundStream *soundStream, sfVector3f position) {
    soundStream->setPosition(position.x, position.y, position.z);
}

extern "C" void sfCustomSoundStream_setRelativeToListener(sfCustomSoundStream *soundStream, bool relative) {
    soundStream->setRelativeToListener(relative);
}

extern "C" void sfCustomSoundStream_setMinDistance(sfCustomSoundStream *soundStream, float distance) {
    soundStream->setMinDistance(distance);
}

extern "C" void sfCustomSoundStream_setAttenuation(sfCustomSoundStream *soundStream, float attenuation) {
    soundStream->setAttenuation(attenuation);
}

extern "C" void sfCustomSoundStream_setPlayingOffset(sfCustomSoundStream *soundStream, int64_t timeOffset) {
    soundStream->setPlayingOffset(sf::microseconds(timeOffset));
}

extern "C" void sfCustomSoundStream_setLoop(sfCustomSoundStream *soundStream, bool loop) {
    soundStream->setLoop(loop);
}

extern "C" float sfCustomSoundStream_getPitch(const sfCustomSoundStream *soundStream) {
    return soundStream->getPitch();
}

extern "C" float sfCustomSoundStream_getVolume(const sfCustomSoundStream *soundStream) {
    return soundStream->getVolume();
}

extern "C" sfVector3f sfCustomSoundStream_getPosition(const sfCustomSoundStream *soundStream) {
    sf::Vector3f pos = soundStream->getPosition();
    return {pos.x, pos.y, pos.z};
}

extern "C" bool sfCustomSoundStream_isRelativeToListener(const sfCustomSoundStream *soundStream) {
    return soundStream->isRelativeToListener();
}

extern "C" float sfCustomSoundStream_getMinDistance(const sfCustomSoundStream *soundStream) {
    return soundStream->getMinDistance();
}

extern "C" float sfCustomSoundStream_getAttenuation(const sfCustomSoundStream *soundStream) {
    return soundStream->getAttenuation();
}

extern "C" bool sfCustomSoundStream_getLoop(const sfCustomSoundStream *soundStream) {
    return soundStream->getLoop();
}

extern "C" int64_t sfCustomSoundStream_getPlayingOffset(const sfCustomSoundStream *soundStream) {
    return soundStream->getPlayingOffset().asMicroseconds();
}
