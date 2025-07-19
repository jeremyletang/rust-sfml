#include "Audio/EffectProcessor.hpp"
#include "Audio/SoundSourceCone.hpp"
#include "Audio/SoundStatus.hpp"
#include "Audio/SoundChannel.hpp"
#include "SFML/Audio/SoundChannel.hpp"
#include "System/Vector3.hpp"
#include <SFML/Audio/SoundStream.hpp>
#include <cstdio>
#include <map>
#include <mutex>

typedef bool (*sfCustomSoundStreamGetDataCb)(sf::SoundStream::Chunk *, void *);
typedef void (*sfCustomSoundStreamSeekCb)(int64_t, void *);

class sfCustomSoundStream final : public sf::SoundStream {
  public:
    sfCustomSoundStream(sfCustomSoundStreamGetDataCb onGetData,
                        sfCustomSoundStreamSeekCb onSeek,
                        unsigned int channelCount,
                        unsigned int sampleRate,
                        const sfSoundChannel *soundChannels,
                        size_t soundChannelMapLen,
                        void *userData) : myGetDataCb(onGetData),
                                          mySeekCallCb(onSeek),
                                          myUserData(userData) {
        std::vector<sf::SoundChannel> castedSoundChannels(soundChannelMapLen);
        for (size_t i = 0; i < soundChannelMapLen; i++) {
            castedSoundChannels.push_back(static_cast<sf::SoundChannel>(soundChannels[i]));
        }
        initialize(channelCount, sampleRate, castedSoundChannels);
    }

  private:
    virtual bool onGetData(Chunk &data) final {
        return (myGetDataCb(&data, myUserData));
    }

    virtual void onSeek(sf::Time timeOffset) final {
        mySeekCallCb(timeOffset.asMicroseconds(), myUserData);
    }

    sfCustomSoundStreamGetDataCb myGetDataCb;
    sfCustomSoundStreamSeekCb mySeekCallCb;
    void *myUserData;
};

extern "C" sfCustomSoundStream *sfCustomSoundStream_new(sfCustomSoundStreamGetDataCb onGetData,
                                                        sfCustomSoundStreamSeekCb onSeek,
                                                        unsigned int channelCount,
                                                        unsigned int sampleRate,
                                                        const sfSoundChannel *channelMap,
                                                        size_t soundChannelMapLen,
                                                        void *userData) {
    return new sfCustomSoundStream(onGetData, onSeek, channelCount, sampleRate, channelMap, soundChannelMapLen, userData);
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

extern "C" sfSoundStatus sfCustomSoundStream_getStatus(const sfCustomSoundStream *soundStream) {
    return static_cast<sfSoundStatus>(soundStream->getStatus());
}

extern "C" unsigned int sfCustomSoundStream_getChannelCount(const sfCustomSoundStream *soundStream) {
    return soundStream->getChannelCount();
}

extern "C" unsigned int sfCustomSoundStream_getSampleRate(const sfCustomSoundStream *soundStream) {
    return soundStream->getSampleRate();
}

extern "C" const std::vector<sf::SoundChannel> *sfCustomSoundStream_getChannelMap(const sfCustomSoundStream *soundStream) {
    return new std::vector(soundStream->getChannelMap());
}

extern "C" void sfCustomSoundStream_setPitch(sfCustomSoundStream *soundStream, float pitch) {
    soundStream->setPitch(pitch);
}

extern "C" void sfCustomSoundStream_setPan(sfCustomSoundStream *soundStream, float pan) {
    soundStream->setPan(pan);
}

extern "C" void sfCustomSoundStream_setVolume(sfCustomSoundStream *soundStream, float volume) {
    soundStream->setVolume(volume);
}

extern "C" void sfCustomSoundStream_setSpatializationEnabled(sfCustomSoundStream *soundStream, bool enabled) {
    soundStream->setSpatializationEnabled(enabled);
}

extern "C" void sfCustomSoundStream_setPosition(sfCustomSoundStream *soundStream, sf::Vector3f position) {
    soundStream->setPosition(position);
}

extern "C" void sfCustomSoundStream_setDirection(sfCustomSoundStream *soundStream, sfVector3f position) {
    soundStream->setDirection(convertVector3(position));
}

extern "C" void sfCustomSoundStream_setCone(sfCustomSoundStream *soundStream, sfSoundSourceCone cone) {
    soundStream->setCone(convertCone(cone));
}

extern "C" void sfCustomSoundStream_setVelocity(sfCustomSoundStream *soundStream, sfVector3f velocity) {
    soundStream->setVelocity(convertVector3(velocity));
}

extern "C" void sfCustomSoundStream_setDopplerFactor(sfCustomSoundStream *soundStream, float factor) {
    soundStream->setDopplerFactor(factor);
}

extern "C" void sfCustomSoundStream_setDirectionalAttenuationFactor(sfCustomSoundStream *soundStream, float factor) {
    soundStream->setDirectionalAttenuationFactor(factor);
}

extern "C" void sfCustomSoundStream_setRelativeToListener(sfCustomSoundStream *soundStream, bool relative) {
    soundStream->setRelativeToListener(relative);
}

extern "C" void sfCustomSoundStream_setMinDistance(sfCustomSoundStream *soundStream, float distance) {
    soundStream->setMinDistance(distance);
}

extern "C" void sfCustomSoundStream_setMaxDistance(sfCustomSoundStream *soundStream, float distance) {
    soundStream->setMaxDistance(distance);
}

extern "C" void sfCustomSoundStream_setMinGain(sfCustomSoundStream *soundStream, float gain) {
    soundStream->setMinGain(gain);
}

extern "C" void sfCustomSoundStream_setMaxGain(sfCustomSoundStream *soundStream, float gain) {
    soundStream->setMaxGain(gain);
}

extern "C" void sfCustomSoundStream_setAttenuation(sfCustomSoundStream *soundStream, float attenuation) {
    soundStream->setAttenuation(attenuation);
}

extern "C" void sfCustomSoundStream_setPlayingOffset(sfCustomSoundStream *soundStream, int64_t timeOffset) {
    soundStream->setPlayingOffset(sf::microseconds(timeOffset));
}

extern "C" void sfCustomSoundStream_setLooping(sfCustomSoundStream *soundStream, bool loop) {
    soundStream->setLooping(loop);
}

extern "C" float sfCustomSoundStream_getPitch(const sfCustomSoundStream *soundStream) {
    return soundStream->getPitch();
}

extern "C" float sfCustomSoundStream_getPan(const sfCustomSoundStream *soundStream) {
    return soundStream->getPan();
}

extern "C" float sfCustomSoundStream_getVolume(const sfCustomSoundStream *soundStream) {
    return soundStream->getVolume();
}

extern "C" bool sfCustomSoundStream_isSpatializationEnabled(const sfCustomSoundStream *soundStream) {
    return soundStream->isSpatializationEnabled();
}

extern "C" sfVector3f sfCustomSoundStream_getPosition(const sfCustomSoundStream *soundStream) {
    return convertVector3(soundStream->getPosition());
}

extern "C" sfVector3f sfCustomSoundStream_getDirection(const sfCustomSoundStream *soundStream) {
    return convertVector3(soundStream->getDirection());
}

extern "C" sfSoundSourceCone sfCustomSoundStream_getCone(const sfCustomSoundStream *soundStream) {
    return convertCone(soundStream->getCone());
}

extern "C" sfVector3f sfCustomSoundStream_getVelocity(const sfCustomSoundStream *soundStream) {
    return convertVector3(soundStream->getVelocity());
}

extern "C" float sfCustomSoundStream_getDopplerFactor(const sfCustomSoundStream *soundStream) {
    return soundStream->getDopplerFactor();
}

extern "C" float sfCustomSoundStream_getDirectionalAttenuationFactor(const sfCustomSoundStream *soundStream) {
    return soundStream->getDirectionalAttenuationFactor();
}

extern "C" bool sfCustomSoundStream_isRelativeToListener(const sfCustomSoundStream *soundStream) {
    return soundStream->isRelativeToListener();
}

extern "C" float sfCustomSoundStream_getMinDistance(const sfCustomSoundStream *soundStream) {
    return soundStream->getMinDistance();
}

extern "C" float sfCustomSoundStream_getMaxDistance(const sfCustomSoundStream *soundStream) {
    return soundStream->getMaxDistance();
}

extern "C" float sfCustomSoundStream_getMinGain(const sfCustomSoundStream *soundStream) {
    return soundStream->getMinGain();
}

extern "C" float sfCustomSoundStream_getMaxGain(const sfCustomSoundStream *soundStream) {
    return soundStream->getMaxGain();
}

extern "C" float sfCustomSoundStream_getAttenuation(const sfCustomSoundStream *soundStream) {
    return soundStream->getAttenuation();
}

extern "C" bool sfCustomSoundStream_isLooping(const sfCustomSoundStream *soundStream) {
    return soundStream->isLooping();
}

extern "C" int64_t sfCustomSoundStream_getPlayingOffset(const sfCustomSoundStream *soundStream) {
    return soundStream->getPlayingOffset().asMicroseconds();
}

static std::map<sfCustomSoundStream *, std::pair<sfEffectProcessor, void *>> processors;
static std::mutex processorMutex;

extern "C" void sfCustomSoundStream_setEffectProcessor(sfCustomSoundStream *soundStream, sfEffectProcessor effectProcessor, void *userData) {
    std::unique_lock<std::mutex> lock(processorMutex);
    if (!effectProcessor) {
        processors.erase(soundStream);
        soundStream->setEffectProcessor(nullptr);
    } else {
        processors[soundStream] = {effectProcessor, userData};
        soundStream->setEffectProcessor(
            [soundStream](const float *inputFrames,
                          unsigned int &inputFrameCount,
                          float *outputFrames,
                          unsigned int &outputFrameCount,
                          unsigned int frameChannelCount) {
                std::unique_lock<std::mutex> lock(processorMutex);
                auto it = processors.find(soundStream);
                if (it != processors.end()) {
                    it->second.first(inputFrames, &inputFrameCount, outputFrames, &outputFrameCount, frameChannelCount, it->second.second);
                }
            });
    }
}

extern "C" void sfCustomSoundStream_del(sfCustomSoundStream *music) {
    sfCustomSoundStream_setEffectProcessor(music, nullptr, nullptr);
    delete music;
}
